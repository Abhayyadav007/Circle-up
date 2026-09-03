//! Handlers for `search`.

use axum::{
    extract::{Query, State},
    Json,
};
use serde::Deserialize;

use crate::core::error::AppError;
use crate::core::state::AppState;
use crate::features::follows::dto::UserSummary;

use super::repository;

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: String,
}

pub async fn search_users(
    Query(params): Query<SearchQuery>,
    State(state): State<AppState>,
) -> Result<Json<Vec<UserSummary>>, AppError> {
    let term = params.q.trim();
    if term.len() < 2 {
        return Ok(Json(Vec::new()));
    }
    let results = repository::users(state.db(), term, 20).await?;
    Ok(Json(results))
}
