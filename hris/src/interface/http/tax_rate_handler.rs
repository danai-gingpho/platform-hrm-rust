use axum::{
    extract::{Path, Query, State},
    routing::get,
    Json, Router,
};
use std::sync::Arc;
use uuid::Uuid;
use crate::application::tax_rate::service::TaxRateService;
use crate::application::tax_rate::dto::CreateTaxRateRequest;
use crate::domain::shared::dtos::PaginationQuery;

pub fn router(service: Arc<TaxRateService>) -> Router {
    Router::new()
        .route("/", get(get_all_tax_rates).post(create_tax_rate))
        .route("/:id", get(get_tax_rate_by_id))
        .with_state(service)
}

async fn get_all_tax_rates(
    State(service): State<Arc<TaxRateService>>,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<crate::domain::shared::dtos::PaginatedResponse<crate::domain::tax_rate::entity::Model>>, String> {
    service.get_all_rates(query).await
        .map(Json)
        .map_err(|e| e.to_string())
}

async fn get_tax_rate_by_id(
    State(service): State<Arc<TaxRateService>>,
    Path(id): Path<Uuid>,
) -> Result<Json<crate::domain::tax_rate::entity::Model>, String> {
    service.get_rate_by_id(id).await
        .map(Json)
        .map_err(|e| e.to_string())
}

async fn create_tax_rate(
    State(service): State<Arc<TaxRateService>>,
    Json(req): Json<CreateTaxRateRequest>,
) -> Result<Json<crate::domain::tax_rate::entity::Model>, String> {
    service.create_rate(req).await
        .map(Json)
        .map_err(|e| e.to_string())
}
