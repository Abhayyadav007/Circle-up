//! Route table for the `auth` slice. No logic here — just path → handler.

use axum::{routing::post, Router};

use crate::core::state::AppState;

use super::handlers;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/auth/signup", post(handlers::signup))
        .route("/auth/login", post(handlers::login))
        .route("/auth/refresh", post(handlers::refresh))
        .route("/auth/oauth/{provider}", post(handlers::oauth))
}
