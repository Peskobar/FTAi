//! TUI-Patcher-Agent v0.3.0 → v1.0
//! AI-powered Android patching service with enterprise observability

use anyhow::Result;
use std::net::SocketAddr;
use tracing::info;

mod config;
mod health;
mod telemetry;

use config::Config;
use telemetry::init_telemetry;

#[tokio::main]
async fn main() -> Result<()> {
    // Load configuration
    let config = Config::load()?;

    // Initialize telemetry (Q3 2025 Sprint: OTEL Integration)
    let _tracer = init_telemetry(&config.telemetry).await?;

    info!(
        service.name = "tui-patcher-agent",
        service.version = "0.3.0",
        "Start"
    );

    use axum::{routing::get, Router};
    let app = Router::new()
        .route("/health", get(health::health_check))
        .route("/ready", get(health::readiness_check));

    let addr: SocketAddr = format!("{}:{}", config.server.host, config.server.port).parse()?;
    info!(addr = %addr, "Serwer nasłuchuje");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    info!("Zatrzymywanie serwera");
    opentelemetry::global::shutdown_tracer_provider();

    Ok(())
}

async fn shutdown_signal() {
    use tokio::signal;

    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
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

    println!("signal received, starting graceful shutdown");
}
