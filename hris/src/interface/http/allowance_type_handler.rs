use axum::{
    extract::{Path, Query, State},
    routing::{get, post, put, delete},
    Json, Router,
};
use std::sync::Arc;
use uuid::Uuid;
use crate::application::allowance_type::service::AllowanceTypeService;
use crate::application::allowance_type::dto::{CreateAllowanceTypeRequest, UpdateAllowanceTypeRequest};
use crate::domain::shared::dtos::PaginationQuery;

pub fn router(service: Arc<AllowanceTypeService>) -> Router {
    Router::new()
        .route("/", get(get_all_allowance_types).post(create_allowance_type))
        .route("/:id", get(get_allowance_type_by_id).put(update_allowance_type).delete(delete_allowance_type))
        .with_state(service)
}

async fn get_all_allowance_types(
    State(service): State<Arc<AllowanceTypeService>>,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<crate::domain::shared::dtos::PaginatedResponse<crate::domain::allowance_type::entity::Model>>, String> {
    service.get_all_allowance_types(query).await
        .map(Json)
        .map_err(|e| e.to_string())
}

async fn get_allowance_type_by_id(
    State(service): State<Arc<AllowanceTypeService>>,
    Path(id): Path<Uuid>,
) -> Result<Json<crate::domain::allowance_type::entity::Model>, String> {
    service.get_allowance_type_by_id(id).await
        .map(Json)
        .map_err(|e| e.to_string())
}

async fn create_allowance_type(
    State(service): State<Arc<AllowanceTypeService>>,
    Json(req): Json<CreateAllowanceTypeRequest>,
) -> Result<Json<crate::domain::allowance_type::entity::Model>, String> {
    service.create_allowance_type(req).await
        .map(Json)
        .map_err(|e| e.to_string())
}

async fn update_allowance_type(
    State(service): State<Arc<AllowanceTypeService>>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateAllowanceTypeRequest>,
) -> Result<Json<crate::domain::allowance_type::entity::Model>, String> {
    service.update_allowance_type(id, req).await
        .map(Json)
        .map_err(|e| e.to_string())
}

async fn delete_allowance_type(
    State(service): State<Arc<AllowanceTypeService>>,
    Path(id): Path<Uuid>,
) -> Result<Json<()>, String> {
    service.delete_allowance_type(id).await
        .map(|_| Json(()))
        .map_err(|e| e.to_string())
}