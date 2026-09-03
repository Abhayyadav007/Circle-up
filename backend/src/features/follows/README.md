# feature: follows

**Responsibility:** the follow graph and its list views. `follow` / `unfollow`
are idempotent and return the recomputed `followers_count`. This graph is what
the home `feed` reads from.

| file            | holds                                                       |
|-----------------|------------------------------------------------------------|
| `routes.rs`     | `/profiles/{username}/follow` + `/followers` + `/following`  |
| `handlers.rs`   | username→id resolution, direction plumbing                  |
| `dto.rs`        | `FollowResponse`, `UserSummary` (shared with search)        |
| `repository.rs` | insert/delete with self-follow guard, keyset edge lists     |

**Frontend mirror:** `mobile/src/features/profile` (follow button + lists).
