# feature: search

**Responsibility:** back the Search/Explore screen. Currently user search only
(`ILIKE` on username / display name); post discovery is served by
`feed::explore`. Swap the repository for Postgres FTS or a search service later —
the route contract doesn't change.

| file            | holds                                  |
|-----------------|----------------------------------------|
| `routes.rs`     | `GET /search/users`                     |
| `handlers.rs`   | min-length guard, query parsing         |
| `repository.rs` | `ILIKE` match returning `UserSummary`   |

**Frontend mirror:** `mobile/src/features/search`.
