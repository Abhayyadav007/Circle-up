//! Shared, cross-cutting infrastructure used by every feature slice.
//!
//! Nothing in here is feature-specific. If a type is only used by one slice, it
//! belongs in that slice, not here.

pub mod auth;
pub mod config;
pub mod error;
pub mod pagination;
pub mod storage;
pub mod telemetry;

pub mod db {
    //! Database pool construction.
    use sqlx::postgres::{PgPool, PgPoolOptions};
    use std::time::Duration;

    use crate::core::config::Config;

    /// Create a Postgres connection pool from config.
    ///
    /// The pool is the *only* thing that knows about the database. Swapping to a
    /// managed provider (Neon, Supabase, RDS) is purely a `DATABASE_URL` change.
    pub async fn connect(config: &Config) -> Result<PgPool, sqlx::Error> {
        PgPoolOptions::new()
            .max_connections(config.db_max_connections)
            .acquire_timeout(Duration::from_secs(5))
            .connect(&config.database_url)
            .await
    }
}

pub mod state;
