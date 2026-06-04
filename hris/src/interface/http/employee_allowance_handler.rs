use axum::{
    extract::{Path, Query, State},
    routing::{get},
    Json, Router,
};
use std::sync::Arc;
use uuid::Uuid;
use crate::application::employee_allowance::service::EmployeeAllowanceService;
use crate::application::employee_allowance::dto::CreateEmployeeAllowanceRequest;
use crate::domain::shared::dtos::PaginationQuery;

pub fn router(service: Arc<EmployeeAllowanceService>) -> Router {
    Router::new()
        .route("/", get(get_all_allowances).post(create_allowance))
        .route("/:id", get(get_allowance_by_id))
        .with_state(service)
}

async fn get_all_allowances(
    State(service): State<Arc<EmployeeAllowanceService>>,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<crate::domain::shared::dtos::PaginatedResponse<crate::domain::employee_allowance::entity::Model>>, String> {
    service.get_all_allowances(query).await
        .map(Json)
        .map_err(|e| e.to_string())
}

async fn get_allowance_by_id(
    State(service): State<Arc<EmployeeAllowanceService>>,
    Path(id): Path<Uuid>,
) -> Result<Json<crate::domain::employee_allowance::entity::Model>, String> {
    service.get_allowance_by_id(id).await
        .map(Json)
        .map_err(|e| e.to_string())
}

async fn create_allowance(
    State(service): State<Arc<EmployeeAllowanceService>>,
    Json(req): Json<CreateEmployeeAllowanceRequest>,
) -> Result<Json<crate::domain::employee_allowance::entity::Model>, String> {
    service.create_allowance(req).await
        .map(Json)
        .map_err(|e| e.to_string())
}
