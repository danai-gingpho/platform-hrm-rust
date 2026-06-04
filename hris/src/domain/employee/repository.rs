use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::employee::entity::Model;
use crate::domain::errors::AppResult;
use crate::domain::shared::dtos::{PaginationQuery, PaginatedResponse};

#[async_trait]
pub trait EmployeeRepository: Send + Sync {
    async fn create(&self, employee: Model) -> AppResult<Model>;
    async fn delete(&self, id: Uuid) -> AppResult<()>;
    async fn update(&self, employee: Model) -> AppResult<Model>;
    async fn find_by_id(&self, id: Uuid) -> AppResult<Model>;
    async fn find_all(&self, query: PaginationQuery) -> AppResult<PaginatedResponse<Model>>;
    async fn find_latest_code(&self) -> AppResult<Option<String>>;
}
