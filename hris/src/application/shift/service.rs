use crate::utils::code_generator::CodeGenerator;
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;
use crate::domain::shift::entity::Model as Shift;
use crate::domain::shift::repository::ShiftRepository;
use crate::domain::errors::{AppError, AppResult};
use crate::domain::shared::dtos::{PaginationQuery, PaginatedResponse};
use crate::application::shift::dto::{CreateShiftRequest, UpdateShiftRequest};
use sea_orm::prelude::ChronoTime;
use std::str::FromStr;

pub struct ShiftService {
    repository: Arc<dyn ShiftRepository>,
}

impl ShiftService {
    pub fn new(repository: Arc<dyn ShiftRepository>) -> Self {
        Self { repository }
    }

    pub async fn get_all_shifts(&self, query: PaginationQuery) -> AppResult<PaginatedResponse<Shift>> {
        self.repository.find_all(query).await
    }

    pub async fn get_shift_by_id(&self, id: Uuid) -> AppResult<Shift> {
        self.repository.find_by_id(id).await
    }

    pub async fn create_shift(&self, req: CreateShiftRequest) -> AppResult<Shift> {
        req.validate().map_err(|e| AppError::Validation(e.to_string()))?;
        
        let code = match req.code {
            Some(c) => c,
            None => {
                let last_code = self.repository.find_latest_code().await?;
                CodeGenerator::generate("SHFT", last_code)
            }
        };

        let start_time = ChronoTime::from_str(&req.start_time)
            .map_err(|_| AppError::Validation("Invalid start_time format".to_string()))?;
        let end_time = ChronoTime::from_str(&req.end_time)
            .map_err(|_| AppError::Validation("Invalid end_time format".to_string()))?;

        let shift = Shift {
            id: Uuid::new_v4(),
            code,
            name: req.name,
            start_time,
            end_time,
            break_minutes: req.break_minutes,
            late_grace_minutes: req.late_grace_minutes,
        };
        
        self.repository.create(shift).await
    }

    pub async fn update_shift(&self, id: Uuid, req: UpdateShiftRequest) -> AppResult<Shift> {
        let mut shift = self.repository.find_by_id(id).await?;
        
        if let Some(name) = req.name { shift.name = name; }
        if let Some(start_time_str) = req.start_time {
            shift.start_time = ChronoTime::from_str(&start_time_str)
                .map_err(|_| AppError::Validation("Invalid start_time format".to_string()))?;
        }
        if let Some(end_time_str) = req.end_time {
            shift.end_time = ChronoTime::from_str(&end_time_str)
                .map_err(|_| AppError::Validation("Invalid end_time format".to_string()))?;
        }
        if let Some(break_minutes) = req.break_minutes { shift.break_minutes = break_minutes; }
        if let Some(late_grace_minutes) = req.late_grace_minutes { shift.late_grace_minutes = late_grace_minutes; }
        
        self.repository.update(shift).await
    }

    pub async fn delete_shift(&self, id: Uuid) -> AppResult<()> {
        self.repository.delete(id).await
    }
}
