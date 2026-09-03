//! Wire contract for `auth`. These structs *are* the public API — change them
//! deliberately and in lockstep with `mobile/src/features/auth/api`.

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub username: String,
    pub password: String,
    #[serde(default)]
    pub display_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

/// Body for `POST /auth/oauth/{provider}`. The client obtains `id_token` from the
/// provider SDK (e.g. `@react-native-google-signin`) and forwards it here.
#[derive(Debug, Deserialize)]
pub struct OAuthRequest {
    pub id_token: String,
}

// Response type is `crate::core::auth::jwt::TokenPair` (already `Serialize`).
