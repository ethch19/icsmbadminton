use axum::{Extension, Router};

mod users;
mod token;

pub use self::users::{User, get_members};
pub use self::token::{Claims, AuthError};
use crate::Result;

pub fn app(db: sqlx::PgPool) -> Router {
    Router::new()
        .merge(users::router())
        .merge(token::router())
        .layer(Extension(db))
    }

pub async fn serve(db: sqlx::PgPool) -> Result<()> {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app(db)).await;
    Ok(())
}
