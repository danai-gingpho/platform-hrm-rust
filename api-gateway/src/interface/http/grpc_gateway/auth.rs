use axum::{
    extract::{State, Json},
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use crate::interface::http::state::AppState;
use crate::infrastructure::grpc::auth::{
    CreateRealmRequest, CreateUserRequest, CreateRoleRequest, CreatePermissionRequest,
    AssignRoleRequest, AssignPermissionRequest
};
use crate::domain::errors::GatewayError;

pub fn routes(_state: AppState) -> Router<AppState> {
    Router::new()
        .route("/realms", post(create_realm))
        .route("/users", post(create_user))
        .route("/roles", post(create_role))
        .route("/permissions", post(create_permission))
        .route("/roles/assign-user", post(assign_role))
        .route("/roles/assign-permission", post(assign_permission))
}

#[derive(Deserialize)]
pub struct CreateRealmDto {
    pub realm_name: String,
}

#[derive(Serialize)]
pub struct CreateRealmResponseDto {
    pub success: bool,
}

async fn create_realm(
    State(mut state): State<AppState>,
    Json(payload): Json<CreateRealmDto>,
) -> Result<Json<CreateRealmResponseDto>, GatewayError> {
    let response = state.auth_client
        .create_realm(CreateRealmRequest {
            realm_name: payload.realm_name,
        })
        .await
        .map_err(|e| GatewayError::Internal(anyhow::anyhow!("gRPC error: {}", e)))?;

    Ok(Json(CreateRealmResponseDto {
        success: response.into_inner().success,
    }))
}

#[derive(Deserialize)]
pub struct CreateUserDto {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub realm: String,
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
    let response = state.auth_client
        .create_user(CreateUserRequest {
            email: payload.email,
            first_name: payload.first_name,
            last_name: payload.last_name,
            realm: payload.realm,
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
    let response = state.auth_client
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
    let response = state.auth_client
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
    let response = state.auth_client
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
    let response = state.auth_client
        .assign_permission_to_role(AssignPermissionRequest {
            role_id: payload.role_id,
            permission_id: payload.permission_id,
        })
        .await
        .map_err(|e| GatewayError::Internal(anyhow::anyhow!("gRPC error: {}", e)))?;

    Ok(Json(response.into_inner().success))
}
