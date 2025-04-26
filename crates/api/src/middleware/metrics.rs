use axum::{
    body::Body,
    extract::MatchedPath,
    http::{Request, Response},
    middleware::Next,
};
use metrics::{counter, histogram};
use std::time::Instant;

/// Middleware for collecting metrics about HTTP requests
pub async fn track_metrics(
    req: Request<Body>,
    next: Next<Body>,
) -> Response<Body> {
    let start = Instant::now();
    let path = if let Some(matched_path) = req.extensions().get::<MatchedPath>() {
        matched_path.as_str().to_owned()
    } else {
        req.uri().path().to_owned()
    };
    let method = req.method().clone();

    let response = next.run(req).await;

    let latency = start.elapsed().as_secs_f64();
    let status = response.status().as_u16().to_string();

    // Record request count
    counter!("http_requests_total", 1,
        "path" => path.clone(),
        "method" => method.to_string(),
        "status" => status.clone(),
    );

    // Record request duration
    histogram!("http_request_duration_seconds",
        latency,
        "path" => path,
        "method" => method.to_string(),
        "status" => status,
    );

    response
}

/// Initialize the metrics recorder
pub fn setup_metrics_recorder() {
    use metrics_exporter_prometheus::PrometheusBuilder;
    use opentelemetry::sdk::metrics::MeterProvider;
    use opentelemetry_prometheus::PrometheusExporter;

    let prometheus_builder = PrometheusBuilder::new();
    prometheus_builder
        .install()
        .expect("failed to install Prometheus recorder");

    let exporter = PrometheusExporter::new(prometheus_builder.build().unwrap());
    let provider = MeterProvider::builder()
        .with_reader(exporter)
        .build();

    opentelemetry::global::set_meter_provider(provider);
}
