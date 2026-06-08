use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use chrono::NaiveDate;
use crate::domain::employment_contract::entity::Model as ContractModel;
use validator::Validate;
use rust_decimal::Decimal;

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct EmploymentContractResponse {
    pub id: Uuid,
    pub employment_id: Uuid,
    pub contract_no: String,
    pub contract_type: String,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub signed_date: Option<NaiveDate>,
    pub basic_salary: Decimal,
    pub salary_type: String,
    pub currency: String,
    pub document_url: String,
}

#[derive(Deserialize, ToSchema, Validate, Clone, Debug)]
pub struct CreateEmploymentContractRequest {
    pub employment_id: Uuid,
    pub contract_no: Option<String>,
    pub contract_type: String,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub signed_date: Option<NaiveDate>,
    pub basic_salary: Decimal,
    pub salary_type: String,
    pub currency: String,
    pub document_url: String,
}

#[derive(Deserialize, ToSchema, Clone, Debug)]
pub struct UpdateEmploymentContractRequest {
    pub contract_type: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub signed_date: Option<NaiveDate>,
    pub basic_salary: Option<Decimal>,
    pub salary_type: Option<String>,
    pub currency: Option<String>,
    pub document_url: Option<String>,
}

impl From<ContractModel> for EmploymentContractResponse {
    fn from(model: ContractModel) -> Self {
        Self {
            id: model.id,
            employment_id: model.employment_id,
            contract_no: model.contract_no,
            contract_type: model.contract_type,
            start_date: model.start_date,
            end_date: model.end_date,
            signed_date: model.signed_date,
            basic_salary: model.basic_salary,
            salary_type: model.salary_type,
            currency: model.currency,
            document_url: model.document_url,
        }
    }
}
