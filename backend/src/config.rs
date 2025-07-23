//! Configuration management for TUI-Patcher-Agent

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub server: ServerConfig,
    pub telemetry: TelemetryConfig,
    pub auth: AuthConfig,
    pub rate_limit: RateLimitConfig,
    pub queue: QueueConfig,
    pub patch: PatchConfig,
    pub alerts: AlertConfig,
    pub ai: AiConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: Option<usize>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TelemetryConfig {
    pub otel_endpoint: String,
    pub service_name: String,
    pub sampling_ratio: f64,
    pub export_timeout_secs: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuthConfig {
    pub require_api_key: bool,
    pub api_keys: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RateLimitConfig {
    pub requests_per_minute: u32,
    pub burst_size: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QueueConfig {
    pub max_concurrent_jobs: usize,
    pub job_timeout_secs: u64,
    pub cleanup_interval_secs: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PatchConfig {
    pub temp_dir: String,
    pub max_file_size_mb: u64,
    pub timeout_secs: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AlertConfig {
    pub enabled: bool,
    pub slack_webhook_url: Option<String>,
    pub queue_size_threshold: u32,
    pub success_rate_threshold: f64,
    pub latency_threshold_ms: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AiConfig {
    pub perplexity_api_key: Option<String>,
    pub perplexity_api_key_ci: Option<String>,
    pub enable_on_device: bool,
    pub model_path: Option<String>,
}

impl Config {
    pub fn load() -> Result<Self> {
        Self::load_from_file(crate::DEFAULT_CONFIG_PATH)
    }

    pub fn load_from_file(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn load_from_env() -> Result<Self> {
        // Load from environment variables for cloud deployments
        Ok(Config {
            server: ServerConfig {
                host: std::env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
                port: std::env::var("SERVER_PORT")
                    .unwrap_or_else(|_| "8080".to_string())
                    .parse()?,
                workers: std::env::var("SERVER_WORKERS")
                    .ok()
                    .and_then(|s| s.parse().ok()),
            },
            telemetry: TelemetryConfig {
                otel_endpoint: std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT")
                    .unwrap_or_else(|_| "http://localhost:4317".to_string()),
                service_name: std::env::var("OTEL_SERVICE_NAME")
                    .unwrap_or_else(|_| "tui-patcher-agent".to_string()),
                sampling_ratio: std::env::var("OTEL_SAMPLING_RATIO")
                    .unwrap_or_else(|_| "0.1".to_string())
                    .parse()
                    .unwrap_or(0.1),
                export_timeout_secs: 30,
            },
            auth: AuthConfig {
                require_api_key: std::env::var("REQUIRE_API_KEY")
                    .unwrap_or_else(|_| "false".to_string())
                    .parse()
                    .unwrap_or(false),
                api_keys: std::env::var("API_KEYS")
                    .unwrap_or_default()
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect(),
            },
            rate_limit: RateLimitConfig {
                requests_per_minute: 100,
                burst_size: 20,
            },
            queue: QueueConfig {
                max_concurrent_jobs: 5,
                job_timeout_secs: 300,
                cleanup_interval_secs: 3600,
            },
            patch: PatchConfig {
                temp_dir: std::env::var("TEMP_DIR").unwrap_or_else(|_| "/tmp".to_string()),
                max_file_size_mb: 100,
                timeout_secs: 180,
            },
            alerts: AlertConfig {
                enabled: std::env::var("ALERTS_ENABLED")
                    .unwrap_or_else(|_| "true".to_string())
                    .parse()
                    .unwrap_or(true),
                slack_webhook_url: std::env::var("SLACK_WEBHOOK_URL").ok(),
                queue_size_threshold: 10,
                success_rate_threshold: 0.95,
                latency_threshold_ms: 2000,
            },
            ai: AiConfig {
                perplexity_api_key: std::env::var("PPX_API_KEY").ok(),
                perplexity_api_key_ci: std::env::var("PPX_API_KEY_CI").ok(),
                enable_on_device: false,
                model_path: None,
            },
        })
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 8080,
                workers: None,
            },
            telemetry: TelemetryConfig {
                otel_endpoint: "http://localhost:4317".to_string(),
                service_name: "tui-patcher-agent".to_string(),
                sampling_ratio: 0.1,
                export_timeout_secs: 30,
            },
            auth: AuthConfig {
                require_api_key: false,
                api_keys: vec![],
            },
            rate_limit: RateLimitConfig {
                requests_per_minute: 100,
                burst_size: 20,
            },
            queue: QueueConfig {
                max_concurrent_jobs: 5,
                job_timeout_secs: 300,
                cleanup_interval_secs: 3600,
            },
            patch: PatchConfig {
                temp_dir: "/tmp".to_string(),
                max_file_size_mb: 100,
                timeout_secs: 180,
            },
            alerts: AlertConfig {
                enabled: true,
                slack_webhook_url: None,
                queue_size_threshold: 10,
                success_rate_threshold: 0.95,
                latency_threshold_ms: 2000,
            },
            ai: AiConfig {
                perplexity_api_key: None,
                perplexity_api_key_ci: None,
                enable_on_device: false,
                model_path: None,
            },
        }
    }
}
