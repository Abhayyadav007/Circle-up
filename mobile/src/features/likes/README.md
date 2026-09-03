# feature: likes (mobile)

Mirrors backend `features/likes`. Tiny by design — one API module and one
optimistic hook, consumed by `feed/PostCard` and `post/PostDetailScreen`.

| path                | responsibility                                    |
|---------------------|--------------------------------------------------|
| `api/likeApi.ts`    | `PUT` / `DELETE /posts/{id}/like`                  |
| `hooks/useLike.ts`  | optimistic toggle, reconciles with server count   |
