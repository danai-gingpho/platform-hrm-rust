use axum::{
    extract::{Path, Query, State},
    routing::{get},
    Json, Router,
};
use std::sync::Arc;
use uuid::Uuid;
use crate::application::employment_contract::service::EmploymentContractService;
use crate::application::employment_contract::dto::CreateEmploymentContractRequest;
use crate::domain::shared::dtos::PaginationQuery;

pub fn router(service: Arc<EmploymentContractService>) -> Router {
    Router::new()
        .route("/", get(get_all_contracts).post(create_contract))
        .route("/:id", get(get_contract_by_id))
        .with_state(service)
}

async fn get_all_contracts(
    State(service): State<Arc<EmploymentContractService>>,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<crate::domain::shared::dtos::PaginatedResponse<crate::domain::employment_contract::entity::Model>>, String> {
    service.get_all_contracts(query).await
        .map(Json)
        .map_err(|e| e.to_string())
}

async fn get_contract_by_id(
    State(service): State<Arc<EmploymentContractService>>,
    Path(id): Path<Uuid>,
) -> Result<Json<crate::domain::employment_contract::entity::Model>, String> {
    service.get_contract_by_id(id).await
        .map(Json)
        .map_err(|e| e.to_string())
}

async fn create_contract(
    State(service): State<Arc<EmploymentContractService>>,
    Json(req): Json<CreateEmploymentContractRequest>,
) -> Result<Json<crate::domain::employment_contract::entity::Model>, String> {
    service.create_contract(req).await
        .map(Json)
        .map_err(|e| e.to_string())
}
