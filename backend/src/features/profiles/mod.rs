//! # `profiles` slice
//!
//! Read and update the public-facing user profile: display name, bio, avatar,
//! and the follower / following / post counts shown on the profile screen.
//!
//! | method | path                   | auth | notes                               |
//! |--------|------------------------|------|-------------------------------------|
//! | GET    | `/profiles/me`         | yes  | the caller's own profile            |
//! | PATCH  | `/profiles/me`         | yes  | update display_name / bio / avatar  |
//! | GET    | `/profiles/{username}` | opt  | includes `is_following` when authed  |

pub mod dto;
pub mod handlers;
pub mod repository;
pub mod routes;
