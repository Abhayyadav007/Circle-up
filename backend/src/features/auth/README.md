# feature: auth

**Responsibility:** create accounts and issue Circleup sessions (JWT access +
refresh). Email/password *and* Google OAuth land here; every other slice just
consumes the `CurrentUser` extractor.

| file             | holds                                                            |
|------------------|-----------------------------------------------------------------|
| `routes.rs`      | `/auth/*` path table                                            |
| `handlers.rs`    | validation, orchestration, get-or-create for OAuth              |
| `dto.rs`         | request bodies (response is `core::auth::jwt::TokenPair`)        |
| `repository.rs`  | `users` table reads/writes for auth                             |

**Add a new OAuth provider (e.g. Apple):**
1. `impl AuthProvider for AppleAuthProvider` in `core/auth/provider.rs`.
2. Register it in `AppState::bootstrap` (`core/state.rs`).
3. Client calls `POST /api/v1/auth/oauth/apple`. No change in this slice.

**Frontend mirror:** `mobile/src/features/auth`.
