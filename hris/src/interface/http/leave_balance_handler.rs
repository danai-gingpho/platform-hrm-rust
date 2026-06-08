use axum::{
    extract::{Path, Query, State},
    routing::{get},
    Json, Router,
};
use std::sync::Arc;
use uuid::Uuid;
use crate::application::leave_balance::service::LeaveBalanceService;
use crate::application::leave_balance::dto::CreateLeaveBalanceRequest;
use crate::domain::shared::dtos::PaginationQuery;

pub fn router(service: Arc<LeaveBalanceService>) -> Router {
    Router::new()
        .route("/", get(get_all_leave_balances).post(create_leave_balance))
        .route("/:id", get(get_leave_balance_by_id))
        .with_state(service)
}

async fn get_all_leave_balances(
    State(service): State<Arc<LeaveBalanceService>>,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<crate::domain::shared::dtos::PaginatedResponse<crate::domain::leave_balance::entity::Model>>, String> {
    service.get_all_leave_balances(query).await
        .map(Json)
        .map_err(|e| e.to_string())
}

async fn get_leave_balance_by_id(
    State(service): State<Arc<LeaveBalanceService>>,
    Path(id): Path<Uuid>,
) -> Result<Json<crate::domain::leave_balance::entity::Model>, String> {
    service.get_leave_balance_by_id(id).await
        .map(Json)
        .map_err(|e| e.to_string())
}

async fn create_leave_balance(
    State(service): State<Arc<LeaveBalanceService>>,
    Json(req): Json<CreateLeaveBalanceRequest>,
) -> Result<Json<crate::domain::leave_balance::entity::Model>, String> {
    service.create_leave_balance(req).await
        .map(Json)
        .map_err(|e| e.to_string())
}
