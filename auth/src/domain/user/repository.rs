use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::user::entity::Model;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: Model) -> anyhow::Result<Model>;
    async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<Model>>;
    async fn find_by_email(&self, email: &str) -> anyhow::Result<Option<Model>>;
    async fn update(&self, user: Model) -> anyhow::Result<Model>;
    async fn delete(&self, id: Uuid) -> anyhow::Result<()>;
}
