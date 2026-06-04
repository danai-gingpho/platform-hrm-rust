use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use chrono::NaiveDate;
use crate::domain::employee_allowance::entity::Model as EmployeeAllowanceModel;
use validator::Validate;
use rust_decimal::Decimal;

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct EmployeeAllowanceResponse {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub allowance_type_id: Uuid,
    pub amount: Decimal,
    pub effective_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
}

#[derive(Deserialize, ToSchema, Validate, Clone, Debug)]
pub struct CreateEmployeeAllowanceRequest {
    pub employee_id: Uuid,
    pub allowance_type_id: Uuid,
    pub amount: Decimal,
    pub effective_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
}

#[derive(Deserialize, ToSchema, Clone, Debug)]
pub struct UpdateEmployeeAllowanceRequest {
    pub amount: Option<Decimal>,
    pub effective_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}

impl From<EmployeeAllowanceModel> for EmployeeAllowanceResponse {
    fn from(model: EmployeeAllowanceModel) -> Self {
        Self {
            id: model.id,
            employee_id: model.employee_id,
            allowance_type_id: model.allowance_type_id,
            amount: model.amount,
            effective_date: model.effective_date,
            end_date: model.end_date,
        }
    }
}
