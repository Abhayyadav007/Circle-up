//! # `auth` slice
//!
//! Owns account creation and session issuance. Everything else in the app trusts
//! the `CurrentUser` extractor ([`crate::core::auth::current_user`]) that this
//! slice's tokens feed.
//!
//! Endpoints (all under `/api/v1/auth`):
//!
//! | method | path              | body                          | returns      |
//! |--------|-------------------|-------------------------------|--------------|
//! | POST   | `/signup`         | email, username, password     | `TokenPair`  |
//! | POST   | `/login`          | email, password               | `TokenPair`  |
//! | POST   | `/refresh`        | refresh_token                 | `TokenPair`  |
//! | POST   | `/oauth/{provider}` | id_token                    | `TokenPair`  |
//!
//! Password may be `NULL` on a row (OAuth-only account). OAuth provider logic is
//! behind [`crate::core::auth::provider::AuthProvider`] — this slice is
//! provider-agnostic.

pub mod dto;
pub mod handlers;
pub mod repository;
pub mod routes;
