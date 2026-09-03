//! # `feed` slice
//!
//! Read-only. Assembles the cursor-paginated home timeline from the `follows`
//! graph and returns the same [`PostView`] shape as the `posts` slice.
//!
//! | method | path            | auth | contents                                    |
//! |--------|-----------------|------|---------------------------------------------|
//! | GET    | `/feed`         | opt  | posts by people you follow (+ your own); falls back to recent-global when unauthenticated |
//! | GET    | `/feed/explore` | opt  | recent global posts for Search/Explore      |
//!
//! Pagination: `?limit=1..50&cursor=<opaque>` — see [`crate::core::pagination`].

pub mod dto;
pub mod handlers;
pub mod repository;
pub mod routes;
