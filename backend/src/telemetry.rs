use crate::config::TelemetryConfig;
use opentelemetry::sdk::{trace, Resource};
use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Inicjalizacja systemu telemetrycznego z wykorzystaniem OpenTelemetry.
/// Funkcja zwraca uchwyt do tracera potrzebny do poprawnego zamknięcia eksportera.
pub async fn init_telemetry(config: &TelemetryConfig) -> anyhow::Result<opentelemetry::sdk::trace::Tracer> {
    let exporter = opentelemetry_otlp::new_exporter()
        .tonic()
        .with_endpoint(&config.otel_endpoint);

    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(exporter)
        .with_trace_config(
            trace::config()
                .with_resource(Resource::new(vec![
                    KeyValue::new("service.name", config.service_name.clone()),
                    KeyValue::new("service.version", crate::VERSION),
                ]))
                .with_sampler(trace::Sampler::TraceIdRatioBased(config.sampling_ratio))
        )
        .with_batch_config(
            trace::BatchConfig::default()
                .with_max_export_batch_size(512)
                .with_max_queue_size(2048)
                .with_scheduled_delay(std::time::Duration::from_millis(500))
                .with_export_timeout(std::time::Duration::from_secs(config.export_timeout_secs))
        )
        .install_batch(opentelemetry::runtime::Tokio)?;

    let otel_layer = tracing_opentelemetry::layer().with_tracer(tracer.clone());

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer().json())
        .with(otel_layer)
        .init();

    Ok(tracer)
}
