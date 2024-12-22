use axum::{Extension, Router, middleware::from_fn};
use tower::ServiceBuilder;

mod users;
mod token;
mod sessions;
mod pg_interval;
mod defaults;

pub use self::users::{User, get_members};
pub use self::token::{Claims, AuthError};
use crate::Result;

pub fn app(db: sqlx::PgPool) -> Router {
    Router::new()
        .merge(users::router())
        .merge(token::router())
        .merge(sessions::router())
        .layer(
            ServiceBuilder::new()
                .layer(from_fn(token::mid_jwt_auth))
                .layer(Extension(db))
        )
}

pub async fn serve(db: sqlx::PgPool) -> Result<()> {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app(db)).await;
    Ok(())
}
