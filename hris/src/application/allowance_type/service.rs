use crate::utils::code_generator::CodeGenerator;
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;
use crate::domain::allowance_type::entity::Model as AllowanceType;
use crate::domain::allowance_type::repository::AllowanceTypeRepository;
use crate::domain::errors::{AppError, AppResult};
use crate::domain::shared::dtos::{PaginationQuery, PaginatedResponse};
use crate::application::allowance_type::dto::{CreateAllowanceTypeRequest, UpdateAllowanceTypeRequest};

pub struct AllowanceTypeService {
    repository: Arc<dyn AllowanceTypeRepository>,
}

impl AllowanceTypeService {
    pub fn new(repository: Arc<dyn AllowanceTypeRepository>) -> Self {
        Self { repository }
    }

    pub async fn get_all_allowance_types(&self, query: PaginationQuery) -> AppResult<PaginatedResponse<AllowanceType>> {
        self.repository.find_all(query).await
    }

    pub async fn get_allowance_type_by_id(&self, id: Uuid) -> AppResult<AllowanceType> {
        self.repository.find_by_id(id).await
    }

    pub async fn create_allowance_type(&self, req: CreateAllowanceTypeRequest) -> AppResult<AllowanceType> {
        req.validate().map_err(|e| AppError::Validation(e.to_string()))?;
        
        let code = match req.code {
            Some(c) => c,
            None => {
                let last_code = self.repository.find_latest_code().await?;
                CodeGenerator::generate("ALLW", last_code)
            }
        };

        let allowance_type = AllowanceType {
            id: Uuid::new_v4(),
            code,
            name: req.name,
            taxable: req.taxable,
        };
        
        self.repository.create(allowance_type).await
    }

    pub async fn update_allowance_type(&self, id: Uuid, req: UpdateAllowanceTypeRequest) -> AppResult<AllowanceType> {
        let mut allowance_type = self.repository.find_by_id(id).await?;
        
        if let Some(name) = req.name { allowance_type.name = name; }
        if let Some(taxable) = req.taxable { allowance_type.taxable = taxable; }
        
        self.repository.update(allowance_type).await
    }

    pub async fn delete_allowance_type(&self, id: Uuid) -> AppResult<()> {
        self.repository.delete(id).await
    }
}
