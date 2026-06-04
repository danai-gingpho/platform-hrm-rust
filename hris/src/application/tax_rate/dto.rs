use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;
use rust_decimal::Decimal;
use crate::domain::tax_rate::entity::Model as TaxRate;

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct TaxRateResponse {
    pub id: Uuid,
    pub year: i32,
    pub min_income: Decimal,
    pub max_income: Decimal,
    pub tax_percent: Decimal,
}

#[derive(Deserialize, ToSchema, Validate, Clone, Debug)]
pub struct CreateTaxRateRequest {
    pub year: i32,
    pub min_income: Decimal,
    pub max_income: Decimal,
    pub tax_percent: Decimal,
}

#[derive(Deserialize, ToSchema, Clone, Debug)]
pub struct UpdateTaxRateRequest {
    pub year: Option<i32>,
    pub min_income: Option<Decimal>,
    pub max_income: Option<Decimal>,
    pub tax_percent: Option<Decimal>,
}

impl From<TaxRate> for TaxRateResponse {
    fn from(m: TaxRate) -> Self {
        Self {
            id: m.id, year: m.year, min_income: m.min_income, max_income: m.max_income,
            tax_percent: m.tax_percent
        }
    }
}
