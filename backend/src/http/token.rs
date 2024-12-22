use argon2::{Argon2, PasswordHash, PasswordVerifier};
use serde::{Serialize, Deserialize};
use serde_json::json;
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use chrono::{Utc, Duration};
use axum::{
	extract::{Request, FromRequestParts},
	http::{request::Parts, StatusCode},
	response::{Response, IntoResponse},
	routing::{post, get},
	middleware::{from_fn, Next},
	Json, RequestPartsExt, async_trait, Extension, Router,
};
use axum_extra::{
	headers::{authorization::Bearer, Authorization},
    typed_header::{TypedHeader, TypedHeaderRejection},
};
use rand::Rng;
use once_cell::sync::Lazy;

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
	pub sub: String, // subject of token
	pub exp: usize, // expiration
	pub roles: Vec<String>, // roles
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
	S: Send + Sync,
{
	type Rejection = AuthError;

	async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
		let TypedHeader(Authorization(bearer)) = parts
			.extract::<TypedHeader<Authorization<Bearer>>>()
			.await
			.map_err(|_| AuthError::InvalidToken)?;
		let token_data = decode::<Claims>(bearer.token(), &ACCESS_KEYS.decoding, &Validation::default())
			.map_err(|_| AuthError::InvalidToken)?;
		Ok(token_data.claims)
	}
}

#[derive(Debug, Serialize)]
struct AuthBody {
	token: String,
	token_type: String,
}

impl AuthBody {
	fn new(token: String) -> Self {
		Self {
			token,
			token_type: "Bearer ".to_string(),
		}
	}
}

#[derive(Debug, Serialize)]
struct LongAuthBody {
	access_token: String,
	refresh_token: String,
	token_type: String,
}

impl LongAuthBody {
	fn new(access_token: String, refresh_token: String) -> Self {
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
	keep_login: bool
}

#[derive(Debug)]
pub enum AuthError {
	WrongCredentials,
	MissingCredentials,
	TokenCreation,
	InvalidToken,
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

pub fn router() -> Router {
	Router::new()
		.route("/v1/login", post(authorize))
		.route("/v1/refresh", get(get_access_token))
}

fn generate_jwt(shortcode: &str, roles: &Vec<String>, expiration: i64, keys: &Lazy<Keys>) -> Result<String, AuthError> {
	let expiration = Utc::now()
		.checked_add_signed(Duration::days(expiration))
		.expect("valid timestamp")
		.timestamp();

	let claims = Claims {
		sub: shortcode.to_owned(),
		exp: expiration as usize,
		roles: roles.clone()
	};

	encode(&Header::default(), &claims, &keys.encoding).map_err(|_| AuthError::TokenCreation)
}


async fn get_access_token(TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>) -> Result<Json<AuthBody>, AuthError> {
	let token_data = decode::<Claims>(bearer.token(), &REFRESH_KEYS.decoding, &Validation::default()).map_err(|_| AuthError::InvalidToken)?;
	let claims = token_data.claims;
	let access_token = generate_jwt(&claims.sub, &claims.roles, 1, &ACCESS_KEYS)?;
	Ok(Json(AuthBody::new(access_token)))
}

async fn authorize(pool: Extension<sqlx::PgPool>, Json(payload): Json<AuthPayload>) -> Result<impl IntoResponse, AuthError> {
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
		let mut roles: Vec<String> = Vec::new();
		if selected_user.admin {
			roles.push(selected_user.admin.to_string());
		}
		match selected_user.tier {
			1 => roles.push(String::from("Member")),
			2 => roles.push(String::from("Team")),
			_ => (),
		}
		let access_token = generate_jwt(&payload.shortcode, &roles, 1, &ACCESS_KEYS)?;
		if payload.keep_login {
			let refresh_token = generate_jwt(&payload.shortcode, &roles, 30, &REFRESH_KEYS)?;
			return Ok((StatusCode::OK, Json(LongAuthBody::new(access_token, refresh_token))).into_response());
		}
		return Ok((StatusCode::OK, Json(AuthBody::new(access_token))).into_response());
	}
	let rand_sleep = rand::thread_rng().gen_range(std::time::Duration::from_millis(100)..=std::time::Duration::from_millis(500));
	tokio::time::sleep(rand_sleep).await;
	Err(AuthError::WrongCredentials)
}

pub async fn mid_jwt_auth(header: Result<TypedHeader<Authorization<Bearer>>, TypedHeaderRejection>, req: Request, next: Next) -> Result<Response, AuthError> {
	match header {
		Ok(TypedHeader(Authorization(bearer))) => {
			if req.uri() == "/v1/refresh" {
				let response = next.run(req).await;
				return Ok(response);
			}
			let token_data = decode::<Claims>(bearer.token(), &ACCESS_KEYS.decoding, &Validation::default()).map_err(|e| {
				eprintln!("Decoding error: {}", e);
				AuthError::InvalidToken
			})?;
			let response = next.run(req).await;
			Ok(response)
		}
		Err(_) => {
            Err(AuthError::InvalidToken)
		}
	}
}
