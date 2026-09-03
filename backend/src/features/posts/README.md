# feature: posts

**Responsibility:** the post lifecycle — create (from a storage object key), read
one, delete (owner only, also removes the stored object), and list one user's
grid. Likes and comments are their own slices; the home feed is `feed`.

| file            | holds                                                            |
|-----------------|-----------------------------------------------------------------|
| `routes.rs`     | `/posts/*` and `/profiles/{username}/posts`                      |
| `handlers.rs`   | upload-type validation, ownership checks, row→`PostView` mapping  |
| `dto.rs`        | `CreatePostRequest`, `CreateUploadRequest`, **`PostView`** (shared with `feed`) |
| `repository.rs` | `SELECT_POST` (shared with `feed`), insert/delete, user grid     |

**Uploads use a presigned PUT URL.** `POST /posts/uploads` returns one
(`core::storage`); the client PUTs the bytes there, then calls `POST /posts` with
the key. With R2 configured the bytes go straight to Cloudflare; in local-dev
mode the URL points at this server's `/media` route (see `core::storage::local`).

**Frontend mirror:** `mobile/src/features/post`.
