//! Argon2id password hashing.

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

use crate::core::error::AppError;

/// Hash a plaintext password for storage in `users.password_hash`.
pub fn hash(plaintext: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(plaintext.as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(|e| AppError::Unexpected(anyhow::anyhow!("password hash failed: {e}")))
}

/// Constant-time verify. Returns `Ok(false)` on mismatch, `Err` only on a
/// malformed stored hash.
pub fn verify(plaintext: &str, stored_hash: &str) -> Result<bool, AppError> {
    let parsed = PasswordHash::new(stored_hash)
        .map_err(|e| AppError::Unexpected(anyhow::anyhow!("bad stored hash: {e}")))?;
    Ok(Argon2::default()
        .verify_password(plaintext.as_bytes(), &parsed)
        .is_ok())
}
