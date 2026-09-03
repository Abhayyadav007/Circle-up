# feature: likes

**Responsibility:** the like relationship between a user and a post. Idempotent
on both sides (`ON CONFLICT DO NOTHING` / `DELETE` no-op). Every response returns
the recomputed `like_count` so the client can reconcile optimistic UI.

| file            | holds                                    |
|-----------------|------------------------------------------|
| `routes.rs`     | `PUT`/`DELETE /posts/{id}/like`           |
| `handlers.rs`   | existence check, response shaping         |
| `dto.rs`        | `LikeResponse`                            |
| `repository.rs` | insert/delete on `likes`, count          |

**Frontend mirror:** `mobile/src/features/likes` (usually consumed inside the feed card).
