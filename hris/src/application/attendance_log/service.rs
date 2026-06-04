use std::sync::Arc;
use validator::Validate;
use crate::domain::attendance_log::entity::Model as AttendanceLog;
use crate::domain::attendance_log::repository::AttendanceLogRepository;
use crate::domain::errors::{AppError, AppResult};
use crate::domain::shared::dtos::{PaginationQuery, PaginatedResponse};
use crate::domain::shared::context::TenantContext;
use crate::application::attendance_log::dto::{CreateAttendanceLogRequest};
use uuid::Uuid;

pub struct AttendanceLogService {
    repository: Arc<dyn AttendanceLogRepository>,
}

impl AttendanceLogService {
    pub fn new(repository: Arc<dyn AttendanceLogRepository>) -> Self {
        Self { repository }
    }

    pub async fn get_all_logs(&self, ctx: &TenantContext, query: PaginationQuery) -> AppResult<PaginatedResponse<AttendanceLog>> {
        self.repository.find_all(ctx, query).await
    }

    pub async fn get_log_by_id(&self, ctx: &TenantContext, id: i64) -> AppResult<AttendanceLog> {
        self.repository.find_by_id(ctx, id).await
    }

    pub async fn create_log(&self, ctx: &TenantContext, req: CreateAttendanceLogRequest) -> AppResult<AttendanceLog> {
        req.validate().map_err(|e| AppError::Validation(e.to_string()))?;
        
        let log = AttendanceLog {
            id: 0, // SeaORM handles auto-increment for i64
            tenant_id: Uuid::nil(), // Will be set by repository
            employee_id: req.employee_id,
            device_id: req.device_id,
            punch_time: req.punch_time,
            punch_type: req.punch_type,
            raw_payload: req.raw_payload,
        };
        
        self.repository.create(ctx, log).await
    }

    pub async fn delete_log(&self, ctx: &TenantContext, id: i64) -> AppResult<()> {
        self.repository.delete(ctx, id).await
    }
}
