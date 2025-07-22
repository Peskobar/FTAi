//! TUI-Patcher-Agent Library
//! Core functionality for Android APK patching with AI assistance

pub mod config;
pub mod telemetry;
pub mod routes;
pub mod middleware;
pub mod queue;
pub mod patch;
pub mod notifications;
pub mod alerts;
pub mod ai;
pub mod plugins;

pub use config::Config;

// Re-export commonly used types
pub use queue::{QueueManager, PatchJob, JobStatus};
pub use patch::{PatchEngine, PatchRequest, PatchResponse};
pub use notifications::NotificationService;

/// Current version of TUI-Patcher-Agent
pub const VERSION: &str = "0.3.0";

/// Default configuration path
pub const DEFAULT_CONFIG_PATH: &str = "config.toml";
