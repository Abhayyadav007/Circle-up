# Circleup

A blue-themed, Instagram-style social app. Monorepo:

```
circleup/
├── backend/         Rust · Axum · SQLx · PostgreSQL   (vertical-slice architecture)
├── mobile/          React Native (Expo) · TypeScript  (feature folders mirror the backend)
├── docker-compose.yml   Postgres (+ optional hot-reload backend) for local dev
└── .github/workflows/   backend.yml · mobile.yml · gate.yml
```

**Backend and mobile map 1:1.** A backend slice `features/<name>` has a mobile
counterpart `mobile/src/features/<name>` with the same responsibility. Adding a
feature means adding a folder on each side.

| domain     | backend `features/` | mobile `src/features/` | notes |
|------------|---------------------|------------------------|-------|
| Auth       | `auth`              | `auth`                 | JWT access/refresh + Google OAuth |
| Profiles   | `profiles`          | `profile`              | profile read/update, follow button lives here on mobile |
| Posts      | `posts`             | `post`                 | presigned R2 uploads |
| Feed       | `feed`              | `feed`                 | cursor-paginated home timeline |
| Likes      | `likes`             | `likes`                | idempotent like/unlike |
| Comments   | `comments`          | `comments`             | flat comments, keyset paged |
| Follows    | `follows`           | `profile`              | follow graph + follower/following lists |
| Search     | `search`            | `search`               | user search; explore reuses the feed |

Each feature folder has its own `README.md` describing what belongs there.

---

## Quick start (new contributor)

Prereqs: Docker, Rust (stable), Node 20+, and the Expo tooling (`npx expo`).

```bash
# 1. clone, then set up env files
cp .env.example .env                    # repo-root: used by docker compose
cp backend/.env.example backend/.env    # backend: used by `cargo run` / `cargo test`

# 2. start Postgres (and, optionally, the API)
docker compose up -d postgres           # just the DB
#   or: docker compose up -d            # DB + API with hot-reload (cargo watch)

# 3. run backend migrations
cd backend
cargo install sqlx-cli --no-default-features --features rustls,postgres   # once
sqlx migrate run                         # applies backend/migrations/*.sql

# 4. run the API natively (skip if you used `docker compose up -d` for backend)
cargo run                                # -> http://localhost:8080
curl localhost:8080/api/v1/health        # {"status":"ok","db":true,...}

# 5. run the app
cd ../mobile
npm install
npx expo start                           # press i / a for simulator, or scan the QR
```

### Running the backend against a managed database later

The connection is swappable via **`DATABASE_URL` alone** — no code changes.
Point it at Neon / Supabase / RDS:

```bash
DATABASE_URL="postgres://user:pass@ep-xyz.neon.tech/circleup?sslmode=require" cargo run
```

---

## Architecture decisions (the short version)

- **Vertical slices, not MVC.** Code is grouped by feature. Each slice owns its
  `routes.rs` (paths only) → `handlers.rs` (HTTP + orchestration) → `dto.rs`
  (wire contract) → `repository.rs` (all SQLx). Shared infra lives in `core/`.
  New engineers add a folder, not a change spread across five layers.
- **`Arc<AppState>` for shared state.** One cheap-to-clone handle carries the DB
  pool, parsed `Config`, the storage service (behind a trait), and the OAuth
  provider registry.
- **One `AppError`.** A single enum implements `IntoResponse`; every handler
  `?`s and every failure is the same JSON envelope
  `{ "error": { "code", "message" } }`.
- **Runtime-checked SQL (`query_as`), not the `query!` macros.** The project
  compiles without a live database or checked-in `sqlx-data.json`, which keeps
  CI and first-clone simple. Swap to the macros later if you want compile-time
  query verification.
- **Storage & OAuth are traits.** `StorageService` and `AuthProvider` are
  swappable without touching handlers. Apple Sign-In is a new `AuthProvider`
  impl + one registration line — deliberately **not** implemented yet (needs a
  paid Apple Developer account; only required for App Store submit).
- **Storage falls back to local disk.** With R2 credentials set, uploads go to
  Cloudflare. Without them (fresh clone), the server writes files under
  `backend/uploads/` and serves them at `/media/<key>` — the full image-post
  flow works with zero external accounts. Switching to R2 is env-vars-only.
- **Uploads bypass the API.** Clients `POST /posts/uploads` for a presigned PUT
  URL, upload bytes straight to storage, then `POST /posts` with the object key.
  (In local-disk mode the "presigned" URL points back at `/media` on this server.)
- **Cursor pagination everywhere.** Keyset on `(created_at, id)` — stable under
  inserts, no `OFFSET`. Response: `{ items, next_cursor }`.
- **Mobile: React Query for server state, Zustand for auth/UI state.** Tokens
  live in the device keychain (`expo-secure-store`). A single Axios client holds
  the base URL + a one-flight refresh interceptor. All colors/spacing/type come
  from `src/theme/theme.ts`.

---

## Database

Local Postgres runs in Docker (`postgres:16`), data persisted in the named
volume `circleup_pgdata` so it survives restarts. Credentials come from the
repo-root `.env` (`POSTGRES_USER` / `POSTGRES_PASSWORD` / `POSTGRES_DB`) — never
hardcoded.

```bash
docker compose up -d postgres     # start
docker compose down               # stop (keeps data)
docker compose down -v            # stop and wipe data

# migrations (from backend/)
sqlx migrate run                  # apply
sqlx migrate add <name>           # scaffold a new backend/migrations/<ts>_<name>.sql
#   or, containerised:
docker compose --profile tools run --rm migrate
```

Core tables: `users`, `posts`, `likes`, `comments`, `follows` — see
`backend/migrations/`.

---

## Image storage

**Local dev (default):** leave the `R2_*` vars blank. The server stores uploads
under `backend/uploads/` and serves them at `/media/<key>`. Set `PUBLIC_URL` in
`backend/.env` to this server's address as the app sees it (`http://localhost:8080`
for the iOS simulator, `http://10.0.2.2:8080` for the Android emulator, your LAN
IP for a physical device) so image URLs resolve on the device.

**Production / sharing a dev bucket:** configure Cloudflare R2 below. No code
changes — the app switches backend automatically when `R2_ACCESS_KEY_ID` and
`R2_SECRET_ACCESS_KEY` are present.

### Cloudflare R2 setup (free tier — under 5 minutes)

R2 is S3-compatible with **zero egress fees** (10 GB storage, 1M Class-A ops/mo free).

1. Cloudflare dashboard → **R2** → **Create bucket** (e.g. `circleup-dev`).
2. **R2** → **Manage R2 API Tokens** → **Create API token** → *Object Read & Write*
   scoped to your bucket. Copy the **Access Key ID** and **Secret Access Key**.
3. Find your **Account ID** on the R2 overview page (right sidebar).
4. Enable public access: bucket → **Settings** → **Public access** → allow the
   `r2.dev` subdomain (or attach a custom domain). Copy that base URL.
5. Fill in `backend/.env` (and the repo-root `.env` if you run the backend in Docker):

   ```
   R2_ACCOUNT_ID=xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
   R2_ACCESS_KEY_ID=xxxxxxxxxxxxxxxxxxxx
   R2_SECRET_ACCESS_KEY=xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
   R2_BUCKET=circleup-dev
   R2_PUBLIC_BASE_URL=https://pub-xxxxxxxx.r2.dev
   ```

The SDK talks to `https://<account_id>.r2.cloudflarestorage.com`. Everything
goes through `StorageService` (`backend/src/core/storage/`) — swapping providers
is one new impl.

---

## Google Sign-In setup

1. [Google Cloud Console](https://console.cloud.google.com/) → create a project.
2. **APIs & Services → OAuth consent screen** → configure (External, add your
   email as a test user).
3. **APIs & Services → Credentials → Create credentials → OAuth client ID**:
   - **Web application** — this client's ID is what the **backend** verifies
     tokens against. Put it in `GOOGLE_CLIENT_ID` (backend `.env`) **and** in
     `mobile/app.json` → `expo.extra.googleWebClientId`.
   - **iOS** — bundle id `app.circleup.mobile`. Put the client ID in
     `expo.extra.googleIosClientId` and its reversed form in the
     `@react-native-google-signin/google-signin` plugin config (`iosUrlScheme`)
     in `app.json`.
   - **Android** — package `app.circleup.mobile` + your debug/release SHA-1.
4. Flow: the app runs the native Google flow → gets an **ID token** → sends it to
   `POST /api/v1/auth/oauth/google` → the backend verifies it and issues a
   Circleup JWT session. Adding Apple later = implement `AuthProvider` for Apple,
   register it, call `POST /api/v1/auth/oauth/apple`.

---

## API surface (v1)

Base path `/api/v1`. Full details in each `backend/src/features/*/README.md`.

```
GET    /health
POST   /auth/signup | /auth/login | /auth/refresh
POST   /auth/oauth/{provider}                 (provider = "google")
GET    /profiles/me            PATCH /profiles/me
GET    /profiles/{username}
PUT    /profiles/{username}/follow             DELETE /profiles/{username}/follow
GET    /profiles/{username}/followers | /following
POST   /posts/uploads                          (presigned R2 URL)
POST   /posts                 GET /posts/{id}   DELETE /posts/{id}
GET    /profiles/{username}/posts
PUT    /posts/{id}/like                         DELETE /posts/{id}/like
GET    /posts/{id}/comments    POST /posts/{id}/comments
DELETE /comments/{id}
GET    /feed | /feed/explore                    (?limit=1..50&cursor=...)
GET    /search/users?q=
```

---

## CI/CD

Three workflows in `.github/workflows/`:

- **`backend.yml`** — on `backend/**`: `cargo fmt --check`, `cargo clippy -D warnings`,
  `cargo test` (with a Postgres service container), `cargo build --release`.
  Caches the cargo registry + `target/`.
- **`mobile.yml`** — on `mobile/**`: `npm ci` (cached), `eslint`, `tsc --noEmit`,
  `jest`, and an `expo export` bundle check.
- **`gate.yml`** — runs on every PR to `main`. Path-filters the diff, invokes
  only the affected suite(s) as reusable workflows, and the **`gate / all-green`**
  job passes only if every triggered suite passed.

**Branch protection:** Settings → Branches → add a rule for `main` →
*Require status checks to pass* → select **`gate / all-green`**. That single
check covers both suites; `backend.yml` / `mobile.yml` still run directly on
pushes to `main`.

---

## Repo conventions & contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md) — branch naming, commit format, and a
step-by-step recipe for adding a new feature slice on both sides.
