use argon2::{Argon2, PasswordHash, PasswordVerifier};
use serde::{Serialize, Deserialize};
use serde_json::json;
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use chrono::{Utc, Duration};
use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{Response, IntoResponse},
    routing::{post, get},
    Json, RequestPartsExt, async_trait, Extension, Router,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use once_cell::sync::Lazy;

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[derive(Debug, Serialize)]
struct AuthBody {
    access_token: String,
    token_type: String,
}

#[derive(Debug, Deserialize)]
struct AuthPayload {
    shortcode: String,
    password: String,
    token: bool,
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

const KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = dotenvy::var("JWT_SECRET").expect("JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});

pub fn router() -> Router {
    Router::new()
        .route("/v1/login", post(authorize))
        .route("/v1/protected", get(protected))
}

fn generate_jwt(username: &str) -> Result<String, AuthError> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::days(1))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: username.to_owned(),
        exp: expiration as usize,
    };

    encode(&Header::default(), &claims, &KEYS.encoding).map_err(|_| AuthError::TokenCreation)
}

fn verify_jwt(token: &str) -> bool {
    decode::<Claims>(token, &KEYS.decoding, &Validation::default()).is_ok()
}

async fn protected(claims: Claims) -> Result<String, AuthError> {
    Ok(String::from("Welcome to the protected area"))
}

async fn authorize(pool: Extension<sqlx::PgPool>, Json(payload): Json<AuthPayload>) -> Result<impl IntoResponse, AuthError> {
    // Check if the user sent the credentials
    if payload.shortcode.is_empty() || payload.password.is_empty() {
        return Err(AuthError::MissingCredentials);
    }

    let selected_user = sqlx::query_as!(
        crate::http::User,
        r#"
        SELECT * FROM auth.users WHERE shortcode = $1
        "#,
        &payload.shortcode,
    )
    .fetch_one(&*pool)
    .await
    .map_err(|_| AuthError::WrongCredentials)?;

    let parsed_hash = PasswordHash::new(&selected_user.password).map_err(|_| AuthError::WrongCredentials)?;
    if Argon2::default().verify_password(payload.password.as_bytes(), &parsed_hash).is_ok() {
        if payload.token {
            // Create the authorization token
            let token = generate_jwt(&payload.shortcode)?;
            // Send the authorized token
            return Ok((StatusCode::OK, Json(AuthBody::new(token))).into_response());
        }
        return Ok(StatusCode::OK.into_response());
    }
    Err(AuthError::WrongCredentials)
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

impl AuthBody {
    fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;
        // Decode the user data
        let token_data = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
            .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}
