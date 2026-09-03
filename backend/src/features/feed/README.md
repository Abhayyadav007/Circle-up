# feature: feed

**Responsibility:** assemble the home timeline. Read-only — it composes `posts`
and `follows` and owns no tables of its own.

| file            | holds                                                        |
|-----------------|------------------------------------------------------------|
| `routes.rs`     | `/feed`, `/feed/explore`                                     |
| `handlers.rs`   | authed→home vs anonymous→explore fallback, row→`PostView`    |
| `dto.rs`        | re-export of `posts::dto::PostView` (shared contract)        |
| `repository.rs` | keyset queries built on `posts::repository::SELECT_POST`     |

**Cursor pagination:** `?limit=1..50&cursor=<opaque>`. Response is
`{ items: PostView[], next_cursor: string | null }`. Keyset on
`(created_at, id)` — stable under new posts, no `OFFSET`.

**Frontend mirror:** `mobile/src/features/feed`.
