//! JWT minting & verification for Circleup's own sessions.
//!
//! Two token kinds, distinguished by the `typ` claim:
//! - `access`  — short-lived (~15m), sent on every API call.
//! - `refresh` — long-lived (~30d), exchanged for a new access token.

use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::core::config::JwtConfig;
use crate::core::error::AppError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TokenKind {
    Access,
    Refresh,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// Subject — the user id.
    pub sub: Uuid,
    pub typ: TokenKind,
    pub iss: String,
    pub iat: i64,
    pub exp: i64,
}

#[derive(Debug, Serialize)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: &'static str,
    pub expires_in: i64,
}

#[derive(Clone)]
pub struct JwtService {
    encoding: EncodingKey,
    decoding: DecodingKey,
    config: JwtConfig,
}

impl JwtService {
    pub fn new(config: JwtConfig) -> Self {
        Self {
            encoding: EncodingKey::from_secret(config.secret.as_bytes()),
            decoding: DecodingKey::from_secret(config.secret.as_bytes()),
            config,
        }
    }

    pub fn issue_pair(&self, user_id: Uuid) -> Result<TokenPair, AppError> {
        Ok(TokenPair {
            access_token: self.issue(user_id, TokenKind::Access, self.config.access_ttl_secs)?,
            refresh_token: self.issue(user_id, TokenKind::Refresh, self.config.refresh_ttl_secs)?,
            token_type: "Bearer",
            expires_in: self.config.access_ttl_secs,
        })
    }

    fn issue(&self, user_id: Uuid, typ: TokenKind, ttl: i64) -> Result<String, AppError> {
        let now = Utc::now().timestamp();
        let claims = Claims {
            sub: user_id,
            typ,
            iss: self.config.issuer.clone(),
            iat: now,
            exp: now + ttl,
        };
        encode(&Header::default(), &claims, &self.encoding)
            .map_err(|e| AppError::Unexpected(e.into()))
    }

    /// Verify a token and assert its kind. Returns `Unauthorized` on any failure.
    pub fn verify(&self, token: &str, expected: TokenKind) -> Result<Claims, AppError> {
        let mut validation = Validation::default();
        validation.set_issuer(&[&self.config.issuer]);
        validation.validate_exp = true;

        let data = decode::<Claims>(token, &self.decoding, &validation)
            .map_err(|_| AppError::Unauthorized)?;

        if data.claims.typ != expected {
            return Err(AppError::Unauthorized);
        }
        Ok(data.claims)
    }
}
