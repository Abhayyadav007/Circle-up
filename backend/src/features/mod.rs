//! Feature slices. Each submodule is a self-contained vertical: routes →
//! handlers → dto → repository.
//!
//! | slice      | owns                                              | frontend mirror        |
//! |------------|---------------------------------------------------|------------------------|
//! | `auth`     | signup, login, refresh, Google OAuth              | `mobile/features/auth` |
//! | `profiles` | user profile read/update, `me`                    | `.../features/profile` |
//! | `posts`    | create/read/delete posts, image upload presign    | `.../features/post`    |
//! | `likes`    | like / unlike a post                              | `.../features/likes`   |
//! | `comments` | add / list / delete comments                      | `.../features/comments`|
//! | `follows`  | follow / unfollow, followers & following lists    | `.../features/profile` |
//! | `feed`     | cursor-paginated home feed                        | `.../features/feed`    |
//! | `search`   | user search (explore reuses `feed`)               | `.../features/search`  |

pub mod auth;
pub mod comments;
pub mod feed;
pub mod follows;
pub mod likes;
pub mod posts;
pub mod profiles;
pub mod search;
