use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    typed_header::{TypedHeader, TypedHeaderRejection},
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use rand::Rng;
use serde::{Deserialize, Serialize};
use tracing::{error, instrument};

use crate::{Error, Result};

pub fn router() -> Router<sqlx::PgPool> {
    Router::new()
        .route("/login", post(authenticate))
        .route("/refresh", get(refresh_token))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessClaims {
    pub sub: String, // subject (userid)
    pub exp: usize,  // expiration
    pub user_id: uuid::Uuid,
    pub name: String, // full name
    pub tier: i16,    // 0 = user, 1 = member, 2 = team
    pub admin: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshClaims {
    pub sub: String,
    pub exp: usize,
    pub jti: uuid::Uuid,
    pub user_id: uuid::Uuid,
}

#[derive(Debug, Serialize)]
struct AuthBody {
    access_token: String,
    refresh_token: Option<String>,
    token_type: String,
}

impl AuthBody {
    fn new(access_token: String, refresh_token: Option<String>) -> Self {
        Self {
            access_token,
            refresh_token,
            token_type: "Bearer ".to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct AuthPayload {
    shortcode: String,
    password: String,
    keep_login: bool,
}

#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}

struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

const ACCESS_KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = dotenvy::var("ACCESS_JWT_SECRET").expect("JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});

const REFRESH_KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = dotenvy::var("REFRESH_JWT_SECRET").expect("JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});

#[instrument(name = "auth_via_login", level = "TRACE")]
async fn authenticate(
    State(pool): State<sqlx::PgPool>,
    Json(payload): Json<AuthPayload>,
) -> Result<impl IntoResponse> {
    if payload.shortcode.is_empty() || payload.password.is_empty() {
        error!(
        return Err(Error::from(AuthError::MissingCredentials));
    }

    let selected_user = sqlx::query_as!(
        crate::http::User,
        r#"
		SELECT * FROM auth.users WHERE shortcode = $1
		"#,
        &payload.shortcode,
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| AuthError::WrongCredentials)?;

    let parsed_hash =
        PasswordHash::new(&selected_user.password).map_err(|_| AuthError::WrongCredentials)?;

    if Argon2::default()
        .verify_password(payload.password.as_bytes(), &parsed_hash)
        .is_ok()
    {
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(1))
            .expect("valid timestamp")
            .timestamp();

        let claims = AccessClaims {
            sub: selected_user.shortcode.clone(),
            exp: expiration as usize,
            user_id: selected_user.id,
            name: selected_user.first_name + &selected_user.surname,
            tier: selected_user.tier,
            admin: selected_user.admin,
        };

        let access_token = encode(&Header::default(), &claims, &ACCESS_KEYS.encoding)
            .map_err(|_| AuthError::TokenCreation)?;

        if payload.keep_login {
            let expiration = Utc::now()
                .checked_add_signed(Duration::weeks(12))
                .expect("valid timestamp")
                .timestamp();

            let jti = crate::http::defaults::default_uuid();
            let _ = sqlx::query!(
                "UPDATE auth.users SET jti = $1 WHERE id = $2",
                jti,
                selected_user.id
            )
            .execute(&pool)
            .await
            .map_err(|_| AuthError::TokenCreation)?;

            let refresh_claims = RefreshClaims {
                sub: selected_user.shortcode,
                exp: expiration as usize,
                user_id: selected_user.id,
                jti,
            };

            let refresh_token = encode(&Header::default(), &refresh_claims, &REFRESH_KEYS.encoding)
                .map_err(|_| AuthError::TokenCreation)?;

            return Ok((
                StatusCode::OK,
                Json(AuthBody::new(access_token, Some(refresh_token))),
            )
                .into_response());
        }
        return Ok((StatusCode::OK, Json(AuthBody::new(access_token, None))).into_response());
    }
    let rand_sleep = rand::thread_rng()
        .gen_range(std::time::Duration::from_millis(100)..=std::time::Duration::from_millis(500));
    tokio::time::sleep(rand_sleep).await;
    Err(Error::from(AuthError::WrongCredentials))
}

#[instrument(level = "trace", skip_all, fields(token))]
async fn refresh_token(
    headers: HeaderMap,
    State(pool): State<sqlx::PgPool>,
) -> Result<impl IntoResponse> {
    if let Some(auth_header) = headers.get("Authorization") {
        let auth_val = auth_header.to_str().map_err(|e| {
            error!(name: "invalid_header", "Cannot convert auth header to string: {}", e);
            AuthError::MissingCredentials
        })?;
        if let Some(auth_tuple) = auth_val.split_once(' ') {
            if auth_tuple.0 == "Bearer" {
                let token = auth_tuple.1;
                let token_data = decode::<RefreshClaims>(
                    &token,
                    &REFRESH_KEYS.decoding,
                    &Validation::default(),
                )
                .map_err(|e| {
                    error!(name: "token_decoding_error", "Cannot decode token into claims: {}", e);
                    AuthError::InvalidToken
                })?;

                let selected_user = sqlx::query_as!(
                    crate::http::User,
                    "SELECT * FROM auth.users WHERE id = $1",
                    &token_data.claims.user_id
                )
                .fetch_one(&pool)
                .await
                .map_err(|e| {
                    error!(name: "db_error", "Cannot fetch corresponding jti from db: {}", e);
                    AuthError::TokenCreation
                })?;
                if let Some(jwt_id) = selected_user.jti {
                    if jwt_id == token_data.claims.jti {
                        let expiration = Utc::now()
                            .checked_add_signed(Duration::hours(1))
                            .expect("valid timestamp")
                            .timestamp();

                        let claims = AccessClaims {
                            sub: selected_user.shortcode.clone(),
                            exp: expiration as usize,
                            user_id: selected_user.id,
                            name: selected_user.first_name + &selected_user.surname,
                            tier: selected_user.tier,
                            admin: selected_user.admin,
                        };

                        let access_token =
                            encode(&Header::default(), &claims, &ACCESS_KEYS.encoding)
                                .map_err(|e| {
                                    error!(name: "token_encoding_error", "Problem creating new access token: {}", e);
                                    AuthError::TokenCreation
                                })?;

                        let expiration = Utc::now()
                            .checked_add_signed(Duration::weeks(12))
                            .expect("valid timestamp")
                            .timestamp();

                        let jti = crate::http::defaults::default_uuid();
                        let _ = sqlx::query!(
                            "UPDATE auth.users SET jti = $1 WHERE id = $2",
                            jti,
                            selected_user.id
                        )
                        .execute(&pool)
                        .await
                        .map_err(|e| {
                            error!(name: "db_error", "Error when updating jti in db: {}", e);
                            AuthError::TokenCreation
                        })?;

                        let refresh_claims = RefreshClaims {
                            sub: selected_user.shortcode,
                            exp: expiration as usize,
                            user_id: selected_user.id,
                            jti,
                        };

                        let refresh_token =
                            encode(&Header::default(), &refresh_claims, &REFRESH_KEYS.encoding)
                                .map_err(|e| {
                                    error!(name: "token_encoding_error", "Problem when encoding new refresh token: {}",e);
                                    AuthError::TokenCreation
                                })?;

                        return Ok((
                            StatusCode::OK,
                            Json(AuthBody::new(access_token, Some(refresh_token))),
                        )
                            .into_response());
                    }
                }
                error!(name: "exception_error", "User does not have jti stored in db");
                return Err(Error::from(AuthError::TokenCreation));
            }
        }
        error!(name: "exception_error", "No bearer found in header");
        return Err(Error::from(AuthError::MissingCredentials));
    }
    error!(name: "exception_error", "No authentication header found");
    Err(Error::from(AuthError::MissingCredentials))
}

#[instrument(level = "trace", skip(req, next))]
pub async fn mid_jwt_auth(
    header: std::result::Result<TypedHeader<Authorization<Bearer>>, TypedHeaderRejection>,
    mut req: Request,
    next: Next,
) -> Result<Response> {
    match header {
        Ok(TypedHeader(Authorization(bearer))) => {
            let token_data = decode::<AccessClaims>(
                bearer.token(),
                &ACCESS_KEYS.decoding,
                &Validation::default(),
            )
            .map_err(|e| {
                error!(name: "token_decoding_error", "Cannot decode token into claims: {}", e);
                AuthError::InvalidToken
            })?;
            req.extensions_mut().insert(token_data.claims);
            Ok(next.run(req).await)
        }
        Err(_) => {
            error!(name: "exception_error", "Authorization header not found");
            Err(Error::from(AuthError::MissingCredentials))
        }
    }
}
