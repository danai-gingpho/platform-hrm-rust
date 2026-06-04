use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Json, Router,
};
use std::sync::Arc;
use uuid::Uuid;
use crate::application::position::service::PositionService;
use crate::application::position::dto::CreatePositionRequest;
use crate::domain::shared::dtos::PaginationQuery;

pub fn router(service: Arc<PositionService>) -> Router {
    Router::new()
        .route("/", get(get_all_positions).post(create_position))
        .route("/:id", get(get_position_by_id))
        .with_state(service)
}

async fn get_all_positions(
    State(service): State<Arc<PositionService>>,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<crate::domain::shared::dtos::PaginatedResponse<crate::domain::position::entity::Model>>, String> {
    service.get_all_positions(query).await
        .map(Json)
        .map_err(|e| e.to_string())
}

async fn get_position_by_id(
    State(service): State<Arc<PositionService>>,
    Path(id): Path<Uuid>,
) -> Result<Json<crate::domain::position::entity::Model>, String> {
    service.get_position_by_id(id).await
        .map(Json)
        .map_err(|e| e.to_string())
}

async fn create_position(
    State(service): State<Arc<PositionService>>,
    Json(req): Json<CreatePositionRequest>,
) -> Result<Json<crate::domain::position::entity::Model>, String> {
    service.create_position(req).await
        .map(Json)
        .map_err(|e| e.to_string())
}
