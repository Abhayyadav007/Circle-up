//! Wire contract for `profiles`. Mirror in `mobile/src/features/profile/api`.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct ProfileResponse {
    pub id: Uuid,
    pub username: String,
    pub display_name: Option<String>,
    pub bio: String,
    pub avatar_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub counts: ProfileCounts,
    /// `Some(true/false)` when the request is authenticated and viewing someone
    /// else; `None` for anonymous requests or your own profile.
    pub is_following: Option<bool>,
    /// True when this payload describes the caller.
    pub is_me: bool,
}

#[derive(Debug, Serialize)]
pub struct ProfileCounts {
    pub posts: i64,
    pub followers: i64,
    pub following: i64,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProfileRequest {
    #[serde(
        default,
        deserialize_with = "crate::features::profiles::dto::opt_field"
    )]
    pub display_name: Option<Option<String>>,
    #[serde(default)]
    pub bio: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::features::profiles::dto::opt_field"
    )]
    pub avatar_url: Option<Option<String>>,
}

/// Distinguish "field absent" (`None`) from "field present and null"
/// (`Some(None)`) so PATCH can clear a value.
pub fn opt_field<'de, D, T>(deserializer: D) -> Result<Option<Option<T>>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Deserialize<'de>,
{
    Ok(Some(Option::<T>::deserialize(deserializer)?))
}
