use axum::{
    extract::{Path, Query, State},
    routing::get,
    Json, Router,
};
use std::sync::Arc;
use uuid::Uuid;
use crate::application::payroll_item::service::PayrollItemService;
use crate::application::payroll_item::dto::CreatePayrollItemRequest;
use crate::domain::shared::dtos::PaginationQuery;

pub fn router(service: Arc<PayrollItemService>) -> Router {
    Router::new()
        .route("/", get(get_all_payroll_items).post(create_payroll_item))
        .route("/:id", get(get_payroll_item_by_id))
        .with_state(service)
}

async fn get_all_payroll_items(
    State(service): State<Arc<PayrollItemService>>,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<crate::domain::shared::dtos::PaginatedResponse<crate::domain::payroll_item::entity::Model>>, String> {
    service.get_all_payroll_items(query).await
        .map(Json)
        .map_err(|e| e.to_string())
}

async fn get_payroll_item_by_id(
    State(service): State<Arc<PayrollItemService>>,
    Path(id): Path<Uuid>,
) -> Result<Json<crate::domain::payroll_item::entity::Model>, String> {
    service.get_payroll_item_by_id(id).await
        .map(Json)
        .map_err(|e| e.to_string())
}

async fn create_payroll_item(
    State(service): State<Arc<PayrollItemService>>,
    Json(req): Json<CreatePayrollItemRequest>,
) -> Result<Json<crate::domain::payroll_item::entity::Model>, String> {
    service.create_payroll_item(req).await
        .map(Json)
        .map_err(|e| e.to_string())
}
