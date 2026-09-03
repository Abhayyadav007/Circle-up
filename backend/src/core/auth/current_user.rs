//! `CurrentUser` — the extractor that guards every protected route.
//!
//! Usage in a handler:
//!
//! ```ignore
//! async fn create_post(user: CurrentUser, State(state): State<AppState>) -> ... {
//!     // `user.id` is a verified, existing user.
//! }
//! ```
//!
//! It reads the `Authorization: Bearer <access_token>` header, verifies the JWT
//! against [`JwtService`], and loads the user row (so deleted users are rejected
//! immediately).

use axum::{
    extract::{FromRef, FromRequestParts},
    http::{header::AUTHORIZATION, request::Parts},
};
use uuid::Uuid;

use crate::core::auth::jwt::{JwtService, TokenKind};
use crate::core::error::AppError;
use crate::core::state::AppState;

#[derive(Debug, Clone)]
pub struct CurrentUser {
    pub id: Uuid,
    pub username: String,
    pub email: String,
}

impl<S> FromRequestParts<S> for CurrentUser
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app = AppState::from_ref(state);

        let token = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .ok_or(AppError::Unauthorized)?;

        let jwt = JwtService::new(app.config().jwt.clone());
        let claims = jwt.verify(token, TokenKind::Access)?;

        let row = sqlx::query_as::<_, (Uuid, String, String)>(
            "SELECT id, username, email FROM users WHERE id = $1",
        )
        .bind(claims.sub)
        .fetch_optional(app.db())
        .await?
        .ok_or(AppError::Unauthorized)?;

        Ok(CurrentUser {
            id: row.0,
            username: row.1,
            email: row.2,
        })
    }
}

/// Optional variant — `Some` when a valid token is present, `None` otherwise.
/// Useful for endpoints like the feed that personalise but don't require auth.
pub struct MaybeUser(pub Option<CurrentUser>);

impl<S> FromRequestParts<S> for MaybeUser
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Ok(MaybeUser(
            CurrentUser::from_request_parts(parts, state).await.ok(),
        ))
    }
}
