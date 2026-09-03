//! DB access for `posts`.
//!
//! [`PostRow`] and [`SELECT_POST`] are shared with the `feed` slice so both
//! return an identical shape.

use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::core::error::AppError;
use crate::core::pagination::Cursor;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct PostRow {
    pub id: Uuid,
    pub caption: String,
    pub image_key: String,
    pub created_at: DateTime<Utc>,
    pub author_id: Uuid,
    pub author_username: String,
    pub author_display_name: Option<String>,
    pub author_avatar_url: Option<String>,
    pub like_count: i64,
    pub comment_count: i64,
    pub liked_by_me: bool,
}

impl PostRow {
    pub fn cursor(&self) -> Cursor {
        Cursor {
            created_at: self.created_at,
            id: self.id,
        }
    }
}

/// `$1` is bound to the viewer's user id (or NULL for anonymous) for the
/// `liked_by_me` sub-select. Append your own `WHERE` / `ORDER BY` / `LIMIT`.
pub const SELECT_POST: &str = r#"
    SELECT p.id, p.caption, p.image_key, p.created_at,
           u.id            AS author_id,
           u.username      AS author_username,
           u.display_name  AS author_display_name,
           u.avatar_url    AS author_avatar_url,
           (SELECT count(*) FROM likes    l WHERE l.post_id = p.id) AS like_count,
           (SELECT count(*) FROM comments c WHERE c.post_id = p.id) AS comment_count,
           EXISTS(SELECT 1 FROM likes l WHERE l.post_id = p.id AND l.user_id = $1) AS liked_by_me
      FROM posts p
      JOIN users u ON u.id = p.author_id
"#;

pub async fn create(
    db: &PgPool,
    author_id: Uuid,
    image_key: &str,
    caption: &str,
) -> Result<Uuid, AppError> {
    Ok(sqlx::query_scalar::<_, Uuid>(
        "INSERT INTO posts (author_id, image_key, caption) VALUES ($1, $2, $3) RETURNING id",
    )
    .bind(author_id)
    .bind(image_key)
    .bind(caption)
    .fetch_one(db)
    .await?)
}

pub async fn by_id(
    db: &PgPool,
    viewer: Option<Uuid>,
    id: Uuid,
) -> Result<Option<PostRow>, AppError> {
    let sql = format!("{SELECT_POST} WHERE p.id = $2");
    Ok(sqlx::query_as::<_, PostRow>(&sql)
        .bind(viewer)
        .bind(id)
        .fetch_optional(db)
        .await?)
}

/// `image_key` is returned so the caller can also clean up the R2 object.
pub async fn delete_owned(
    db: &PgPool,
    id: Uuid,
    owner_id: Uuid,
) -> Result<Option<String>, AppError> {
    Ok(sqlx::query_scalar::<_, String>(
        "DELETE FROM posts WHERE id = $1 AND author_id = $2 RETURNING image_key",
    )
    .bind(id)
    .bind(owner_id)
    .fetch_optional(db)
    .await?)
}

pub async fn list_by_username(
    db: &PgPool,
    viewer: Option<Uuid>,
    username: &str,
    after: Option<Cursor>,
    limit: i64,
) -> Result<Vec<PostRow>, AppError> {
    let sql = format!(
        "{SELECT_POST}
         WHERE u.username = $2
           AND ($3::timestamptz IS NULL OR (p.created_at, p.id) < ($3::timestamptz, $4::uuid))
         ORDER BY p.created_at DESC, p.id DESC
         LIMIT $5"
    );
    Ok(sqlx::query_as::<_, PostRow>(&sql)
        .bind(viewer)
        .bind(username)
        .bind(after.as_ref().map(|c| c.created_at))
        .bind(after.as_ref().map(|c| c.id))
        .bind(limit + 1)
        .fetch_all(db)
        .await?)
}
