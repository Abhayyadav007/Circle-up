# feature: profiles

**Responsibility:** the public user profile — display name, bio, avatar, and the
post/follower/following counts. Account *creation* is `auth`; the follow *graph*
is `follows`; this slice just reads and lightly edits the `users` row.

| file            | holds                                                    |
|-----------------|---------------------------------------------------------|
| `routes.rs`     | `/profiles/*` table                                      |
| `handlers.rs`   | ownership checks, response shaping                       |
| `dto.rs`        | `ProfileResponse`, `UpdateProfileRequest` (PATCH semantics) |
| `repository.rs` | profile SELECT with count sub-queries; partial UPDATE    |

**Frontend mirror:** `mobile/src/features/profile`.
