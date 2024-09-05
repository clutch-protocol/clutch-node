use axum::{routing::get, Router};
use prometheus_client::encoding::text::encode as prometheus_encode;
use prometheus_client::metrics::counter::Counter;
use prometheus_client::registry::Registry;
use std::sync::{Arc, Mutex};

use super::config::AppConfig;

lazy_static::lazy_static! {
    pub static ref DEFAULT_COUNTER: Counter = Counter::default();

    static ref REGISTRY: Arc<Mutex<Registry>> = {
        let mut registry = Registry::default();
        registry.register(
            "http_requests",
            "Number of HTTP requests received",
            DEFAULT_COUNTER.clone(),
        );
        Arc::new(Mutex::new(registry))
    };
}

pub fn serve_metrics(config: &AppConfig) {
    let addr = config.serve_metric_addr.clone();

    tokio::spawn(async move {
        let app = Router::new()
            .route("/", get(track_and_respond))
            .route("/metrics", get(metrics_handler));

        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    });
}

async fn track_and_respond() -> &'static str {
    DEFAULT_COUNTER.inc();
    "Hello, World!"
}

async fn metrics_handler() -> String {
    let mut buffer = String::new();
    let registry = REGISTRY.lock().unwrap();
    prometheus_encode(&mut buffer, &*registry).unwrap();
    buffer
}