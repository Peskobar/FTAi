# TUI-Patcher-Agent Edge Deployment Guide

## Overview
This guide covers deploying TUI-Patcher-Agent in edge environments, including Android/Termux, IoT devices, and resource-constrained scenarios.

## Edge Architecture

### Components
```
┌─────────────────── Edge Device ────────────────────┐
│                                                    │
│  ┌──────────────┐    ┌─────────────────────────┐   │
│  │ TUI-Patcher  │───▶│    Local Processing     │   │
│  │    Edge      │    │   (Lightweight Core)    │   │
│  │              │    │                         │   │
│  └──────────────┘    └─────────────────────────┘   │
│           │                                        │
│           ▼                                        │
│  ┌─────────────────────────────────────────────┐   │
│  │           Local Storage                     │   │
│  │        (SQLite/Files)                      │   │
│  └─────────────────────────────────────────────┘   │
│                                                    │
└────────────────────────────────────────────────────┘
                         │
                         ▼ (Optional Sync)
┌─────────────────────────────────────────────────────┐
│              Cloud TUI-Patcher Cluster             │
│                 (Full Features)                    │
└─────────────────────────────────────────────────────┘
```

## Supported Platforms

### 1. Android/Termux
**Target:** Android devices with Termux environment
**Use Case:** Mobile app patching, local development

### 2. Raspberry Pi / IoT
**Target:** ARM-based single-board computers
**Use Case:** Local patching stations, offline environments

### 3. Embedded Linux
**Target:** Resource-constrained Linux devices
**Use Case:** Factory environments, air-gapped networks

## Edge Binary Configuration

### Cargo.toml Features
```toml
[features]
default = ["edge-runtime"]
edge-runtime = [
    "minimal-deps",
    "local-storage", 
    "offline-mode"
]
minimal-deps = []
local-storage = ["sqlite", "sled"]
offline-mode = []

# Reduced dependencies for edge
[dependencies]
# Core (required)
tokio = { version = "1.35", features = ["rt", "net", "fs"] }
serde = { version = "1.0", features = ["derive"] }
anyhow = "1.0"

# Local storage
sqlite = { version = "0.32", optional = true }
sled = { version = "0.34", optional = true }

# HTTP (minimal)
hyper = { version = "1.0", features = ["client", "http1"] }

# Remove heavy dependencies
# opentelemetry = { version = "0.23", optional = true }  # Cloud only
# tracing-opentelemetry = { version = "0.23", optional = true }  # Cloud only
```

### Edge Main Binary (src/edge/main.rs)
```rust
use std::path::PathBuf;
use tokio::signal;
use anyhow::Result;

mod config;
mod storage;
mod patch;
mod sync;

use config::EdgeConfig;
use storage::LocalStorage;
use patch::EdgePatchEngine;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize edge-specific logging
    init_edge_logging()?;

    // Load configuration
    let config = EdgeConfig::load_from_file("edge.toml")?;

    // Initialize local storage
    let storage = LocalStorage::new(&config.storage_path).await?;

    // Initialize patch engine
    let patch_engine = EdgePatchEngine::new(storage.clone(), config.clone()).await?;

    // Start background sync (if enabled)
    let sync_handle = if config.sync_enabled {
        Some(start_sync_service(storage.clone(), config.clone()).await?)
    } else {
        None
    };

    // Start CLI interface
    start_cli_interface(patch_engine, config).await?;

    // Wait for shutdown signal
    signal::ctrl_c().await?;
    println!("Shutting down edge runtime...");

    if let Some(handle) = sync_handle {
        handle.abort();
    }

    Ok(())
}

async fn init_edge_logging() -> Result<()> {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer()
            .with_target(false)
            .with_thread_ids(false)
            .with_file(false)
            .with_line_number(false)
            .compact())
        .init();

    Ok(())
}
```

## Installation Methods

### 1. Android/Termux Installation
```bash
# Update Termux packages
pkg update && pkg upgrade

# Install required dependencies
pkg install rust git build-essential

# Clone repository
git clone https://github.com/tui-patcher/tui-patcher-agent.git
cd tui-patcher-agent

# Build edge binary
cargo build --release --features edge-runtime --bin tui-patcher-edge

# Install to PATH
cp target/release/tui-patcher-edge $PREFIX/bin/

# Create config directory
mkdir -p ~/.config/tui-patcher-edge

# Generate default config
tui-patcher-edge --generate-config > ~/.config/tui-patcher-edge/config.toml
```

### 2. Raspberry Pi Installation
```bash
# Install Rust on Raspberry Pi OS
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Clone and build
git clone https://github.com/tui-patcher/tui-patcher-agent.git
cd tui-patcher-agent

# Build for ARM
cargo build --release --features edge-runtime --bin tui-patcher-edge

# Install as systemd service
sudo cp target/release/tui-patcher-edge /usr/local/bin/
sudo cp scripts/tui-patcher-edge.service /etc/systemd/system/
sudo systemctl enable tui-patcher-edge
sudo systemctl start tui-patcher-edge
```

### 3. Docker Edge Container
```dockerfile
# Dockerfile.edge
FROM rust:1.75-slim as builder

WORKDIR /app
COPY . .

# Build edge binary
RUN cargo build --release --features edge-runtime --bin tui-patcher-edge

FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y     ca-certificates     libssl3     && rm -rf /var/lib/apt/lists/*

# Copy binary
COPY --from=builder /app/target/release/tui-patcher-edge /usr/local/bin/

# Create data directory
RUN mkdir -p /data

# Set working directory
WORKDIR /data

# Expose port for local API
EXPOSE 8080

# Run edge binary
CMD ["tui-patcher-edge", "--config", "/data/edge.toml"]
```

## Configuration

### Edge Configuration (edge.toml)
```toml
[service]
name = "tui-patcher-edge"
bind_address = "127.0.0.1:8080"
log_level = "info"

[storage]
type = "sqlite"  # "sqlite" or "sled"
path = "./data/edge.db"
max_size_mb = 100

[processing]
# Reduced resource limits
max_concurrent_jobs = 2
max_memory_mb = 512
temp_dir = "./tmp"

[sync]
enabled = false  # Disable by default for offline
cloud_endpoint = "https://api.tui-patcher.com"
sync_interval_minutes = 60
api_key = ""  # Set if sync enabled

[features]
# Enable only essential features
basic_patching = true
ai_analysis = false  # Disable AI for edge
metrics_collection = false  # Disable OTEL
alerting = false  # Disable alerts

[security]
# Simplified security for edge
require_auth = false
allow_local_only = true
max_file_size_mb = 50
```

## Edge-Specific Features

### 1. Local Storage Implementation
```rust
// src/edge/storage.rs
use serde::{Deserialize, Serialize};
use std::path::Path;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchJob {
    pub id: String,
    pub status: JobStatus,
    pub input_path: String,
    pub output_path: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JobStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}

pub struct LocalStorage {
    db: sqlite::Connection,
}

impl LocalStorage {
    pub async fn new(db_path: &Path) -> Result<Self> {
        let db = sqlite::open(db_path)?;

        // Initialize schema
        db.execute("
            CREATE TABLE IF NOT EXISTS patch_jobs (
                id TEXT PRIMARY KEY,
                status TEXT NOT NULL,
                input_path TEXT NOT NULL,
                output_path TEXT,
                created_at TEXT NOT NULL,
                completed_at TEXT,
                error_message TEXT
            )
        ")?;

        Ok(Self { db })
    }

    pub async fn create_job(&self, job: &PatchJob) -> Result<()> {
        let mut statement = self.db.prepare("
            INSERT INTO patch_jobs 
            (id, status, input_path, output_path, created_at, completed_at, error_message)
            VALUES (?, ?, ?, ?, ?, ?, ?)
        ")?;

        statement.bind((
            1, job.id.as_str(),
            2, format!("{:?}", job.status).as_str(),
            3, job.input_path.as_str(),
            4, job.output_path.as_deref().unwrap_or(""),
            5, job.created_at.to_rfc3339().as_str(),
            6, job.completed_at.map(|t| t.to_rfc3339()).as_deref().unwrap_or(""),
            7, job.error_message.as_deref().unwrap_or(""),
        ))?;

        statement.next()?;
        Ok(())
    }

    pub async fn get_job(&self, id: &str) -> Result<Option<PatchJob>> {
        let mut statement = self.db.prepare("
            SELECT id, status, input_path, output_path, created_at, completed_at, error_message
            FROM patch_jobs WHERE id = ?
        ")?;

        statement.bind((1, id))?;

        if let Some(Ok(row)) = statement.next() {
            let job = PatchJob {
                id: row.read::<&str, _>(0).to_string(),
                status: match row.read::<&str, _>(1) {
                    "Pending" => JobStatus::Pending,
                    "Processing" => JobStatus::Processing,
                    "Completed" => JobStatus::Completed,
                    "Failed" => JobStatus::Failed,
                    _ => JobStatus::Failed,
                },
                input_path: row.read::<&str, _>(2).to_string(),
                output_path: {
                    let path = row.read::<&str, _>(3);
                    if path.is_empty() { None } else { Some(path.to_string()) }
                },
                created_at: chrono::DateTime::parse_from_rfc3339(row.read::<&str, _>(4))?
                    .with_timezone(&chrono::Utc),
                completed_at: {
                    let time_str = row.read::<&str, _>(5);
                    if time_str.is_empty() {
                        None
                    } else {
                        Some(chrono::DateTime::parse_from_rfc3339(time_str)?
                            .with_timezone(&chrono::Utc))
                    }
                },
                error_message: {
                    let msg = row.read::<&str, _>(6);
                    if msg.is_empty() { None } else { Some(msg.to_string()) }
                },
            };
            Ok(Some(job))
        } else {
            Ok(None)
        }
    }
}
```

### 2. Lightweight Patch Engine
```rust
// src/edge/patch.rs
use std::path::{Path, PathBuf};
use std::process::Command;
use anyhow::{Result, Context};
use tokio::fs;

pub struct EdgePatchEngine {
    storage: LocalStorage,
    config: EdgeConfig,
    temp_dir: PathBuf,
}

impl EdgePatchEngine {
    pub async fn new(storage: LocalStorage, config: EdgeConfig) -> Result<Self> {
        let temp_dir = PathBuf::from(&config.processing.temp_dir);
        fs::create_dir_all(&temp_dir).await?;

        Ok(Self {
            storage,
            config,
            temp_dir,
        })
    }

    pub async fn process_apk(&self, input_path: &Path) -> Result<PathBuf> {
        // Simplified patching without AI
        let job_id = uuid::Uuid::new_v4().to_string();
        let output_path = self.temp_dir.join(format!("{}_patched.apk", job_id));

        // Create job record
        let job = PatchJob {
            id: job_id.clone(),
            status: JobStatus::Processing,
            input_path: input_path.to_string_lossy().to_string(),
            output_path: Some(output_path.to_string_lossy().to_string()),
            created_at: chrono::Utc::now(),
            completed_at: None,
            error_message: None,
        };

        self.storage.create_job(&job).await?;

        // Basic patching using apktool
        let success = self.run_apktool_patch(input_path, &output_path).await?;

        // Update job status
        let final_status = if success {
            JobStatus::Completed
        } else {
            JobStatus::Failed
        };

        // Update job record would go here...

        if success {
            Ok(output_path)
        } else {
            Err(anyhow::anyhow!("Patch processing failed"))
        }
    }

    async fn run_apktool_patch(&self, input: &Path, output: &Path) -> Result<bool> {
        // Extract APK
        let extract_dir = self.temp_dir.join("extracted");
        fs::create_dir_all(&extract_dir).await?;

        let extract_output = Command::new("apktool")
            .args(&["d", input.to_str().unwrap(), "-o", extract_dir.to_str().unwrap()])
            .output()
            .context("Failed to extract APK")?;

        if !extract_output.status.success() {
            return Ok(false);
        }

        // Apply basic patches (placeholder)
        self.apply_basic_patches(&extract_dir).await?;

        // Rebuild APK
        let rebuild_output = Command::new("apktool")
            .args(&["b", extract_dir.to_str().unwrap(), "-o", output.to_str().unwrap()])
            .output()
            .context("Failed to rebuild APK")?;

        // Cleanup
        fs::remove_dir_all(&extract_dir).await.ok();

        Ok(rebuild_output.status.success())
    }

    async fn apply_basic_patches(&self, extract_dir: &Path) -> Result<()> {
        // Implement basic patching logic
        // This would contain the core patching algorithms
        // without AI/ML dependencies

        // Example: Remove debug flags
        let manifest_path = extract_dir.join("AndroidManifest.xml");
        if manifest_path.exists() {
            // Parse and modify manifest
            // Implementation depends on specific patches needed
        }

        Ok(())
    }
}
```

### 3. CLI Interface
```rust
// src/edge/cli.rs
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "tui-patcher-edge")]
#[command(about = "TUI-Patcher Edge Runtime for resource-constrained environments")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(long, default_value = "edge.toml")]
    pub config: PathBuf,

    #[arg(long)]
    pub verbose: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Patch an APK file
    Patch {
        /// Input APK file path
        input: PathBuf,

        /// Output APK file path (optional)
        #[arg(long)]
        output: Option<PathBuf>,
    },

    /// Check status of a patch job
    Status {
        /// Job ID to check
        job_id: String,
    },

    /// List all jobs
    List {
        /// Show only active jobs
        #[arg(long)]
        active: bool,
    },

    /// Start daemon mode
    Daemon,

    /// Generate default configuration
    GenerateConfig,

    /// Sync with cloud (if enabled)
    Sync,
}

pub async fn run_cli(engine: EdgePatchEngine, config: EdgeConfig) -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Patch { input, output } => {
            println!("Processing APK: {}", input.display());

            match engine.process_apk(&input).await {
                Ok(result_path) => {
                    if let Some(output_path) = output {
                        fs::copy(&result_path, &output_path).await?;
                        println!("Patched APK saved to: {}", output_path.display());
                    } else {
                        println!("Patched APK available at: {}", result_path.display());
                    }
                },
                Err(e) => {
                    eprintln!("Error processing APK: {}", e);
                    std::process::exit(1);
                }
            }
        },

        Commands::Status { job_id } => {
            match engine.get_job_status(&job_id).await {
                Ok(Some(job)) => {
                    println!("Job {}: {:?}", job_id, job.status);
                    if let Some(error) = job.error_message {
                        println!("Error: {}", error);
                    }
                },
                Ok(None) => {
                    println!("Job {} not found", job_id);
                },
                Err(e) => {
                    eprintln!("Error checking status: {}", e);
                }
            }
        },

        Commands::List { active } => {
            // Implementation for listing jobs
            println!("Listing jobs...");
        },

        Commands::Daemon => {
            println!("Starting daemon mode...");
            // Start HTTP server for API access
            start_daemon_server(engine, config).await?;
        },

        Commands::GenerateConfig => {
            let default_config = EdgeConfig::default();
            let config_toml = toml::to_string_pretty(&default_config)?;
            println!("{}", config_toml);
        },

        Commands::Sync => {
            if config.sync.enabled {
                println!("Syncing with cloud...");
                // Implementation for cloud sync
            } else {
                println!("Sync is disabled in configuration");
            }
        },
    }

    Ok(())
}
```

## Resource Optimization

### Memory Management
```rust
// src/edge/resources.rs
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

pub struct ResourceMonitor {
    max_memory_mb: usize,
    current_memory: Arc<AtomicUsize>,
}

impl ResourceMonitor {
    pub fn new(max_memory_mb: usize) -> Self {
        Self {
            max_memory_mb,
            current_memory: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn check_memory_available(&self, required_mb: usize) -> bool {
        let current = self.current_memory.load(Ordering::Relaxed);
        current + required_mb <= self.max_memory_mb
    }

    pub fn allocate_memory(&self, mb: usize) -> Option<MemoryGuard> {
        if self.check_memory_available(mb) {
            self.current_memory.fetch_add(mb, Ordering::Relaxed);
            Some(MemoryGuard {
                mb,
                monitor: self.current_memory.clone(),
            })
        } else {
            None
        }
    }
}

pub struct MemoryGuard {
    mb: usize,
    monitor: Arc<AtomicUsize>,
}

impl Drop for MemoryGuard {
    fn drop(&mut self) {
        self.monitor.fetch_sub(self.mb, Ordering::Relaxed);
    }
}
```

### Disk Space Management
```rust
// Cleanup strategy for edge environments
pub async fn cleanup_old_files(data_dir: &Path, max_age_days: u64) -> Result<()> {
    use std::time::{Duration, SystemTime};

    let cutoff = SystemTime::now() - Duration::from_secs(max_age_days * 24 * 3600);

    let mut entries = fs::read_dir(data_dir).await?;
    while let Some(entry) = entries.next_entry().await? {
        let metadata = entry.metadata().await?;
        if let Ok(modified) = metadata.modified() {
            if modified < cutoff {
                if metadata.is_file() {
                    fs::remove_file(entry.path()).await?;
                    println!("Cleaned up old file: {}", entry.path().display());
                }
            }
        }
    }

    Ok(())
}
```

## Deployment Scripts

### Systemd Service (scripts/tui-patcher-edge.service)
```ini
[Unit]
Description=TUI-Patcher Edge Runtime
After=network.target
Wants=network.target

[Service]
Type=simple
User=tui-patcher
Group=tui-patcher
WorkingDirectory=/opt/tui-patcher-edge
ExecStart=/usr/local/bin/tui-patcher-edge daemon --config /etc/tui-patcher-edge/config.toml
Restart=always
RestartSec=30
StandardOutput=journal
StandardError=journal

# Resource limits
MemoryMax=512M
CPUQuota=50%

# Security
NoNewPrivileges=true
PrivateTmp=true
ProtectHome=true
ProtectSystem=strict
ReadWritePaths=/opt/tui-patcher-edge/data

[Install]
WantedBy=multi-user.target
```

### Auto-update Script (scripts/update-edge.sh)
```bash
#!/bin/bash
# Auto-update script for edge deployments

set -euo pipefail

EDGE_DIR="/opt/tui-patcher-edge"
BACKUP_DIR="${EDGE_DIR}/backup"
BINARY_PATH="/usr/local/bin/tui-patcher-edge"
SERVICE_NAME="tui-patcher-edge"

echo "Starting TUI-Patcher Edge update..."

# Create backup
mkdir -p "$BACKUP_DIR"
if [[ -f "$BINARY_PATH" ]]; then
    cp "$BINARY_PATH" "$BACKUP_DIR/tui-patcher-edge.$(date +%Y%m%d_%H%M%S)"
fi

# Download latest version
echo "Downloading latest edge binary..."
wget -q "https://releases.tui-patcher.com/latest/tui-patcher-edge-$(uname -m)"      -O "$BINARY_PATH.new"

# Verify download
if [[ ! -f "$BINARY_PATH.new" ]]; then
    echo "Failed to download new binary"
    exit 1
fi

chmod +x "$BINARY_PATH.new"

# Stop service
echo "Stopping service..."
sudo systemctl stop "$SERVICE_NAME"

# Replace binary
mv "$BINARY_PATH.new" "$BINARY_PATH"

# Start service
echo "Starting service..."
sudo systemctl start "$SERVICE_NAME"

# Verify service is running
sleep 5
if sudo systemctl is-active "$SERVICE_NAME" >/dev/null; then
    echo "Update completed successfully"
else
    echo "Service failed to start, rolling back..."
    # Rollback logic would go here
    exit 1
fi
```

## Troubleshooting

### Common Issues

#### 1. Memory Constraints
```bash
# Check memory usage
cat /proc/meminfo
ps aux --sort=-rss | head -10

# Reduce memory limits in config
[processing]
max_concurrent_jobs = 1
max_memory_mb = 256
```

#### 2. Storage Space
```bash
# Check disk usage
df -h
du -sh /opt/tui-patcher-edge/*

# Clean up old files
tui-patcher-edge cleanup --older-than 7d
```

#### 3. Missing Dependencies
```bash
# Install apktool on Termux
pkg install apktool

# Install on Raspberry Pi
sudo apt-get install apktool android-tools-adb
```

### Monitoring Edge Deployment
```bash
# Check service status
systemctl status tui-patcher-edge

# View logs
journalctl -u tui-patcher-edge -f

# Check configuration
tui-patcher-edge --config /etc/tui-patcher-edge/config.toml validate
```

This edge deployment guide provides a complete framework for running TUI-Patcher-Agent in resource-constrained environments while maintaining core functionality and reliability.
