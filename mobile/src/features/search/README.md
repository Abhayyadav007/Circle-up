# feature: search (mobile)

Mirrors backend `features/search`. People search now; the explore grid helper is
here too (`searchApi.explore`) for when the screen grows a discovery tab.

| path                          | responsibility                          |
|-------------------------------|----------------------------------------|
| `api/searchApi.ts`            | `GET /search/users`, `GET /feed/explore`|
| `hooks/useSearch.ts`          | debounce-free query, min 2 chars         |
| `screens/SearchScreen.tsx`    | search box + tappable results list       |
