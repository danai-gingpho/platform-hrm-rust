use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use crate::domain::leave_balance::entity::Model as LeaveBalanceModel;
use validator::Validate;
use rust_decimal::Decimal;

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct LeaveBalanceResponse {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub leave_type_id: Uuid,
    pub year: i32,
    pub entitled_days: Decimal,
    pub used_days: Decimal,
    pub remaining_days: Decimal,
}

#[derive(Deserialize, ToSchema, Validate, Clone, Debug)]
pub struct CreateLeaveBalanceRequest {
    pub employee_id: Uuid,
    pub leave_type_id: Uuid,
    pub year: i32,
    pub entitled_days: Decimal,
    pub used_days: Decimal,
}

#[derive(Deserialize, ToSchema, Clone, Debug)]
pub struct UpdateLeaveBalanceRequest {
    pub entitled_days: Option<Decimal>,
    pub used_days: Option<Decimal>,
}

impl From<LeaveBalanceModel> for LeaveBalanceResponse {
    fn from(model: LeaveBalanceModel) -> Self {
        Self {
            id: model.id,
            employee_id: model.employee_id,
            leave_type_id: model.leave_type_id,
            year: model.year,
            entitled_days: model.entitled_days,
            used_days: model.used_days,
            remaining_days: model.remaining_days,
        }
    }
}
