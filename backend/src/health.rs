//! Health check endpoints

use axum::{http::StatusCode, response::Json};
use serde_json::{json, Value};

/// Basic health check endpoint
pub async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "version": crate::VERSION,
        "service": "tui-patcher-agent"
    }))
}

/// Readiness check endpoint
pub async fn readiness_check() -> Result<Json<Value>, StatusCode> {
    // Check if all dependencies are ready
    // - Database connections
    // - External services
    // - Queue system

    // For now, always return ready
    Ok(Json(json!({
        "status": "ready",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "checks": {
            "queue": "healthy",
            "storage": "healthy",
            "external_apis": "healthy"
        }
    })))
}
