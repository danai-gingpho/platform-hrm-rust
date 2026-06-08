use axum::{
    extract::{State, Json},
    http::StatusCode,
    response::IntoResponse,
};
use crate::application::iam::service::PlatformService;
use crate::proto::CreateCompanyRequest;
use std::sync::Arc;

pub async fn create_company(
    State(service): State<Arc<PlatformService>>,
    Json(payload): Json<CreateCompanyRequest>,
) -> impl IntoResponse {
    match service.create_company(payload).await {
        Ok(company) => (StatusCode::CREATED, Json(company)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
