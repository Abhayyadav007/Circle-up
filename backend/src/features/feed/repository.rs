//! DB access for `feed`. Builds on `posts::repository::SELECT_POST` so the row
//! shape (and therefore `PostView`) stays identical to the `posts` slice.

use sqlx::PgPool;
use uuid::Uuid;

use crate::core::error::AppError;
use crate::core::pagination::Cursor;
use crate::features::posts::repository::{PostRow, SELECT_POST};

/// Home timeline: posts authored by users the viewer follows, plus the viewer's
/// own posts. Newest first, keyset-paginated.
pub async fn home(
    db: &PgPool,
    viewer: Uuid,
    after: Option<Cursor>,
    limit: i64,
) -> Result<Vec<PostRow>, AppError> {
    let sql = format!(
        "{SELECT_POST}
         WHERE (
                 p.author_id = $1
              OR p.author_id IN (SELECT followee_id FROM follows WHERE follower_id = $1)
               )
           AND ($2::timestamptz IS NULL OR (p.created_at, p.id) < ($2::timestamptz, $3::uuid))
         ORDER BY p.created_at DESC, p.id DESC
         LIMIT $4"
    );
    Ok(sqlx::query_as::<_, PostRow>(&sql)
        .bind(viewer) // $1 — also feeds SELECT_POST's `liked_by_me` ($1)
        .bind(after.as_ref().map(|c| c.created_at))
        .bind(after.as_ref().map(|c| c.id))
        .bind(limit + 1)
        .fetch_all(db)
        .await?)
}

/// Explore / global recent feed. `viewer` may be `None` (anonymous).
pub async fn explore(
    db: &PgPool,
    viewer: Option<Uuid>,
    after: Option<Cursor>,
    limit: i64,
) -> Result<Vec<PostRow>, AppError> {
    let sql = format!(
        "{SELECT_POST}
         WHERE ($2::timestamptz IS NULL OR (p.created_at, p.id) < ($2::timestamptz, $3::uuid))
         ORDER BY p.created_at DESC, p.id DESC
         LIMIT $4"
    );
    Ok(sqlx::query_as::<_, PostRow>(&sql)
        .bind(viewer)
        .bind(after.as_ref().map(|c| c.created_at))
        .bind(after.as_ref().map(|c| c.id))
        .bind(limit + 1)
        .fetch_all(db)
        .await?)
}
