use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;
use rust_decimal::Decimal;
use chrono::NaiveDate;
use crate::domain::social_security_rate::entity::Model as SocialSecurityRate;

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct SocialSecurityRateResponse {
    pub id: Uuid,
    pub effective_date: NaiveDate,
    pub employee_percent: Decimal,
    pub employer_percent: Decimal,
    pub max_salary: Decimal,
}

#[derive(Deserialize, ToSchema, Validate, Clone, Debug)]
pub struct CreateSocialSecurityRateRequest {
    pub effective_date: NaiveDate,
    pub employee_percent: Decimal,
    pub employer_percent: Decimal,
    pub max_salary: Decimal,
}

#[derive(Deserialize, ToSchema, Clone, Debug)]
pub struct UpdateSocialSecurityRateRequest {
    pub effective_date: Option<NaiveDate>,
    pub employee_percent: Option<Decimal>,
    pub employer_percent: Option<Decimal>,
    pub max_salary: Option<Decimal>,
}

impl From<SocialSecurityRate> for SocialSecurityRateResponse {
    fn from(m: SocialSecurityRate) -> Self {
        Self {
            id: m.id, effective_date: m.effective_date, employee_percent: m.employee_percent,
            employer_percent: m.employer_percent, max_salary: m.max_salary
        }
    }
}
