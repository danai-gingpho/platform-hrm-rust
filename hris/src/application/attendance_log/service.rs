use std::sync::Arc;
use validator::Validate;
use crate::domain::attendance_log::entity::Model as AttendanceLog;
use crate::domain::attendance_log::repository::AttendanceLogRepository;
use crate::domain::errors::{AppError, AppResult};
use crate::domain::shared::dtos::{PaginationQuery, PaginatedResponse};
use crate::application::attendance_log::dto::{CreateAttendanceLogRequest};

pub struct AttendanceLogService {
    repository: Arc<dyn AttendanceLogRepository>,
}

impl AttendanceLogService {
    pub fn new(repository: Arc<dyn AttendanceLogRepository>) -> Self {
        Self { repository }
    }

    pub async fn get_all_logs(&self, query: PaginationQuery) -> AppResult<PaginatedResponse<AttendanceLog>> {
        self.repository.find_all(query).await
    }

    pub async fn get_log_by_id(&self, id: i64) -> AppResult<AttendanceLog> {
        self.repository.find_by_id(id).await
    }

    pub async fn create_log(&self, req: CreateAttendanceLogRequest) -> AppResult<AttendanceLog> {
        req.validate().map_err(|e| AppError::Validation(e.to_string()))?;
        
        let log = AttendanceLog {
            id: 0, // SeaORM handles auto-increment for i64
            employee_id: req.employee_id,
            device_id: req.device_id,
            punch_time: req.punch_time,
            punch_type: req.punch_type,
            raw_payload: req.raw_payload,
        };
        
        self.repository.create(log).await
    }

    pub async fn delete_log(&self, id: i64) -> AppResult<()> {
        self.repository.delete(id).await
    }
}
