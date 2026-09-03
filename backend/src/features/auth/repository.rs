//! All DB access for the `auth` slice.

use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::core::error::AppError;

/// The subset of the `users` row that auth cares about.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct AuthUser {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: Option<String>,
    pub google_id: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: DateTime<Utc>,
}

pub struct NewUser<'a> {
    pub email: &'a str,
    pub username: &'a str,
    pub password_hash: Option<&'a str>,
    pub google_id: Option<&'a str>,
    pub avatar_url: Option<&'a str>,
    pub display_name: Option<&'a str>,
}

const USER_COLS: &str = "id, username, email, password_hash, google_id, avatar_url, created_at";

pub async fn find_by_email(db: &PgPool, email: &str) -> Result<Option<AuthUser>, AppError> {
    let sql = format!("SELECT {USER_COLS} FROM users WHERE email = $1");
    Ok(sqlx::query_as::<_, AuthUser>(&sql)
        .bind(email)
        .fetch_optional(db)
        .await?)
}

pub async fn find_by_google_id(db: &PgPool, google_id: &str) -> Result<Option<AuthUser>, AppError> {
    let sql = format!("SELECT {USER_COLS} FROM users WHERE google_id = $1");
    Ok(sqlx::query_as::<_, AuthUser>(&sql)
        .bind(google_id)
        .fetch_optional(db)
        .await?)
}

pub async fn find_by_id(db: &PgPool, id: Uuid) -> Result<Option<AuthUser>, AppError> {
    let sql = format!("SELECT {USER_COLS} FROM users WHERE id = $1");
    Ok(sqlx::query_as::<_, AuthUser>(&sql)
        .bind(id)
        .fetch_optional(db)
        .await?)
}

pub async fn insert_user(db: &PgPool, new: NewUser<'_>) -> Result<AuthUser, AppError> {
    let sql = format!(
        "INSERT INTO users (email, username, password_hash, google_id, avatar_url, display_name)
         VALUES ($1, $2, $3, $4, $5, $6)
         RETURNING {USER_COLS}"
    );
    sqlx::query_as::<_, AuthUser>(&sql)
        .bind(new.email)
        .bind(new.username)
        .bind(new.password_hash)
        .bind(new.google_id)
        .bind(new.avatar_url)
        .bind(new.display_name)
        .fetch_one(db)
        .await
        .map_err(map_unique_violation)
}

/// Link a Google identity to an existing (email/password) account.
pub async fn attach_google_id(
    db: &PgPool,
    user_id: Uuid,
    google_id: &str,
    avatar_url: Option<&str>,
) -> Result<AuthUser, AppError> {
    let sql = format!(
        "UPDATE users
            SET google_id = $2,
                avatar_url = COALESCE(users.avatar_url, $3),
                updated_at = now()
          WHERE id = $1
        RETURNING {USER_COLS}"
    );
    Ok(sqlx::query_as::<_, AuthUser>(&sql)
        .bind(user_id)
        .bind(google_id)
        .bind(avatar_url)
        .fetch_one(db)
        .await?)
}

fn map_unique_violation(err: sqlx::Error) -> AppError {
    if let sqlx::Error::Database(db_err) = &err {
        if db_err.is_unique_violation() {
            let msg = match db_err.constraint() {
                Some(c) if c.contains("email") => "email already registered",
                Some(c) if c.contains("username") => "username already taken",
                _ => "account already exists",
            };
            return AppError::conflict(msg);
        }
    }
    AppError::Database(err)
}
