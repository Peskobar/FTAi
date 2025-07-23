use crate::config::TelemetryConfig;
use anyhow::Result;
use opentelemetry::sdk::{self, trace, Resource};
use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub async fn init_telemetry(cfg: &TelemetryConfig) -> Result<sdk::trace::Tracer> {
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint(cfg.otel_endpoint.clone())
                .with_timeout(std::time::Duration::from_secs(cfg.export_timeout_secs)),
        )
        .with_trace_config(
            trace::config()
                .with_resource(Resource::new(vec![KeyValue::new(
                    "service.name",
                    cfg.service_name.clone(),
                )]))
                .with_sampler(trace::Sampler::TraceIdRatioBased(cfg.sampling_ratio)),
        )
        .install_batch(opentelemetry::runtime::Tokio)?;

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer().json())
        .with(tracing_opentelemetry::layer().with_tracer(tracer.clone()))
        .init();

    Ok(tracer)
}
