use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use chrono::{DateTime, FixedOffset};
use crate::domain::attendance_log::entity::Model as AttendanceLogModel;
use validator::Validate;
use serde_json::Value;

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct AttendanceLogResponse {
    pub id: i64,
    pub employee_id: Uuid,
    pub device_id: String,
    pub punch_time: DateTime<FixedOffset>,
    pub punch_type: String,
    pub raw_payload: Value,
}

#[derive(Deserialize, ToSchema, Validate, Clone, Debug)]
pub struct CreateAttendanceLogRequest {
    pub employee_id: Uuid,
    pub device_id: String,
    pub punch_time: DateTime<FixedOffset>,
    pub punch_type: String,
    pub raw_payload: Value,
}

impl From<AttendanceLogModel> for AttendanceLogResponse {
    fn from(model: AttendanceLogModel) -> Self {
        Self {
            id: model.id,
            employee_id: model.employee_id,
            device_id: model.device_id,
            punch_time: model.punch_time,
            punch_type: model.punch_type,
            raw_payload: model.raw_payload,
        }
    }
}
