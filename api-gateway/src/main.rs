use api_gateway::{
    bootstrap, 
    config::Config, 
    interface::{http, grpc}, 
    observability,
};
use tonic::transport::Server;
use tower::ServiceExt as _; 
use axum::ServiceExt as _;
use http_body_util::BodyExt;
use axum::response::IntoResponse;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv();

    observability::tracing::init();
    observability::metrics::init();

    let config_path = std::env::var("CONFIG_PATH").unwrap_or_else(|_| "config.toml".into());
    let cfg = Config::load(&config_path)?;

    tracing::info!(
        addr = %cfg.server.addr,
        config = %config_path,
        upstreams = cfg.upstream.len(),
        routes = cfg.route.len(),
        "starting api-gateway with multiplexing"
    );

    let state = bootstrap::build_state(&cfg).await?;
    
    // 1. Build Tonic Server
    let grpc_service = Server::builder()
        .add_service(grpc::health::service())
        .add_service(
            tonic_reflection::server::Builder::configure()
                .register_encoded_file_descriptor_set(tonic_health::pb::FILE_DESCRIPTOR_SET)
                .build_v1alpha()?
        )
        .into_service()
        .map_request(|req: axum::http::Request<axum::body::Body>| {
            req.map(|b| tonic::body::BoxBody::new(b.map_err(|e| tonic::Status::internal(e.to_string()))))
        })
        .map_response(|resp| resp.map(axum::body::Body::new))
        .handle_error(|e: Box<dyn std::error::Error + Send + Sync>| async move {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Internal Service Error: {}", e),
            ).into_response()
        });

    // 2. Build Axum Router and use Tonic as fallback for gRPC
    let app = http::build(state.clone(), cfg.clone())
        .fallback_service(grpc_service);

    let listener = tokio::net::TcpListener::bind(&cfg.server.addr).await?;
    
    axum::serve(listener, app).await?;

    Ok(())
}
