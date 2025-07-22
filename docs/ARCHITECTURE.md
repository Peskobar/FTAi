# TUI-Patcher-Agent Architecture Guide

## System Overview
TUI-Patcher-Agent is an enterprise-grade microservice for AI-powered Android application patching, designed to achieve 99%+ success rates with sub-1.5s latency.

## Current Architecture (v0.3.0)

### Core Components
```
┌─────────────┐    ┌──────────────┐    ┌─────────────────┐
│ API Gateway │───▶│ Queue Manager│───▶│ Patching Engine │
│ (Axum/Tokio)│    │              │    │                 │
└─────────────┘    └──────────────┘    └─────────────────┘
       │                                          │
       ▼                                          ▼
┌─────────────┐                          ┌─────────────────┐
│ Prometheus  │                          │  File Storage   │
│ Metrics     │                          │                 │
└─────────────┘                          └─────────────────┘
```

### Request Flow
1. **API Gateway** receives HTTP requests via Axum
2. **Queue Manager** handles async task scheduling
3. **Patching Engine** processes APK/AAB files
4. **File Storage** manages input/output artifacts
5. **Prometheus** collects basic metrics

## Target Architecture (Post-Roadmap)

### Layered Enterprise Architecture
```
┌───────────── Edge & External Layer ─────────────────┐
│ Edge Runtime │ Perplexity AI │ WASM Marketplace     │
│ (Termux/IoT) │ Dev Assistant │ (Plugin Ecosystem)   │
└─────────────────────────────────────────────────────┘
                          │
┌───────────── Core Services Layer ───────────────────┐
│                API Gateway Enhanced                  │
│              (Axum + OTEL + Trace)                  │
├─────────────────────────┬────────────────────────────┤
│ LSTM Autoscaler │ Circuit Breaker │ Self-healing    │
│                 │                  │ Queue           │
├─────────────────┴──────────────────┴─────────────────┤
│              On-device AI (TensorFlow Lite)         │
└─────────────────────────────────────────────────────┘
                          │
┌───────────── Infrastructure Layer ──────────────────┐
│ OTEL Collector │ SBOM Generator │ SLSA L3 Compliance │
│ Kubernetes     │ Monitoring     │ Security Scanning  │
└─────────────────────────────────────────────────────┘
```

## Sprint Q3 2025: Hardening & Visibility Implementation

### 1. OpenTelemetry Integration (8 SP)
**Files Modified:**
```
src/telemetry/mod.rs              # OTEL bootstrap
src/telemetry/tracing.rs          # Trace provider
src/telemetry/metrics.rs          # Metrics provider
src/main.rs                       # Integration point
config/otel.toml                  # Configuration
```

**Key Features:**
- OTLP export to Jaeger/Zipkin
- Semantic conventions compliance
- Distributed tracing support

### 2. Traceparent Middleware (5 SP)
**Files Modified:**
```
src/middleware/tracing.rs         # W3C Trace Context
src/middleware/mod.rs             # Middleware exports
src/routes/mod.rs                 # Router integration
src/extractors/trace_context.rs  # Custom extractors
```

**Capabilities:**
- W3C Trace Context propagation
- Cross-service correlation
- Span lifecycle management

### 3. Slack Alert System (3 SP)
**Files Modified:**
```
src/notifications/slack.rs       # Webhook client
src/alerts/mod.rs                 # Alert logic
src/alerts/rules_engine.rs       # Configurable rules
config.toml                       # Webhook config
```

**Alert Rules:**
- Queue size > 10 tasks
- Success rate < 95%
- Latency p95 > 3s
- Rate limiting: max 1/5min

### 4. SBOM Pipeline (6 SP)
**Files Modified:**
```
build.rs                          # SBOM generation
.github/workflows/sbom.yml        # CI automation
src/routes/sbom.rs               # HTTP endpoint
Dockerfile                       # Multi-stage build
scripts/slsa-verify.sh           # Verification
```

**Compliance Features:**
- CycloneDX format generation
- SLSA Level 3 attestation
- Dependency vulnerability scanning

## Directory Structure
```
tui-patcher-agent/
├── src/
│   ├── main.rs                   # Application entry
│   ├── lib.rs                    # Library exports
│   ├── config/                   # Configuration
│   ├── routes/                   # HTTP endpoints
│   ├── middleware/               # Axum middleware
│   ├── telemetry/                # OTEL integration
│   ├── notifications/            # Alert systems
│   ├── alerts/                   # Alert logic
│   ├── queue/                    # Task management
│   ├── patch/                    # Core patching
│   ├── ai/                       # ML components
│   ├── plugins/                  # WASM plugins
│   └── edge/                     # Edge runtime
├── config/                       # Configuration files
├── k8s/                         # Kubernetes manifests
├── helm/                        # Helm charts
├── .github/workflows/           # CI/CD pipelines
├── docs/                        # Documentation
├── examples/                    # Usage examples
└── assets/                      # Static resources
```

## Data Flow Patterns

### 1. HTTP Request Lifecycle
```
Client Request
    ↓
[API Gateway + OTEL Middleware]
    ↓
[Authentication & Rate Limiting]
    ↓
[Business Logic Routing]
    ↓
[Queue Management]
    ↓
[Patch Processing]
    ↓
[Response + Metrics]
```

### 2. Observability Data Flow
```
Application Spans
    ↓
[OTEL Collector]
    ↓
[Jaeger/Zipkin] ←→ [Prometheus] ←→ [Grafana]
    ↓
[Alert Manager]
    ↓
[Slack Notifications]
```

## Security Architecture

### Authentication Flow
1. **API Key Validation** (Header-based)
2. **Rate Limiting** (Token bucket)
3. **Input Sanitization** (APK validation)
4. **Output Validation** (Patch verification)

### SLSA Level 3 Requirements
- ✅ Provenance generation
- ✅ Non-falsifiable metadata
- ✅ Isolated builds
- ✅ Parameterless builds

## Performance Characteristics

### Current Metrics (v0.3.0)
- **Throughput:** ~400 req/min
- **Success Rate:** 95%
- **P95 Latency:** 2.4s
- **Queue Depth:** ~15 tasks

### Target Metrics (v1.0)
- **Throughput:** ~1000 req/min
- **Success Rate:** ≥99%
- **P95 Latency:** ≤1.5s
- **Queue Depth:** <5 tasks (auto-scaling)

## Deployment Scenarios

### 1. Cloud Native (Kubernetes)
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: tui-patcher-agent
spec:
  replicas: 3
  selector:
    matchLabels:
      app: tui-patcher-agent
  template:
    spec:
      containers:
      - name: agent
        image: tui-patcher:v0.3.0
        env:
        - name: OTEL_EXPORTER_OTLP_ENDPOINT
          value: "http://otel-collector:4317"
```

### 2. Edge Deployment (Termux)
```bash
# Install on Android/Termux
pkg install rust
cargo install --path . --features edge-runtime
tui-patcher-edge --config edge.toml
```

### 3. Local Development
```bash
# Docker Compose setup
docker-compose -f docker-compose.dev.yml up
```

## Future Roadmap Integration

### Q4 2025: Intelligent Patching
- LSTM-based autoscaling
- TensorFlow Lite integration
- Circuit breaker patterns

### H1 2026: Ecosystem & Plugins
- WASM plugin marketplace
- Perplexity AI dev assistant
- Advanced edge capabilities

## Monitoring & Alerting

### Key Metrics
- `tui_patcher_requests_total` (Counter)
- `tui_patcher_request_duration_seconds` (Histogram)
- `tui_patcher_queue_size` (Gauge)
- `tui_patcher_success_rate` (Gauge)

### Alert Conditions
```yaml
groups:
- name: tui-patcher
  rules:
  - alert: HighLatency
    expr: histogram_quantile(0.95, tui_patcher_request_duration_seconds) > 1.5
  - alert: LowSuccessRate
    expr: tui_patcher_success_rate < 0.99
  - alert: QueueBacklog
    expr: tui_patcher_queue_size > 10
```

## Development Guidelines

### Code Organization
- **Domain-driven design** with clear module boundaries
- **Async-first** architecture using Tokio
- **Error handling** with `anyhow` and `thiserror`
- **Configuration** via TOML files and environment variables

### Testing Strategy
- Unit tests for core business logic
- Integration tests for HTTP endpoints
- Load tests for performance validation
- Security tests for vulnerability scanning

### CI/CD Pipeline
1. **Build** - Cargo check + test
2. **Security** - Cargo audit + SBOM generation
3. **Package** - Docker image + SLSA attestation
4. **Deploy** - Helm chart + health checks

This architecture provides a solid foundation for the enterprise transformation of TUI-Patcher-Agent while maintaining backward compatibility and supporting future enhancements.
