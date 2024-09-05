use axum::{routing::get, Router};
use prometheus_client::encoding::text::encode as prometheus_encode;
use prometheus_client::metrics::counter::Counter;
use prometheus_client::metrics::family::Family;
use prometheus_client::metrics::gauge::Gauge;
use prometheus_client::registry::Registry;
use prometheus_client::encoding::{EncodeLabelSet, EncodeLabelValue};
use std::sync::{Arc, Mutex};

use super::config::AppConfig;

// Define custom label types
#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
struct Labels {
    method: Method,
    path: String,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelValue)]
enum Method {
    GET,
    PUT,
}

pub fn serve_metrics(config: &AppConfig) {
    let addr = config.serve_metric_addr.clone();

    tokio::spawn(async move {


        // Create the Prometheus registry
        let mut registry = Registry::default();

        // Create a Family of counters for HTTP requests, labeled by method and path
        let http_requests: Family<Labels, Gauge> = Family::default();

        // Register the family with the registry
        registry.register(
            "http_requests",
            "Number of HTTP requests received",
            http_requests.clone(),
        );

        // Share the registry using Arc/Mutex for thread safety
        let registry = Arc::new(Mutex::new(registry));
        let http_requests = Arc::new(http_requests);

        // Define the Axum app with / and /metrics routes
        let app = Router::new()
            .route("/", get(move || track_and_respond(Arc::clone(&http_requests), "/")))
            .route("/metrics", get(move || metrics_handler(Arc::clone(&registry))));

        // Bind to the address and start the server
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    });
}

// Track HTTP requests and respond to the client
async fn track_and_respond(http_requests: Arc<Family<Labels, Gauge>>, path: &'static str) -> &'static str {
    // Record a GET request to the specified path
    http_requests
        .get_or_create(&Labels {
            method: Method::GET,
            path: path.to_string(),
        })
        .inc();

    "Hello, World!"
}

// Metrics handler for the /metrics endpoint
async fn metrics_handler(registry: Arc<Mutex<Registry>>) -> String {
    let mut buffer = String::new();
    let registry = registry.lock().unwrap();
    prometheus_encode(&mut buffer, &*registry).unwrap();
    buffer
}