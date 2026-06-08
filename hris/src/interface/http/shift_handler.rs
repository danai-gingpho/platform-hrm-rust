use axum::{
    extract::{Path, Query, State},
    routing::get,
    Json, Router,
};
use std::sync::Arc;
use uuid::Uuid;
use crate::application::shift::service::ShiftService;
use crate::application::shift::dto::{CreateShiftRequest, UpdateShiftRequest};
use crate::domain::shared::dtos::PaginationQuery;

pub fn router(service: Arc<ShiftService>) -> Router {
    Router::new()
        .route("/", get(get_all_shifts).post(create_shift))
        .route("/:id", get(get_shift_by_id).put(update_shift).delete(delete_shift))
        .with_state(service)
} 

async fn get_all_shifts(
    State(service): State<Arc<ShiftService>>,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<crate::domain::shared::dtos::PaginatedResponse<crate::domain::shift::entity::Model>>, String> {
    service.get_all_shifts(query).await
        .map(Json)
        .map_err(|e| e.to_string())
}

async fn get_shift_by_id(
    State(service): State<Arc<ShiftService>>,
    Path(id): Path<Uuid>,
) -> Result<Json<crate::domain::shift::entity::Model>, String> {
    service.get_shift_by_id(id).await
        .map(Json)
        .map_err(|e| e.to_string())
}  

async fn create_shift(
    State(service): State<Arc<ShiftService>>,
    Json(req): Json<CreateShiftRequest>,
) -> Result<Json<crate::domain::shift::entity::Model>, String> {
    service.create_shift(req).await
        .map(Json)
        .map_err(|e| e.to_string())
}

async fn update_shift(
    State(service): State<Arc<ShiftService>>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateShiftRequest>,
) -> Result<Json<crate::domain::shift::entity::Model>, String> {
    service.update_shift(id, req).await
        .map(Json)
        .map_err(|e| e.to_string())
}

async fn delete_shift(
    State(service): State<Arc<ShiftService>>,
    Path(id): Path<Uuid>,
) -> Result<Json<()>, String> {
    service.delete_shift(id).await
        .map(|_| Json(()))
        .map_err(|e| e.to_string())
}