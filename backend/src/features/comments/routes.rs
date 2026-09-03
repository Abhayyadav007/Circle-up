//! Route table for the `comments` slice.

use axum::{
    routing::{delete, get},
    Router,
};

use crate::core::state::AppState;

use super::handlers;

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/posts/{id}/comments",
            get(handlers::list).post(handlers::create),
        )
        .route("/comments/{id}", delete(handlers::remove))
}
