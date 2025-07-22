//! TUI-Patcher-Agent v0.3.0 → v1.0
//! AI-powered Android patching service with enterprise observability

use anyhow::Result;
use tracing::info;
use std::net::SocketAddr;

mod config;
mod telemetry;
mod routes;
mod middleware;
mod queue;
mod patch;
mod notifications;
mod alerts;

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
        "Starting TUI-Patcher-Agent"
    );

    // Build application with all middleware
    let app = routes::create_router()
        .layer(middleware::tracing::TraceparentLayer::new())  // Q3: Traceparent Middleware
        .layer(middleware::metrics::MetricsLayer::new())
        .layer(middleware::auth::AuthLayer::new(config.auth.clone()))
        .layer(middleware::rate_limit::RateLimitLayer::new(config.rate_limit.clone()));

    // Initialize background services
    let alert_service = alerts::AlertService::new(config.alerts.clone()).await?;
    tokio::spawn(async move {
        alert_service.run().await;
    });

    // Start server
    let addr: SocketAddr = format!("{}:{}", config.server.host, config.server.port).parse()?;
    info!(addr = %addr, "Server listening");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    info!("Server shutting down");
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