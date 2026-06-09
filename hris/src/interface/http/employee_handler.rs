use crate::application::employee::dto::{CreateEmployeeRequest, UpdateEmployeeRequest};
use crate::application::employee::service::EmployeeService;
use crate::domain::shared::dtos::PaginationQuery;
use axum::{
    extract::{Path, Query, State},
    routing::get,
    Json, Router,
};
use std::sync::Arc;
use uuid::Uuid;

pub fn router(service: Arc<EmployeeService>) -> Router {
    Router::new()
        .route("/", get(get_all_employees).post(create_employee))
        .route(
            "/:id",
            get(get_employee_by_id)
                .put(update_employee)
                .delete(delete_employee),
        )
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

async fn update_employee(
    State(service): State<Arc<EmployeeService>>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateEmployeeRequest>,
) -> Result<Json<crate::domain::employee::entity::Model>, String> {
    service.update_employee(id, req).await
        .map(Json)
        .map_err(|e| e.to_string())
}

async fn delete_employee(
    State(service): State<Arc<EmployeeService>>,
    Path(id): Path<Uuid>,
) -> Result<Json<()>, String> {
    service.delete_employee(id).await
        .map(|_| Json(()))
        .map_err(|e| e.to_string())
}
