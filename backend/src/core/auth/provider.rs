//! Pluggable OAuth / OIDC providers.
//!
//! The `auth` feature never mentions "Google" — it asks the registry for a
//! provider by name and calls [`AuthProvider::verify_id_token`]. To add Apple
//! Sign-In later: implement `AuthProvider` for `AppleAuthProvider` and register
//! it in [`AppState::bootstrap`]. Nothing in `features/auth` changes.

use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use serde::Deserialize;

use crate::core::error::AppError;

/// A verified identity coming back from an external provider.
#[derive(Debug, Clone)]
pub struct ExternalIdentity {
    /// Stable, provider-unique subject id (stored as `users.google_id` etc.).
    pub subject: String,
    pub email: String,
    pub email_verified: bool,
    pub name: Option<String>,
    pub avatar_url: Option<String>,
    /// e.g. `"google"`.
    pub provider: &'static str,
}

#[async_trait]
pub trait AuthProvider: Send + Sync {
    /// Short lowercase name used as the registry key and route param
    /// (`POST /auth/oauth/:provider`).
    fn name(&self) -> &'static str;

    /// Verify a client-supplied ID token and return the identity it asserts.
    async fn verify_id_token(&self, id_token: &str) -> Result<ExternalIdentity, AppError>;
}

/// Lookup table of registered providers.
#[derive(Clone, Default)]
pub struct AuthProviderRegistry {
    providers: HashMap<&'static str, Arc<dyn AuthProvider>>,
}

impl AuthProviderRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, provider: Arc<dyn AuthProvider>) {
        self.providers.insert(provider.name(), provider);
    }

    pub fn get(&self, name: &str) -> Result<Arc<dyn AuthProvider>, AppError> {
        self.providers
            .get(name)
            .cloned()
            .ok_or_else(|| AppError::bad_request(format!("unknown auth provider: {name}")))
    }
}

// ---------------------------------------------------------------------------
// Google
// ---------------------------------------------------------------------------

/// Verifies Google ID tokens.
///
/// For scaffold simplicity this calls Google's `tokeninfo` endpoint. For
/// production, verify the JWT signature locally against Google's JWKS
/// (`https://www.googleapis.com/oauth2/v3/certs`) to avoid a network hop per
/// login — the shape of [`ExternalIdentity`] stays identical.
pub struct GoogleAuthProvider {
    expected_aud: String,
    http: reqwest::Client,
}

impl GoogleAuthProvider {
    pub fn new(client_id: String) -> Self {
        Self {
            expected_aud: client_id,
            http: reqwest::Client::new(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct GoogleTokenInfo {
    /// Subject.
    sub: String,
    aud: String,
    email: Option<String>,
    #[serde(default)]
    email_verified: String, // tokeninfo returns "true"/"false" as strings
    name: Option<String>,
    picture: Option<String>,
    #[serde(default)]
    exp: String,
}

#[async_trait]
impl AuthProvider for GoogleAuthProvider {
    fn name(&self) -> &'static str {
        "google"
    }

    async fn verify_id_token(&self, id_token: &str) -> Result<ExternalIdentity, AppError> {
        let info: GoogleTokenInfo = self
            .http
            .get("https://oauth2.googleapis.com/tokeninfo")
            .query(&[("id_token", id_token)])
            .send()
            .await
            .map_err(|e| AppError::Upstream(format!("google tokeninfo request failed: {e}")))?
            .error_for_status()
            .map_err(|_| AppError::Unauthorized)?
            .json()
            .await
            .map_err(|e| AppError::Upstream(format!("google tokeninfo decode failed: {e}")))?;

        if !self.expected_aud.is_empty() && info.aud != self.expected_aud {
            return Err(AppError::Unauthorized);
        }
        if let Ok(exp) = info.exp.parse::<i64>() {
            if exp < chrono::Utc::now().timestamp() {
                return Err(AppError::Unauthorized);
            }
        }

        let email = info
            .email
            .ok_or_else(|| AppError::bad_request("google account has no email"))?;

        Ok(ExternalIdentity {
            subject: info.sub,
            email,
            email_verified: info.email_verified == "true",
            name: info.name,
            avatar_url: info.picture,
            provider: "google",
        })
    }
}
