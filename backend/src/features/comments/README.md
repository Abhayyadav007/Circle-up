# feature: comments

**Responsibility:** comments on a post. Listing is cursor-paginated oldest→newest
(so new comments append at the bottom like a chat). Deletion is allowed for the
comment's author or the post's author.

| file            | holds                                             |
|-----------------|--------------------------------------------------|
| `routes.rs`     | `/posts/{id}/comments`, `/comments/{id}`           |
| `handlers.rs`   | length validation, permission-aware `can_delete`   |
| `dto.rs`        | `CreateCommentRequest`, `CommentView`              |
| `repository.rs` | insert with FK→404 mapping, keyset list, guarded delete |

**Frontend mirror:** `mobile/src/features/comments`.
