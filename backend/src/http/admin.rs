use anyhow::{Result};
use validator::{Validate, ValidationError};
use regex::Regex;
use std::time::Duration;
use once_cell::sync::Lazy;
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
}
use serde::{Serialize, Deserialize};
use axum::{
    Json
    Extension,
    Router,
    routing::post,
}

use error::Error;

static USERNAME_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[0-9A-Za-z_]+$").unwrap());
static PASSWORD_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(?=.*[A-Z])(?=.*[a-z])(?=.*[0-9])[A-Za-z\d]*$".unwrap());

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug)]
pub struct Admin {
    pub id: Option<uuid::Uuid>,
    #[validate(length(min=5, max=20), regex="USERNAME_REGEX")]
    pub username: String,
    #[validate(length(min=8, max=32), regex="PASSWORD_REGEX")]
    pub password: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub last_login: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug)]
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

pub fn router() -> Router {
    Router::new().route("/v1/user", post(Admin::new))
}

impl Admin {
    pub async fn get_all(pool: sqlx::PgPool) -> Result<Vec<Admin>, Error> { 
        sqlx::query_as!(Admin,
            r#"
            SELECT * FROM auth.admins;
            "#
        )
            .fetch_all(&pool)
            .await
    }

    pub async fn new(pool: Extension<sqlx::PgPool>, Json(body): Json<Admin>) -> Result<(), Error> {
        body.validate().map_err(Error::Validation)?;
    
        let Admin { username, password, .. } = body;

        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2.hash_password(password, &salt)?.to_string();

        sqlx::query!(
            r#"
            INSERT INTO auth.admins(username, password)
            VALUES ($1, $2)
            "#,
            username,
            password_hash
        )
        .execute(&*pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(dbe) if dbe.constraint() == Some("auth_admins_username_key") => Error::Conflict("Username already taken".into())
            _ => e.into(),
        })?;

        Ok(())
    }

    pub async fn verify(self, pool: sqlx::PgPool) -> Result<(), Error> {
        self.validate().map_err(Error::Validation)?;

        let parsed_hash = PasswordHash::new(&user.password).map_err(Error::Hash)?;

        let optional_user = sqlx::query!(
            r#"
            SELECT password FROM auth.admins WHERE username = $1
            "#,
            self.username
        )
        .fetch_optional(&pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => Error::NotFound,
            _ => e.into(),
        })?;

        if let Some(u) = optional_user {
            if u == parsed_hash {
                println!("Login successful for user[{}]", self.username);
                Ok(())
            } else {
                println!("Login failed for user[{}]", self.username);
            }
        }

        //Prevent validation leak from runtime checks
        let rand_sleep = rand::thread_rng().gen_range(Duration::from_millis(100)..=Duration::from_millis(500));
        tokio::time::sleep(rand_sleep).await?;

        Err(Error::Invalid("Invalid username/password".into()))
    }
}
