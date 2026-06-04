use serde::{Deserialize, Serialize};
use validator::Validate;
use rust_decimal::Decimal;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateLeaveTypeRequest {
    #[validate(length(min = 1))]
    pub code: Option<String>,
    #[validate(length(min = 1))]
    pub name: String,
    pub paid: bool,
    pub max_days_per_year: Decimal,
    pub gender_restriction: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateLeaveTypeRequest {
    pub name: Option<String>,
    pub paid: Option<bool>,
    pub max_days_per_year: Option<Decimal>,
    pub gender_restriction: Option<String>,
}
