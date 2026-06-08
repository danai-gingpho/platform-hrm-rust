use axum::{
    extract::{Path, Query, State},
    routing::{get},
    Json, Router,
};
use std::sync::Arc;
use uuid::Uuid;
use crate::application::employment::service::EmploymentService;
use crate::application::employment::dto::CreateEmploymentRequest;
use crate::domain::shared::dtos::PaginationQuery;

pub fn router(service: Arc<EmploymentService>) -> Router {
    Router::new()
        .route("/", get(get_all_employments).post(create_employment))
        .route("/:id", get(get_employment_by_id))
        .with_state(service)
}

async fn get_all_employments(
    State(service): State<Arc<EmploymentService>>,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<crate::domain::shared::dtos::PaginatedResponse<crate::domain::employment::entity::Model>>, String> {
    service.get_all_employments(query).await
        .map(Json)
        .map_err(|e| e.to_string())
}

async fn get_employment_by_id(
    State(service): State<Arc<EmploymentService>>,
    Path(id): Path<Uuid>,
) -> Result<Json<crate::domain::employment::entity::Model>, String> {
    service.get_employment_by_id(id).await
        .map(Json)
        .map_err(|e| e.to_string())
}

async fn create_employment(
    State(service): State<Arc<EmploymentService>>,
    Json(req): Json<CreateEmploymentRequest>,
) -> Result<Json<crate::domain::employment::entity::Model>, String> {
    service.create_employment(req).await
        .map(Json)
        .map_err(|e| e.to_string())
}
