use uuid::Uuid;
use axum::{
    http::StatusCode,
    Extension,
    Router,
    routing::{get, post},
    extract::Form,
};
use validator::Validate;
use serde::{Serialize, Deserialize};
use sqlx::postgres::types::PgInterval;

use crate::{Result, Error};
use crate::http::defaults::{default_time, default_uuid};

#[derive(sqlx::FromRow, Debug, Deserialize, Serialize, Validate)]
pub struct SessionForm {
    #[serde(default = "default_uuid")]
    pub id: Uuid,
    pub author_id: Uuid,
    #[validate(length(min=1, max=50))]
    pub title: String,
    pub description: String,
    pub location: String,
    pub tier: i16,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub end_time: chrono::DateTime<chrono::Utc>,
    #[serde(default, with="crate::http::pg_interval")]
    pub recurrence: Option<PgInterval>,
    pub recurrence_end: Option<chrono::DateTime<chrono::Utc>>,
    pub user_limit: Option<i16>,
    #[serde(default = "default_time")]
    pub created_at: chrono::DateTime<chrono::Utc>
}

pub fn router() -> Router {
    Router::new().route("/v1/create-session", post(create_session))
}

async fn create_session(pool: Extension<sqlx::PgPool>, Form(payload): Form<SessionForm>) -> Result<StatusCode> {
    Ok(StatusCode::OK)
}
