use uuid::Uuid;
use serde::{Serialize, Deserialize};
use axum::{
    http::StatusCode,
    Extension,
    Router,
    routing::{get, post},
    extract::Json,
};
use validator::Validate;

use crate::{Result, Error};

#[derive(sqlx::FromRow, Debug, Deserialize, Serialize, Validate)]
pub struct SessionForm {
    pub id: Uuid,
    pub author_id: Uuid,
    #[validate(length(min=1, max=50))]
    pub title: String,
    pub description: String,
    pub location: String,
    pub tier: i16;
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub end_time: chrono::DateTime<chrono::Utc>,
    pub recurrence: Option<sqlx::postgres::types::PgInterval>,
    pub recurrence_end: Option<chrono::DateTime<chrono::Utc>>,
    pub user_limit: Option<i16>,
    pub created_at: chrono::DateTime<chrono::Utc>
}

