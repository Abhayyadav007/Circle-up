//! Cloudflare R2 implementation of [`StorageService`] via the S3-compatible API.

use std::time::Duration;

use async_trait::async_trait;
use aws_sdk_s3::{
    config::{Credentials, Region},
    presigning::PresigningConfig,
    primitives::ByteStream,
    Client,
};

use crate::core::config::StorageConfig;
use crate::core::error::AppError;

use super::{PresignedUpload, StorageService};

#[derive(Clone)]
pub struct R2Storage {
    client: Client,
    bucket: String,
    public_base_url: String,
    presign_ttl: Duration,
}

impl R2Storage {
    pub async fn new(cfg: &StorageConfig) -> Self {
        let credentials = Credentials::new(
            cfg.r2_access_key_id.clone(),
            cfg.r2_secret_access_key.clone(),
            None,
            None,
            "circleup-r2",
        );

        let s3_config = aws_sdk_s3::Config::builder()
            .behavior_version_latest()
            // R2 ignores the region but the SDK requires one.
            .region(Region::new("auto"))
            .endpoint_url(cfg.r2_endpoint())
            .credentials_provider(credentials)
            .force_path_style(true)
            .build();

        Self {
            client: Client::from_conf(s3_config),
            bucket: cfg.r2_bucket.clone(),
            public_base_url: cfg.public_base_url.trim_end_matches('/').to_string(),
            presign_ttl: Duration::from_secs(cfg.presign_ttl_secs),
        }
    }
}

#[async_trait]
impl StorageService for R2Storage {
    async fn upload(
        &self,
        key: &str,
        content_type: &str,
        bytes: Vec<u8>,
    ) -> Result<String, AppError> {
        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .content_type(content_type)
            .body(ByteStream::from(bytes))
            .send()
            .await
            .map_err(|e| AppError::Storage(format!("put_object failed: {e}")))?;
        Ok(key.to_string())
    }

    async fn presign_upload(
        &self,
        key: &str,
        content_type: &str,
    ) -> Result<PresignedUpload, AppError> {
        let presign_config = PresigningConfig::expires_in(self.presign_ttl)
            .map_err(|e| AppError::Storage(format!("bad presign config: {e}")))?;

        let presigned = self
            .client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .content_type(content_type)
            .presigned(presign_config)
            .await
            .map_err(|e| AppError::Storage(format!("presign failed: {e}")))?;

        Ok(PresignedUpload {
            url: presigned.uri().to_string(),
            method: "PUT",
            headers: vec![("content-type".to_string(), content_type.to_string())],
            key: key.to_string(),
            expires_in: self.presign_ttl.as_secs(),
        })
    }

    fn get_url(&self, key: &str) -> String {
        if self.public_base_url.is_empty() {
            format!("{}/{}/{}", "r2://", self.bucket, key)
        } else {
            format!("{}/{}", self.public_base_url, key)
        }
    }

    async fn delete(&self, key: &str) -> Result<(), AppError> {
        self.client
            .delete_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await
            .map_err(|e| AppError::Storage(format!("delete_object failed: {e}")))?;
        Ok(())
    }
}
