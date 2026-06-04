use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Json, Router, Extension,
};
use std::sync::Arc;
use crate::application::attendance_log::service::AttendanceLogService;
use crate::application::attendance_log::dto::CreateAttendanceLogRequest;
use crate::domain::shared::dtos::PaginationQuery;
use crate::domain::shared::context::TenantContext;

pub fn router(service: Arc<AttendanceLogService>) -> Router {
    Router::new()
        .route("/", get(get_all_logs).post(create_log))
        .with_state(service)
}

async fn get_all_logs(
    State(service): State<Arc<AttendanceLogService>>,
    Extension(ctx): Extension<TenantContext>,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<crate::domain::shared::dtos::PaginatedResponse<crate::domain::attendance_log::entity::Model>>, String> {
    service.get_all_logs(&ctx, query).await
        .map(Json)
        .map_err(|e| e.to_string())
}

async fn create_log(
    State(service): State<Arc<AttendanceLogService>>,
    Extension(ctx): Extension<TenantContext>,
    Json(req): Json<CreateAttendanceLogRequest>,
) -> Result<Json<crate::domain::attendance_log::entity::Model>, String> {
    service.create_log(&ctx, req).await
        .map(Json)
        .map_err(|e| e.to_string())
}
