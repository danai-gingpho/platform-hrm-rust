use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use crate::domain::department::entity::Model as DepartmentModel;
use validator::Validate;

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct DepartmentResponse {
    pub id: Uuid,
    pub legal_entity_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub code: String,
    pub name: String,
    pub cost_center: String,
}

#[derive(Deserialize, ToSchema, Validate, Clone, Debug)]
pub struct CreateDepartmentRequest {
    pub legal_entity_id: Uuid,
    pub parent_id: Option<Uuid>,
    #[validate(length(min = 1, max = 50))]
    pub code: Option<String>,
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    pub cost_center: String,
}

#[derive(Deserialize, ToSchema, Clone, Debug)]
pub struct UpdateDepartmentRequest {
    pub parent_id: Option<Uuid>,
    pub name: Option<String>,
    pub cost_center: Option<String>,
}

impl From<DepartmentModel> for DepartmentResponse {
    fn from(model: DepartmentModel) -> Self {
        Self {
            id: model.id,
            legal_entity_id: model.legal_entity_id,
            parent_id: model.parent_id,
            code: model.code,
            name: model.name,
            cost_center: model.cost_center,
        }
    }
}
