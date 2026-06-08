use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;
use crate::domain::legal_entity::entity::Model as LegalEntity;
use crate::domain::legal_entity::repository::LegalEntityRepository;
use crate::domain::errors::{AppError, AppResult};
use crate::domain::shared::dtos::{PaginationQuery, PaginatedResponse};
use crate::application::legal_entity::dto::{CreateLegalEntityRequest, UpdateLegalEntityRequest};
use crate::utils::code_generator::CodeGenerator;
use chrono::Utc;

pub struct LegalEntityService {
    repository: Arc<dyn LegalEntityRepository>,
}

impl LegalEntityService {
    pub fn new(repository: Arc<dyn LegalEntityRepository>) -> Self {
        Self { repository }
    }

    pub async fn get_all_legal_entities(&self, query: PaginationQuery) -> AppResult<PaginatedResponse<LegalEntity>> {
        self.repository.find_all(query).await
    }

    pub async fn get_legal_entity_by_id(&self, id: Uuid) -> AppResult<LegalEntity> {
        self.repository.find_by_id(id).await
    }

    pub async fn create_legal_entity(&self, req: CreateLegalEntityRequest) -> AppResult<LegalEntity> {
        req.validate().map_err(|e| AppError::Validation(e.to_string()))?;
        
        let code = match req.code {
            Some(c) => c,
            None => {
                let last_code = self.repository.find_latest_code().await?;
                CodeGenerator::generate("LE", last_code)
            }
        };

        let legal_entity = LegalEntity {
            id: Uuid::new_v4(),
            code,
            tax_id: req.tax_id,
            name_th: req.name_th,
            name_en: req.name_en,
            address: req.address,
            phone: req.phone,
            email: req.email,
            is_active: true,
            created_at: Utc::now().into(),
            updated_at: Utc::now().into(),
        };
        
        self.repository.create(legal_entity).await
    }

    pub async fn update_legal_entity(&self, id: Uuid, req: UpdateLegalEntityRequest) -> AppResult<LegalEntity> {
        let mut legal_entity = self.repository.find_by_id(id).await?;
        
        if let Some(tax_id) = req.tax_id { legal_entity.tax_id = tax_id; }
        if let Some(name_th) = req.name_th { legal_entity.name_th = name_th; }
        if let Some(name_en) = req.name_en { legal_entity.name_en = name_en; }
        if let Some(address) = req.address { legal_entity.address = address; }
        if let Some(phone) = req.phone { legal_entity.phone = phone; }
        if let Some(email) = req.email { legal_entity.email = email; }
        if let Some(is_active) = req.is_active { legal_entity.is_active = is_active; }
        
        legal_entity.updated_at = Utc::now().into();
        
        self.repository.update(legal_entity).await
    }

    pub async fn delete_legal_entity(&self, id: Uuid) -> AppResult<()> {
        self.repository.delete(id).await
    }
}
