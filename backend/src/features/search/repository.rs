//! DB access for `search`.

use sqlx::PgPool;

use crate::core::error::AppError;
use crate::features::follows::dto::UserSummary;

/// Case-insensitive prefix + substring match on username / display name.
pub async fn users(db: &PgPool, term: &str, limit: i64) -> Result<Vec<UserSummary>, AppError> {
    let like = format!("%{}%", term.replace('%', "\\%").replace('_', "\\_"));
    Ok(sqlx::query_as::<_, UserSummary>(
        r#"
        SELECT id, username, display_name, avatar_url
          FROM users
         WHERE username ILIKE $1 OR display_name ILIKE $1
         ORDER BY (username ILIKE $2) DESC, username ASC
         LIMIT $3
        "#,
    )
    .bind(&like)
    .bind(format!("{term}%"))
    .bind(limit)
    .fetch_all(db)
    .await?)
}
