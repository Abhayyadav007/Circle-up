//! Route table for the `follows` slice.

use axum::{
    routing::{get, put},
    Router,
};

use crate::core::state::AppState;

use super::handlers;

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/profiles/{username}/follow",
            put(handlers::follow).delete(handlers::unfollow),
        )
        .route(
            "/profiles/{username}/followers",
            get(handlers::list_followers),
        )
        .route(
            "/profiles/{username}/following",
            get(handlers::list_following),
        )
}
