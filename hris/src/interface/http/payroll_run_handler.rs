use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Json, Router,
};
use std::sync::Arc;
use uuid::Uuid;
use crate::application::payroll_run::service::PayrollRunService;
use crate::application::payroll_run::dto::CreatePayrollRunRequest;
use crate::domain::shared::dtos::PaginationQuery;

pub fn router(service: Arc<PayrollRunService>) -> Router {
    Router::new()
        .route("/", get(get_all_payroll_runs).post(create_payroll_run))
        .route("/:id", get(get_payroll_run_by_id))
        .with_state(service)
}

async fn get_all_payroll_runs(
    State(service): State<Arc<PayrollRunService>>,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<crate::domain::shared::dtos::PaginatedResponse<crate::domain::payroll_run::entity::Model>>, String> {
    service.get_all_payroll_runs(query).await
        .map(Json)
        .map_err(|e| e.to_string())
}

async fn get_payroll_run_by_id(
    State(service): State<Arc<PayrollRunService>>,
    Path(id): Path<Uuid>,
) -> Result<Json<crate::domain::payroll_run::entity::Model>, String> {
    service.get_payroll_run_by_id(id).await
        .map(Json)
        .map_err(|e| e.to_string())
}

async fn create_payroll_run(
    State(service): State<Arc<PayrollRunService>>,
    Json(req): Json<CreatePayrollRunRequest>,
) -> Result<Json<crate::domain::payroll_run::entity::Model>, String> {
    service.create_payroll_run(req).await
        .map(Json)
        .map_err(|e| e.to_string())
}
