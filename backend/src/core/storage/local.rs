//! On-disk storage backend, served by this server at `/media/{key}`.
//!
//! Used automatically in local dev when R2 credentials are not configured, so a
//! new contributor can exercise the full image-post flow without a Cloudflare
//! account. **Not for production** — no CDN, no redundancy.

use std::path::{Path, PathBuf};

use async_trait::async_trait;
use tokio::fs;

use crate::core::error::AppError;

use super::{PresignedUpload, StorageService};

#[derive(Clone)]
pub struct LocalStorage {
    root: PathBuf,
    /// e.g. `http://localhost:8080` — this server, as reachable by the client.
    public_url: String,
    presign_ttl_secs: u64,
}

impl LocalStorage {
    pub fn new(dir: &str, public_url: &str, presign_ttl_secs: u64) -> Self {
        Self {
            root: PathBuf::from(dir),
            public_url: public_url.trim_end_matches('/').to_string(),
            presign_ttl_secs,
        }
    }

    /// Resolve a key to a path, refusing anything that escapes `root`.
    fn path_for(&self, key: &str) -> Result<PathBuf, AppError> {
        if key.is_empty() || key.contains("..") || key.starts_with('/') || key.contains('\\') {
            return Err(AppError::bad_request("invalid object key"));
        }
        Ok(self.root.join(key))
    }
}

#[async_trait]
impl StorageService for LocalStorage {
    async fn upload(
        &self,
        key: &str,
        _content_type: &str,
        bytes: Vec<u8>,
    ) -> Result<String, AppError> {
        let path = self.path_for(key)?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .await
                .map_err(|e| AppError::Storage(format!("mkdir failed: {e}")))?;
        }
        fs::write(&path, &bytes)
            .await
            .map_err(|e| AppError::Storage(format!("write failed: {e}")))?;
        Ok(key.to_string())
    }

    async fn presign_upload(
        &self,
        key: &str,
        content_type: &str,
    ) -> Result<PresignedUpload, AppError> {
        // "Presigned" here just means: PUT the bytes to our own /media route.
        Ok(PresignedUpload {
            url: format!("{}/media/{}", self.public_url, key),
            method: "PUT",
            headers: vec![("content-type".to_string(), content_type.to_string())],
            key: key.to_string(),
            expires_in: self.presign_ttl_secs,
        })
    }

    fn get_url(&self, key: &str) -> String {
        format!("{}/media/{}", self.public_url, key)
    }

    async fn delete(&self, key: &str) -> Result<(), AppError> {
        let path = self.path_for(key)?;
        match fs::remove_file(&path).await {
            Ok(()) => Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(e) => Err(AppError::Storage(format!("delete failed: {e}"))),
        }
    }

    async fn fetch(&self, key: &str) -> Result<Option<(Vec<u8>, String)>, AppError> {
        let path = self.path_for(key)?;
        match fs::read(&path).await {
            Ok(bytes) => Ok(Some((bytes, content_type_for(&path)))),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(AppError::Storage(format!("read failed: {e}"))),
        }
    }

    fn serves_media(&self) -> bool {
        true
    }
}

fn content_type_for(path: &Path) -> String {
    match path.extension().and_then(|e| e.to_str()) {
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("png") => "image/png",
        Some("webp") => "image/webp",
        Some("gif") => "image/gif",
        _ => "application/octet-stream",
    }
    .to_string()
}
