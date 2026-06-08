use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;
use crate::domain::employment::entity::Model as Employment;
use crate::domain::employment::repository::EmploymentRepository;
use crate::domain::errors::{AppError, AppResult};
use crate::domain::shared::dtos::{PaginationQuery, PaginatedResponse};
use crate::application::employment::dto::{CreateEmploymentRequest, UpdateEmploymentRequest};
use chrono::Utc;

pub struct EmploymentService {
    repository: Arc<dyn EmploymentRepository>,
}

impl EmploymentService {
    pub fn new(repository: Arc<dyn EmploymentRepository>) -> Self {
        Self { repository }
    }

    pub async fn get_all_employments(&self, query: PaginationQuery) -> AppResult<PaginatedResponse<Employment>> {
        self.repository.find_all(query).await
    }

    pub async fn get_employment_by_id(&self, id: Uuid) -> AppResult<Employment> {
        self.repository.find_by_id(id).await
    }

    pub async fn create_employment(&self, req: CreateEmploymentRequest) -> AppResult<Employment> {
        req.validate().map_err(|e| AppError::Validation(e.to_string()))?;
        
        let employment = Employment {
            id: Uuid::new_v4(),
            employee_id: req.employee_id,
            legal_entity_id: req.legal_entity_id,
            branch_id: req.branch_id,
            department_id: req.department_id,
            position_id: req.position_id,
            manager_employee_id: req.manager_employee_id,
            employment_type: req.employment_type,
            employment_status: req.employment_status,
            hire_date: req.hire_date,
            probation_end_date: req.probation_end_date,
            resignation_date: None,
            last_working_date: None,
            payroll_group: req.payroll_group,
            work_location: req.work_location,
            created_at: Utc::now().into(),
        };
        
        self.repository.create(employment).await
    }

    pub async fn update_employment(&self, id: Uuid, req: UpdateEmploymentRequest) -> AppResult<Employment> {
        let mut employment = self.repository.find_by_id(id).await?;
        
        if let Some(val) = req.legal_entity_id { employment.legal_entity_id = val; }
        if let Some(val) = req.branch_id { employment.branch_id = val; }
        if let Some(val) = req.department_id { employment.department_id = val; }
        if let Some(val) = req.position_id { employment.position_id = val; }
        if let Some(val) = req.manager_employee_id { employment.manager_employee_id = Some(val); }
        if let Some(val) = req.employment_type { employment.employment_type = val; }
        if let Some(val) = req.employment_status { employment.employment_status = val; }
        if let Some(val) = req.hire_date { employment.hire_date = val; }
        if let Some(val) = req.probation_end_date { employment.probation_end_date = Some(val); }
        if let Some(val) = req.resignation_date { employment.resignation_date = Some(val); }
        if let Some(val) = req.last_working_date { employment.last_working_date = Some(val); }
        if let Some(val) = req.payroll_group { employment.payroll_group = val; }
        if let Some(val) = req.work_location { employment.work_location = val; }
        
        self.repository.update(employment).await
    }

    pub async fn delete_employment(&self, id: Uuid) -> AppResult<()> {
        self.repository.delete(id).await
    }
}
