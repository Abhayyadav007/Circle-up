# feature: profile (mobile)

Mirrors backend `features/profiles` **and** `features/follows` (follow button +
follower/following lists live here on the client).

| path                            | responsibility                                   |
|---------------------------------|-------------------------------------------------|
| `api/profileApi.ts`             | profile read/update, follow/unfollow, edge lists |
| `hooks/useProfile.ts`           | `useProfile(username?)` + `useFollow(username)`   |
| `screens/ProfileScreen.tsx`     | header (avatar, counts, bio, follow/logout) + 3-col post grid |

`ProfileScreen` is reused by the **Profile tab** (own profile) and the
**UserProfile** stack route (`{ username }`).
