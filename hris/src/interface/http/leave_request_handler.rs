use axum::{
    extract::{Path, Query, State},
    routing::{get},
    Json, Router,
};
use std::sync::Arc;
use uuid::Uuid;
use crate::application::leave_request::service::LeaveRequestService;
use crate::application::leave_request::dto::CreateLeaveRequestRequest;
use crate::domain::shared::dtos::PaginationQuery;

pub fn router(service: Arc<LeaveRequestService>) -> Router {
    Router::new()
        .route("/", get(get_all_leave_requests).post(create_leave_request))
        .route("/:id", get(get_leave_request_by_id))
        .with_state(service)
}

async fn get_all_leave_requests(
    State(service): State<Arc<LeaveRequestService>>,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<crate::domain::shared::dtos::PaginatedResponse<crate::domain::leave_request::entity::Model>>, String> {
    service.get_all_leave_requests(query).await
        .map(Json)
        .map_err(|e| e.to_string())
}

async fn get_leave_request_by_id(
    State(service): State<Arc<LeaveRequestService>>,
    Path(id): Path<Uuid>,
) -> Result<Json<crate::domain::leave_request::entity::Model>, String> {
    service.get_leave_request_by_id(id).await
        .map(Json)
        .map_err(|e| e.to_string())
}

async fn create_leave_request(
    State(service): State<Arc<LeaveRequestService>>,
    Json(req): Json<CreateLeaveRequestRequest>,
) -> Result<Json<crate::domain::leave_request::entity::Model>, String> {
    service.create_leave_request(req).await
        .map(Json)
        .map_err(|e| e.to_string())
}
