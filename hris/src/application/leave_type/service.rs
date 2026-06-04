use crate::utils::code_generator::CodeGenerator;
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;
use crate::domain::leave_type::entity::Model as LeaveType;
use crate::domain::leave_type::repository::LeaveTypeRepository;
use crate::domain::errors::{AppError, AppResult};
use crate::domain::shared::dtos::{PaginationQuery, PaginatedResponse};
use crate::application::leave_type::dto::{CreateLeaveTypeRequest, UpdateLeaveTypeRequest};

pub struct LeaveTypeService {
    repository: Arc<dyn LeaveTypeRepository>,
}

impl LeaveTypeService {
    pub fn new(repository: Arc<dyn LeaveTypeRepository>) -> Self {
        Self { repository }
    }

    pub async fn get_all_leave_types(&self, query: PaginationQuery) -> AppResult<PaginatedResponse<LeaveType>> {
        self.repository.find_all(query).await
    }

    pub async fn get_leave_type_by_id(&self, id: Uuid) -> AppResult<LeaveType> {
        self.repository.find_by_id(id).await
    }

    pub async fn create_leave_type(&self, req: CreateLeaveTypeRequest) -> AppResult<LeaveType> {
        req.validate().map_err(|e| AppError::Validation(e.to_string()))?;
        
        let code = match req.code {
            Some(c) => c,
            None => {
                let last_code = self.repository.find_latest_code().await?;
                CodeGenerator::generate("LEAV", last_code)
            }
        };

        let leave_type = LeaveType {
            id: Uuid::new_v4(),
            code,
            name: req.name,
            paid: req.paid,
            max_days_per_year: req.max_days_per_year,
            gender_restriction: req.gender_restriction,
        };
        
        self.repository.create(leave_type).await
    }

    pub async fn update_leave_type(&self, id: Uuid, req: UpdateLeaveTypeRequest) -> AppResult<LeaveType> {
        let mut leave_type = self.repository.find_by_id(id).await?;
        
        if let Some(name) = req.name { leave_type.name = name; }
        if let Some(paid) = req.paid { leave_type.paid = paid; }
        if let Some(max_days_per_year) = req.max_days_per_year { leave_type.max_days_per_year = max_days_per_year; }
        if let Some(gender_restriction) = req.gender_restriction { leave_type.gender_restriction = gender_restriction; }
        
        self.repository.update(leave_type).await
    }

    pub async fn delete_leave_type(&self, id: Uuid) -> AppResult<()> {
        self.repository.delete(id).await
    }
}
