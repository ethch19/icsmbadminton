use axum::{Extension, Router};

mod admin;

pub use self::admin::Admin;
use crate::error::Error;

pub fn app(db: sqlx::PgPool) -> Router {
    Router::new()
        .merge(admin::router())
        .layer(Extension(db))
    }

pub async fn serve(db: sqlx::PgPool) -> Result<(), Error> {
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app(db).into_make_service())
        .await
        .map_err(Error::AxumServer);
    }
}
