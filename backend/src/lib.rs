//! TUI-Patcher-Agent Library
//! Core functionality for Android APK patching with AI assistance

pub mod config;
pub mod health;
pub mod telemetry;

pub use config::Config;
/// Current version of TUI-Patcher-Agent
pub const VERSION: &str = "0.3.0";

/// Default configuration path
pub const DEFAULT_CONFIG_PATH: &str = "config.toml";
