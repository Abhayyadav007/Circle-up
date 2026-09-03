//! Circleup backend library crate.
//!
//! # Architecture: vertical slices
//!
//! Code is grouped by **feature**, not by technical layer. Each feature lives in
//! [`features`] and owns its full request lifecycle:
//!
//! ```text
//! features/<name>/
//!   routes.rs      -> route table only (path -> handler), no logic
//!   handlers.rs    -> HTTP concerns + orchestration
//!   dto.rs         -> request/response structs (the wire contract)
//!   repository.rs  -> all SQLx / DB access for this feature
//!   mod.rs         -> re-exports + a doc comment describing the slice
//! ```
//!
//! Anything shared by more than one slice lives in [`core`]: [`core::state::AppState`],
//! [`core::error::AppError`], [`core::config::Config`], auth utilities
//! ([`core::auth`]), and the storage abstraction ([`core::storage`]).
//!
//! Frontend feature folders mirror these names 1:1 (see `/mobile/src/features`).

pub mod app;
pub mod core;
pub mod features;

/// Convenient result alias used across the crate.
pub type Result<T, E = core::error::AppError> = std::result::Result<T, E>;
