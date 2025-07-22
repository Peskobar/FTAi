//! W3C Trace Context propagation middleware
//! Q3 2025 Sprint: Traceparent implementation

use axum::{
    extract::Request,
    http::{HeaderMap, HeaderName, HeaderValue},
    middleware::Next,
    response::Response,
};
use opentelemetry::{
    global,
    propagation::{Extractor, Injector, TextMapPropagator},
    trace::{SpanContext, SpanKind, TraceContextExt, Tracer},
    Context,
};
use std::collections::HashMap;
use tower::{Layer, Service};
use tracing::{instrument, Span};

/// Layer for W3C Trace Context propagation
#[derive(Clone)]
pub struct TraceparentLayer;

impl TraceparentLayer {
    pub fn new() -> Self {
        Self
    }
}

impl<S> Layer<S> for TraceparentLayer {
    type Service = TraceparentService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        TraceparentService { inner }
    }
}

/// Service implementation for trace context propagation
#[derive(Clone)]
pub struct TraceparentService<S> {
    inner: S,
}

impl<S> Service<Request> for TraceparentService<S>
where
    S: Service<Request, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = Response;
    type Error = S::Error;
    type Future = std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        let inner = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, inner);

        Box::pin(async move {
            let response = trace_middleware(req, |req| inner.call(req)).await;
            response
        })
    }
}

/// Main trace middleware function
#[instrument(skip(req, next))]
pub async fn trace_middleware<F, Fut>(
    mut req: Request,
    next: F,
) -> Result<Response, axum::Error>
where
    F: FnOnce(Request) -> Fut,
    Fut: std::future::Future<Output = Result<Response, axum::Error>>,
{
    // Extract trace context from incoming headers
    let parent_context = extract_trace_context(req.headers());

    // Get tracer
    let tracer = global::tracer("tui-patcher-agent");

    // Create span with trace context
    let span = tracer
        .span_builder("http_request")
        .with_kind(SpanKind::Server)
        .with_attributes(vec![
            opentelemetry::KeyValue::new("http.method", req.method().to_string()),
            opentelemetry::KeyValue::new("http.url", req.uri().to_string()),
            opentelemetry::KeyValue::new("http.scheme", 
                req.uri().scheme_str().unwrap_or("http").to_string()),
            opentelemetry::KeyValue::new("http.host", 
                req.headers()
                    .get("host")
                    .and_then(|h| h.to_str().ok())
                    .unwrap_or("unknown")
                    .to_string()),
            opentelemetry::KeyValue::new("user_agent.original",
                req.headers()
                    .get("user-agent")
                    .and_then(|h| h.to_str().ok())
                    .unwrap_or("unknown")
                    .to_string()),
        ])
        .start_with_context(&tracer, &parent_context);

    // Set span context for downstream services
    let trace_id = span.span_context().trace_id().to_string();
    let span_id = span.span_context().span_id().to_string();

    // Add trace context to request extensions
    req.extensions_mut().insert(TraceContext {
        trace_id: trace_id.clone(),
        span_id: span_id.clone(),
        span_context: span.span_context().clone(),
    });

    // Execute request within span context
    let cx = Context::current_with_span(span);
    let response = cx.with(|| next(req)).await;

    match response {
        Ok(mut response) => {
            // Add trace headers to response
            inject_trace_headers(response.headers_mut(), &trace_id, &span_id);

            // Set span status based on response
            let status_code = response.status().as_u16();
            if status_code >= 400 {
                let span = tracing::Span::current();
                span.record("error", true);
                span.record("http.status_code", status_code);
            }

            Ok(response)
        }
        Err(e) => {
            // Record error in span
            let span = tracing::Span::current();
            span.record("error", true);
            span.record("error.message", e.to_string().as_str());
            Err(e)
        }
    }
}

/// Extract trace context from HTTP headers
fn extract_trace_context(headers: &HeaderMap) -> Context {
    let extractor = HeaderExtractor::new(headers);
    global::get_text_map_propagator(|propagator| {
        propagator.extract(&extractor)
    })
}

/// Inject trace headers into response
fn inject_trace_headers(headers: &mut HeaderMap, trace_id: &str, span_id: &str) {
    // Add custom trace headers for easier debugging
    if let Ok(trace_header) = HeaderValue::from_str(trace_id) {
        headers.insert(
            HeaderName::from_static("x-trace-id"),
            trace_header,
        );
    }

    if let Ok(span_header) = HeaderValue::from_str(span_id) {
        headers.insert(
            HeaderName::from_static("x-span-id"),
            span_header,
        );
    }

    // Inject W3C trace context into response headers
    let mut injector = HeaderInjector::new(headers);
    global::get_text_map_propagator(|propagator| {
        let context = Context::current();
        propagator.inject(&context, &mut injector);
    });
}

/// HTTP header extractor for trace context
struct HeaderExtractor<'a> {
    headers: &'a HeaderMap,
}

impl<'a> HeaderExtractor<'a> {
    fn new(headers: &'a HeaderMap) -> Self {
        Self { headers }
    }
}

impl<'a> Extractor for HeaderExtractor<'a> {
    fn get(&self, key: &str) -> Option<&str> {
        self.headers
            .get(key)
            .and_then(|value| value.to_str().ok())
    }

    fn keys(&self) -> Vec<&str> {
        self.headers
            .keys()
            .map(|key| key.as_str())
            .collect()
    }
}

/// HTTP header injector for trace context
struct HeaderInjector<'a> {
    headers: &'a mut HeaderMap,
}

impl<'a> HeaderInjector<'a> {
    fn new(headers: &'a mut HeaderMap) -> Self {
        Self { headers }
    }
}

impl<'a> Injector for HeaderInjector<'a> {
    fn set(&mut self, key: &str, value: String) {
        if let Ok(header_name) = HeaderName::try_from(key) {
            if let Ok(header_value) = HeaderValue::from_str(&value) {
                self.headers.insert(header_name, header_value);
            }
        }
    }
}

/// Trace context information attached to requests
#[derive(Debug, Clone)]
pub struct TraceContext {
    pub trace_id: String,
    pub span_id: String,
    pub span_context: SpanContext,
}

impl TraceContext {
    /// Get trace context from request extensions
    pub fn from_request(req: &Request) -> Option<&Self> {
        req.extensions().get::<TraceContext>()
    }

    /// Create a child span context
    pub fn create_child_span(&self, operation_name: &str) -> Context {
        let tracer = global::tracer("tui-patcher-agent");
        let span = tracer
            .span_builder(operation_name)
            .with_kind(SpanKind::Internal)
            .start(&tracer);
        Context::current_with_span(span)
    }
}
