#[derive(sqlx::FromRow, Debug)]
pub struct Admin {
    pub id: uuid::Uuid,
    pub username: String,
    pub password: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub last_login: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(sqlx::FromRow, Debug)]
pub struct Form {
    pub id: uuid::Uuid,
    pub author_id: uuid::Uuid,
    pub title: String,
    pub subtitle: String,
    pub description: String,
    pub sub_limit: Option<i16>,
    pub immutable: bool,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}
