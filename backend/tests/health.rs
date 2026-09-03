//! Integration test for the health endpoint.
//!
//! Requires a running Postgres (see `docker-compose.yml`). CI provides one as a
//! service container; locally, `docker compose up -d postgres` first.
//!
//! Run with: `DATABASE_URL=postgres://circleup:circleup@localhost:5432/circleup cargo test`

use std::sync::Arc;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use circleup_backend::core::config::Config;
use circleup_backend::core::state::AppState;
use circleup_backend::core::storage::{PresignedUpload, StorageService};
use circleup_backend::{app, core::error::AppError};
use http_body_util::BodyExt;
use tower::ServiceExt;

/// A no-op storage backend so tests don't need R2 credentials.
struct NullStorage;

#[async_trait::async_trait]
impl StorageService for NullStorage {
    async fn upload(&self, key: &str, _ct: &str, _bytes: Vec<u8>) -> Result<String, AppError> {
        Ok(key.to_string())
    }
    async fn presign_upload(&self, key: &str, ct: &str) -> Result<PresignedUpload, AppError> {
        Ok(PresignedUpload {
            url: format!("https://example.test/{key}"),
            method: "PUT",
            headers: vec![("content-type".into(), ct.into())],
            key: key.to_string(),
            expires_in: 900,
        })
    }
    fn get_url(&self, key: &str) -> String {
        format!("https://cdn.example.test/{key}")
    }
    async fn delete(&self, _key: &str) -> Result<(), AppError> {
        Ok(())
    }
}

async fn test_state() -> AppState {
    let _ = dotenvy::dotenv();
    let config =
        Config::from_env().expect("test env not configured (need DATABASE_URL, JWT_SECRET)");
    let db = circleup_backend::core::db::connect(&config)
        .await
        .expect("connect to test db");
    sqlx::migrate!("./migrations")
        .run(&db)
        .await
        .expect("migrations");
    AppState::from_parts(config, db, Arc::new(NullStorage))
}

#[tokio::test]
async fn health_returns_ok() {
    let app = app::router(test_state().await);

    let res = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    let bytes = res.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(json["status"], "ok");
    assert_eq!(json["db"], true);
}

#[tokio::test]
async fn unknown_route_is_404() {
    let app = app::router(test_state().await);
    let res = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/does-not-exist")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}
