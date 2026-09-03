//! Handlers for `likes`.

use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;

use crate::core::auth::current_user::CurrentUser;
use crate::core::error::AppError;
use crate::core::state::AppState;

use super::dto::LikeResponse;
use super::repository;

pub async fn like(
    user: CurrentUser,
    Path(post_id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<LikeResponse>, AppError> {
    if !repository::post_exists(state.db(), post_id).await? {
        return Err(AppError::NotFound("post"));
    }
    let like_count = repository::like(state.db(), user.id, post_id).await?;
    Ok(Json(LikeResponse {
        liked: true,
        like_count,
    }))
}

pub async fn unlike(
    user: CurrentUser,
    Path(post_id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<LikeResponse>, AppError> {
    let like_count = repository::unlike(state.db(), user.id, post_id).await?;
    Ok(Json(LikeResponse {
        liked: false,
        like_count,
    }))
}
