//! Handlers for `posts`.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};

use crate::core::auth::current_user::{CurrentUser, MaybeUser};
use crate::core::error::AppError;
use crate::core::pagination::{Page, PageParams};
use crate::core::state::AppState;
use crate::core::storage::{build_key, PresignedUpload};

use super::dto::{CreatePostRequest, CreateUploadRequest, PostAuthor, PostView};
use super::repository::{self, PostRow};

const ALLOWED_TYPES: [&str; 3] = ["image/jpeg", "image/png", "image/webp"];

pub async fn create_upload_url(
    user: CurrentUser,
    State(state): State<AppState>,
    Json(body): Json<CreateUploadRequest>,
) -> Result<Json<PresignedUpload>, AppError> {
    if !ALLOWED_TYPES.contains(&body.content_type.as_str()) {
        return Err(AppError::bad_request(
            "content_type must be jpeg, png or webp",
        ));
    }
    let ext = match body.content_type.as_str() {
        "image/jpeg" => "jpg",
        "image/png" => "png",
        _ => "webp",
    };
    let key = build_key(&format!("posts/{}", user.id), ext);
    let presigned = state
        .storage()
        .presign_upload(&key, &body.content_type)
        .await?;
    Ok(Json(presigned))
}

pub async fn create_post(
    user: CurrentUser,
    State(state): State<AppState>,
    Json(body): Json<CreatePostRequest>,
) -> Result<(StatusCode, Json<PostView>), AppError> {
    let caption = body.caption.unwrap_or_default();
    if caption.len() > 2_200 {
        return Err(AppError::bad_request("caption too long"));
    }
    // Basic guard that the key belongs to this user's namespace.
    if !body.image_key.starts_with(&format!("posts/{}/", user.id)) {
        return Err(AppError::bad_request("image_key does not belong to you"));
    }

    let id = repository::create(state.db(), user.id, &body.image_key, &caption).await?;
    let row = repository::by_id(state.db(), Some(user.id), id)
        .await?
        .ok_or(AppError::NotFound("post"))?;

    Ok((StatusCode::CREATED, Json(view(&state, row))))
}

pub async fn get_post(
    MaybeUser(viewer): MaybeUser,
    Path(id): Path<uuid::Uuid>,
    State(state): State<AppState>,
) -> Result<Json<PostView>, AppError> {
    let row = repository::by_id(state.db(), viewer.as_ref().map(|v| v.id), id)
        .await?
        .ok_or(AppError::NotFound("post"))?;
    Ok(Json(view(&state, row)))
}

pub async fn delete_post(
    user: CurrentUser,
    Path(id): Path<uuid::Uuid>,
    State(state): State<AppState>,
) -> Result<StatusCode, AppError> {
    match repository::delete_owned(state.db(), id, user.id).await? {
        Some(image_key) => {
            // Best-effort object cleanup; a failure here shouldn't 500 the delete.
            if let Err(err) = state.storage().delete(&image_key).await {
                tracing::warn!(%image_key, %err, "failed to delete R2 object for post");
            }
            Ok(StatusCode::NO_CONTENT)
        }
        None => Err(AppError::NotFound("post")),
    }
}

pub async fn list_user_posts(
    MaybeUser(viewer): MaybeUser,
    Path(username): Path<String>,
    Query(params): Query<PageParams>,
    State(state): State<AppState>,
) -> Result<Json<Page<PostView>>, AppError> {
    let limit = params.limit();
    let rows = repository::list_by_username(
        state.db(),
        viewer.as_ref().map(|v| v.id),
        &username.to_lowercase(),
        params.decode_cursor()?,
        limit,
    )
    .await?;

    let views: Vec<PostView> = rows.into_iter().map(|r| view(&state, r)).collect();
    Ok(Json(Page::build(views, limit, |v| {
        crate::core::pagination::Cursor {
            created_at: v.created_at,
            id: v.id,
        }
    })))
}

/// Map a DB row to the wire type, resolving the image key to a public URL.
pub(crate) fn view(state: &AppState, row: PostRow) -> PostView {
    PostView {
        id: row.id,
        caption: row.caption,
        image_url: state.storage().get_url(&row.image_key),
        created_at: row.created_at,
        author: PostAuthor {
            id: row.author_id,
            username: row.author_username,
            display_name: row.author_display_name,
            avatar_url: row.author_avatar_url,
        },
        like_count: row.like_count,
        comment_count: row.comment_count,
        liked_by_me: row.liked_by_me,
    }
}
