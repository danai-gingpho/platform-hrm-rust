use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;
use crate::domain::salary_structure::entity::Model as SalaryStructure;

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct SalaryStructureResponse {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub company_id: Uuid,
}

#[derive(Deserialize, ToSchema, Validate, Clone, Debug)]
pub struct CreateSalaryStructureRequest {
    #[validate(length(min = 2, max = 50))]
    pub code: Option<String>,
    #[validate(length(min = 1))]
    pub name: String,
    pub company_id: Uuid,
}

#[derive(Deserialize, ToSchema, Clone, Debug)]
pub struct UpdateSalaryStructureRequest {
    pub name: Option<String>,
    pub company_id: Option<Uuid>,
}

impl From<SalaryStructure> for SalaryStructureResponse {
    fn from(m: SalaryStructure) -> Self {
        Self { id: m.id, code: m.code, name: m.name, company_id: m.company_id }
    }
}
