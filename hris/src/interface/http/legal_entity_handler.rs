use axum::{
    extract::{Path, Query, State},
    routing::get,
    Json, Router,
};
use std::sync::Arc;
use uuid::Uuid;
use crate::application::legal_entity::service::LegalEntityService;
use crate::application::legal_entity::dto::{CreateLegalEntityRequest, UpdateLegalEntityRequest};
use crate::domain::shared::dtos::PaginationQuery;

pub fn router(service: Arc<LegalEntityService>) -> Router {
    Router::new()
        .route("/", get(get_all_legal_entities).post(create_legal_entity))
        .route("/:id", get(get_legal_entity_by_id).put(update_legal_entity).delete(delete_legal_entity))
        .with_state(service)
}

async fn get_all_legal_entities(
    State(service): State<Arc<LegalEntityService>>,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<crate::domain::shared::dtos::PaginatedResponse<crate::domain::legal_entity::entity::Model>>, String> {
    service.get_all_legal_entities(query).await
        .map(Json)
        .map_err(|e| e.to_string())
}

async fn get_legal_entity_by_id(
    State(service): State<Arc<LegalEntityService>>,
    Path(id): Path<Uuid>,
) -> Result<Json<crate::domain::legal_entity::entity::Model>, String> {
    service.get_legal_entity_by_id(id).await
        .map(Json)
        .map_err(|e| e.to_string())
}

async fn create_legal_entity(
    State(service): State<Arc<LegalEntityService>>,
    Json(req): Json<CreateLegalEntityRequest>,
) -> Result<Json<crate::domain::legal_entity::entity::Model>, String> {
    service.create_legal_entity(req).await
        .map(Json)
        .map_err(|e| e.to_string())
}

async fn update_legal_entity(
    State(service): State<Arc<LegalEntityService>>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateLegalEntityRequest>,
) -> Result<Json<crate::domain::legal_entity::entity::Model>, String> {
    service.update_legal_entity(id, req).await
        .map(Json)
        .map_err(|e| e.to_string())
}

async fn delete_legal_entity(
    State(service): State<Arc<LegalEntityService>>,
    Path(id): Path<Uuid>,
) -> Result<Json<()>, String> {
    service.delete_legal_entity(id).await
        .map(|_| Json(()))
        .map_err(|e| e.to_string())
}
