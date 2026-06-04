use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Json, Router, Extension,
};
use std::sync::Arc;
use uuid::Uuid;
use crate::application::branch::service::BranchService;
use crate::application::branch::dto::CreateBranchRequest;
use crate::domain::shared::dtos::PaginationQuery;
use crate::domain::shared::context::TenantContext;

pub fn router(service: Arc<BranchService>) -> Router {
    Router::new()
        .route("/", get(get_all_branches).post(create_branch))
        .route("/:id", get(get_branch_by_id))
        .with_state(service)
}

async fn get_all_branches(
    State(service): State<Arc<BranchService>>,
    Extension(ctx): Extension<TenantContext>,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<crate::domain::shared::dtos::PaginatedResponse<crate::domain::branch::entity::Model>>, String> {
    service.get_all_branches(&ctx, query).await
        .map(Json)
        .map_err(|e| e.to_string())
}

async fn get_branch_by_id(
    State(service): State<Arc<BranchService>>,
    Extension(ctx): Extension<TenantContext>,
    Path(id): Path<Uuid>,
) -> Result<Json<crate::domain::branch::entity::Model>, String> {
    service.get_branch_by_id(&ctx, id).await
        .map(Json)
        .map_err(|e| e.to_string())
}

async fn create_branch(
    State(service): State<Arc<BranchService>>,
    Extension(ctx): Extension<TenantContext>,
    Json(req): Json<CreateBranchRequest>,
) -> Result<Json<crate::domain::branch::entity::Model>, String> {
    service.create_branch(&ctx, req).await
        .map(Json)
        .map_err(|e| e.to_string())
}
