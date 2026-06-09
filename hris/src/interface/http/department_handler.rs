use crate::application::department::dto::{CreateDepartmentRequest, UpdateDepartmentRequest};
use crate::application::department::service::DepartmentService;
use crate::domain::shared::dtos::PaginationQuery;
use axum::{
    extract::{Path, Query, State},
    routing::get,
    Json, Router,
};
use std::sync::Arc;
use uuid::Uuid;

pub fn router(service: Arc<DepartmentService>) -> Router {
    Router::new()
        .route("/", get(get_all_departments).post(create_department))
        .route(
            "/:id",
            get(get_department_by_id)
                .put(update_department)
                .delete(delete_department),
        )
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

async fn update_department(
    State(service): State<Arc<DepartmentService>>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateDepartmentRequest>,
) -> Result<Json<crate::domain::department::entity::Model>, String> {
    service.update_department(id, req).await
        .map(Json)
        .map_err(|e| e.to_string())
}

async fn delete_department(
    State(service): State<Arc<DepartmentService>>,
    Path(id): Path<Uuid>,
) -> Result<Json<()>, String> {
    service.delete_department(id).await
        .map(|_| Json(()))
        .map_err(|e| e.to_string())
}
