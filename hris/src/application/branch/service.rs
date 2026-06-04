use crate::utils::code_generator::CodeGenerator;
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;
use crate::domain::branch::entity::Model as Branch;
use crate::domain::branch::repository::BranchRepository;
use crate::domain::errors::{AppError, AppResult};
use crate::domain::shared::dtos::{PaginationQuery, PaginatedResponse};
use crate::application::branch::dto::{CreateBranchRequest, UpdateBranchRequest};
use crate::domain::shared::context::TenantContext;
use chrono::Utc;

pub struct BranchService {
    repository: Arc<dyn BranchRepository>,
}

impl BranchService {
    pub fn new(repository: Arc<dyn BranchRepository>) -> Self {
        Self { repository }
    }

    pub async fn get_all_branches(&self, ctx: &TenantContext, query: PaginationQuery) -> AppResult<PaginatedResponse<Branch>> {
        self.repository.find_all(ctx, query).await
    }

    pub async fn get_branch_by_id(&self, ctx: &TenantContext, id: Uuid) -> AppResult<Branch> {
        self.repository.find_by_id(ctx, id).await
    }

    pub async fn create_branch(&self, ctx: &TenantContext, req: CreateBranchRequest) -> AppResult<Branch> {
        req.validate().map_err(|e| AppError::Validation(e.to_string()))?;
        
        let code = match req.code {
            Some(c) => c,
            None => {
                let last_code = self.repository.find_latest_code(ctx).await?;
                CodeGenerator::generate("BRCH", last_code)
            }
        };

        let branch = Branch {
            id: Uuid::new_v4(),
            tenant_id: Uuid::nil(), // Set by repository
            company_id: req.company_id,
            code,
            name: req.name,
            timezone: req.timezone,
            address: req.address,
            created_at: Utc::now().into(),
        };
        
        self.repository.create(ctx, branch).await
    }

    pub async fn update_branch(&self, ctx: &TenantContext, id: Uuid, req: UpdateBranchRequest) -> AppResult<Branch> {
        let mut branch = self.repository.find_by_id(ctx, id).await?;
        
        if let Some(val) = req.name { branch.name = val; }
        if let Some(val) = req.timezone { branch.timezone = val; }
        if let Some(val) = req.address { branch.address = val; }
        
        self.repository.update(ctx, branch).await
    }

    pub async fn delete_branch(&self, ctx: &TenantContext, id: Uuid) -> AppResult<()> {
        self.repository.delete(ctx, id).await
    }
}
