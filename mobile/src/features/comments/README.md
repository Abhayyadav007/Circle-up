# feature: comments (mobile)

Mirrors backend `features/comments`. Rendered inside `post/PostDetailScreen`.

| path                          | responsibility                              |
|-------------------------------|--------------------------------------------|
| `api/commentApi.ts`           | list (paged) / create / delete              |
| `hooks/useComments.ts`        | infinite query + add/remove mutations       |
| `components/CommentItem.tsx`  | single comment row with permission-aware delete |
| `components/CommentComposer.tsx` | text input + post button                  |
