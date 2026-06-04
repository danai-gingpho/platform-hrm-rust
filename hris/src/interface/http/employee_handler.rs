use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Json, Router,
};
use std::sync::Arc;
use uuid::Uuid;
use crate::application::employee::service::EmployeeService;
use crate::application::employee::dto::CreateEmployeeRequest;
use crate::domain::shared::dtos::PaginationQuery;

pub fn router(service: Arc<EmployeeService>) -> Router {
    Router::new()
        .route("/", get(get_all_employees).post(create_employee))
        .route("/:id", get(get_employee_by_id))
        .with_state(service)
}

async fn get_all_employees(
    State(service): State<Arc<EmployeeService>>,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<crate::domain::shared::dtos::PaginatedResponse<crate::domain::employee::entity::Model>>, String> {
    service.get_all_employees(query).await
        .map(Json)
        .map_err(|e| e.to_string())
}

async fn get_employee_by_id(
    State(service): State<Arc<EmployeeService>>,
    Path(id): Path<Uuid>,
) -> Result<Json<crate::domain::employee::entity::Model>, String> {
    service.get_employee_by_id(id).await
        .map(Json)
        .map_err(|e| e.to_string())
}

async fn create_employee(
    State(service): State<Arc<EmployeeService>>,
    Json(req): Json<CreateEmployeeRequest>,
) -> Result<Json<crate::domain::employee::entity::Model>, String> {
    service.create_employee(req).await
        .map(Json)
        .map_err(|e| e.to_string())
}
