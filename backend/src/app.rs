//! Router assembly.
//!
//! This is the one place that knows about every feature slice. Adding a feature
//! is a one-line change here plus a new `features/<name>` folder.

use std::time::Duration;

use axum::{
    body::Bytes,
    extract::{Path, State},
    http::{header, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use serde_json::json;
use tower_http::{
    cors::{Any, CorsLayer},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};

use crate::core::error::AppError;
use crate::core::state::AppState;
use crate::features;

/// Max bytes accepted by the local-dev `PUT /media` route.
const MAX_UPLOAD_BYTES: usize = 15 * 1024 * 1024;

/// Build the complete application router.
pub fn router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let api = Router::new()
        .route("/health", get(health))
        .merge(features::auth::routes::router())
        .merge(features::profiles::routes::router())
        .merge(features::posts::routes::router())
        .merge(features::likes::routes::router())
        .merge(features::comments::routes::router())
        .merge(features::follows::routes::router())
        .merge(features::feed::routes::router())
        .merge(features::search::routes::router());

    Router::new()
        .route("/", get(root))
        .nest("/api/v1", api)
        // Media route — only functional with the local-disk storage backend
        // (dev). With R2, objects are served from Cloudflare and this 404s.
        .route(
            "/media/{*key}",
            get(get_media)
                .put(put_media)
                .layer(axum::extract::DefaultBodyLimit::max(MAX_UPLOAD_BYTES)),
        )
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(30),
        ))
        .with_state(state)
}

async fn root() -> Json<serde_json::Value> {
    Json(json!({ "service": "circleup-api", "docs": "/api/v1/health" }))
}

/// Serve a stored object (local-disk backend only).
async fn get_media(State(state): State<AppState>, Path(key): Path<String>) -> Response {
    match state.storage().fetch(&key).await {
        Ok(Some((bytes, content_type))) => (
            [
                (header::CONTENT_TYPE, content_type),
                (
                    header::CACHE_CONTROL,
                    "public, max-age=31536000, immutable".to_string(),
                ),
            ],
            bytes,
        )
            .into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(err) => err.into_response(),
    }
}

/// Accept a client upload for the local-disk backend (stands in for a presigned
/// R2 PUT). No-op route when R2 is configured.
async fn put_media(
    State(state): State<AppState>,
    Path(key): Path<String>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<StatusCode, AppError> {
    if !state.storage().serves_media() {
        return Err(AppError::NotFound("route"));
    }
    let content_type = headers
        .get(header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream")
        .to_string();

    state
        .storage()
        .upload(&key, &content_type, body.to_vec())
        .await?;
    Ok(StatusCode::OK)
}

/// Liveness/readiness probe. Returns 200 with build + db status.
async fn health(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> Json<serde_json::Value> {
    let db_ok = sqlx::query_scalar::<_, i32>("SELECT 1")
        .fetch_one(state.db())
        .await
        .is_ok();

    Json(json!({
        "status": if db_ok { "ok" } else { "degraded" },
        "version": env!("CARGO_PKG_VERSION"),
        "db": db_ok,
    }))
}
