//! DB access for `likes`.

use sqlx::PgPool;
use uuid::Uuid;

use crate::core::error::AppError;

pub async fn post_exists(db: &PgPool, post_id: Uuid) -> Result<bool, AppError> {
    Ok(
        sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM posts WHERE id = $1)")
            .bind(post_id)
            .fetch_one(db)
            .await?,
    )
}

/// Insert ignoring conflicts. Returns the fresh like count.
pub async fn like(db: &PgPool, user_id: Uuid, post_id: Uuid) -> Result<i64, AppError> {
    sqlx::query("INSERT INTO likes (user_id, post_id) VALUES ($1, $2) ON CONFLICT DO NOTHING")
        .bind(user_id)
        .bind(post_id)
        .execute(db)
        .await?;
    count(db, post_id).await
}

pub async fn unlike(db: &PgPool, user_id: Uuid, post_id: Uuid) -> Result<i64, AppError> {
    sqlx::query("DELETE FROM likes WHERE user_id = $1 AND post_id = $2")
        .bind(user_id)
        .bind(post_id)
        .execute(db)
        .await?;
    count(db, post_id).await
}

pub async fn count(db: &PgPool, post_id: Uuid) -> Result<i64, AppError> {
    Ok(
        sqlx::query_scalar::<_, i64>("SELECT count(*) FROM likes WHERE post_id = $1")
            .bind(post_id)
            .fetch_one(db)
            .await?,
    )
}
