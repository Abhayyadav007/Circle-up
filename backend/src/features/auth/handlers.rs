//! Request handling + orchestration for `auth`. No SQL here (that's
//! `repository`); no HTTP wiring here (that's `routes`).

use axum::{
    extract::{Path, State},
    Json,
};

use crate::core::auth::jwt::{JwtService, TokenKind, TokenPair};
use crate::core::auth::{password, provider::ExternalIdentity};
use crate::core::error::{AppError, FieldError};
use crate::core::state::AppState;

use super::dto::{LoginRequest, OAuthRequest, RefreshRequest, SignupRequest};
use super::repository::{self, NewUser};

pub async fn signup(
    State(state): State<AppState>,
    Json(body): Json<SignupRequest>,
) -> Result<Json<TokenPair>, AppError> {
    validate_signup(&body)?;

    let hash = password::hash(&body.password)?;
    let user = repository::insert_user(
        state.db(),
        NewUser {
            email: &body.email.trim().to_lowercase(),
            username: &body.username.trim().to_lowercase(),
            password_hash: Some(&hash),
            google_id: None,
            avatar_url: None,
            display_name: body.display_name.as_deref(),
        },
    )
    .await?;

    Ok(Json(jwt(&state).issue_pair(user.id)?))
}

pub async fn login(
    State(state): State<AppState>,
    Json(body): Json<LoginRequest>,
) -> Result<Json<TokenPair>, AppError> {
    let user = repository::find_by_email(state.db(), &body.email.trim().to_lowercase())
        .await?
        .ok_or(AppError::Unauthorized)?;

    let hash = user.password_hash.as_deref().ok_or_else(|| {
        AppError::bad_request("this account uses Google Sign-In — continue with Google")
    })?;

    if !password::verify(&body.password, hash)? {
        return Err(AppError::Unauthorized);
    }

    Ok(Json(jwt(&state).issue_pair(user.id)?))
}

pub async fn refresh(
    State(state): State<AppState>,
    Json(body): Json<RefreshRequest>,
) -> Result<Json<TokenPair>, AppError> {
    let claims = jwt(&state).verify(&body.refresh_token, TokenKind::Refresh)?;
    // Ensure the user still exists.
    repository::find_by_id(state.db(), claims.sub)
        .await?
        .ok_or(AppError::Unauthorized)?;
    Ok(Json(jwt(&state).issue_pair(claims.sub)?))
}

/// `POST /auth/oauth/{provider}` — verify a provider ID token, then get-or-create
/// the local user and issue our own session.
pub async fn oauth(
    State(state): State<AppState>,
    Path(provider): Path<String>,
    Json(body): Json<OAuthRequest>,
) -> Result<Json<TokenPair>, AppError> {
    let provider = state.auth_providers().get(&provider)?;
    let identity = provider.verify_id_token(&body.id_token).await?;

    if !identity.email_verified {
        return Err(AppError::bad_request("provider email is not verified"));
    }

    let user = upsert_oauth_user(&state, &identity).await?;
    Ok(Json(jwt(&state).issue_pair(user.id)?))
}

/// get-or-create for OAuth: match on provider subject, else link to an existing
/// account with the same email, else create a fresh password-less account.
async fn upsert_oauth_user(
    state: &AppState,
    identity: &ExternalIdentity,
) -> Result<repository::AuthUser, AppError> {
    if let Some(user) = repository::find_by_google_id(state.db(), &identity.subject).await? {
        return Ok(user);
    }

    if let Some(existing) = repository::find_by_email(state.db(), &identity.email).await? {
        return repository::attach_google_id(
            state.db(),
            existing.id,
            &identity.subject,
            identity.avatar_url.as_deref(),
        )
        .await;
    }

    let username = derive_username(&identity.email);
    repository::insert_user(
        state.db(),
        NewUser {
            email: &identity.email.to_lowercase(),
            username: &username,
            password_hash: None,
            google_id: Some(&identity.subject),
            avatar_url: identity.avatar_url.as_deref(),
            display_name: identity.name.as_deref(),
        },
    )
    .await
}

fn jwt(state: &AppState) -> JwtService {
    JwtService::new(state.config().jwt.clone())
}

fn derive_username(email: &str) -> String {
    let base: String = email
        .split('@')
        .next()
        .unwrap_or("user")
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '.')
        .take(20)
        .collect();
    let suffix = uuid::Uuid::new_v4().simple().to_string();
    format!("{}_{}", base.to_lowercase(), &suffix[..6])
}

fn validate_signup(body: &SignupRequest) -> Result<(), AppError> {
    let mut errors = Vec::new();
    if !body.email.contains('@') {
        errors.push(FieldError {
            field: "email".into(),
            message: "must be a valid email".into(),
        });
    }
    let uname = body.username.trim();
    if uname.len() < 3 || uname.len() > 30 {
        errors.push(FieldError {
            field: "username".into(),
            message: "must be 3–30 characters".into(),
        });
    }
    if !uname
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '.')
    {
        errors.push(FieldError {
            field: "username".into(),
            message: "only letters, digits, '.' and '_'".into(),
        });
    }
    if body.password.len() < 8 {
        errors.push(FieldError {
            field: "password".into(),
            message: "must be at least 8 characters".into(),
        });
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(AppError::Validation(errors))
    }
}
