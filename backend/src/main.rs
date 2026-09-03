//! Circleup API entrypoint.
//!
//! All wiring lives in the library crate (`circleup_backend`) so that
//! integration tests can build the same `Router` without spawning a process.

use std::net::SocketAddr;

use circleup_backend::{app, core::config::Config, core::state::AppState, core::telemetry};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load `.env` (backend/.env) if present — ignored in production where real
    // environment variables are injected by the platform.
    let _ = dotenvy::dotenv();

    let config = Config::from_env()?;
    telemetry::init(&config);

    let state = AppState::bootstrap(config.clone()).await?;

    // Run pending migrations on boot. In production you may prefer to run these
    // as a separate deploy step (`sqlx migrate run`) — flip `run_migrations`.
    if config.run_migrations_on_start {
        tracing::info!("running database migrations");
        sqlx::migrate!("./migrations").run(state.db()).await?;
    }

    let router = app::router(state.clone());

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!(%addr, "circleup api listening");

    axum::serve(
        listener,
        router.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("shutdown signal received");
}
