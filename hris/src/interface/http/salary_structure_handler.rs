use axum::{
    extract::{Path, Query, State},
    routing::get,
    Json, Router,
};
use std::sync::Arc;
use uuid::Uuid;
use crate::application::salary_structure::service::SalaryStructureService;
use crate::application::salary_structure::dto::CreateSalaryStructureRequest;
use crate::domain::shared::dtos::PaginationQuery;

pub fn router(service: Arc<SalaryStructureService>) -> Router {
    Router::new()
        .route("/", get(get_all_salary_structures).post(create_salary_structure))
        .route("/:id", get(get_salary_structure_by_id))
        .with_state(service)
}

async fn get_all_salary_structures(
    State(service): State<Arc<SalaryStructureService>>,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<crate::domain::shared::dtos::PaginatedResponse<crate::domain::salary_structure::entity::Model>>, String> {
    service.get_all_salary_structures(query).await
        .map(Json)
        .map_err(|e| e.to_string())
}

async fn get_salary_structure_by_id(
    State(service): State<Arc<SalaryStructureService>>,
    Path(id): Path<Uuid>,
) -> Result<Json<crate::domain::salary_structure::entity::Model>, String> {
    service.get_salary_structure_by_id(id).await
        .map(Json)
        .map_err(|e| e.to_string())
}

async fn create_salary_structure(
    State(service): State<Arc<SalaryStructureService>>,
    Json(req): Json<CreateSalaryStructureRequest>,
) -> Result<Json<crate::domain::salary_structure::entity::Model>, String> {
    service.create_salary_structure(req).await
        .map(Json)
        .map_err(|e| e.to_string())
}
