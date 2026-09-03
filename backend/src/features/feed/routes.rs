//! Route table for the `feed` slice.

use axum::{routing::get, Router};

use crate::core::state::AppState;

use super::handlers;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/feed", get(handlers::home))
        .route("/feed/explore", get(handlers::explore))
}
