//! Route table for the `posts` slice.

use axum::{
    routing::{get, post},
    Router,
};

use crate::core::state::AppState;

use super::handlers;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/posts/uploads", post(handlers::create_upload_url))
        .route("/posts", post(handlers::create_post))
        .route(
            "/posts/{id}",
            get(handlers::get_post).delete(handlers::delete_post),
        )
        .route("/profiles/{username}/posts", get(handlers::list_user_posts))
}
