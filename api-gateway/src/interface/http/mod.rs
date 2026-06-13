pub mod error;
pub mod grpc_gateway;
pub mod middleware;
pub mod proxy;
pub mod state;

use std::sync::Arc;
use std::time::Duration;

use axum::{
    body::Body,
    http::{HeaderName, Request},
    middleware::from_fn,
    routing::{any, get},
    Json, Router,
};
use serde_json::json;
use tower::ServiceBuilder;
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};
use tower_http::{
    compression::CompressionLayer,
    cors::CorsLayer,
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    timeout::TimeoutLayer,
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

use state::AppState;

pub fn build(state: AppState, cfg: crate::config::Config) -> Router {
    let request_id_header: HeaderName = HeaderName::from_static("x-request-id");

    let gov_config = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(cfg.server.rate_limit_rps)
            .burst_size(cfg.server.rate_limit_burst)
            .finish()
            .expect("rate limit config"),
    );

    let public = Router::new()
        .route("/health", get(health))
        .route("/metrics", get(metrics_stub));

    let mw_state = state.clone();
    let protected = Router::new()
        .merge(gateway)
        .fallback(any(proxy::handler::proxy))
        .layer(from_fn(move |req, next| {
            let s = mw_state.clone();
            async move { middleware::auth::middleware(s, req, next).await }
        }));

    let app = Router::new()
        .merge(public)
        .merge(protected)
        .layer(from_fn(middleware::metrics::middleware))
        .layer(GovernorLayer {
            config: gov_config,
        })
        .with_state(state)
        .layer(
            ServiceBuilder::new()
                .layer(SetRequestIdLayer::new(
                    request_id_header.clone(),
                    MakeRequestUuid,
                ))
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(
                            DefaultMakeSpan::new()
                                .level(Level::INFO)
                                .include_headers(false),
                        )
                        .on_request(|req: &Request<Body>, _span: &tracing::Span| {
                            tracing::info!(
                                method = %req.method(),
                                uri    = %req.uri(),
                                "→"
                            );
                        })
                        .on_response(DefaultOnResponse::new().level(Level::INFO)),
                )
                .layer(PropagateRequestIdLayer::new(request_id_header))
                .layer(CorsLayer::permissive())
                .layer(CompressionLayer::new())
                .layer(TimeoutLayer::with_status_code(
                    axum::http::StatusCode::GATEWAY_TIMEOUT,
                    Duration::from_millis(cfg.server.timeout_ms),
                )),
        );

    app
}

async fn health() -> Json<serde_json::Value> {
    Json(json!({ "status": "ok", "service": "api-gateway" }))
}

async fn metrics_stub() -> String {
    crate::observability::metrics::render()
}
