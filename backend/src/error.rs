use thiserror:Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("DataStore error")]
    Sqlx(#[from] sqlx::Error),

    #[error("{0}")]
    Conflict(String),

    #[error("DB: Row not found")]
    NotFound(#[from] sqlx::Error::RowNotFound),

    #[error("Internal error")]
    Anyhow(#[from] anyhow::Error),

    #[error("Validation error")]
    Validation(#[from] validator::ValidationError),

    #[error("Hashing error")]
    Hash(#[from] argon2::Error),

    #[error("Invalid error")]
    Invalid(String),

    #[error("Env error")]
    Env(#[from] dotenvy::Error),

    #[error("Axum server error")]
    AxumServer(#[from] ,

    #[error("Unknown error")]
    Unknown,
}
