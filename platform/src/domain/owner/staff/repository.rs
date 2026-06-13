use async_trait::async_trait;
use uuid::Uuid;
use super::entity::Staff;
use anyhow::Result;

#[async_trait]
pub trait StaffRepository: Send + Sync {
    async fn create(&self, staff: &Staff) -> Result<()>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Staff>>;
    async fn find_by_email(&self, email: &str) -> Result<Option<Staff>>;
    async fn update(&self, staff: &Staff) -> Result<()>;
    async fn delete(&self, id: Uuid) -> Result<()>;
}
