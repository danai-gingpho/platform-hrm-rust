use async_trait::async_trait;
use uuid::Uuid;
use anyhow::Result;

#[async_trait]
pub trait RolePermissionRepository: Send + Sync {
    async fn assign(&self, role_id: Uuid, permission_id: Uuid) -> Result<()>;
    async fn remove(&self, role_id: Uuid, permission_id: Uuid) -> Result<()>;
    async fn find_permissions_by_role_id(&self, role_id: Uuid) -> Result<Vec<Uuid>>;
}
