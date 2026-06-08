use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;
use crate::domain::department::entity::Model as Department;
use crate::domain::department::repository::DepartmentRepository;
use crate::domain::errors::{AppError, AppResult};
use crate::domain::shared::dtos::{PaginationQuery, PaginatedResponse};
use crate::application::department::dto::{CreateDepartmentRequest, UpdateDepartmentRequest};
use crate::utils::code_generator::CodeGenerator;

pub struct DepartmentService {
    repository: Arc<dyn DepartmentRepository>,
}

impl DepartmentService {
    pub fn new(repository: Arc<dyn DepartmentRepository>) -> Self {
        Self { repository }
    }

    pub async fn get_all_departments(&self, query: PaginationQuery) -> AppResult<PaginatedResponse<Department>> {
        self.repository.find_all(query).await
    }

    pub async fn get_department_by_id(&self, id: Uuid) -> AppResult<Department> {
        self.repository.find_by_id(id).await
    }

    pub async fn create_department(&self, req: CreateDepartmentRequest) -> AppResult<Department> {
        req.validate().map_err(|e| AppError::Validation(e.to_string()))?;
        
        let code = match req.code {
            Some(c) => c,
            None => {
                let last_code = self.repository.find_latest_code().await?;
                CodeGenerator::generate("DEPT", last_code)
            }
        };

        let department = Department {
            id: Uuid::new_v4(),
            legal_entity_id: req.legal_entity_id,
            parent_id: req.parent_id,
            code,
            name: req.name,
            cost_center: req.cost_center,
        };
        
        self.repository.create(department).await
    }

    pub async fn update_department(&self, id: Uuid, req: UpdateDepartmentRequest) -> AppResult<Department> {
        let mut department = self.repository.find_by_id(id).await?;
        
        if let Some(val) = req.parent_id { department.parent_id = Some(val); }
        if let Some(val) = req.name { department.name = val; }
        if let Some(val) = req.cost_center { department.cost_center = val; }
        
        self.repository.update(department).await
    }

    pub async fn delete_department(&self, id: Uuid) -> AppResult<()> {
        self.repository.delete(id).await
    }
}
