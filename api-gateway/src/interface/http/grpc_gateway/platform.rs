use axum::{
    extract::{State, Json},
    routing::{post, get},
    Router,
};
use serde::{Deserialize, Serialize};
use crate::interface::http::state::AppState;
use crate::infrastructure::grpc::platform::{
    CreateCompanyRequest, CreateUserRequest, CreateRoleRequest, CreatePermissionRequest,
    AssignRoleRequest, AssignPermissionRequest, GetCompanyDbCredentialsRequest
};
use crate::domain::errors::GatewayError;

pub fn routes(_state: AppState) -> Router<AppState> {
    Router::new()
        .route("/companies", post(create_company))
        .route("/companies/:id/db-credentials", get(get_company_db))
        .route("/users", post(create_user))
        .route("/roles", post(create_role))
        .route("/permissions", post(create_permission))
        .route("/roles/assign-user", post(assign_role))
        .route("/roles/assign-permission", post(assign_permission))
}

// Company
#[derive(Deserialize)]
pub struct CreateCompanyDto {
    pub name: String,
    pub code: String,
    pub admin_email: String,
}

#[derive(Serialize)]
pub struct CompanyResponseDto {
    pub id: String,
    pub name: String,
    pub code: String,
    pub schema_name: String,
}

async fn create_company(
    State(mut state): State<AppState>,
    Json(payload): Json<CreateCompanyDto>,
) -> Result<Json<CompanyResponseDto>, GatewayError> {
    let response = state.platform_client
        .create_company(CreateCompanyRequest {
            name: payload.name,
            code: payload.code,
            admin_email: payload.admin_email,
        })
        .await
        .map_err(|e| GatewayError::Internal(anyhow::anyhow!("gRPC error: {}", e)))?;

    let res = response.into_inner();
    Ok(Json(CompanyResponseDto {
        id: res.id,
        name: res.name,
        code: res.code,
        schema_name: res.schema_name,
    }))
}

#[derive(Serialize)]
pub struct DbCredentialsDto {
    pub username: String,
    pub host: String,
    pub db_name: String,
    pub schema_name: String,
}

async fn get_company_db(
    State(mut state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<DbCredentialsDto>, GatewayError> {
    let response = state.platform_client
        .get_company_db_credentials(GetCompanyDbCredentialsRequest {
            company_id: id,
        })
        .await
        .map_err(|e| GatewayError::Internal(anyhow::anyhow!("gRPC error: {}", e)))?;

    let res = response.into_inner();
    Ok(Json(DbCredentialsDto {
        username: res.username,
        host: res.host,
        db_name: res.db_name,
        schema_name: res.schema_name,
    }))
}

// User
#[derive(Deserialize)]
pub struct CreateUserDto {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub company_id: String,
}

#[derive(Serialize)]
pub struct UserResponseDto {
    pub id: String,
    pub email: String,
    pub keycloak_id: String,
}

async fn create_user(
    State(mut state): State<AppState>,
    Json(payload): Json<CreateUserDto>,
) -> Result<Json<UserResponseDto>, GatewayError> {
    let response = state.platform_client
        .create_user(CreateUserRequest {
            email: payload.email,
            first_name: payload.first_name,
            last_name: payload.last_name,
            company_id: payload.company_id,
        })
        .await
        .map_err(|e| GatewayError::Internal(anyhow::anyhow!("gRPC error: {}", e)))?;

    let res = response.into_inner();
    Ok(Json(UserResponseDto {
        id: res.id,
        email: res.email,
        keycloak_id: res.keycloak_id,
    }))
}

// RBAC
#[derive(Deserialize)]
pub struct CreateRoleDto {
    pub name: String,
    pub description: String,
}

#[derive(Serialize)]
pub struct RoleResponseDto {
    pub id: String,
    pub name: String,
}

async fn create_role(
    State(mut state): State<AppState>,
    Json(payload): Json<CreateRoleDto>,
) -> Result<Json<RoleResponseDto>, GatewayError> {
    let response = state.platform_client
        .create_role(CreateRoleRequest {
            name: payload.name,
            description: payload.description,
        })
        .await
        .map_err(|e| GatewayError::Internal(anyhow::anyhow!("gRPC error: {}", e)))?;

    let res = response.into_inner();
    Ok(Json(RoleResponseDto {
        id: res.id,
        name: res.name,
    }))
}

#[derive(Deserialize)]
pub struct CreatePermissionDto {
    pub name: String,
    pub code: String,
}

#[derive(Serialize)]
pub struct PermissionResponseDto {
    pub id: String,
    pub name: String,
    pub code: String,
}

async fn create_permission(
    State(mut state): State<AppState>,
    Json(payload): Json<CreatePermissionDto>,
) -> Result<Json<PermissionResponseDto>, GatewayError> {
    let response = state.platform_client
        .create_permission(CreatePermissionRequest {
            name: payload.name,
            code: payload.code,
        })
        .await
        .map_err(|e| GatewayError::Internal(anyhow::anyhow!("gRPC error: {}", e)))?;

    let res = response.into_inner();
    Ok(Json(PermissionResponseDto {
        id: res.id,
        name: res.name,
        code: res.code,
    }))
}

#[derive(Deserialize)]
pub struct AssignRoleDto {
    pub user_id: String,
    pub role_id: String,
}

async fn assign_role(
    State(mut state): State<AppState>,
    Json(payload): Json<AssignRoleDto>,
) -> Result<Json<bool>, GatewayError> {
    let response = state.platform_client
        .assign_role_to_user(AssignRoleRequest {
            user_id: payload.user_id,
            role_id: payload.role_id,
        })
        .await
        .map_err(|e| GatewayError::Internal(anyhow::anyhow!("gRPC error: {}", e)))?;

    Ok(Json(response.into_inner().success))
}

#[derive(Deserialize)]
pub struct AssignPermissionDto {
    pub role_id: String,
    pub permission_id: String,
}

async fn assign_permission(
    State(mut state): State<AppState>,
    Json(payload): Json<AssignPermissionDto>,
) -> Result<Json<bool>, GatewayError> {
    let response = state.platform_client
        .assign_permission_to_role(AssignPermissionRequest {
            role_id: payload.role_id,
            permission_id: payload.permission_id,
        })
        .await
        .map_err(|e| GatewayError::Internal(anyhow::anyhow!("gRPC error: {}", e)))?;

    Ok(Json(response.into_inner().success))
}
