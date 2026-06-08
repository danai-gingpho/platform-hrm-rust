use axum::{
    extract::{Path, Query, State},
    routing::get,
    Json, Router,
};
use std::sync::Arc;
use crate::application::attendance_log::service::AttendanceLogService;
use crate::application::attendance_log::dto::CreateAttendanceLogRequest;
use crate::domain::shared::dtos::PaginationQuery;

pub fn router(service: Arc<AttendanceLogService>) -> Router {
    Router::new()
        .route("/", get(get_all_logs).post(create_log))
        .route("/:id", get(get_log_by_id))
        .with_state(service)
}

async fn get_all_logs(
    State(service): State<Arc<AttendanceLogService>>,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<crate::domain::shared::dtos::PaginatedResponse<crate::domain::attendance_log::entity::Model>>, String> {
    service.get_all_logs(query).await
        .map(Json)
        .map_err(|e| e.to_string())
}

async fn get_log_by_id(
    State(service): State<Arc<AttendanceLogService>>,
    Path(id): Path<i64>,
) -> Result<Json<crate::domain::attendance_log::entity::Model>, String> {
    service.get_log_by_id(id).await
        .map(Json)
        .map_err(|e| e.to_string())
}

async fn create_log(
    State(service): State<Arc<AttendanceLogService>>,
    Json(req): Json<CreateAttendanceLogRequest>,
) -> Result<Json<crate::domain::attendance_log::entity::Model>, String> {
    service.create_log(req).await
        .map(Json)
        .map_err(|e| e.to_string())
}
