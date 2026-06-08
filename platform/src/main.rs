use std::net::SocketAddr;
use tower::ServiceBuilder;
use platform::interface::grpc::platform_service::PlatformServiceImpl;
use platform::proto::platform_service_server::PlatformServiceServer;
use std::sync::Arc;
use platform::middleware::{auth::auth_middleware, company::company_middleware};
use platform::application::iam::service::PlatformService;
use platform::application::iam::rbac::RbacService;
use platform::interface::http::{create_router, AppState};
use platform::proto::auth::auth_service_client::AuthServiceClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/hrm_platform".to_string());
    let db = sea_orm::Database::connect(&db_url).await?;
    let db = Arc::new(db);

    // Handle migrations
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|arg| arg == "--migrate") {
        platform::db::migrations::run_migrations(db.as_ref()).await?;
        if args.len() == 2 {
            return Ok(());
        }
    }

    // Auth Service gRPC Client
    let auth_service_url = std::env::var("AUTH_SERVICE_URL").unwrap_or_else(|_| "http://localhost:50051".to_string());
    let auth_client = AuthServiceClient::connect(auth_service_url).await?;

    let platform_service = Arc::new(PlatformService::new(db.clone(), auth_client.clone()));
    let rbac_service = Arc::new(RbacService::new(auth_client.clone()));

    let grpc_impl = PlatformServiceImpl::new(platform_service.clone(), rbac_service.clone());
    let grpc_server = PlatformServiceServer::new(grpc_impl);

    let state = AppState {
        platform_service: platform_service.clone(),
        rbac_service: rbac_service.clone(),
    };

    let app = create_router(state)
        .nest_service("/platform.PlatformService", grpc_server)
        .layer(
            ServiceBuilder::new()
                .layer(axum::Extension(db.clone()))
                .layer(axum::middleware::from_fn(auth_middleware))
                .layer(axum::middleware::from_fn(company_middleware))
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
