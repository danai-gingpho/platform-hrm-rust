use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;
use crate::domain::salary_structure_item::entity::Model as SalaryStructureItem;

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct SalaryStructureItemResponse {
    pub id: Uuid,
    pub salary_structure_id: Uuid,
    pub code: String,
    pub name: String,
    pub item_type: String,
    pub calculation_type: String,
    pub taxable: bool,
    pub sso_calculatable: bool,
    pub sequence: i32,
}

#[derive(Deserialize, ToSchema, Validate, Clone, Debug)]
pub struct CreateSalaryStructureItemRequest {
    pub salary_structure_id: Uuid,
    #[validate(length(min = 2, max = 50))]
    pub code: Option<String>,
    #[validate(length(min = 1))]
    pub name: String,
    pub item_type: String,
    pub calculation_type: String,
    pub taxable: bool,
    pub sso_calculatable: bool,
    pub sequence: i32,
}

#[derive(Deserialize, ToSchema, Clone, Debug)]
pub struct UpdateSalaryStructureItemRequest {
    pub code: Option<String>,
    pub name: Option<String>,
    pub item_type: Option<String>,
    pub calculation_type: Option<String>,
    pub taxable: Option<bool>,
    pub sso_calculatable: Option<bool>,
    pub sequence: Option<i32>,
}

impl From<SalaryStructureItem> for SalaryStructureItemResponse {
    fn from(m: SalaryStructureItem) -> Self {
        Self {
            id: m.id, salary_structure_id: m.salary_structure_id, code: m.code, name: m.name,
            item_type: m.item_type, calculation_type: m.calculation_type,
            taxable: m.taxable, sso_calculatable: m.sso_calculatable, sequence: m.sequence
        }
    }
}
