use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::company::entity::Model;

#[async_trait]
pub trait CompanyRepository: Send + Sync {
    async fn create(&self, company: Model) -> anyhow::Result<Model>;
    async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<Model>>;
    async fn find_by_code(&self, code: &str) -> anyhow::Result<Option<Model>>;
    async fn update(&self, company: Model) -> anyhow::Result<Model>;
    async fn delete(&self, id: Uuid) -> anyhow::Result<()>;
}
