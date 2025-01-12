use axum::{middleware::from_fn, Router};

mod defaults;
mod pg_interval;
mod sessions;
mod token;
mod users;

pub use self::token::AuthError;
pub use self::users::{get_members, User};
use crate::Result;

pub fn router_app(db: sqlx::PgPool) -> Router {
    let user_router = Router::new().merge(token::router()).merge(users::router());
    let v1_routes = Router::new()
        .nest("/sessions", sessions::router())
        .layer(from_fn(token::mid_jwt_auth)) // all routes above are protected
        .nest("/users", user_router);
    Router::new().nest("/api/v1", v1_routes).with_state(db)
}

pub async fn serve(db: sqlx::PgPool) -> Result<()> {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, router_app(db)).await;
    Ok(())
}
