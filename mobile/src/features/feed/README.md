# feature: feed (mobile)

Mirrors backend `features/feed`. The home timeline + explore grid.

| path                     | responsibility                                     |
|--------------------------|---------------------------------------------------|
| `api/feedApi.ts`         | `GET /feed`, `GET /feed/explore` (cursor paged)    |
| `hooks/useFeed.ts`       | `useInfiniteQuery` wrapper, flattens pages         |
| `components/PostCard.tsx`| one feed item (image, like, comment count, caption)|
| `screens/FeedScreen.tsx` | `FlatList` with pull-to-refresh + infinite scroll  |

Likes are handled by the `likes` feature (`useLike`) inside `PostCard`.
