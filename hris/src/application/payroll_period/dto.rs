use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use utoipa::ToSchema;
use chrono::NaiveDate;

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreatePayrollPeriodRequest {
    pub legal_entity_id: Uuid,
    #[validate(length(min = 1))]
    pub period_code: Option<String>,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub payment_date: NaiveDate,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdatePayrollPeriodRequest {
    pub period_code: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub payment_date: Option<NaiveDate>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PayrollPeriodResponse {
    pub id: Uuid,
    pub legal_entity_id: Uuid,
    pub period_code: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub payment_date: NaiveDate,
    pub status: String,
}
