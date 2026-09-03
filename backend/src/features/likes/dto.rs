//! Wire contract for `likes`. Mirror in `mobile/src/features/likes/api`.

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct LikeResponse {
    pub liked: bool,
    pub like_count: i64,
}
