//! Object storage abstraction.
//!
//! Handlers and repositories depend only on the [`StorageService`] trait, never
//! on `aws-sdk-s3` directly. Swapping Cloudflare R2 for S3 / GCS / a local disk
//! is a new `impl StorageService` plus one line in [`AppState::bootstrap`].
//!
//! The preferred upload path is **presigned PUT URLs**: the client uploads
//! straight to R2 and only tells our API the resulting object key. Large image
//! bytes never pass through the Axum process.
//!
//! When R2 credentials are absent (fresh clone, no Cloudflare account yet) the
//! app falls back to [`local::LocalStorage`]: files are written to disk and
//! served by this server at `/media/{key}`. The client flow is identical — the
//! presigned URL just points back at us instead of R2.

pub mod local;
pub mod r2;

use async_trait::async_trait;

use crate::core::error::AppError;

/// A presigned request the client can execute directly against the bucket.
#[derive(Debug, Clone, serde::Serialize)]
pub struct PresignedUpload {
    /// Fully-qualified URL to `PUT` the bytes to.
    pub url: String,
    /// HTTP method (always `PUT` for now).
    pub method: &'static str,
    /// Headers the client must echo back on the upload request.
    pub headers: Vec<(String, String)>,
    /// The object key to send back to our API once the upload completes.
    pub key: String,
    /// Seconds until `url` stops working.
    pub expires_in: u64,
}

#[async_trait]
pub trait StorageService: Send + Sync {
    /// Server-side upload (used for small assets / tests). Returns the key.
    async fn upload(
        &self,
        key: &str,
        content_type: &str,
        bytes: Vec<u8>,
    ) -> Result<String, AppError>;

    /// Mint a presigned PUT URL for a client-side upload.
    async fn presign_upload(
        &self,
        key: &str,
        content_type: &str,
    ) -> Result<PresignedUpload, AppError>;

    /// Public URL for a stored object (via the bucket's public domain).
    fn get_url(&self, key: &str) -> String;

    /// Delete an object. Missing objects are not an error.
    async fn delete(&self, key: &str) -> Result<(), AppError>;

    /// Read object bytes + content-type. Only implemented by backends that serve
    /// media *through* this server (local dev). R2 serves objects from its own
    /// public domain, so it returns `None` and the `/media` route 404s.
    async fn fetch(&self, _key: &str) -> Result<Option<(Vec<u8>, String)>, AppError> {
        Ok(None)
    }

    /// Whether this backend accepts uploads at `PUT /media/{key}` (local dev).
    fn serves_media(&self) -> bool {
        false
    }
}

/// Build a namespaced object key, e.g. `posts/<uuid>/<uuid>.jpg`.
pub fn build_key(prefix: &str, ext: &str) -> String {
    let ext = ext.trim_start_matches('.');
    format!("{prefix}/{}.{ext}", uuid::Uuid::new_v4())
}
