use tonic::transport::Server;
use auth::interface::grpc::auth_service::AuthServiceImpl;
use auth::proto::auth_service_server::AuthServiceServer;
use auth::infrastructure::keycloak::KeycloakClient;
use auth::application::iam::rbac::RbacService;
use std::sync::Arc;
use sea_orm::DatabaseConnection;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/hrm_auth".to_string());
    let db = sea_orm::Database::connect(&db_url).await?;
    let db = Arc::new(db);

    let keycloak_url = std::env::var("KEYCLOAK_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
    let keycloak_admin_token = std::env::var("KEYCLOAK_ADMIN_TOKEN").unwrap_or_default();
    let keycloak_client = Arc::new(KeycloakClient::new(keycloak_url, keycloak_admin_token));

    let rbac_service = Arc::new(RbacService::new(db.clone()));
    let auth_service = AuthServiceImpl::new(keycloak_client, rbac_service);

    let addr = "[0, 0, 0, 0]:50051".parse()?;
    tracing::info!("Auth Service listening on {}", addr);

    Server::builder()
        .add_service(AuthServiceServer::new(auth_service))
        .serve(addr)
        .await?;

    Ok(())
}
