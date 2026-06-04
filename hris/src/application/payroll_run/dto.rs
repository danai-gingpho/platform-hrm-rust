use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use utoipa::ToSchema;
use rust_decimal::Decimal;
use chrono::DateTime;
use chrono::Utc;

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreatePayrollRunRequest {
    pub payroll_period_id: Uuid,
    pub employee_id: Uuid,
    pub gross_income: Decimal,
    pub total_deduction: Decimal,
    pub taxable_income: Decimal,
    pub tax_amount: Decimal,
    pub social_security: Decimal,
    pub net_income: Decimal,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdatePayrollRunRequest {
    pub gross_income: Option<Decimal>,
    pub total_deduction: Option<Decimal>,
    pub taxable_income: Option<Decimal>,
    pub tax_amount: Option<Decimal>,
    pub social_security: Option<Decimal>,
    pub net_income: Option<Decimal>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PayrollRunResponse {
    pub id: Uuid,
    pub payroll_period_id: Uuid,
    pub employee_id: Uuid,
    pub gross_income: Decimal,
    pub total_deduction: Decimal,
    pub taxable_income: Decimal,
    pub tax_amount: Decimal,
    pub social_security: Decimal,
    pub net_income: Decimal,
    pub status: String,
    pub calculated_at: DateTime<Utc>,
}
