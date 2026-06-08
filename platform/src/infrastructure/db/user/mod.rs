pub mod model;
use async_trait::async_trait;
use sea_orm::{DatabaseConnection, EntityTrait, ActiveModelTrait, IntoActiveModel, ColumnTrait, QueryFilter};
use uuid::Uuid;
use crate::domain::user::entity::Model as UserModel;
use crate::domain::user::repository::UserRepository;
pub use model::Entity;

pub struct SeaOrmUserRepository {
    db: DatabaseConnection,
}

impl SeaOrmUserRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UserRepository for SeaOrmUserRepository {
    async fn create(&self, user: UserModel) -> anyhow::Result<UserModel> {
        let active_model = user.into_active_model();
        let saved = active_model.insert(&self.db).await?;
        Ok(saved)
    }

    async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<UserModel>> {
        let model = Entity::find_by_id(id).one(&self.db).await?;
        Ok(model)
    }

    async fn find_by_email(&self, email: &str) -> anyhow::Result<Option<UserModel>> {
        let model = Entity::find().filter(crate::domain::user::entity::Column::Email.eq(email)).one(&self.db).await?;
        Ok(model)
    }

    async fn update(&self, user: UserModel) -> anyhow::Result<UserModel> {
        let active_model = user.into_active_model();
        let updated = active_model.update(&self.db).await?;
        Ok(updated)
    }

    async fn delete(&self, id: Uuid) -> anyhow::Result<()> {
        Entity::delete_by_id(id).exec(&self.db).await?;
        Ok(())
    }
}
