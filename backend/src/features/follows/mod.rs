//! # `follows` slice
//!
//! The follow graph. `follows(follower_id, followee_id)` with a CHECK that you
//! can't follow yourself. Drives the home `feed` and the profile counts.
//!
//! | method | path                             | auth | returns                        |
//! |--------|----------------------------------|------|--------------------------------|
//! | PUT    | `/profiles/{username}/follow`     | yes  | `{ following: true, followers }`|
//! | DELETE | `/profiles/{username}/follow`     | yes  | `{ following: false, followers}`|
//! | GET    | `/profiles/{username}/followers`  | opt  | `Page<UserSummary>`            |
//! | GET    | `/profiles/{username}/following`  | opt  | `Page<UserSummary>`            |

pub mod dto;
pub mod handlers;
pub mod repository;
pub mod routes;
