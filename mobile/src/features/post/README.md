# feature: post (mobile)

Mirrors backend `features/posts`. Single-post view and the create flow.

| path                            | responsibility                                       |
|---------------------------------|----------------------------------------------------|
| `api/postApi.ts`                | get / delete / by-user + the 3-step upload helpers   |
| `hooks/usePost.ts`              | single-post query                                   |
| `hooks/useCreatePost.ts`        | presign → PUT to R2 → create row, then invalidate feed |
| `screens/PostDetailScreen.tsx`  | post + comments + composer                          |
| `screens/CreatePostScreen.tsx`  | image picker + caption + share                      |

**Upload never routes image bytes through the API** — `useCreatePost` PUTs
straight to the presigned R2 URL.
