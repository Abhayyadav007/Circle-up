//! Route table for the `search` slice.

use axum::{routing::get, Router};

use crate::core::state::AppState;

use super::handlers;

pub fn router() -> Router<AppState> {
    Router::new().route("/search/users", get(handlers::search_users))
}
