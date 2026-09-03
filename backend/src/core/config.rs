//! Typed application configuration, loaded once at startup from the environment.
//!
//! Every value has exactly one source: an environment variable (optionally
//! seeded from `backend/.env` in local dev). See `backend/.env.example` for the
//! full list with descriptions.

use std::env;

use crate::core::error::AppError;

#[derive(Debug, Clone)]
pub struct Config {
    /// TCP port the HTTP server binds to.
    pub port: u16,
    /// Public base URL of this server as reachable by clients (no trailing
    /// slash). Used to build local-storage media URLs in dev. iOS simulator:
    /// `http://localhost:8080`; Android emulator: `http://10.0.2.2:8080`;
    /// physical device: `http://<your-LAN-ip>:8080`.
    pub public_url: String,
    /// `RUST_LOG`-style filter string.
    pub log_filter: String,
    /// Emit logs as JSON (true in prod, pretty in dev).
    pub log_json: bool,

    /// Postgres connection string. The *only* knob needed to point at a
    /// different database (local Docker, Neon, Supabase, RDS, ...).
    pub database_url: String,
    pub db_max_connections: u32,
    /// Run `migrations/` on boot. Disable to run them as a separate deploy step.
    pub run_migrations_on_start: bool,

    pub jwt: JwtConfig,
    pub google_oauth: GoogleOAuthConfig,
    pub storage: StorageConfig,
}

#[derive(Debug, Clone)]
pub struct JwtConfig {
    /// HMAC signing secret. MUST be long and random in production.
    pub secret: String,
    /// Access token lifetime in seconds.
    pub access_ttl_secs: i64,
    /// Refresh token lifetime in seconds.
    pub refresh_ttl_secs: i64,
    /// `iss` / `aud` claim value.
    pub issuer: String,
}

#[derive(Debug, Clone)]
pub struct GoogleOAuthConfig {
    /// OAuth 2.0 client ID — also the expected `aud` of Google ID tokens.
    pub client_id: String,
    /// Client secret. Only needed for the auth-code flow; kept here so it lives
    /// in exactly one place.
    pub client_secret: String,
}

#[derive(Debug, Clone)]
pub struct StorageConfig {
    /// Cloudflare account ID — used to build the R2 endpoint URL.
    pub r2_account_id: String,
    pub r2_access_key_id: String,
    pub r2_secret_access_key: String,
    pub r2_bucket: String,
    /// Public base URL for objects (R2 public bucket URL or a custom domain),
    /// used by `StorageService::get_url`.
    pub public_base_url: String,
    /// Presigned upload URL lifetime in seconds.
    pub presign_ttl_secs: u64,
    /// Directory the local dev storage backend writes to (used only when R2 is
    /// not configured).
    pub local_dir: String,
}

impl StorageConfig {
    /// S3-compatible endpoint for this account's R2.
    pub fn r2_endpoint(&self) -> String {
        format!("https://{}.r2.cloudflarestorage.com", self.r2_account_id)
    }

    /// Use real Cloudflare R2 when credentials are present; otherwise fall back
    /// to on-disk local storage served by this server (dev convenience).
    pub fn use_r2(&self) -> bool {
        !self.r2_access_key_id.is_empty() && !self.r2_secret_access_key.is_empty()
    }
}

impl Config {
    pub fn from_env() -> Result<Self, AppError> {
        let port = parse_or("PORT", 8080)?;
        Ok(Self {
            port,
            public_url: opt("PUBLIC_URL")
                .unwrap_or_else(|| format!("http://localhost:{port}"))
                .trim_end_matches('/')
                .to_string(),
            log_filter: opt("RUST_LOG").unwrap_or_else(|| "info,circleup_backend=debug".into()),
            log_json: parse_or("LOG_JSON", false)?,

            database_url: req("DATABASE_URL")?,
            db_max_connections: parse_or("DATABASE_MAX_CONNECTIONS", 10)?,
            run_migrations_on_start: parse_or("RUN_MIGRATIONS_ON_START", true)?,

            jwt: JwtConfig {
                secret: req("JWT_SECRET")?,
                access_ttl_secs: parse_or("JWT_ACCESS_TTL_SECS", 900)?, // 15m
                refresh_ttl_secs: parse_or("JWT_REFRESH_TTL_SECS", 60 * 60 * 24 * 30)?, // 30d
                issuer: opt("JWT_ISSUER").unwrap_or_else(|| "circleup".into()),
            },

            google_oauth: GoogleOAuthConfig {
                client_id: opt("GOOGLE_CLIENT_ID").unwrap_or_default(),
                client_secret: opt("GOOGLE_CLIENT_SECRET").unwrap_or_default(),
            },

            storage: StorageConfig {
                r2_account_id: opt("R2_ACCOUNT_ID").unwrap_or_default(),
                r2_access_key_id: opt("R2_ACCESS_KEY_ID").unwrap_or_default(),
                r2_secret_access_key: opt("R2_SECRET_ACCESS_KEY").unwrap_or_default(),
                r2_bucket: opt("R2_BUCKET").unwrap_or_else(|| "circleup-dev".into()),
                public_base_url: opt("R2_PUBLIC_BASE_URL").unwrap_or_default(),
                presign_ttl_secs: parse_or("R2_PRESIGN_TTL_SECS", 900)?,
                local_dir: opt("LOCAL_STORAGE_DIR").unwrap_or_else(|| "./uploads".into()),
            },
        })
    }
}

fn req(key: &str) -> Result<String, AppError> {
    env::var(key).map_err(|_| AppError::Config(format!("missing required env var: {key}")))
}

fn opt(key: &str) -> Option<String> {
    env::var(key).ok().filter(|v| !v.is_empty())
}

fn parse_or<T>(key: &str, default: T) -> Result<T, AppError>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    match env::var(key) {
        Ok(v) if !v.is_empty() => v
            .parse::<T>()
            .map_err(|e| AppError::Config(format!("invalid {key}: {e}"))),
        _ => Ok(default),
    }
}
