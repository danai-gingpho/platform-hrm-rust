use crate::infrastructure::grpc::hris::{
    CreateBranchRequest, CreateEmployeeRequest, CreateLegalEntityRequest, DeleteEmployeeRequest,
    GetBranchRequest, GetEmployeeRequest, GetLegalEntityRequest, ListEmployeesRequest,
    UpdateEmployeeRequest,
};
use axum::{
    extract::{Json, Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Router,
};
use serde::{Deserialize, Serialize};
use crate::interface::http::state::AppState;
use crate::domain::errors::GatewayError;

pub fn routes(_state: AppState) -> Router<AppState> {
    Router::new()
        .route("/employees", post(create_employee).get(list_employees))
        .route(
            "/employees/:id",
            get(get_employee).put(update_employee).delete(delete_employee),
        )
        .route("/legal-entities", post(create_legal_entity))
        .route("/legal-entities/:id", get(get_legal_entity))
        .route("/branches", post(create_branch))
        .route("/branches/:id", get(get_branch))
}

// Branch
#[derive(Deserialize)]
pub struct CreateBranchDto {
    pub legal_entity_id: String,
    pub code: String,
    pub name: String,
    pub timezone: String,
    pub address: Option<String>,
}

#[derive(Serialize)]
pub struct BranchResponseDto {
    pub id: String,
    pub legal_entity_id: String,
    pub code: String,
    pub name: String,
    pub timezone: String,
    pub address: Option<String>,
    pub created_at: String,
}

async fn create_branch(
    State(mut state): State<AppState>,
    Json(payload): Json<CreateBranchDto>,
) -> Result<Json<BranchResponseDto>, GatewayError> {
    let response = state.hris_branch_client
        .create_branch(CreateBranchRequest {
            legal_entity_id: Some(payload.legal_entity_id),
            code: Some(payload.code),
            name: Some(payload.name),
            timezone: Some(payload.timezone),
            address: Some(payload.address.unwrap_or_default()),
        })
        .await
        .map_err(|e| GatewayError::Internal(anyhow::anyhow!("gRPC error: {}", e)))?;

    let res = response.into_inner();
    Ok(Json(BranchResponseDto {
        id: res.id,
        legal_entity_id: res.legal_entity_id,
        code: res.code,
        name: res.name,
        timezone: res.timezone,
        address: Some(res.address),
        created_at: res.created_at,
    }))
}

async fn get_branch(
    State(mut state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<BranchResponseDto>, GatewayError> {
    let response = state.hris_branch_client
        .get_branch(GetBranchRequest { id })
        .await
        .map_err(|e| GatewayError::Internal(anyhow::anyhow!("gRPC error: {}", e)))?;

    let res = response.into_inner();
    Ok(Json(BranchResponseDto {
        id: res.id,
        legal_entity_id: res.legal_entity_id,
        code: res.code,
        name: res.name,
        timezone: res.timezone,
        address: Some(res.address),
        created_at: res.created_at,
    }))
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

#[derive(Deserialize)]
pub struct UpdateEmployeeDto {
    pub citizen_id: Option<String>,
    pub passport_no: Option<String>,
    pub title: Option<String>,
    pub first_name_th: Option<String>,
    pub last_name_th: Option<String>,
    pub email: Option<String>,
    pub status: Option<String>,
}

#[derive(Deserialize)]
pub struct PaginationQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

#[derive(Serialize)]
pub struct ListResponseDto<T> {
    pub items: Vec<T>,
    pub total_items: u32,
    pub total_pages: u32,
    pub current_page: u32,
    pub per_page: u32,
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

async fn update_employee(
    State(mut state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateEmployeeDto>,
) -> Result<Json<EmployeeResponseDto>, GatewayError> {
    let response = state.hris_employee_client
        .update_employee(UpdateEmployeeRequest {
            id,
            citizen_id: payload.citizen_id,
            passport_no: payload.passport_no,
            title: payload.title,
            first_name_th: payload.first_name_th,
            last_name_th: payload.last_name_th,
            email: payload.email,
            status: payload.status,
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

async fn delete_employee(
    State(mut state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, GatewayError> {
    state.hris_employee_client
        .delete_employee(DeleteEmployeeRequest { id })
        .await
        .map_err(|e| GatewayError::Internal(anyhow::anyhow!("gRPC error: {}", e)))?;

    Ok(StatusCode::NO_CONTENT)
}

async fn list_employees(
    State(mut state): State<AppState>,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<ListResponseDto<EmployeeResponseDto>>, GatewayError> {
    let response = state.hris_employee_client
        .list_employees(ListEmployeesRequest {
            page: query.page.unwrap_or(1),
            per_page: query.per_page.unwrap_or(10),
        })
        .await
        .map_err(|e| GatewayError::Internal(anyhow::anyhow!("gRPC error: {}", e)))?;

    let res = response.into_inner();
    let items = res.items.into_iter().map(|item| EmployeeResponseDto {
        id: item.id,
        employee_no: item.employee_no,
        first_name_th: item.first_name_th,
        last_name_th: item.last_name_th,
        email: item.email,
        status: item.status,
    }).collect();

    Ok(Json(ListResponseDto {
        items,
        total_items: res.total_items,
        total_pages: res.total_pages,
        current_page: res.current_page,
        per_page: res.per_page,
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
