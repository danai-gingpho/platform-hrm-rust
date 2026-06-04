use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Json, Router,
};
use std::sync::Arc;
use uuid::Uuid;
use crate::application::social_security_rate::service::SocialSecurityRateService;
use crate::application::social_security_rate::dto::CreateSocialSecurityRateRequest;
use crate::domain::shared::dtos::PaginationQuery;

pub fn router(service: Arc<SocialSecurityRateService>) -> Router {
    Router::new()
        .route("/", get(get_all_social_security_rates).post(create_social_security_rate))
        .route("/:id", get(get_social_security_rate_by_id))
        .with_state(service)
}

async fn get_all_social_security_rates(
    State(service): State<Arc<SocialSecurityRateService>>,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<crate::domain::shared::dtos::PaginatedResponse<crate::domain::social_security_rate::entity::Model>>, String> {
    service.get_all_rates(query).await
        .map(Json)
        .map_err(|e| e.to_string())
}

async fn get_social_security_rate_by_id(
    State(service): State<Arc<SocialSecurityRateService>>,
    Path(id): Path<Uuid>,
) -> Result<Json<crate::domain::social_security_rate::entity::Model>, String> {
    service.get_rate_by_id(id).await
        .map(Json)
        .map_err(|e| e.to_string())
}

async fn create_social_security_rate(
    State(service): State<Arc<SocialSecurityRateService>>,
    Json(req): Json<CreateSocialSecurityRateRequest>,
) -> Result<Json<crate::domain::social_security_rate::entity::Model>, String> {
    service.create_rate(req).await
        .map(Json)
        .map_err(|e| e.to_string())
}
