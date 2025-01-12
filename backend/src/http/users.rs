use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use axum::http::StatusCode;
use axum::{
    extract::{Json, Query, State},
    response::{IntoResponse, Response},
    routing::post,
    Router,
};
use once_cell::sync::Lazy;
use regex::Regex;
use reqwest::header;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use validator::Validate;

use crate::http::defaults::{default_time, default_uuid};
use crate::{Error, Result};

static USERNAME_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[0-9A-Za-z_]+$").unwrap());
static PASSWORD_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^.(.*[A-Za-z0-9])(.*\d).+$").unwrap());
static NAME_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[A-Za-z]+$").unwrap());

#[derive(sqlx::FromRow, Debug, Validate, Deserialize, Serialize)]
pub struct PendingUser {
    #[serde(default = "default_uuid")]
    pub id: uuid::Uuid,
    #[serde(default = "default_uuid")]
    pub verification_token: uuid::Uuid,
    #[validate(length(min=1, max=20), regex(path = *NAME_REGEX))]
    pub first_name: String,
    #[validate(length(min=1, max=20), regex(path = *NAME_REGEX))]
    pub surname: String,
    pub shortcode: String,
    pub cid: String,
    #[validate(length(min=8, max=32), regex(path = *PASSWORD_REGEX))]
    pub password: String,
    #[serde(default = "default_time")]
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(sqlx::FromRow, Debug, Validate, Deserialize, Serialize)]
pub struct User {
    #[serde(default = "default_uuid")]
    pub id: uuid::Uuid,
    #[validate(length(min=1, max=20), regex(path = *NAME_REGEX))]
    pub first_name: String,
    #[validate(length(min=1, max=20), regex(path = *NAME_REGEX))]
    pub surname: String,
    pub shortcode: String,
    pub cid: String,
    #[validate(length(min=8, max=32), regex(path = *PASSWORD_REGEX))]
    pub password: String,
    pub admin: bool,
    pub tier: i16,
    pub jti: Option<uuid::Uuid>,
    #[serde(default = "default_time")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_login: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct UserAuth {
    pub shortcode: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct VerificationToken {
    pub token: uuid::Uuid,
}

pub fn router() -> Router<sqlx::PgPool> {
    Router::new()
        .route("/register", post(PendingUser::create))
        .route("/verify", post(PendingUser::verify))
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Member {
    first_name: String,
    surname: String,
    cid: String,
    email: String,
    login: String,
    order_no: i32,
    member_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct TeamMember {
    first_name: String,
    surname: String,
    cid: String,
    email: String,
    login: String,
}

async fn get_team_id(products: Value) -> Result<i64> {
    if let Some(p) = products.as_array() {
        for (_, v) in p.iter().enumerate() {
            if let Some(name) = v.get("Name") {
                if let Some(lower) = name.as_str() {
                    let lower = lower.to_lowercase();
                    if lower.contains("team") && lower.contains("membership") {
                        if let Some(id) = v.get("ID").and_then(|v| v.as_i64()) {
                            return Ok(id);
                        } else {
                            return Err(Error::NotFoundTeamMembership(String::from(
                                "Cannot get ID",
                            )));
                        }
                    }
                }
            }
        }
        return Err(Error::NotFoundTeamMembership(String::from(
            "Cannot find product",
        )));
    } else {
        return Err(Error::NotFoundTeamMembership(String::from("Invalid Json")));
    }
}

async fn get_team_members(pool: &sqlx::PgPool, team_members: sqlx::types::JsonValue) -> Result<()> {
    if let Some(x) = team_members.as_array() {
        let team_firstname: Vec<String> = x
            .into_iter()
            .map(|member| String::from(member["Customer"]["FirstName"].as_str().unwrap()))
            .collect();
        let team_surname: Vec<String> = x
            .into_iter()
            .map(|member| String::from(member["Customer"]["Surname"].as_str().unwrap()))
            .collect();
        let team_cid: Vec<String> = x
            .into_iter()
            .map(|member| String::from(member["Customer"]["CID"].as_str().unwrap()))
            .collect();
        let team_email: Vec<String> = x
            .into_iter()
            .map(|member| String::from(member["Customer"]["Email"].as_str().unwrap()))
            .collect();
        let team_login: Vec<String> = x
            .into_iter()
            .map(|member| String::from(member["Customer"]["Login"].as_str().unwrap()))
            .collect();

        sqlx::query!(
			"INSERT INTO records.team_members SELECT * FROM UNNEST($1::text[], $2::text[], $3::text[], $4::text[], $5::text[])",
			&team_firstname[..],
			&team_surname[..],
			&team_cid[..],
			&team_email[..],
			&team_login[..]
		)
		.execute(pool)
		.await?;

        Ok(())
    } else {
        Err(Error::AddingTeam(String::from("No array in team_members")))
    }
}

pub async fn get_members(pool: &sqlx::PgPool) -> Result<()> {
    let key = dotenvy::var("EA_KEY")?;
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("X-API-Key", header::HeaderValue::from_str(&key)?);

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    let members = client
        .get("https://eactivities.union.ic.ac.uk/API/CSP/658/reports/members")
        .send()
        .await?
        .json::<Vec<Member>>()
        .await?;

    let products: Value = client
        .get("https://eactivities.union.ic.ac.uk/API/CSP/658/reports/products")
        .send()
        .await?
        .json()
        .await?;

    let id = get_team_id(products).await?;

    let team_membership_url = [
        "https://eactivities.union.ic.ac.uk/API/CSP/658/products/",
        &id.to_string(),
        "/sales",
    ]
    .concat();
    let team_members: Value = client.get(team_membership_url).send().await?.json().await?;

    sqlx::query!("TRUNCATE TABLE records.team_members")
        .execute(pool)
        .await?;

    let _ = get_team_members(pool, team_members).await?;

    sqlx::query!("TRUNCATE TABLE records.members")
        .execute(pool)
        .await?;

    for member in members {
        sqlx::query!(
            r#"
			INSERT INTO records.members VALUES ($1, $2, $3, $4, $5, $6, $7)
			"#,
            member.first_name,
            member.surname,
            member.cid,
            member.email,
            member.login,
            member.order_no,
            member.member_type
        )
        .execute(pool)
        .await?;
    }

    Ok(())
}

async fn check_tier(pool: &sqlx::PgPool, cid: &str, shortcode: &str) -> Result<i16> {
    let team = sqlx::query!(
        r#"
		SELECT cid, login FROM records.team_members WHERE cid = $1 AND login = $2
		"#,
        cid,
        shortcode
    )
    .fetch_optional(pool)
    .await?;

    if team.is_some() {
        return Ok(2);
    }

    let user = sqlx::query!(
        r#"
		SELECT cid, login FROM records.members WHERE cid = $1 AND login = $2
		"#,
        cid,
        shortcode
    )
    .fetch_optional(pool)
    .await?;

    if user.is_some() {
        Ok(1)
    } else {
        Ok(0)
    }
}

impl PendingUser {
    pub async fn create(
        State(pool): State<sqlx::PgPool>,
        Json(req): Json<PendingUser>,
    ) -> Result<Response> {
        req.validate()?;

        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(req.password.as_bytes(), &salt)?
            .to_string();

        let tier = check_tier(&pool, &req.cid, &req.shortcode).await?;

        if tier != 0 {
            sqlx::query!(
                "INSERT INTO auth.users VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9)",
                req.id,
                req.first_name,
                req.surname,
                req.shortcode,
                req.cid,
                password_hash,
                false,
                tier,
                req.created_at
            )
            .execute(&pool)
            .await?;
            Ok(StatusCode::CREATED.into_response())
        } else {
            sqlx::query!(
				r#"
				INSERT INTO auth.pending_users(id, verification_token, first_name, surname, shortcode, cid, password, created_at)
				VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
				"#,
				req.id,
				req.verification_token,
				req.first_name,
				req.surname,
				req.shortcode,
				req.cid,
				password_hash,
				req.created_at
			)
			.execute(&pool)
			.await?;
            // NEED TO IMPLEMENT EMAIL VERIFICATION
            Ok((StatusCode::CREATED, req.verification_token.to_string()).into_response())
        }
    }

    pub async fn verify(
        State(pool): State<sqlx::PgPool>,
        Query(token): Query<VerificationToken>,
    ) -> Result<StatusCode> {
        let pending_user = sqlx::query_as!(
            PendingUser,
            r#"
			SELECT * FROM auth.pending_users WHERE verification_token = $1
			"#,
            token.token
        )
        .fetch_optional(&pool)
        .await?;

        if let Some(x) = pending_user {
            let tier = check_tier(&pool, &x.cid, &x.shortcode).await?;
            sqlx::query!(
                r#"
				WITH new_user AS (
					DELETE FROM auth.pending_users
					WHERE verification_token = $1
					RETURNING *
				)
				INSERT INTO auth.users(id, first_name, surname, shortcode, cid, password, admin, tier)
				SELECT id, first_name, surname, shortcode, cid, password, $2, $3
				FROM new_user 
				"#,
                token.token,
                false,
                tier
            )
            .execute(&pool)
            .await?;

            return Ok(StatusCode::CREATED);
        }
        Err(Error::UnprocessableEntity("Invalid Token".into()))
    }
}
