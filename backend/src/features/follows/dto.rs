//! Wire contract for `follows`. Mirror in `mobile/src/features/profile/api`.

use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct FollowResponse {
    pub following: bool,
    pub followers_count: i64,
}

/// Compact user row for follower / following lists and search results.
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct UserSummary {
    pub id: Uuid,
    pub username: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
}
