pub mod application;
pub mod db;
pub mod domain;
pub mod interface;
pub mod infrastructure;

pub mod owner {
    tonic::include_proto!("owner");
}

use std::sync::Arc;
use tonic::transport::Server;
use crate::owner::owner_auth_service_server::OwnerAuthServiceServer;
use crate::interface::grpc::auth_handler::AuthHandler;
use crate::application::auth::AuthService;
use crate::application::staff::StaffService;
use crate::infrastructure::db::staff::SeaOrmStaffRepository;
use crate::infrastructure::db::role::SeaOrmRoleRepository;
use crate::infrastructure::db::permission::SeaOrmPermissionRepository;
use crate::infrastructure::db::user_role::SeaOrmStaffRoleRepository;
use crate::infrastructure::db::role_permission::SeaOrmRolePermissionRepository;
use crate::infrastructure::db::create_connection;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    // 1. Initialize Database
    let db = create_connection().await?;
    println!("Database connected.");

    // Seed Data
    if let Err(e) = crate::infrastructure::db::seeder::seed(db.clone()).await {
        println!("Seeding failed: {}", e);
    }

    // 2. Initialize Repositories
    let staff_repo = Arc::new(SeaOrmStaffRepository::new(db.clone()));
    let role_repo = Arc::new(SeaOrmRoleRepository::new(db.clone()));
    let perm_repo = Arc::new(SeaOrmPermissionRepository::new(db.clone()));
    let staff_role_repo = Arc::new(SeaOrmStaffRoleRepository::new(db.clone()));
    let role_perm_repo = Arc::new(SeaOrmRolePermissionRepository::new(db.clone()));

    // 3. Initialize Application Services
    let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());
    
    let auth_service = Arc::new(AuthService::new(
        staff_repo.clone(),
        staff_role_repo.clone(),
        role_perm_repo.clone(),
        perm_repo.clone(),
        jwt_secret,
    ));
    
    let staff_service = Arc::new(StaffService::new(staff_repo.clone()));

    // 4. Initialize gRPC Handlers
    let auth_handler = AuthHandler::new(auth_service, staff_service);

    // 5. Start gRPC Server
    let addr = std::env::var("SERVER_ADDR").unwrap_or_else(|_| "[::1]:50051".to_string()).parse()?;
    println!("OwnerService listening on {}", addr);

    Server::builder()
        .add_service(OwnerAuthServiceServer::new(auth_handler))
        .serve(addr)
        .await?;

    Ok(())
}
