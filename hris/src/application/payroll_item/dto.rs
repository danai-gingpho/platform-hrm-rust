use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use utoipa::ToSchema;
use rust_decimal::Decimal;

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreatePayrollItemRequest {
    pub payroll_run_id: Uuid,
    #[validate(length(min = 1))]
    pub item_code: String,
    #[validate(length(min = 1))]
    pub item_name: String,
    pub item_category: String,
    pub quantity: Decimal,
    pub rate: Decimal,
    pub amount: Decimal,
    pub taxable: bool,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdatePayrollItemRequest {
    pub item_code: Option<String>,
    pub item_name: Option<String>,
    pub item_category: Option<String>,
    pub quantity: Option<Decimal>,
    pub rate: Option<Decimal>,
    pub amount: Option<Decimal>,
    pub taxable: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PayrollItemResponse {
    pub id: Uuid,
    pub payroll_run_id: Uuid,
    pub item_code: String,
    pub item_name: String,
    pub item_category: String,
    pub quantity: Decimal,
    pub rate: Decimal,
    pub amount: Decimal,
    pub taxable: bool,
}
