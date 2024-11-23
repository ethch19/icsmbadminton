use derive_more::From;
use axum:: {
    response::{IntoResponse, Response},
    http::StatusCode,
};
use sqlx::migrate::MigrateError;
use crate::http::AuthError;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    Conflict(String),

    UnprocessableEntity(String),

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
        let body = match self {
            Error::Validator(_) => "Validation error",
            _ => "Exception error",
        };

        println!("API error: {self:?}"); // Log it by serialising to json file

        (self.status_code(), body).into_response()
    }
}

impl Error {
    fn status_code(&self) -> StatusCode {
        use Error::*;

        match self {
            Sqlx(_) | Migrate(_) | PasswordHash(_) | DotEnv(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Validator(_) | UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Auth(_) => StatusCode::FORBIDDEN,
            Conflict(_) => StatusCode::CONFLICT,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
