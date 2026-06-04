use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::company::entity::Model;
use crate::domain::errors::AppResult;
use crate::domain::shared::dtos::{PaginationQuery, PaginatedResponse};
use crate::domain::shared::context::TenantContext;

#[async_trait]
pub trait CompanyRepository: Send + Sync {
    async fn create(&self, ctx: &TenantContext, company: Model) -> AppResult<Model>;
    async fn find_by_user_id(&self, ctx: &TenantContext, user_id: Uuid, query: PaginationQuery) -> AppResult<PaginatedResponse<Model>>;
    async fn delete(&self, ctx: &TenantContext, id: Uuid) -> AppResult<()>;
    async fn update(&self, ctx: &TenantContext, company: Model) -> AppResult<Model>;
    async fn find_by_id(&self, ctx: &TenantContext, id: Uuid) -> AppResult<Model>;
    async fn find_all(&self, ctx: &TenantContext, query: PaginationQuery) -> AppResult<PaginatedResponse<Model>>;
    async fn find_latest_code(&self, ctx: &TenantContext) -> AppResult<Option<String>>;
}
