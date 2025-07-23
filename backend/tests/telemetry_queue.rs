use tui_patcher_agent::{
    config::TelemetryConfig,
    queue::{PatchOptions, PatchRequest, PatchType, QueueManager},
    telemetry::init_telemetry,
};

#[tokio::test]
async fn init_telemetry_returns_tracer() {
    let cfg = TelemetryConfig {
        otel_endpoint: "http://localhost:4317".into(),
        service_name: "test-service".into(),
        sampling_ratio: 0.0,
        export_timeout_secs: 1,
    };
    let tracer = init_telemetry(&cfg).await.expect("telemetry init");
    opentelemetry::global::shutdown_tracer_provider();
    let _ = tracer; // aby tracer nie był nieużywany
}

#[tokio::test]
async fn queue_enqueue_and_len() {
    let q = QueueManager::new(1);
    let req = PatchRequest {
        apk_path: "a.apk".into(),
        patch_type: PatchType::Feature,
        options: PatchOptions::default(),
    };
    q.enqueue(req).await;
    assert_eq!(q.len().await, 1);
}
