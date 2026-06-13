use async_trait::async_trait;
use uuid::Uuid;
use anyhow::Result;

#[async_trait]
pub trait StaffRoleRepository: Send + Sync {
    async fn assign(&self, staff_id: Uuid, role_id: Uuid) -> Result<()>;
    async fn remove(&self, staff_id: Uuid, role_id: Uuid) -> Result<()>;
    async fn find_roles_by_staff_id(&self, staff_id: Uuid) -> Result<Vec<Uuid>>;
}
