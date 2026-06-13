use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::employee::entity::Model;
use crate::domain::errors::AppResult;
use crate::domain::shared::dtos::{PaginationQuery, PaginatedResponse};

#[async_trait]
pub trait EmployeeRepository: Send + Sync {
    async fn create(&self, tenant_id: &str, employee: Model) -> AppResult<Model>;
    async fn delete(&self, tenant_id: &str, id: Uuid) -> AppResult<()>;
    async fn update(&self, tenant_id: &str, employee: Model) -> AppResult<Model>;
    async fn find_by_id(&self, tenant_id: &str, id: Uuid) -> AppResult<Model>;
    async fn find_all(&self, tenant_id: &str, query: PaginationQuery) -> AppResult<PaginatedResponse<Model>>;
    async fn find_latest_code(&self, tenant_id: &str) -> AppResult<Option<String>>;
}
