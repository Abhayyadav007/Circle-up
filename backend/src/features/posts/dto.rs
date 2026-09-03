//! Wire contract for `posts`. `PostView` is also returned by the `feed` slice —
//! keep it stable. Mirror in `mobile/src/features/post/api`.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateUploadRequest {
    /// MIME type of the file to upload, e.g. `image/jpeg`.
    pub content_type: String,
}

#[derive(Debug, Deserialize)]
pub struct CreatePostRequest {
    /// The `key` returned by `POST /posts/uploads`.
    pub image_key: String,
    #[serde(default)]
    pub caption: Option<String>,
}

/// Author summary embedded in a post payload.
#[derive(Debug, Serialize)]
pub struct PostAuthor {
    pub id: Uuid,
    pub username: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
}

/// Canonical post representation used everywhere a post is returned.
#[derive(Debug, Serialize)]
pub struct PostView {
    pub id: Uuid,
    pub caption: String,
    pub image_url: String,
    pub created_at: DateTime<Utc>,
    pub author: PostAuthor,
    pub like_count: i64,
    pub comment_count: i64,
    /// `true` when the authenticated caller has liked this post.
    pub liked_by_me: bool,
}
