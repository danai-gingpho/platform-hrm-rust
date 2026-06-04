use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;
use crate::domain::employee_allowance::entity::Model as EmployeeAllowance;
use crate::domain::employee_allowance::repository::EmployeeAllowanceRepository;
use crate::domain::errors::{AppError, AppResult};
use crate::domain::shared::dtos::{PaginationQuery, PaginatedResponse};
use crate::application::employee_allowance::dto::{CreateEmployeeAllowanceRequest, UpdateEmployeeAllowanceRequest};

pub struct EmployeeAllowanceService {
    repository: Arc<dyn EmployeeAllowanceRepository>,
}

impl EmployeeAllowanceService {
    pub fn new(repository: Arc<dyn EmployeeAllowanceRepository>) -> Self {
        Self { repository }
    }

    pub async fn get_all_allowances(&self, query: PaginationQuery) -> AppResult<PaginatedResponse<EmployeeAllowance>> {
        self.repository.find_all(query).await
    }

    pub async fn get_allowance_by_id(&self, id: Uuid) -> AppResult<EmployeeAllowance> {
        self.repository.find_by_id(id).await
    }

    pub async fn create_allowance(&self, req: CreateEmployeeAllowanceRequest) -> AppResult<EmployeeAllowance> {
        req.validate().map_err(|e| AppError::Validation(e.to_string()))?;
        
        let allowance = EmployeeAllowance {
            id: Uuid::new_v4(),
            employee_id: req.employee_id,
            allowance_type_id: req.allowance_type_id,
            amount: req.amount,
            effective_date: req.effective_date,
            end_date: req.end_date,
        };
        
        self.repository.create(allowance).await
    }

    pub async fn update_allowance(&self, id: Uuid, req: UpdateEmployeeAllowanceRequest) -> AppResult<EmployeeAllowance> {
        let mut allowance = self.repository.find_by_id(id).await?;
        
        if let Some(val) = req.amount { allowance.amount = val; }
        if let Some(val) = req.effective_date { allowance.effective_date = val; }
        if let Some(val) = req.end_date { allowance.end_date = Some(val); }
        
        self.repository.update(allowance).await
    }

    pub async fn delete_allowance(&self, id: Uuid) -> AppResult<()> {
        self.repository.delete(id).await
    }
}
