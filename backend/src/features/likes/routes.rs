//! Route table for the `likes` slice.

use axum::{routing::put, Router};

use crate::core::state::AppState;

use super::handlers;

pub fn router() -> Router<AppState> {
    Router::new().route(
        "/posts/{id}/like",
        put(handlers::like).delete(handlers::unlike),
    )
}
