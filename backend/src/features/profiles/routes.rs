//! Route table for the `profiles` slice.

use axum::{routing::get, Router};

use crate::core::state::AppState;

use super::handlers;

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/profiles/me",
            get(handlers::get_me).patch(handlers::update_me),
        )
        .route("/profiles/{username}", get(handlers::get_by_username))
}
