//! # `likes` slice
//!
//! Like / unlike a post. Idempotent: liking twice is a no-op, unliking a
//! not-liked post is a no-op. The `likes` table PK is `(user_id, post_id)`.
//!
//! | method | path                  | auth | returns                     |
//! |--------|-----------------------|------|-----------------------------|
//! | PUT    | `/posts/{id}/like`    | yes  | `{ liked: true,  like_count }` |
//! | DELETE | `/posts/{id}/like`    | yes  | `{ liked: false, like_count }` |

pub mod dto;
pub mod handlers;
pub mod repository;
pub mod routes;
