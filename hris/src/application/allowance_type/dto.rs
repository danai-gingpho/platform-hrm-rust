use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateAllowanceTypeRequest {
    #[validate(length(min = 1))]
    pub code: Option<String>,
    #[validate(length(min = 1))]
    pub name: String,
    pub taxable: bool,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateAllowanceTypeRequest {
    pub name: Option<String>,
    pub taxable: Option<bool>,
}
