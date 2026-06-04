use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use crate::domain::position::entity::Model as PositionModel;
use validator::Validate;

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct PositionResponse {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub level: i32,
    pub job_grade: String,
}

#[derive(Deserialize, ToSchema, Validate, Clone, Debug)]
pub struct CreatePositionRequest {
    #[validate(length(min = 1, max = 50))]
    pub code: Option<String>,
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    pub level: i32,
    pub job_grade: String,
}

#[derive(Deserialize, ToSchema, Clone, Debug)]
pub struct UpdatePositionRequest {
    pub name: Option<String>,
    pub level: Option<i32>,
    pub job_grade: Option<String>,
}

impl From<PositionModel> for PositionResponse {
    fn from(model: PositionModel) -> Self {
        Self {
            id: model.id,
            code: model.code,
            name: model.name,
            level: model.level,
            job_grade: model.job_grade,
        }
    }
}
