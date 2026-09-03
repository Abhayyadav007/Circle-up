//! Handlers for `feed`.

use axum::{
    extract::{Query, State},
    Json,
};

use crate::core::auth::current_user::MaybeUser;
use crate::core::error::AppError;
use crate::core::pagination::{Cursor, Page, PageParams};
use crate::core::state::AppState;
use crate::features::posts::handlers::view as post_view;

use super::dto::PostView;
use super::repository;

pub async fn home(
    MaybeUser(viewer): MaybeUser,
    Query(params): Query<PageParams>,
    State(state): State<AppState>,
) -> Result<Json<Page<PostView>>, AppError> {
    let limit = params.limit();
    let after = params.decode_cursor()?;

    let rows = match &viewer {
        Some(user) => repository::home(state.db(), user.id, after, limit).await?,
        // Not signed in → show the explore feed instead of an empty timeline.
        None => repository::explore(state.db(), None, after, limit).await?,
    };

    Ok(Json(page_of(&state, rows, limit)))
}

pub async fn explore(
    MaybeUser(viewer): MaybeUser,
    Query(params): Query<PageParams>,
    State(state): State<AppState>,
) -> Result<Json<Page<PostView>>, AppError> {
    let limit = params.limit();
    let rows = repository::explore(
        state.db(),
        viewer.as_ref().map(|v| v.id),
        params.decode_cursor()?,
        limit,
    )
    .await?;
    Ok(Json(page_of(&state, rows, limit)))
}

fn page_of(
    state: &AppState,
    rows: Vec<crate::features::posts::repository::PostRow>,
    limit: i64,
) -> Page<PostView> {
    let views: Vec<PostView> = rows.into_iter().map(|r| post_view(state, r)).collect();
    Page::build(views, limit, |v| Cursor {
        created_at: v.created_at,
        id: v.id,
    })
}
