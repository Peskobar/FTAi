# TUI-Patcher-Agent Observability Guide

## Overview
This guide covers the comprehensive observability implementation for TUI-Patcher-Agent, focusing on the Q3 2025 "Hardening & Visibility" sprint deliverables.

## OpenTelemetry Integration

### Architecture
```
Application Code
    ↓
[OTEL SDK] ←→ [Tracing Provider] ←→ [Metrics Provider]
    ↓
[OTEL Collector]
    ↓
[Jaeger] [Prometheus] [Grafana]
```

### Configuration

#### Environment Variables
```bash
# OTEL Collector Endpoint
export OTEL_EXPORTER_OTLP_ENDPOINT="http://localhost:4317"
export OTEL_SERVICE_NAME="tui-patcher-agent"
export OTEL_SERVICE_VERSION="0.3.0"

# Sampling Configuration
export OTEL_TRACES_SAMPLER="traceidratio"
export OTEL_TRACES_SAMPLER_ARG="0.1"  # 10% sampling

# Resource Attributes
export OTEL_RESOURCE_ATTRIBUTES="deployment.environment=production,service.namespace=tui-patcher"
```

#### OTEL Configuration (config/otel.toml)
```toml
[exporter]
endpoint = "http://otel-collector:4317"
timeout = "30s"
compression = "gzip"

[tracer]
service_name = "tui-patcher-agent"
service_version = "0.3.0"
sampling_ratio = 0.1

[metrics]
export_interval = "30s"
export_timeout = "10s"

[logging]
level = "info"
structured = true
```

### Implementation Details

#### Tracer Initialization (src/telemetry/tracing.rs)
```rust
use opentelemetry::sdk::{trace, Resource};
use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;

pub async fn init_tracer() -> Result<Tracer, Box<dyn std::error::Error + Send + Sync>> {
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint("http://otel-collector:4317"),
        )
        .with_trace_config(
            trace::config()
                .with_resource(Resource::new(vec![
                    KeyValue::new("service.name", "tui-patcher-agent"),
                    KeyValue::new("service.version", "0.3.0"),
                    KeyValue::new("service.namespace", "tui-patcher"),
                ]))
                .with_sampler(trace::Sampler::TraceIdRatioBased(0.1)),
        )
        .install_batch(opentelemetry::runtime::Tokio)?;

    Ok(tracer)
}
```

#### Metrics Provider (src/telemetry/metrics.rs)
```rust
use opentelemetry::metrics::{Counter, Histogram, Gauge};
use opentelemetry::global;

pub struct Metrics {
    pub requests_total: Counter<u64>,
    pub request_duration: Histogram<f64>,
    pub queue_size: Gauge<u64>,
    pub success_rate: Gauge<f64>,
}

impl Metrics {
    pub fn new() -> Self {
        let meter = global::meter("tui-patcher-agent");

        Self {
            requests_total: meter
                .u64_counter("tui_patcher_requests_total")
                .with_description("Total number of requests processed")
                .init(),

            request_duration: meter
                .f64_histogram("tui_patcher_request_duration_seconds")
                .with_description("Request processing duration in seconds")
                .init(),

            queue_size: meter
                .u64_gauge("tui_patcher_queue_size")
                .with_description("Current queue size")
                .init(),

            success_rate: meter
                .f64_gauge("tui_patcher_success_rate")
                .with_description("Success rate over last 5 minutes")
                .init(),
        }
    }
}
```

## Distributed Tracing

### Traceparent Middleware Implementation

#### W3C Trace Context Propagation
```rust
use axum::{extract::Request, middleware::Next, response::Response};
use tracing::{instrument, Span};
use opentelemetry::trace::TraceContextExt;

#[instrument(skip(req, next))]
pub async fn trace_middleware(req: Request, next: Next) -> Response {
    // Extract trace context from headers
    let trace_context = extract_trace_context(&req);

    // Create span with trace context
    let span = tracing::info_span!(
        "http_request",
        method = %req.method(),
        uri = %req.uri(),
        version = ?req.version(),
        trace_id = %trace_context.trace_id(),
        span_id = %trace_context.span_id(),
    );

    // Process request within span
    let response = span.in_scope(|| next.run(req)).await;

    // Add trace headers to response
    add_trace_headers(response, &trace_context)
}

fn extract_trace_context(req: &Request) -> SpanContext {
    use opentelemetry::propagation::Extractor;

    let extractor = HeaderExtractor::new(req.headers());
    global::get_text_map_propagator(|propagator| {
        propagator.extract(&extractor)
    })
}
```

### Span Hierarchy
```
http_request (root span)
├── auth_validation
├── rate_limiting_check
├── queue_management
│   ├── queue_enqueue
│   └── queue_process
├── patch_processing
│   ├── apk_analysis
│   ├── patch_generation
│   └── apk_rebuild
└── response_formation
```

### Custom Span Attributes
```rust
// Business context attributes
span.set_attribute("apk.package_name", package_name);
span.set_attribute("apk.version_code", version_code);
span.set_attribute("patch.type", patch_type);
span.set_attribute("queue.position", queue_position);

// Technical attributes
span.set_attribute("http.route", route);
span.set_attribute("user.id", user_id);
span.set_attribute("request.size_bytes", content_length);
```

## Metrics Collection

### Core Business Metrics
```rust
#[derive(Clone)]
pub struct BusinessMetrics {
    // Request metrics
    pub requests_total: Counter<u64>,
    pub requests_duration: Histogram<f64>,
    pub requests_in_flight: Gauge<i64>,

    // Success/failure metrics
    pub success_rate: Gauge<f64>,
    pub error_rate: Gauge<f64>,
    pub patch_success_total: Counter<u64>,
    pub patch_failure_total: Counter<u64>,

    // Queue metrics
    pub queue_size: Gauge<u64>,
    pub queue_wait_time: Histogram<f64>,
    pub queue_processing_time: Histogram<f64>,

    // Resource metrics
    pub memory_usage: Gauge<f64>,
    pub cpu_usage: Gauge<f64>,
    pub disk_usage: Gauge<f64>,
}
```

### Metric Labels and Dimensions
```rust
// Request labels
let labels = [
    ("method", request.method().as_str()),
    ("route", route.as_str()),
    ("status_code", status_code.as_str()),
    ("user_agent", user_agent.as_str()),
];

// Business labels
let business_labels = [
    ("patch_type", patch_type.as_str()),
    ("android_version", android_version.as_str()),
    ("app_category", app_category.as_str()),
];
```

## Alert System Integration

### Slack Webhook Configuration
```rust
use slack_hook::{Slack, PayloadBuilder};

pub struct SlackNotifier {
    webhook_url: String,
    channel: String,
    username: String,
}

impl SlackNotifier {
    pub async fn send_alert(&self, alert: &Alert) -> Result<(), SlackError> {
        let slack = Slack::new(&self.webhook_url)?;

        let payload = PayloadBuilder::new()
            .channel(&self.channel)
            .username(&self.username)
            .icon_emoji(":warning:")
            .text(&format!("🚨 TUI-Patcher Alert: {}", alert.title))
            .attachments(vec![
                attachment_builder()
                    .color("danger")
                    .title(&alert.title)
                    .text(&alert.description)
                    .field(Field::new("Metric", &alert.metric, true))
                    .field(Field::new("Value", &alert.value, true))
                    .field(Field::new("Threshold", &alert.threshold, true))
                    .field(Field::new("Trace ID", &alert.trace_id, true))
                    .build()?
            ])
            .build()?;

        slack.send(&payload).await
    }
}
```

### Alert Rules Engine
```rust
pub struct AlertRule {
    pub name: String,
    pub condition: AlertCondition,
    pub threshold: f64,
    pub duration: Duration,
    pub severity: Severity,
    pub enabled: bool,
}

pub enum AlertCondition {
    GreaterThan(MetricType),
    LessThan(MetricType),
    PercentageChange(MetricType, Duration),
}

// Predefined alert rules
pub fn default_alert_rules() -> Vec<AlertRule> {
    vec![
        AlertRule {
            name: "High Queue Size".to_string(),
            condition: AlertCondition::GreaterThan(MetricType::QueueSize),
            threshold: 10.0,
            duration: Duration::from_secs(60),
            severity: Severity::Warning,
            enabled: true,
        },
        AlertRule {
            name: "Low Success Rate".to_string(),
            condition: AlertCondition::LessThan(MetricType::SuccessRate),
            threshold: 0.95,
            duration: Duration::from_secs(300),
            severity: Severity::Critical,
            enabled: true,
        },
        AlertRule {
            name: "High P95 Latency".to_string(),
            condition: AlertCondition::GreaterThan(MetricType::LatencyP95),
            threshold: 1.5,
            duration: Duration::from_secs(180),
            severity: Severity::Warning,
            enabled: true,
        },
    ]
}
```

## SBOM and Compliance Monitoring

### SBOM Generation Pipeline
```bash
#!/bin/bash
# scripts/generate-sbom.sh

set -euo pipefail

echo "Generating SBOM for TUI-Patcher-Agent..."

# Install cargo-cyclonedx if not present
if ! cargo cyclonedx --version >/dev/null 2>&1; then
    cargo install cargo-cyclonedx
fi

# Generate CycloneDX SBOM
cargo cyclonedx --format json --output cyclonedx.json

# Validate SBOM format
echo "Validating SBOM format..."
if command -v cyclonedx-cli >/dev/null 2>&1; then
    cyclonedx-cli validate cyclonedx.json
fi

# Generate SPDX format (optional)
if command -v cargo-spdx >/dev/null 2>&1; then
    cargo spdx --output spdx.json
fi

echo "SBOM generation complete:"
echo "- CycloneDX: cyclonedx.json"
echo "- SPDX: spdx.json (if available)"

# Upload to artifact store
if [[ "${CI:-false}" == "true" ]]; then
    echo "Uploading SBOM to artifact store..."
    # Implementation depends on your artifact storage
fi
```

### SLSA Level 3 Attestation
```yaml
# .github/workflows/slsa-attestation.yml
name: SLSA Attestation

on:
  release:
    types: [published]

jobs:
  attestation:
    runs-on: ubuntu-latest
    permissions:
      id-token: write
      contents: read
      attestations: write

    steps:
    - uses: actions/checkout@v4

    - name: Build artifact
      run: |
        cargo build --release
        cp target/release/tui-patcher-agent ./

    - name: Generate SBOM
      run: |
        cargo install cargo-cyclonedx
        cargo cyclonedx --format json --output cyclonedx.json

    - name: Generate attestation
      uses: actions/attest-build-provenance@v1
      with:
        subject-path: |
          tui-patcher-agent
          cyclonedx.json
```

## Monitoring Dashboards

### Grafana Dashboard Configuration
```json
{
  "dashboard": {
    "id": null,
    "title": "TUI-Patcher-Agent Observability",
    "tags": ["tui-patcher", "observability"],
    "timezone": "browser",
    "panels": [
      {
        "title": "Request Rate",
        "type": "stat",
        "targets": [
          {
            "expr": "rate(tui_patcher_requests_total[5m])",
            "legendFormat": "req/sec"
          }
        ]
      },
      {
        "title": "Success Rate",
        "type": "gauge",
        "targets": [
          {
            "expr": "tui_patcher_success_rate",
            "legendFormat": "Success Rate"
          }
        ],
        "fieldConfig": {
          "defaults": {
            "min": 0,
            "max": 1,
            "thresholds": {
              "steps": [
                {"color": "red", "value": 0},
                {"color": "yellow", "value": 0.95},
                {"color": "green", "value": 0.99}
              ]
            }
          }
        }
      },
      {
        "title": "P95 Latency",
        "type": "timeseries",
        "targets": [
          {
            "expr": "histogram_quantile(0.95, rate(tui_patcher_request_duration_seconds_bucket[5m]))",
            "legendFormat": "P95 Latency"
          }
        ]
      },
      {
        "title": "Queue Size",
        "type": "timeseries",
        "targets": [
          {
            "expr": "tui_patcher_queue_size",
            "legendFormat": "Queue Size"
          }
        ]
      }
    ]
  }
}
```

### Jaeger Query Examples
```bash
# Find traces with high latency
curl "http://jaeger:16686/api/traces?service=tui-patcher-agent&minDuration=2s"

# Find failed patch operations
curl "http://jaeger:16686/api/traces?service=tui-patcher-agent&tags={"error":"true"}"

# Find traces for specific APK
curl "http://jaeger:16686/api/traces?service=tui-patcher-agent&tags={"apk.package_name":"com.example.app"}"
```

## Performance Monitoring

### SLI/SLO Configuration
```yaml
# SLI (Service Level Indicators)
sli:
  availability:
    metric: "sum(rate(tui_patcher_requests_total{code!~"5.."}[5m])) / sum(rate(tui_patcher_requests_total[5m]))"
    threshold: 0.99

  latency:
    metric: "histogram_quantile(0.95, rate(tui_patcher_request_duration_seconds_bucket[5m]))"
    threshold: 1.5

  throughput:
    metric: "sum(rate(tui_patcher_requests_total[5m]))"
    threshold: 10

# SLO (Service Level Objectives)
slo:
  availability: "99.9% over 30 days"
  latency: "95% of requests under 1.5s"
  throughput: "Handle at least 1000 req/min"
```

### Error Tracking
```rust
use tracing::{error, warn, info};

// Structured error logging
#[instrument(skip(self))]
pub async fn process_patch(&self, request: PatchRequest) -> Result<PatchResponse, PatchError> {
    let _timer = self.metrics.request_duration.start_timer();

    match self.validate_request(&request).await {
        Ok(_) => {
            info!(
                apk.package_name = %request.package_name,
                apk.version = %request.version,
                "Processing patch request"
            );
        },
        Err(e) => {
            warn!(
                error = %e,
                apk.package_name = %request.package_name,
                "Request validation failed"
            );
            self.metrics.error_rate.add(1, &[]);
            return Err(PatchError::ValidationFailed(e));
        }
    }

    // Processing logic...

    match result {
        Ok(response) => {
            self.metrics.success_rate.add(1, &[]);
            info!("Patch processing completed successfully");
            Ok(response)
        },
        Err(e) => {
            error!(
                error = %e,
                apk.package_name = %request.package_name,
                "Patch processing failed"
            );
            self.metrics.error_rate.add(1, &[]);
            Err(e)
        }
    }
}
```

## Observability Best Practices

### 1. Correlation IDs
- Generate unique correlation ID for each request
- Propagate through all service calls
- Include in all log messages and traces

### 2. Structured Logging
- Use consistent log format (JSON in production)
- Include relevant context in every log message
- Avoid sensitive data in logs

### 3. Metric Naming
- Follow OpenTelemetry semantic conventions
- Use consistent naming patterns
- Include relevant labels for dimensionality

### 4. Trace Sampling
- Use appropriate sampling rates for production
- Ensure critical paths are always traced
- Consider cost vs. visibility trade-offs

### 5. Alert Hygiene
- Avoid alert fatigue with proper thresholds
- Include relevant context in alert messages
- Provide clear escalation paths

This observability implementation provides comprehensive visibility into TUI-Patcher-Agent operations, enabling proactive monitoring, rapid troubleshooting, and data-driven optimization decisions.
