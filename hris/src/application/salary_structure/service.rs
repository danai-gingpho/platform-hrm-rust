use crate::utils::code_generator::CodeGenerator;
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;
use crate::domain::salary_structure::entity::Model as SalaryStructure;
use crate::domain::salary_structure::repository::SalaryStructureRepository;
use crate::domain::errors::{AppError, AppResult};
use crate::domain::shared::dtos::{PaginationQuery, PaginatedResponse};
use crate::application::salary_structure::dto::{CreateSalaryStructureRequest, UpdateSalaryStructureRequest};

pub struct SalaryStructureService {
    repository: Arc<dyn SalaryStructureRepository>,
}

impl SalaryStructureService {
    pub fn new(repository: Arc<dyn SalaryStructureRepository>) -> Self {
        Self { repository }
    }

    pub async fn get_all_salary_structures(&self, query: PaginationQuery) -> AppResult<PaginatedResponse<SalaryStructure>> {
        self.repository.find_all(query).await
    }

    pub async fn get_salary_structure_by_id(&self, id: Uuid) -> AppResult<SalaryStructure> {
        self.repository.find_by_id(id).await
    }

    pub async fn create_salary_structure(&self, req: CreateSalaryStructureRequest) -> AppResult<SalaryStructure> {
        req.validate().map_err(|e| AppError::Validation(e.to_string()))?;
        
        let code = match req.code {
            Some(c) => c,
            None => {
                let last_code = self.repository.find_latest_code().await?;
                CodeGenerator::generate("SALY", last_code)
            }
        };

        let structure = SalaryStructure {
            id: Uuid::new_v4(),
            code,
            name: req.name,
            legal_entity_id: req.legal_entity_id,
        };
        
        self.repository.create(structure).await
    }

    pub async fn update_salary_structure(&self, id: Uuid, req: UpdateSalaryStructureRequest) -> AppResult<SalaryStructure> {
        let mut structure = self.repository.find_by_id(id).await?;
        
        if let Some(val) = req.name { structure.name = val; }
        if let Some(val) = req.legal_entity_id { structure.legal_entity_id = val; }
        self.repository.update(structure).await
    }

    pub async fn delete_salary_structure(&self, id: Uuid) -> AppResult<()> {
        self.repository.delete(id).await
    }
}
