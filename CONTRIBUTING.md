# Contributing to Circleup

## Branch naming

```
<type>/<short-kebab-summary>

feat/google-oauth
fix/feed-cursor-off-by-one
chore/bump-axum
refactor/storage-trait
docs/r2-setup
```

Branch off `main`. Keep branches short-lived.

## Commit conventions

[Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<optional scope>): <summary in imperative mood>

feat(posts): add presigned upload endpoint
fix(auth): reject refresh tokens with wrong `typ`
test(feed): cover empty-timeline fallback
```

Types: `feat`, `fix`, `chore`, `refactor`, `docs`, `test`, `perf`, `ci`.
Scope is usually the feature slice name (`auth`, `feed`, `posts`, …).

One logical change per commit. Rebase, don't merge `main` into your branch.

## Pull requests

- PRs target `main`. The **`gate / all-green`** check must pass (it runs the
  backend and/or mobile suite depending on what you touched).
- Run the relevant checks locally first:
  ```bash
  # backend
  cd backend && cargo fmt && cargo clippy --all-targets -- -D warnings && cargo test
  # mobile
  cd mobile && npm run lint && npm run typecheck && npm test
  ```
- If you changed a wire type (`dto.rs` on the backend), update the matching
  mobile type (`src/api/types.ts` or the feature's `api/`) **in the same PR**.
- Update the feature's `README.md` if you changed its responsibility.

## Running migrations

```bash
cd backend
sqlx migrate run                 # apply pending migrations to $DATABASE_URL
sqlx migrate add <name>          # create backend/migrations/<timestamp>_<name>.sql
sqlx migrate info                # show applied / pending
```

Migrations are **forward-only** and plain SQL. Never edit a migration that has
been merged — add a new one. CI applies them against a fresh Postgres, so a
broken migration fails the build.

---

## How to add a new feature slice

Say you're adding **bookmarks** (`POST/DELETE /posts/{id}/bookmark`, `GET /bookmarks`).

### 1. Backend — `backend/src/features/bookmarks/`

```
bookmarks/
├── mod.rs          # doc comment: what this slice owns + endpoint table
├── routes.rs       # pub fn router() -> Router<AppState>   (paths only)
├── handlers.rs     # async fns: extract, validate, orchestrate, shape response
├── dto.rs          # request/response structs (#[derive(Serialize/Deserialize)])
├── repository.rs   # all SQLx for this slice
└── README.md       # responsibility + file table + frontend mirror
```

1. Add a migration: `sqlx migrate add bookmarks` → write the `bookmarks` table.
2. Register the module in `backend/src/features/mod.rs`:
   ```rust
   pub mod bookmarks;
   ```
3. Mount the router in `backend/src/app.rs`:
   ```rust
   .merge(features::bookmarks::routes::router())
   ```
4. Protected routes take the `CurrentUser` extractor
   (`crate::core::auth::current_user::CurrentUser`). Paginated routes take
   `Query<PageParams>` and return `Page<T>` (`crate::core::pagination`).
5. Return `Result<Json<T>, AppError>` — never `unwrap`, never a bespoke error type.
6. `cargo fmt && cargo clippy --all-targets -- -D warnings && cargo test`.

### 2. Mobile — `mobile/src/features/bookmarks/`

Use the **same folder name** as the backend slice.

```
bookmarks/
├── api/bookmarkApi.ts   # typed axios calls via `@/api/client`
├── hooks/useBookmarks.ts# React Query queries/mutations, keys from `@/api/queryClient`
├── screens/             # screen components (functional, composed from @/components/ui)
├── components/          # slice-local components
└── README.md
```

1. Mirror wire types in `mobile/src/api/types.ts` (or keep slice-local types in
   the feature's `api/`).
2. Add query keys to `queryKeys` in `mobile/src/api/queryClient.ts`.
3. Build UI only from `@/components/ui` + `@/theme` — no raw hex, no magic numbers.
4. Wire screens into `src/navigation/` (`RootNavigator` or `TabNavigator`) and add
   the route to `src/navigation/types.ts`.
5. `npm run lint && npm run typecheck && npm test`.

### 3. Naming must match

Backend `features/bookmarks` ↔ mobile `features/bookmarks`. If the natural mobile
name differs (e.g. backend `profiles` ↔ mobile `profile`), document the mapping
in the table in the root `README.md`.

---

## Code style

**Backend:** `rustfmt.toml` + `clippy.toml` at `backend/`, both enforced in CI
(`clippy` with `-D warnings`). Prefer `?` + `AppError` over `match`-and-map.

**Mobile:** ESLint (`eslint-config-expo` + import ordering) and `tsc --strict`,
both enforced in CI (`--max-warnings 0`). Functional components only. Import
order: external → `@/…` → relative, groups separated by a blank line
(`eslint --fix` sorts this for you).
