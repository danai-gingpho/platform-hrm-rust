use crate::utils::code_generator::CodeGenerator;
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;
use crate::domain::salary_structure_item::entity::Model as SalaryStructureItem;
use crate::domain::salary_structure_item::repository::SalaryStructureItemRepository;
use crate::domain::errors::{AppError, AppResult};
use crate::domain::shared::dtos::{PaginationQuery, PaginatedResponse};
use crate::application::salary_structure_item::dto::{CreateSalaryStructureItemRequest, UpdateSalaryStructureItemRequest};

pub struct SalaryStructureItemService {
    repository: Arc<dyn SalaryStructureItemRepository>,
}

impl SalaryStructureItemService {
    pub fn new(repository: Arc<dyn SalaryStructureItemRepository>) -> Self {
        Self { repository }
    }

    pub async fn get_all_items(&self, query: PaginationQuery) -> AppResult<PaginatedResponse<SalaryStructureItem>> {
        self.repository.find_all(query).await
    }

    pub async fn get_item_by_id(&self, id: Uuid) -> AppResult<SalaryStructureItem> {
        self.repository.find_by_id(id).await
    }

    pub async fn create_item(&self, req: CreateSalaryStructureItemRequest) -> AppResult<SalaryStructureItem> {
        req.validate().map_err(|e| AppError::Validation(e.to_string()))?;
        
        let code = match req.code {
            Some(c) => c,
            None => {
                let last_code = self.repository.find_latest_code().await?;
                CodeGenerator::generate("ITEM", last_code)
            }
        };

        let item = SalaryStructureItem {
            id: Uuid::new_v4(),
            salary_structure_id: req.salary_structure_id,
            code,
            name: req.name,
            item_type: req.item_type,
            calculation_type: req.calculation_type,
            taxable: req.taxable,
            sso_calculatable: req.sso_calculatable,
            sequence: req.sequence,
        };
        
        self.repository.create(item).await
    }

    pub async fn update_item(&self, id: Uuid, req: UpdateSalaryStructureItemRequest) -> AppResult<SalaryStructureItem> {
        let mut item = self.repository.find_by_id(id).await?;
        
        if let Some(val) = req.name { item.name = val; }
        if let Some(val) = req.item_type { item.item_type = val; }
        if let Some(val) = req.calculation_type { item.calculation_type = val; }
        if let Some(val) = req.taxable { item.taxable = val; }
        if let Some(val) = req.sso_calculatable { item.sso_calculatable = val; }
        if let Some(val) = req.sequence { item.sequence = val; }
        
        self.repository.update(item).await
    }

    pub async fn delete_item(&self, id: Uuid) -> AppResult<()> {
        self.repository.delete(id).await
    }
}
