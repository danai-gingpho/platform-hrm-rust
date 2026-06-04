use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use chrono::{DateTime, FixedOffset};
use crate::domain::branch::entity::Model as BranchModel;
use validator::Validate;

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct BranchResponse {
    pub id: Uuid,
    pub company_id: Uuid,
    pub code: String,
    pub name: String,
    pub timezone: String,
    pub address: String,
    pub created_at: DateTime<FixedOffset>,
}

#[derive(Deserialize, ToSchema, Validate, Clone, Debug)]
pub struct CreateBranchRequest {
    pub company_id: Uuid,
    #[validate(length(min = 1, max = 50))]
    pub code: Option<String>,
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    pub timezone: String,
    pub address: String,
}

#[derive(Deserialize, ToSchema, Clone, Debug)]
pub struct UpdateBranchRequest {
    pub name: Option<String>,
    pub timezone: Option<String>,
    pub address: Option<String>,
}

impl From<BranchModel> for BranchResponse {
    fn from(model: BranchModel) -> Self {
        Self {
            id: model.id,
            company_id: model.company_id,
            code: model.code,
            name: model.name,
            timezone: model.timezone,
            address: model.address,
            created_at: model.created_at,
        }
    }
}
