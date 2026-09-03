//! Handlers for `follows`.

use axum::{
    extract::{Path, Query, State},
    Json,
};

use crate::core::auth::current_user::CurrentUser;
use crate::core::error::AppError;
use crate::core::pagination::{Page, PageParams};
use crate::core::state::AppState;

use super::dto::{FollowResponse, UserSummary};
use super::repository::{self, Direction};

async fn resolve(state: &AppState, username: &str) -> Result<uuid::Uuid, AppError> {
    repository::user_id_by_username(state.db(), &username.to_lowercase())
        .await?
        .ok_or(AppError::NotFound("user"))
}

pub async fn follow(
    user: CurrentUser,
    Path(username): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<FollowResponse>, AppError> {
    let target = resolve(&state, &username).await?;
    let followers_count = repository::follow(state.db(), user.id, target).await?;
    Ok(Json(FollowResponse {
        following: true,
        followers_count,
    }))
}

pub async fn unfollow(
    user: CurrentUser,
    Path(username): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<FollowResponse>, AppError> {
    let target = resolve(&state, &username).await?;
    let followers_count = repository::unfollow(state.db(), user.id, target).await?;
    Ok(Json(FollowResponse {
        following: false,
        followers_count,
    }))
}

pub async fn list_followers(
    Path(username): Path<String>,
    Query(params): Query<PageParams>,
    State(state): State<AppState>,
) -> Result<Json<Page<UserSummary>>, AppError> {
    list(&state, &username, Direction::Followers, params).await
}

pub async fn list_following(
    Path(username): Path<String>,
    Query(params): Query<PageParams>,
    State(state): State<AppState>,
) -> Result<Json<Page<UserSummary>>, AppError> {
    list(&state, &username, Direction::Following, params).await
}

async fn list(
    state: &AppState,
    username: &str,
    direction: Direction,
    params: PageParams,
) -> Result<Json<Page<UserSummary>>, AppError> {
    let target = resolve(state, username).await?;
    let limit = params.limit();
    let rows = repository::list_edges(
        state.db(),
        target,
        direction,
        params.decode_cursor()?,
        limit,
    )
    .await?;

    // Build the page on the raw rows (they carry the cursor), then map.
    let page = Page::build(rows, limit, |r| r.cursor());
    Ok(Json(Page {
        items: page.items.into_iter().map(|r| r.into_summary()).collect(),
        next_cursor: page.next_cursor,
    }))
}
