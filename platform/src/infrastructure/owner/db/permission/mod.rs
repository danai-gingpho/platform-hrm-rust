pub mod model;
use async_trait::async_trait;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, ActiveModelTrait, Set};
use uuid::Uuid;
use crate::domain::owner::permission::{Permission, PermissionRepository};
use crate::infrastructure::owner::db::permission::model::Entity as PermissionEntity;
use anyhow::Result;

pub struct SeaOrmPermissionRepository {
    db: DatabaseConnection,
}

impl SeaOrmPermissionRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl PermissionRepository for SeaOrmPermissionRepository {
    async fn create(&self, p: &Permission) -> Result<()> {
        let active_model = model::ActiveModel {
            id: Set(p.id),
            name: Set(p.name.clone()),
            code: Set(p.code.clone()),
            created_at: Set(p.created_at.into()),
        };
        active_model.insert(&self.db).await?;
        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Permission>> {
        let model = PermissionEntity::find_by_id(id).one(&self.db).await?;
        Ok(model.map(|m| Permission {
            id: m.id,
            name: m.name,
            code: m.code,
            created_at: m.created_at.into(),
        }))
    }

    async fn find_by_code(&self, code: &str) -> Result<Option<Permission>> {
        let model = PermissionEntity::find()
            .filter(model::Column::Code.eq(code))
            .one(&self.db)
            .await?;
        Ok(model.map(|m| Permission {
            id: m.id,
            name: m.name,
            code: m.code,
            created_at: m.created_at.into(),
        }))
    }

    async fn list(&self) -> Result<Vec<Permission>> {
        let models = PermissionEntity::find().all(&self.db).await?;
        Ok(models.into_iter().map(|m| Permission {
            id: m.id,
            name: m.name,
            code: m.code,
            created_at: m.created_at.into(),
        }).collect())
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        PermissionEntity::delete_by_id(id).exec(&self.db).await?;
        Ok(())
    }
}
