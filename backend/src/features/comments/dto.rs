//! Wire contract for `comments`. Mirror in `mobile/src/features/comments/api`.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateCommentRequest {
    pub body: String,
}

#[derive(Debug, Serialize)]
pub struct CommentAuthor {
    pub id: Uuid,
    pub username: String,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CommentView {
    pub id: Uuid,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub author: CommentAuthor,
    /// True when the caller may delete this comment (its author or the post's).
    pub can_delete: bool,
}
