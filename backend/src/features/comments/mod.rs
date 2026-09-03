//! # `comments` slice
//!
//! Add, list (cursor-paginated, oldest→newest), and delete comments on a post.
//! A comment can be deleted by its author **or** by the post's author.
//!
//! | method | path                      | auth  | returns              |
//! |--------|---------------------------|-------|----------------------|
//! | POST   | `/posts/{id}/comments`    | yes   | `CommentView`        |
//! | GET    | `/posts/{id}/comments`    | opt   | `Page<CommentView>`  |
//! | DELETE | `/comments/{id}`          | owner | 204                  |

pub mod dto;
pub mod handlers;
pub mod repository;
pub mod routes;
