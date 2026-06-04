use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use chrono::{NaiveDate, DateTime, FixedOffset};
use crate::domain::leave_request::entity::Model as LeaveRequestModel;
use validator::Validate;
use rust_decimal::Decimal;

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct LeaveRequestResponse {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub leave_type_id: Uuid,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub total_days: Decimal,
    pub reason: String,
    pub status: String,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<DateTime<FixedOffset>>,
}

#[derive(Deserialize, ToSchema, Validate, Clone, Debug)]
pub struct CreateLeaveRequestRequest {
    pub employee_id: Uuid,
    pub leave_type_id: Uuid,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub total_days: Decimal,
    pub reason: String,
}

#[derive(Deserialize, ToSchema, Clone, Debug)]
pub struct UpdateLeaveRequestRequest {
    pub status: Option<String>,
    pub approved_by: Option<Uuid>,
}

impl From<LeaveRequestModel> for LeaveRequestResponse {
    fn from(model: LeaveRequestModel) -> Self {
        Self {
            id: model.id,
            employee_id: model.employee_id,
            leave_type_id: model.leave_type_id,
            start_date: model.start_date,
            end_date: model.end_date,
            total_days: model.total_days,
            reason: model.reason,
            status: model.status,
            approved_by: model.approved_by,
            approved_at: model.approved_at,
        }
    }
}
