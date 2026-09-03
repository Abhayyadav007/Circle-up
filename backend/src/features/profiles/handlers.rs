//! Handlers for `profiles`.

use axum::{
    extract::{Path, State},
    Json,
};

use crate::core::auth::current_user::{CurrentUser, MaybeUser};
use crate::core::error::AppError;
use crate::core::state::AppState;

use super::dto::{ProfileCounts, ProfileResponse, UpdateProfileRequest};
use super::repository::{self, ProfilePatch, ProfileRow};

pub async fn get_me(
    user: CurrentUser,
    State(state): State<AppState>,
) -> Result<Json<ProfileResponse>, AppError> {
    let row = repository::by_id(state.db(), user.id)
        .await?
        .ok_or(AppError::NotFound("user"))?;
    Ok(Json(to_response(row, None, true)))
}

pub async fn update_me(
    user: CurrentUser,
    State(state): State<AppState>,
    Json(body): Json<UpdateProfileRequest>,
) -> Result<Json<ProfileResponse>, AppError> {
    if let Some(bio) = &body.bio {
        if bio.len() > 500 {
            return Err(AppError::bad_request("bio must be 500 characters or fewer"));
        }
    }

    let row = repository::update(
        state.db(),
        user.id,
        ProfilePatch {
            display_name: body.display_name.as_ref().map(|o| o.as_deref()),
            bio: body.bio.as_deref(),
            avatar_url: body.avatar_url.as_ref().map(|o| o.as_deref()),
        },
    )
    .await?;

    Ok(Json(to_response(row, None, true)))
}

pub async fn get_by_username(
    MaybeUser(viewer): MaybeUser,
    Path(username): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<ProfileResponse>, AppError> {
    let row = repository::by_username(state.db(), &username.to_lowercase())
        .await?
        .ok_or(AppError::NotFound("user"))?;

    let is_me = viewer.as_ref().map(|v| v.id) == Some(row.id);
    let is_following = match &viewer {
        Some(v) if !is_me => Some(repository::is_following(state.db(), v.id, row.id).await?),
        _ => None,
    };

    Ok(Json(to_response(row, is_following, is_me)))
}

fn to_response(row: ProfileRow, is_following: Option<bool>, is_me: bool) -> ProfileResponse {
    ProfileResponse {
        id: row.id,
        username: row.username,
        display_name: row.display_name,
        bio: row.bio,
        avatar_url: row.avatar_url,
        created_at: row.created_at,
        counts: ProfileCounts {
            posts: row.posts_count,
            followers: row.followers_count,
            following: row.following_count,
        },
        is_following,
        is_me,
    }
}
