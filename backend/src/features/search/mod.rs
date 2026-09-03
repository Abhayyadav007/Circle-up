//! # `search` slice
//!
//! Powers the Search/Explore screen. User search is a prefix/substring match on
//! username and display name; post discovery reuses the `feed` explore query.
//!
//! | method | path                  | auth | returns             |
//! |--------|-----------------------|------|---------------------|
//! | GET    | `/search/users?q=`    | opt  | `Vec<UserSummary>`  |
//!
//! For anything more than a scaffold, back this with Postgres full-text search
//! (`tsvector`) or a dedicated search service — the handler contract stays the same.

pub mod handlers;
pub mod repository;
pub mod routes;
