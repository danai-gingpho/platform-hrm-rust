use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::salary_structure_item::entity::Model;
use crate::domain::errors::AppResult;
use crate::domain::shared::dtos::{PaginationQuery, PaginatedResponse};

#[async_trait]
pub trait SalaryStructureItemRepository: Send + Sync {
    async fn create(&self, salary_structure_item: Model) -> AppResult<Model>;
    async fn delete(&self, id: Uuid) -> AppResult<()>;
    async fn update(&self, salary_structure_item: Model) -> AppResult<Model>;
    async fn find_by_id(&self, id: Uuid) -> AppResult<Model>;
    async fn find_all(&self, query: PaginationQuery) -> AppResult<PaginatedResponse<Model>>;
    async fn find_latest_code(&self) -> AppResult<Option<String>>;
}
