# Circleup backend

Axum + SQLx + PostgreSQL. **Vertical-slice architecture** — see
`src/lib.rs` for the full layout doc.

```
src/
├── main.rs            process entrypoint (wiring lives in the lib crate)
├── lib.rs             crate doc: the slice pattern
├── app.rs             router assembly — the one file that knows every slice
├── core/              shared infra (not feature-specific)
│   ├── config.rs      typed Config, loaded from env
│   ├── error.rs       AppError enum + IntoResponse
│   ├── state.rs       Arc<AppState>: db pool, config, storage, auth providers
│   ├── pagination.rs  cursor/keyset pagination helpers
│   ├── auth/          jwt, CurrentUser extractor, password hashing, AuthProvider trait
│   └── storage/       StorageService trait + Cloudflare R2 impl
└── features/          one folder per feature (routes / handlers / dto / repository)
    ├── auth/  profiles/  posts/  feed/  likes/  comments/  follows/  search/
```

## Common commands

```bash
cargo run                                   # start (reads backend/.env)
cargo test                                  # needs a running Postgres
cargo fmt && cargo clippy --all-targets -- -D warnings

sqlx migrate run                            # apply migrations/
sqlx migrate add <name>                     # new migration file
```

Env vars: copy `.env.example` → `.env`. `DATABASE_URL` is the only knob needed
to change databases (local Docker → Neon/Supabase/RDS), no code changes.

## Notes

- SQL uses runtime-checked `sqlx::query_as` (not the `query!` macros), so the
  crate builds without a live DB or `sqlx-data.json`.
- Every handler returns `Result<_, AppError>`; failures render as
  `{ "error": { "code", "message" } }`.
- Image uploads use presigned R2 PUT URLs — bytes never transit this process.
