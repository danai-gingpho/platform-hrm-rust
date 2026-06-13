use async_trait::async_trait;
use uuid::Uuid;
use super::entity::Permission;
use anyhow::Result;

#[async_trait]
pub trait PermissionRepository: Send + Sync {
    async fn create(&self, permission: &Permission) -> Result<()>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Permission>>;
    async fn find_by_code(&self, code: &str) -> Result<Option<Permission>>;
    async fn list(&self) -> Result<Vec<Permission>>;
    async fn delete(&self, id: Uuid) -> Result<()>;
}
