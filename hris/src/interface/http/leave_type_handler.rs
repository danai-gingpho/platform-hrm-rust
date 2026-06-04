use axum::{
    extract::{Path, Query, State},
    routing::{get, post, put, delete},
    Json, Router,
};
use std::sync::Arc;
use uuid::Uuid;
use crate::application::leave_type::service::LeaveTypeService;
use crate::application::leave_type::dto::{CreateLeaveTypeRequest, UpdateLeaveTypeRequest};
use crate::domain::shared::dtos::PaginationQuery;

pub fn router(service: Arc<LeaveTypeService>) -> Router {
    Router::new()
        .route("/", get(get_all_leave_types).post(create_leave_type))
        .route("/:id", get(get_leave_type_by_id).put(update_leave_type).delete(delete_leave_type))
        .with_state(service)
}

async fn get_all_leave_types(
    State(service): State<Arc<LeaveTypeService>>,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<crate::domain::shared::dtos::PaginatedResponse<crate::domain::leave_type::entity::Model>>, String> {
    service.get_all_leave_types(query).await
        .map(Json)
        .map_err(|e| e.to_string())
}

async fn get_leave_type_by_id(
    State(service): State<Arc<LeaveTypeService>>,
    Path(id): Path<Uuid>,
) -> Result<Json<crate::domain::leave_type::entity::Model>, String> {
    service.get_leave_type_by_id(id).await
        .map(Json)
        .map_err(|e| e.to_string())
}

async fn create_leave_type(
    State(service): State<Arc<LeaveTypeService>>,
    Json(req): Json<CreateLeaveTypeRequest>,
) -> Result<Json<crate::domain::leave_type::entity::Model>, String> {
    service.create_leave_type(req).await
        .map(Json)
        .map_err(|e| e.to_string())
}

async fn update_leave_type(
    State(service): State<Arc<LeaveTypeService>>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateLeaveTypeRequest>,
) -> Result<Json<crate::domain::leave_type::entity::Model>, String> {
    service.update_leave_type(id, req).await
        .map(Json)
        .map_err(|e| e.to_string())
}

async fn delete_leave_type(
    State(service): State<Arc<LeaveTypeService>>,
    Path(id): Path<Uuid>,
) -> Result<Json<()>, String> {
    service.delete_leave_type(id).await
        .map(|_| Json(()))
        .map_err(|e| e.to_string())
}
