use crate::application::salary_structure_item::dto::CreateSalaryStructureItemRequest;
use crate::application::salary_structure_item::service::SalaryStructureItemService;
use crate::domain::shared::dtos::PaginationQuery;
use axum::{
    extract::{Path, Query, State},
    routing::get,
    Json, Router,
};
use std::sync::Arc;
use uuid::Uuid;

pub fn router(service: Arc<SalaryStructureItemService>) -> Router {
    Router::new()
        .route(
            "/",
            get(get_all_salary_structure_items).post(create_salary_structure_item),
        )
        .route("/:id", get(get_salary_structure_item_by_id))
        .with_state(service)
}

async fn get_all_salary_structure_items(
    State(service): State<Arc<SalaryStructureItemService>>,
    Query(query): Query<PaginationQuery>,
) -> Result<
    Json<
        crate::domain::shared::dtos::PaginatedResponse<
            crate::domain::salary_structure_item::entity::Model,
        >,
    >,
    String,
> {
    service
        .get_all_items(query)
        .await
        .map(Json)
        .map_err(|e| e.to_string())
}

async fn get_salary_structure_item_by_id(
    State(service): State<Arc<SalaryStructureItemService>>,
    Path(id): Path<Uuid>,
) -> Result<Json<crate::domain::salary_structure_item::entity::Model>, String> {
    service
        .get_item_by_id(id)
        .await
        .map(Json)
        .map_err(|e| e.to_string())
}

async fn create_salary_structure_item(
    State(service): State<Arc<SalaryStructureItemService>>,
    Json(req): Json<CreateSalaryStructureItemRequest>,
) -> Result<Json<crate::domain::salary_structure_item::entity::Model>, String> {
    service
        .create_item(req)
        .await
        .map(Json)
        .map_err(|e| e.to_string())
}
