use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;
use crate::domain::company::entity::Model as Company;
use crate::domain::company::repository::CompanyRepository;
use crate::domain::errors::{AppError, AppResult};
use crate::domain::shared::dtos::{PaginationQuery, PaginatedResponse};
use crate::application::company::dto::{CreateCompanyRequest, UpdateCompanyRequest};
use crate::utils::code_generator::CodeGenerator;
use chrono::Utc;

pub struct CompanyService {
    repository: Arc<dyn CompanyRepository>,
}

use crate::domain::shared::context::TenantContext;

impl CompanyService {
    pub fn new(repository: Arc<dyn CompanyRepository>) -> Self {
        Self { repository }
    }

    pub async fn get_all_companies(&self, ctx: &TenantContext, query: PaginationQuery) -> AppResult<PaginatedResponse<Company>> {
        self.repository.find_all(ctx, query).await
    }

    pub async fn get_company_by_id(&self, ctx: &TenantContext, id: Uuid) -> AppResult<Company> {
        self.repository.find_by_id(ctx, id).await
    }

    pub async fn create_company(&self, ctx: &TenantContext, req: CreateCompanyRequest) -> AppResult<Company> {
        req.validate().map_err(|e| AppError::Validation(e.to_string()))?;
        
        let code = match req.code {
            Some(c) => c,
            None => {
                let last_code = self.repository.find_latest_code(ctx).await?;
                CodeGenerator::generate("COMP", last_code)
            }
        };

        let company = Company {
            id: Uuid::new_v4(),
            tenant_id: Uuid::nil(), // Will be set by repository or here
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
        
        self.repository.create(ctx, company).await
    }

    pub async fn update_company(&self, ctx: &TenantContext, id: Uuid, req: UpdateCompanyRequest) -> AppResult<Company> {
        let mut company = self.repository.find_by_id(ctx, id).await?;
        
        if let Some(tax_id) = req.tax_id { company.tax_id = tax_id; }
        if let Some(name_th) = req.name_th { company.name_th = name_th; }
        if let Some(name_en) = req.name_en { company.name_en = name_en; }
        if let Some(address) = req.address { company.address = address; }
        if let Some(phone) = req.phone { company.phone = phone; }
        if let Some(email) = req.email { company.email = email; }
        if let Some(is_active) = req.is_active { company.is_active = is_active; }
        
        company.updated_at = Utc::now().into();
        
        self.repository.update(ctx, company).await
    }

    pub async fn delete_company(&self, ctx: &TenantContext, id: Uuid) -> AppResult<()> {
        self.repository.delete(ctx, id).await
    }
}
