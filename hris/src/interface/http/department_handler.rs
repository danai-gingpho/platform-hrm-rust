use axum::{
    extract::{Path, Query, State},
    routing::get,
    Json, Router,
};
use std::sync::Arc;
use uuid::Uuid;
use crate::application::department::service::DepartmentService;
use crate::application::department::dto::CreateDepartmentRequest;
use crate::domain::shared::dtos::PaginationQuery;

pub fn router(service: Arc<DepartmentService>) -> Router {
    Router::new()
        .route("/", get(get_all_departments).post(create_department))
        .route("/:id", get(get_department_by_id))
        .with_state(service)
}

async fn get_all_departments(
    State(service): State<Arc<DepartmentService>>,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<crate::domain::shared::dtos::PaginatedResponse<crate::domain::department::entity::Model>>, String> {
    service.get_all_departments(query).await
        .map(Json)
        .map_err(|e| e.to_string())
}

async fn get_department_by_id(
    State(service): State<Arc<DepartmentService>>,
    Path(id): Path<Uuid>,
) -> Result<Json<crate::domain::department::entity::Model>, String> {
    service.get_department_by_id(id).await
        .map(Json)
        .map_err(|e| e.to_string())
}

async fn create_department(
    State(service): State<Arc<DepartmentService>>,
    Json(req): Json<CreateDepartmentRequest>,
) -> Result<Json<crate::domain::department::entity::Model>, String> {
    service.create_department(req).await
        .map(Json)
        .map_err(|e| e.to_string())
}
