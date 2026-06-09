use axum::{
    extract::{State, Json},
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use crate::interface::http::state::AppState;
use crate::infrastructure::grpc::owner::{LoginRequest, CreateStaffRequest};
use crate::domain::errors::GatewayError;

pub fn routes(_state: AppState) -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
        .route("/staff", post(create_staff))
        .route("/status", axum::routing::get(get_system_status))
}

#[derive(Serialize)]
pub struct SystemStatusDto {
    pub status: String,
    pub active_tenants: u32,
    pub version: String,
}

async fn get_system_status(
    State(mut state): State<AppState>,
) -> Result<Json<SystemStatusDto>, GatewayError> {
    let response = state.owner_client
        .get_system_status(crate::infrastructure::grpc::owner::SystemStatusRequest {})
        .await
        .map_err(|e| GatewayError::Internal(anyhow::anyhow!("gRPC error: {}", e)))?;

    let res = response.into_inner();
    Ok(Json(SystemStatusDto {
        status: res.status,
        active_tenants: res.active_tenants,
        version: res.version,
    }))
}

#[derive(Deserialize)]
pub struct LoginDto {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponseDto {
    pub access_token: String,
    pub staff: Option<StaffResponseDto>,
}

#[derive(Serialize)]
pub struct StaffResponseDto {
    pub id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

async fn login(
    State(mut state): State<AppState>,
    Json(payload): Json<LoginDto>,
) -> Result<Json<AuthResponseDto>, GatewayError> {
    let response = state.owner_client
        .login(LoginRequest {
            email: payload.email,
            password: payload.password,
        })
        .await
        .map_err(|e| GatewayError::Internal(anyhow::anyhow!("gRPC error: {}", e)))?;

    let res = response.into_inner();
    Ok(Json(AuthResponseDto {
        access_token: res.access_token,
        staff: res.staff.map(|s| StaffResponseDto {
            id: s.id,
            email: s.email,
            first_name: s.first_name,
            last_name: s.last_name,
        }),
    }))
}

#[derive(Deserialize)]
pub struct CreateStaffDto {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}

async fn create_staff(
    State(mut state): State<AppState>,
    Json(payload): Json<CreateStaffDto>,
) -> Result<Json<StaffResponseDto>, GatewayError> {
    let response = state.owner_client
        .create_staff(CreateStaffRequest {
            email: payload.email,
            password: payload.password,
            first_name: payload.first_name,
            last_name: payload.last_name,
        })
        .await
        .map_err(|e| GatewayError::Internal(anyhow::anyhow!("gRPC error: {}", e)))?;

    let s = response.into_inner();
    Ok(Json(StaffResponseDto {
        id: s.id,
        email: s.email,
        first_name: s.first_name,
        last_name: s.last_name,
    }))
}
