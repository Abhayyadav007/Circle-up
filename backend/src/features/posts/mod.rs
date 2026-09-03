//! # `posts` slice
//!
//! Create, read, and delete image posts. Image bytes go **straight to R2** via a
//! presigned URL; this API only ever handles the resulting object key.
//!
//! Upload flow:
//! 1. `POST /posts/uploads` → `{ url, key, headers, ... }` (presigned PUT).
//! 2. Client `PUT`s the file bytes to `url`.
//! 3. `POST /posts` with `{ image_key: key, caption }`.
//!
//! | method | path                      | auth  | notes                     |
//! |--------|---------------------------|-------|---------------------------|
//! | POST   | `/posts/uploads`          | yes   | mint presigned upload URL |
//! | POST   | `/posts`                  | yes   | create post from a key    |
//! | GET    | `/posts/{id}`             | opt   | single post               |
//! | DELETE | `/posts/{id}`            | owner | deletes row + R2 object   |
//! | GET    | `/profiles/{username}/posts` | opt | that user's grid (paged) |

pub mod dto;
pub mod handlers;
pub mod repository;
pub mod routes;
