use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::role::entity::Model;

#[async_trait]
pub trait RoleRepository: Send + Sync {
    async fn create(&self, role: Model) -> anyhow::Result<Model>;
    async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<Model>>;
    async fn find_by_name(&self, name: &str) -> anyhow::Result<Option<Model>>;
    async fn update(&self, role: Model) -> anyhow::Result<Model>;
    async fn delete(&self, id: Uuid) -> anyhow::Result<()>;
}
