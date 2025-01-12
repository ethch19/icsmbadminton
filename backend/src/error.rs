use crate::http::AuthError;
use axum::{
    extract::Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use derive_more::From;
use sqlx::migrate::MigrateError;
use tracing::{event, Level};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    Conflict(String),

    UnprocessableEntity(String),

    #[from]
    Auth(AuthError),

    NotFoundTeamMembership(String),

    AddingTeam(String),

    // -- Externals
    #[from]
    Sqlx(sqlx::Error),

    #[from]
    Migrate(MigrateError),

    #[from]
    Validator(validator::ValidationErrors),

    #[from]
    PasswordHash(argon2::password_hash::Error),

    #[from]
    Reqwest(reqwest::Error),

    #[from]
    InvalidHeader(reqwest::header::InvalidHeaderValue),

    #[from]
    DotEnv(dotenvy::Error),
}

// region:    --- Error Boilerplate

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

// endregion: --- Error Boilerplate

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        use Error::*;

        let body = match &self {
            Conflict(v) | UnprocessableEntity(v) | NotFoundTeamMembership(v) | AddingTeam(v) => {
                format!("{:?}", v)
            }
            Auth(e) => format!("{:?}", e),
            Sqlx(e) => format!("{:?}", e),
            Validator(e) => format!("{:?}", e),
            PasswordHash(e) => format!("{:?}", e),
            Reqwest(e) => format!("{:?}", e),
            InvalidHeader(e) => format!("{:?}", e),
            DotEnv(e) => format!("{:?}", e),
            _ => String::from("Unknown error thrown"),
        };
        event!(Level::ERROR, body);
        (self.status_code(), Json(body)).into_response()
    }
}

impl Error {
    fn status_code(&self) -> StatusCode {
        use Error::*;

        match self {
            Sqlx(_) | PasswordHash(_) | DotEnv(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Validator(_) | UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Conflict(_) => StatusCode::CONFLICT,
            Auth(AuthError::WrongCredentials) => StatusCode::UNAUTHORIZED,
            Auth(AuthError::MissingCredentials | AuthError::InvalidToken) | InvalidHeader(_) => {
                StatusCode::BAD_REQUEST
            }
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
