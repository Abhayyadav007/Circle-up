//! Handlers for `comments`.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::core::auth::current_user::{CurrentUser, MaybeUser};
use crate::core::error::AppError;
use crate::core::pagination::{Page, PageParams};
use crate::core::state::AppState;

use super::dto::{CommentAuthor, CommentView, CreateCommentRequest};
use super::repository::{self, CommentRow};

pub async fn create(
    user: CurrentUser,
    Path(post_id): Path<Uuid>,
    State(state): State<AppState>,
    Json(body): Json<CreateCommentRequest>,
) -> Result<(StatusCode, Json<CommentView>), AppError> {
    let text = body.body.trim();
    if text.is_empty() || text.len() > 1_000 {
        return Err(AppError::bad_request("comment must be 1–1000 characters"));
    }
    let row = repository::create(state.db(), post_id, user.id, text).await?;
    Ok((StatusCode::CREATED, Json(view(row, Some(user.id)))))
}

pub async fn list(
    MaybeUser(viewer): MaybeUser,
    Path(post_id): Path<Uuid>,
    Query(params): Query<PageParams>,
    State(state): State<AppState>,
) -> Result<Json<Page<CommentView>>, AppError> {
    let limit = params.limit();
    let rows = repository::list(state.db(), post_id, params.decode_cursor()?, limit).await?;
    let viewer_id = viewer.as_ref().map(|v| v.id);

    let views: Vec<CommentView> = rows.into_iter().map(|r| view(r, viewer_id)).collect();
    Ok(Json(Page::build(views, limit, |v| {
        crate::core::pagination::Cursor {
            created_at: v.created_at,
            id: v.id,
        }
    })))
}

pub async fn remove(
    user: CurrentUser,
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<StatusCode, AppError> {
    if repository::delete_permitted(state.db(), id, user.id).await? {
        Ok(StatusCode::NO_CONTENT)
    } else {
        // Either it doesn't exist or the caller isn't allowed — don't leak which.
        Err(AppError::NotFound("comment"))
    }
}

fn view(row: CommentRow, viewer_id: Option<Uuid>) -> CommentView {
    let can_delete = viewer_id
        .map(|v| v == row.author_id || v == row.post_author_id)
        .unwrap_or(false);
    CommentView {
        id: row.id,
        body: row.body,
        created_at: row.created_at,
        author: CommentAuthor {
            id: row.author_id,
            username: row.author_username,
            avatar_url: row.author_avatar_url,
        },
        can_delete,
    }
}
