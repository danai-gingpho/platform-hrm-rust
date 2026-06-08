use axum::{
    extract::{State, Json, Path},
    routing::{post, get},
    Router,
};
use serde::{Deserialize, Serialize};
use crate::interface::http::state::AppState;
use crate::infrastructure::grpc::hris::{
    GetEmployeeRequest, CreateEmployeeRequest,
    GetLegalEntityRequest, CreateLegalEntityRequest,
};
use crate::domain::errors::GatewayError;

pub fn routes(_state: AppState) -> Router<AppState> {
    Router::new()
        .route("/employees", post(create_employee))
        .route("/employees/:id", get(get_employee))
        .route("/legal-entities", post(create_legal_entity))
        .route("/legal-entities/:id", get(get_legal_entity))
}

// Employee
#[derive(Deserialize)]
pub struct CreateEmployeeDto {
    pub employee_no: String,
    pub citizen_id: String,
    pub title: String,
    pub first_name_th: String,
    pub last_name_th: String,
    pub email: String,
}

#[derive(Serialize)]
pub struct EmployeeResponseDto {
    pub id: String,
    pub employee_no: String,
    pub first_name_th: String,
    pub last_name_th: String,
    pub email: String,
    pub status: String,
}

async fn create_employee(
    State(mut state): State<AppState>,
    Json(payload): Json<CreateEmployeeDto>,
) -> Result<Json<EmployeeResponseDto>, GatewayError> {
    let response = state.hris_employee_client
        .create_employee(CreateEmployeeRequest {
            employee_no: payload.employee_no,
            citizen_id: payload.citizen_id,
            title: payload.title,
            first_name_th: payload.first_name_th,
            last_name_th: payload.last_name_th,
            email: payload.email,
            ..Default::default()
        })
        .await
        .map_err(|e| GatewayError::Internal(anyhow::anyhow!("gRPC error: {}", e)))?;

    let res = response.into_inner();
    Ok(Json(EmployeeResponseDto {
        id: res.id,
        employee_no: res.employee_no,
        first_name_th: res.first_name_th,
        last_name_th: res.last_name_th,
        email: res.email,
        status: res.status,
    }))
}

async fn get_employee(
    State(mut state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<EmployeeResponseDto>, GatewayError> {
    let response = state.hris_employee_client
        .get_employee(GetEmployeeRequest { id })
        .await
        .map_err(|e| GatewayError::Internal(anyhow::anyhow!("gRPC error: {}", e)))?;

    let res = response.into_inner();
    Ok(Json(EmployeeResponseDto {
        id: res.id,
        employee_no: res.employee_no,
        first_name_th: res.first_name_th,
        last_name_th: res.last_name_th,
        email: res.email,
        status: res.status,
    }))
}

// Legal Entity
#[derive(Deserialize)]
pub struct CreateLegalEntityDto {
    pub code: String,
    pub tax_id: String,
    pub name_th: String,
}

#[derive(Serialize)]
pub struct LegalEntityResponseDto {
    pub id: String,
    pub code: String,
    pub name_th: String,
    pub is_active: bool,
}

async fn create_legal_entity(
    State(mut state): State<AppState>,
    Json(payload): Json<CreateLegalEntityDto>,
) -> Result<Json<LegalEntityResponseDto>, GatewayError> {
    let response = state.hris_legal_entity_client
        .create_legal_entity(CreateLegalEntityRequest {
            code: payload.code,
            tax_id: payload.tax_id,
            name_th: payload.name_th,
            ..Default::default()
        })
        .await
        .map_err(|e| GatewayError::Internal(anyhow::anyhow!("gRPC error: {}", e)))?;

    let res = response.into_inner();
    Ok(Json(LegalEntityResponseDto {
        id: res.id,
        code: res.code,
        name_th: res.name_th,
        is_active: res.is_active,
    }))
}

async fn get_legal_entity(
    State(mut state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<LegalEntityResponseDto>, GatewayError> {
    let response = state.hris_legal_entity_client
        .get_legal_entity(GetLegalEntityRequest { id })
        .await
        .map_err(|e| GatewayError::Internal(anyhow::anyhow!("gRPC error: {}", e)))?;

    let res = response.into_inner();
    Ok(Json(LegalEntityResponseDto {
        id: res.id,
        code: res.code,
        name_th: res.name_th,
        is_active: res.is_active,
    }))
}
