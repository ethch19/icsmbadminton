use axum::{Router, routing::get};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    let pool = PgPoolOptions::new().max_connections(5).connect("postgres://postgres:'j92kloa00s@1s'@localhost:5432/icsmbdmt").await?;

    let 
}
