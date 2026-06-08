use crate::utils::code_generator::CodeGenerator;
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;
use crate::domain::position::entity::Model as Position;
use crate::domain::position::repository::PositionRepository;
use crate::domain::errors::{AppError, AppResult};
use crate::domain::shared::dtos::{PaginationQuery, PaginatedResponse};
use crate::application::position::dto::{CreatePositionRequest, UpdatePositionRequest};

pub struct PositionService {
    repository: Arc<dyn PositionRepository>,
}

impl PositionService {
    pub fn new(repository: Arc<dyn PositionRepository>) -> Self {
        Self { repository }
    }

    pub async fn get_all_positions(&self, query: PaginationQuery) -> AppResult<PaginatedResponse<Position>> {
        self.repository.find_all(query).await
    }

    pub async fn get_position_by_id(&self, id: Uuid) -> AppResult<Position> {
        self.repository.find_by_id(id).await
    }

    pub async fn create_position(&self, req: CreatePositionRequest) -> AppResult<Position> {
        req.validate().map_err(|e| AppError::Validation(e.to_string()))?;
        
        let code = match req.code {
            Some(c) => c,
            None => {
                let last_code = self.repository.find_latest_code().await?;
                CodeGenerator::generate("POS", last_code)
            }
        };

        let position = Position {
            id: Uuid::new_v4(),
            code,
            name: req.name,
            level: req.level,
            job_grade: req.job_grade,
        };
        
        self.repository.create(position).await
    }

    pub async fn update_position(&self, id: Uuid, req: UpdatePositionRequest) -> AppResult<Position> {
        let mut position = self.repository.find_by_id(id).await?;
        
        if let Some(val) = req.name { position.name = val; }
        if let Some(val) = req.level { position.level = val; }
        if let Some(val) = req.job_grade { position.job_grade = val; }
        
        self.repository.update(position).await
    }

    pub async fn delete_position(&self, id: Uuid) -> AppResult<()> {
        self.repository.delete(id).await
    }
}
