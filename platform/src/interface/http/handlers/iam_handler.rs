use axum::{
    extract::{State, Json},
    http::StatusCode,
    response::IntoResponse,
};
use crate::application::iam::service::PlatformService;
use crate::application::iam::rbac::RbacService;
use crate::proto::{CreateUserRequest, CreateRoleRequest, CreatePermissionRequest, AssignRoleRequest, AssignPermissionRequest};
use std::sync::Arc;

pub async fn create_user(
    State((platform_service, _)): State<(Arc<PlatformService>, Arc<RbacService>)>,
    Json(payload): Json<CreateUserRequest>,
) -> impl IntoResponse {
    match platform_service.create_user(payload).await {
        Ok(user) => (StatusCode::CREATED, Json(user)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn create_role(
    State((_, rbac_service)): State<(Arc<PlatformService>, Arc<RbacService>)>,
    Json(payload): Json<CreateRoleRequest>,
) -> impl IntoResponse {
    match rbac_service.create_role(payload).await {
        Ok(role) => (StatusCode::CREATED, Json(role)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn create_permission(
    State((_, rbac_service)): State<(Arc<PlatformService>, Arc<RbacService>)>,
    Json(payload): Json<CreatePermissionRequest>,
) -> impl IntoResponse {
    match rbac_service.create_permission(payload).await {
        Ok(perm) => (StatusCode::CREATED, Json(perm)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn assign_role(
    State((_, rbac_service)): State<(Arc<PlatformService>, Arc<RbacService>)>,
    Json(payload): Json<AssignRoleRequest>,
) -> impl IntoResponse {
    match rbac_service.assign_role_to_user(payload).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn assign_permission(
    State((_, rbac_service)): State<(Arc<PlatformService>, Arc<RbacService>)>,
    Json(payload): Json<AssignPermissionRequest>,
) -> impl IntoResponse {
    match rbac_service.assign_permission_to_role(payload).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
