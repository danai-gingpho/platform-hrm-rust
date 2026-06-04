use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Json, Router,
};
use std::sync::Arc;
use uuid::Uuid;
use crate::application::payroll_period::service::PayrollPeriodService;
use crate::application::payroll_period::dto::CreatePayrollPeriodRequest;
use crate::domain::shared::dtos::PaginationQuery;

pub fn router(service: Arc<PayrollPeriodService>) -> Router {
    Router::new()
        .route("/", get(get_all_payroll_periods).post(create_payroll_period))
        .route("/:id", get(get_payroll_period_by_id))
        .with_state(service)
}

async fn get_all_payroll_periods(
    State(service): State<Arc<PayrollPeriodService>>,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<crate::domain::shared::dtos::PaginatedResponse<crate::domain::payroll_period::entity::Model>>, String> {
    service.get_all_payroll_periods(query).await
        .map(Json)
        .map_err(|e| e.to_string())
}

async fn get_payroll_period_by_id(
    State(service): State<Arc<PayrollPeriodService>>,
    Path(id): Path<Uuid>,
) -> Result<Json<crate::domain::payroll_period::entity::Model>, String> {
    service.get_payroll_period_by_id(id).await
        .map(Json)
        .map_err(|e| e.to_string())
}

async fn create_payroll_period(
    State(service): State<Arc<PayrollPeriodService>>,
    Json(req): Json<CreatePayrollPeriodRequest>,
) -> Result<Json<crate::domain::payroll_period::entity::Model>, String> {
    service.create_payroll_period(req).await
        .map(Json)
        .map_err(|e| e.to_string())
}
