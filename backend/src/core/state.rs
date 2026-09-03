//! `AppState` — the shared handle every handler receives via `State<AppState>`.
//!
//! It is a thin `Arc` newtype: cloning is cheap (bumps a refcount) and it holds
//! the DB pool, parsed config, and the storage service behind its trait.

use std::sync::Arc;

use sqlx::PgPool;

use crate::core::auth::provider::{AuthProviderRegistry, GoogleAuthProvider};
use crate::core::config::Config;
use crate::core::db;
use crate::core::error::AppError;
use crate::core::storage::{local::LocalStorage, r2::R2Storage, StorageService};

#[derive(Clone)]
pub struct AppState(Arc<Inner>);

struct Inner {
    config: Config,
    db: PgPool,
    storage: Arc<dyn StorageService>,
    auth_providers: AuthProviderRegistry,
}

impl AppState {
    /// Build state from config: connect the pool, construct the storage client,
    /// and register OAuth providers.
    pub async fn bootstrap(config: Config) -> Result<Self, AppError> {
        let db = db::connect(&config).await?;

        let storage: Arc<dyn StorageService> = if config.storage.use_r2() {
            tracing::info!(bucket = %config.storage.r2_bucket, "storage: Cloudflare R2");
            Arc::new(R2Storage::new(&config.storage).await)
        } else {
            tracing::warn!(
                dir = %config.storage.local_dir,
                "storage: local disk (R2 not configured) — set R2_* env vars for production"
            );
            Arc::new(LocalStorage::new(
                &config.storage.local_dir,
                &config.public_url,
                config.storage.presign_ttl_secs,
            ))
        };

        let mut auth_providers = AuthProviderRegistry::new();
        auth_providers.register(Arc::new(GoogleAuthProvider::new(
            config.google_oauth.client_id.clone(),
        )));

        Ok(Self(Arc::new(Inner {
            config,
            db,
            storage,
            auth_providers,
        })))
    }

    /// Build state from parts — used by integration tests.
    pub fn from_parts(config: Config, db: PgPool, storage: Arc<dyn StorageService>) -> Self {
        let mut auth_providers = AuthProviderRegistry::new();
        auth_providers.register(Arc::new(GoogleAuthProvider::new(
            config.google_oauth.client_id.clone(),
        )));
        Self(Arc::new(Inner {
            config,
            db,
            storage,
            auth_providers,
        }))
    }

    pub fn config(&self) -> &Config {
        &self.0.config
    }

    pub fn db(&self) -> &PgPool {
        &self.0.db
    }

    pub fn storage(&self) -> &Arc<dyn StorageService> {
        &self.0.storage
    }

    pub fn auth_providers(&self) -> &AuthProviderRegistry {
        &self.0.auth_providers
    }
}
