//! DB access for `comments`.

use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::core::error::AppError;
use crate::core::pagination::Cursor;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct CommentRow {
    pub id: Uuid,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub author_id: Uuid,
    pub author_username: String,
    pub author_avatar_url: Option<String>,
    /// Author of the post this comment belongs to (for delete permission).
    pub post_author_id: Uuid,
}

impl CommentRow {
    pub fn cursor(&self) -> Cursor {
        Cursor {
            created_at: self.created_at,
            id: self.id,
        }
    }
}

const SELECT_COMMENT: &str = r#"
    SELECT c.id, c.body, c.created_at,
           u.id         AS author_id,
           u.username    AS author_username,
           u.avatar_url  AS author_avatar_url,
           p.author_id   AS post_author_id
      FROM comments c
      JOIN users u ON u.id = c.author_id
      JOIN posts p ON p.id = c.post_id
"#;

pub async fn create(
    db: &PgPool,
    post_id: Uuid,
    author_id: Uuid,
    body: &str,
) -> Result<CommentRow, AppError> {
    let id = sqlx::query_scalar::<_, Uuid>(
        "INSERT INTO comments (post_id, author_id, body) VALUES ($1, $2, $3) RETURNING id",
    )
    .bind(post_id)
    .bind(author_id)
    .bind(body)
    .fetch_one(db)
    .await
    .map_err(|e| match &e {
        sqlx::Error::Database(d) if d.is_foreign_key_violation() => AppError::NotFound("post"),
        _ => AppError::Database(e),
    })?;

    by_id(db, id).await?.ok_or(AppError::NotFound("comment"))
}

pub async fn by_id(db: &PgPool, id: Uuid) -> Result<Option<CommentRow>, AppError> {
    let sql = format!("{SELECT_COMMENT} WHERE c.id = $1");
    Ok(sqlx::query_as::<_, CommentRow>(&sql)
        .bind(id)
        .fetch_optional(db)
        .await?)
}

pub async fn list(
    db: &PgPool,
    post_id: Uuid,
    after: Option<Cursor>,
    limit: i64,
) -> Result<Vec<CommentRow>, AppError> {
    let sql = format!(
        "{SELECT_COMMENT}
         WHERE c.post_id = $1
           AND ($2::timestamptz IS NULL OR (c.created_at, c.id) > ($2::timestamptz, $3::uuid))
         ORDER BY c.created_at ASC, c.id ASC
         LIMIT $4"
    );
    Ok(sqlx::query_as::<_, CommentRow>(&sql)
        .bind(post_id)
        .bind(after.as_ref().map(|c| c.created_at))
        .bind(after.as_ref().map(|c| c.id))
        .bind(limit + 1)
        .fetch_all(db)
        .await?)
}

/// Delete if `actor` is the comment author or the post author. Returns whether a
/// row was removed.
pub async fn delete_permitted(db: &PgPool, id: Uuid, actor: Uuid) -> Result<bool, AppError> {
    let affected = sqlx::query(
        r#"
        DELETE FROM comments c
        USING posts p
        WHERE c.id = $1
          AND p.id = c.post_id
          AND ($2 = c.author_id OR $2 = p.author_id)
        "#,
    )
    .bind(id)
    .bind(actor)
    .execute(db)
    .await?
    .rows_affected();
    Ok(affected > 0)
}
