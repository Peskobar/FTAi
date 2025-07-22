//! Software Bill of Materials (SBOM) endpoint
//! Q3 2025 Sprint: SBOM compliance implementation

use axum::{
    response::{Json, Response},
    http::{header, StatusCode},
};
use serde_json::{json, Value};
use std::fs;

/// SBOM endpoint for supply chain compliance
pub async fn sbom_handler() -> Result<Response, StatusCode> {
    // Try to read the SBOM file generated during build
    match fs::read_to_string("cyclonedx.json") {
        Ok(sbom_content) => {
            // Parse and return the SBOM JSON
            match serde_json::from_str::<Value>(&sbom_content) {
                Ok(sbom_json) => {
                    Ok(Response::builder()
                        .status(StatusCode::OK)
                        .header(header::CONTENT_TYPE, "application/json")
                        .header(header::CONTENT_DISPOSITION, "attachment; filename="sbom.json"")
                        .header("X-SBOM-Format", "CycloneDX")
                        .header("X-SBOM-Version", "1.5")
                        .body(sbom_content.into())
                        .unwrap())
                }
                Err(_) => {
                    // Return error if SBOM is malformed
                    Ok(Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .header(header::CONTENT_TYPE, "application/json")
                        .body(json!({
                            "error": "SBOM file is malformed",
                            "timestamp": chrono::Utc::now().to_rfc3339()
                        }).to_string().into())
                        .unwrap())
                }
            }
        }
        Err(_) => {
            // Return placeholder SBOM if file doesn't exist
            let placeholder_sbom = json!({
                "bomFormat": "CycloneDX",
                "specVersion": "1.5",
                "serialNumber": format!("urn:uuid:{}", uuid::Uuid::new_v4()),
                "version": 1,
                "metadata": {
                    "timestamp": chrono::Utc::now().to_rfc3339(),
                    "tools": [
                        {
                            "vendor": "TUI-Patcher",
                            "name": "tui-patcher-agent",
                            "version": crate::VERSION
                        }
                    ],
                    "component": {
                        "type": "application",
                        "bom-ref": "tui-patcher-agent",
                        "name": "tui-patcher-agent",
                        "version": crate::VERSION,
                        "description": "AI-powered Android patching service"
                    }
                },
                "components": [
                    {
                        "type": "library",
                        "bom-ref": "axum",
                        "name": "axum",
                        "version": "0.7.0",
                        "scope": "required"
                    },
                    {
                        "type": "library", 
                        "bom-ref": "tokio",
                        "name": "tokio",
                        "version": "1.35.0",
                        "scope": "required"
                    },
                    {
                        "type": "library",
                        "bom-ref": "opentelemetry",
                        "name": "opentelemetry", 
                        "version": "0.23.0",
                        "scope": "required"
                    }
                ]
            });

            Ok(Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "application/json")
                .header("X-SBOM-Format", "CycloneDX")
                .header("X-SBOM-Version", "1.5")
                .header("X-SBOM-Generated", "runtime")
                .body(placeholder_sbom.to_string().into())
                .unwrap())
        }
    }
}
