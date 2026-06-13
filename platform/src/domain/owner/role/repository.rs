use async_trait::async_trait;
use uuid::Uuid;
use super::entity::Role;
use anyhow::Result;

#[async_trait]
pub trait RoleRepository: Send + Sync {
    async fn create(&self, role: &Role) -> Result<()>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Role>>;
    async fn find_by_name(&self, name: &str) -> Result<Option<Role>>;
    async fn list(&self) -> Result<Vec<Role>>;
    async fn delete(&self, id: Uuid) -> Result<()>;
}
