use axum::{
    extract::{Path, Query, State},
    routing::get,
    Json, Router,
};
use std::sync::Arc;
use uuid::Uuid;
use crate::application::branch::service::BranchService;
use crate::application::branch::dto::{CreateBranchRequest, UpdateBranchRequest};
use crate::domain::shared::dtos::PaginationQuery;

pub fn router(service: Arc<BranchService>) -> Router {
    Router::new()
        .route("/", get(get_all_branches).post(create_branch))
        .route("/:id", get(get_branch_by_id).put(update_branch).delete(delete_branch))
        .with_state(service)
}

async fn get_all_branches(
    State(service): State<Arc<BranchService>>,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<crate::domain::shared::dtos::PaginatedResponse<crate::domain::branch::entity::Model>>, String> {
    service.get_all_branches(query).await
        .map(Json)
        .map_err(|e| e.to_string())
}

async fn get_branch_by_id(
    State(service): State<Arc<BranchService>>,
    Path(id): Path<Uuid>,
) -> Result<Json<crate::domain::branch::entity::Model>, String> {
    service.get_branch_by_id(id).await
        .map(Json)
        .map_err(|e| e.to_string())
}

async fn create_branch(
    State(service): State<Arc<BranchService>>,
    Json(req): Json<CreateBranchRequest>,
) -> Result<Json<crate::domain::branch::entity::Model>, String> {
    service.create_branch(req).await
        .map(Json)
        .map_err(|e| e.to_string())
}

async fn update_branch(
    State(service): State<Arc<BranchService>>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateBranchRequest>,
) -> Result<Json<crate::domain::branch::entity::Model>, String> {
    service.update_branch(id, req).await
        .map(Json)
        .map_err(|e| e.to_string())
}

async fn delete_branch(
    State(service): State<Arc<BranchService>>,
    Path(id): Path<Uuid>,
) -> Result<Json<()>, String> {
    service.delete_branch(id).await
        .map(|_| Json(()))
        .map_err(|e| e.to_string())
}
