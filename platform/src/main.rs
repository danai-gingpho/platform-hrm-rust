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

// Owner imports
use platform::proto::owner::owner_auth_service_server::OwnerAuthServiceServer;
use platform::interface::owner::grpc::auth_handler::AuthHandler;
use platform::application::owner::auth::AuthService as OwnerAuthService;
use platform::application::owner::staff::StaffService as OwnerStaffService;
use platform::infrastructure::owner::db::staff::SeaOrmStaffRepository;
use platform::infrastructure::owner::db::role::SeaOrmRoleRepository;
use platform::infrastructure::owner::db::permission::SeaOrmPermissionRepository;
use platform::infrastructure::owner::db::user_role::SeaOrmStaffRoleRepository;
use platform::infrastructure::owner::db::role_permission::SeaOrmRolePermissionRepository;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    // 1. Initialize Databases
    let platform_db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/hrm_platform".to_string());
    let platform_db = sea_orm::Database::connect(&platform_db_url).await?;
    let platform_db = Arc::new(platform_db);

    let owner_db_url = std::env::var("OWNER_DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/hrm_owner".to_string());
    let owner_db = sea_orm::Database::connect(&owner_db_url).await?;
    let owner_db = Arc::new(owner_db);

    // Handle migrations for platform
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|arg| arg == "--migrate") {
        platform::db::migrations::run_migrations(platform_db.as_ref()).await?;
        if args.len() == 2 {
            return Ok(());
        }
    }

    // 2. Initialize Owner Repositories & Services
    let staff_repo = Arc::new(SeaOrmStaffRepository::new((*owner_db).clone()));
    let role_repo = Arc::new(SeaOrmRoleRepository::new((*owner_db).clone()));
    let perm_repo = Arc::new(SeaOrmPermissionRepository::new((*owner_db).clone()));
    let staff_role_repo = Arc::new(SeaOrmStaffRoleRepository::new((*owner_db).clone()));
    let role_perm_repo = Arc::new(SeaOrmRolePermissionRepository::new((*owner_db).clone()));

    let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());
    let owner_auth_service = Arc::new(OwnerAuthService::new(
        staff_repo.clone(),
        staff_role_repo.clone(),
        role_perm_repo.clone(),
        perm_repo.clone(),
        jwt_secret,
    ));
    let owner_staff_service = Arc::new(OwnerStaffService::new(staff_repo.clone()));

    // 3. Initialize Platform Services
    // Auth Service gRPC Client (Keycloak wrapper)
    let auth_service_url = std::env::var("AUTH_SERVICE_URL").unwrap_or_else(|_| "http://localhost:50051".to_string());
    let auth_client = AuthServiceClient::connect(auth_service_url).await?;

    let platform_service = Arc::new(PlatformService::new(platform_db.clone(), auth_client.clone()));
    let rbac_service = Arc::new(RbacService::new(auth_client.clone()));

    // 4. Initialize gRPC Handlers
    let platform_grpc_impl = PlatformServiceImpl::new(platform_service.clone(), rbac_service.clone());
    let platform_grpc_server = PlatformServiceServer::new(platform_grpc_impl);

    let owner_grpc_impl = AuthHandler::new(owner_auth_service, owner_staff_service);
    let owner_grpc_server = OwnerAuthServiceServer::new(owner_grpc_impl);

    // 5. Axum Router & gRPC integration
    let state = AppState {
        platform_service: platform_service.clone(),
        rbac_service: rbac_service.clone(),
    };

    let app = create_router(state)
        .nest_service("/platform.PlatformService", platform_grpc_server)
        .nest_service("/owner.OwnerAuthService", owner_grpc_server)
        .layer(
            ServiceBuilder::new()
                .layer(axum::Extension(platform_db.clone()))
                .layer(axum::middleware::from_fn(auth_middleware))
                .layer(axum::middleware::from_fn(company_middleware))
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("Unified Platform Service listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
