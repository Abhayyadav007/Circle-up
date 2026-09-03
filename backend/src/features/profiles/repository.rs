//! DB access for `profiles`.

use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::core::error::AppError;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ProfileRow {
    pub id: Uuid,
    pub username: String,
    pub display_name: Option<String>,
    pub bio: String,
    pub avatar_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub posts_count: i64,
    pub followers_count: i64,
    pub following_count: i64,
}

const SELECT_PROFILE: &str = r#"
    SELECT u.id, u.username, u.display_name, u.bio, u.avatar_url, u.created_at,
           (SELECT count(*) FROM posts   p WHERE p.author_id  = u.id) AS posts_count,
           (SELECT count(*) FROM follows f WHERE f.followee_id = u.id) AS followers_count,
           (SELECT count(*) FROM follows f WHERE f.follower_id = u.id) AS following_count
      FROM users u
"#;

pub async fn by_id(db: &PgPool, id: Uuid) -> Result<Option<ProfileRow>, AppError> {
    let sql = format!("{SELECT_PROFILE} WHERE u.id = $1");
    Ok(sqlx::query_as::<_, ProfileRow>(&sql)
        .bind(id)
        .fetch_optional(db)
        .await?)
}

pub async fn by_username(db: &PgPool, username: &str) -> Result<Option<ProfileRow>, AppError> {
    let sql = format!("{SELECT_PROFILE} WHERE u.username = $1");
    Ok(sqlx::query_as::<_, ProfileRow>(&sql)
        .bind(username)
        .fetch_optional(db)
        .await?)
}

pub struct ProfilePatch<'a> {
    /// `None` = leave; `Some(v)` = set to `v` (v may be null).
    pub display_name: Option<Option<&'a str>>,
    pub bio: Option<&'a str>,
    pub avatar_url: Option<Option<&'a str>>,
}

pub async fn update(
    db: &PgPool,
    id: Uuid,
    patch: ProfilePatch<'_>,
) -> Result<ProfileRow, AppError> {
    // COALESCE-style partial update using per-field "should update" flags.
    sqlx::query(
        r#"
        UPDATE users SET
            display_name = CASE WHEN $2 THEN $3 ELSE display_name END,
            bio          = COALESCE($4, bio),
            avatar_url   = CASE WHEN $5 THEN $6 ELSE avatar_url END,
            updated_at   = now()
        WHERE id = $1
        "#,
    )
    .bind(id)
    .bind(patch.display_name.is_some())
    .bind(patch.display_name.flatten())
    .bind(patch.bio)
    .bind(patch.avatar_url.is_some())
    .bind(patch.avatar_url.flatten())
    .execute(db)
    .await?;

    by_id(db, id).await?.ok_or(AppError::NotFound("user"))
}

pub async fn is_following(db: &PgPool, follower: Uuid, followee: Uuid) -> Result<bool, AppError> {
    Ok(sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM follows WHERE follower_id = $1 AND followee_id = $2)",
    )
    .bind(follower)
    .bind(followee)
    .fetch_one(db)
    .await?)
}
