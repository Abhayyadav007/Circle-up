//! DB access for `follows`.

use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::core::error::AppError;
use crate::core::pagination::Cursor;

use super::dto::UserSummary;

pub async fn user_id_by_username(db: &PgPool, username: &str) -> Result<Option<Uuid>, AppError> {
    Ok(
        sqlx::query_scalar::<_, Uuid>("SELECT id FROM users WHERE username = $1")
            .bind(username)
            .fetch_optional(db)
            .await?,
    )
}

pub async fn follow(db: &PgPool, follower: Uuid, followee: Uuid) -> Result<i64, AppError> {
    if follower == followee {
        return Err(AppError::bad_request("you can't follow yourself"));
    }
    sqlx::query(
        "INSERT INTO follows (follower_id, followee_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
    )
    .bind(follower)
    .bind(followee)
    .execute(db)
    .await?;
    followers_count(db, followee).await
}

pub async fn unfollow(db: &PgPool, follower: Uuid, followee: Uuid) -> Result<i64, AppError> {
    sqlx::query("DELETE FROM follows WHERE follower_id = $1 AND followee_id = $2")
        .bind(follower)
        .bind(followee)
        .execute(db)
        .await?;
    followers_count(db, followee).await
}

pub async fn followers_count(db: &PgPool, user_id: Uuid) -> Result<i64, AppError> {
    Ok(
        sqlx::query_scalar::<_, i64>("SELECT count(*) FROM follows WHERE followee_id = $1")
            .bind(user_id)
            .fetch_one(db)
            .await?,
    )
}

/// One entry in a followers/following list, carrying the keyset position.
#[derive(Debug, sqlx::FromRow)]
pub struct EdgeRow {
    pub id: Uuid,
    pub username: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub edge_created_at: DateTime<Utc>,
}

impl EdgeRow {
    pub fn cursor(&self) -> Cursor {
        Cursor {
            created_at: self.edge_created_at,
            id: self.id,
        }
    }
    pub fn into_summary(self) -> UserSummary {
        UserSummary {
            id: self.id,
            username: self.username,
            display_name: self.display_name,
            avatar_url: self.avatar_url,
        }
    }
}

/// `direction`: `"followers"` lists users who follow `user_id`; `"following"`
/// lists users that `user_id` follows.
pub async fn list_edges(
    db: &PgPool,
    user_id: Uuid,
    direction: Direction,
    after: Option<Cursor>,
    limit: i64,
) -> Result<Vec<EdgeRow>, AppError> {
    let (filter_col, join_col) = match direction {
        Direction::Followers => ("f.followee_id", "f.follower_id"),
        Direction::Following => ("f.follower_id", "f.followee_id"),
    };
    let sql = format!(
        r#"
        SELECT u.id, u.username, u.display_name, u.avatar_url,
               f.created_at AS edge_created_at
          FROM follows f
          JOIN users u ON u.id = {join_col}
         WHERE {filter_col} = $1
           AND ($2::timestamptz IS NULL OR (f.created_at, u.id) < ($2::timestamptz, $3::uuid))
         ORDER BY f.created_at DESC, u.id DESC
         LIMIT $4
        "#
    );
    Ok(sqlx::query_as::<_, EdgeRow>(&sql)
        .bind(user_id)
        .bind(after.as_ref().map(|c| c.created_at))
        .bind(after.as_ref().map(|c| c.id))
        .bind(limit + 1)
        .fetch_all(db)
        .await?)
}

#[derive(Clone, Copy)]
pub enum Direction {
    Followers,
    Following,
}
