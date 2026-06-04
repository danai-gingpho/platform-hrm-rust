use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Json, Router, Extension,
};
use std::sync::Arc;
use uuid::Uuid;
use crate::application::company::service::CompanyService;
use crate::application::company::dto::{CreateCompanyRequest, CompanyResponse};
use crate::domain::shared::dtos::PaginationQuery;
use crate::domain::shared::context::TenantContext;

pub fn router(service: Arc<CompanyService>) -> Router {
    Router::new()
        .route("/", get(get_all_companies).post(create_company))
        .route("/:id", get(get_company_by_id))
        .with_state(service)
}

async fn get_all_companies(
    State(service): State<Arc<CompanyService>>,
    Extension(ctx): Extension<TenantContext>,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<crate::domain::shared::dtos::PaginatedResponse<crate::domain::company::entity::Model>>, String> {
    service.get_all_companies(&ctx, query).await
        .map(Json)
        .map_err(|e| e.to_string())
}

async fn get_company_by_id(
    State(service): State<Arc<CompanyService>>,
    Extension(ctx): Extension<TenantContext>,
    Path(id): Path<Uuid>,
) -> Result<Json<crate::domain::company::entity::Model>, String> {
    service.get_company_by_id(&ctx, id).await
        .map(Json)
        .map_err(|e| e.to_string())
}

async fn create_company(
    State(service): State<Arc<CompanyService>>,
    Extension(ctx): Extension<TenantContext>,
    Json(req): Json<CreateCompanyRequest>,
) -> Result<Json<crate::domain::company::entity::Model>, String> {
    service.create_company(&ctx, req).await
        .map(Json)
        .map_err(|e| e.to_string())
}
