//! Authentication primitives shared by the `auth` feature and every protected
//! route.
//!
//! - [`jwt`] — mint & verify our own access/refresh tokens.
//! - [`current_user`] — the [`current_user::CurrentUser`] extractor that guards
//!   protected handlers.
//! - [`password`] — Argon2 hashing/verification for email+password accounts.
//! - [`provider`] — the [`provider::AuthProvider`] trait and Google impl. Add
//!   Apple/GitHub/etc. here without touching the `auth` feature.

pub mod current_user;
pub mod jwt;
pub mod password;
pub mod provider;
