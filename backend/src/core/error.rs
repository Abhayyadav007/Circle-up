//! The one error type every handler returns.
//!
//! `AppError` implements [`IntoResponse`], so handlers can just `?` their way
//! through fallible code and every failure becomes a consistent JSON envelope:
//!
//! ```json
//! { "error": { "code": "not_found", "message": "post not found" } }
//! ```

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("configuration error: {0}")]
    Config(String),

    #[error("{0}")]
    BadRequest(String),

    #[error("unauthorized")]
    Unauthorized,

    #[error("forbidden")]
    Forbidden,

    #[error("{0} not found")]
    NotFound(&'static str),

    #[error("{0}")]
    Conflict(String),

    #[error("validation failed")]
    Validation(Vec<FieldError>),

    #[error("upstream provider error: {0}")]
    Upstream(String),

    #[error("storage error: {0}")]
    Storage(String),

    #[error(transparent)]
    Database(#[from] sqlx::Error),

    #[error(transparent)]
    Migration(#[from] sqlx::migrate::MigrateError),

    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

#[derive(Debug, serde::Serialize)]
pub struct FieldError {
    pub field: String,
    pub message: String,
}

impl AppError {
    fn parts(&self) -> (StatusCode, &'static str) {
        match self {
            AppError::BadRequest(_) => (StatusCode::BAD_REQUEST, "bad_request"),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "unauthorized"),
            AppError::Forbidden => (StatusCode::FORBIDDEN, "forbidden"),
            AppError::NotFound(_) => (StatusCode::NOT_FOUND, "not_found"),
            AppError::Conflict(_) => (StatusCode::CONFLICT, "conflict"),
            AppError::Validation(_) => (StatusCode::UNPROCESSABLE_ENTITY, "validation_error"),
            AppError::Upstream(_) => (StatusCode::BAD_GATEWAY, "upstream_error"),
            AppError::Storage(_) => (StatusCode::BAD_GATEWAY, "storage_error"),
            AppError::Config(_)
            | AppError::Database(_)
            | AppError::Migration(_)
            | AppError::Unexpected(_) => (StatusCode::INTERNAL_SERVER_ERROR, "internal_error"),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code) = self.parts();

        // Log the full detail server-side; never leak internals to clients.
        if status.is_server_error() {
            tracing::error!(error = %self, "request failed");
        } else {
            tracing::debug!(error = %self, "request rejected");
        }

        let message = match status {
            StatusCode::INTERNAL_SERVER_ERROR => "something went wrong".to_string(),
            _ => self.to_string(),
        };

        let mut body = json!({ "error": { "code": code, "message": message } });
        if let AppError::Validation(fields) = &self {
            body["error"]["fields"] = json!(fields);
        }

        (status, Json(body)).into_response()
    }
}

/// Ergonomic helper: `AppError::bad_request("...")`.
impl AppError {
    pub fn bad_request(msg: impl Into<String>) -> Self {
        AppError::BadRequest(msg.into())
    }
    pub fn conflict(msg: impl Into<String>) -> Self {
        AppError::Conflict(msg.into())
    }
}
