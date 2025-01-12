use axum::{
    extract::{State, Form},
    http::StatusCode,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::postgres::types::PgInterval;
use uuid::Uuid;
use validator::Validate;

use crate::http::defaults::{default_time, default_uuid};
use crate::{Error, Result};

#[derive(sqlx::FromRow, Debug, Deserialize, Serialize, Validate)]
pub struct SessionForm {
    #[serde(default = "default_uuid")]
    pub id: Uuid,
    pub author_id: Uuid,
    #[validate(length(min = 1, max = 50))]
    pub title: String,
    pub description: String,
    pub location: String,
    pub tier: i16,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub end_time: chrono::DateTime<chrono::Utc>,
    #[serde(default, with = "crate::http::pg_interval")]
    pub recurrence: Option<PgInterval>,
    pub recurrence_end: Option<chrono::DateTime<chrono::Utc>>,
    pub user_limit: Option<i16>,
    #[serde(default = "default_time")]
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub fn router() -> Router<sqlx::PgPool> {
    Router::new().route("/create", post(create_session))
}

async fn create_session(
    State(pool): State<sqlx::PgPool>,
    Form(payload): Form<SessionForm>,
) -> Result<StatusCode> {
    Ok(StatusCode::OK)
}
