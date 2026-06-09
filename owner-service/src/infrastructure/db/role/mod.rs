use async_trait::async_trait;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, ActiveModelTrait, Set};
use uuid::Uuid;
use crate::domain::role::{Role, RoleRepository};
use crate::infrastructure::db::role::model::Entity as RoleEntity;
use crate::infrastructure::db::role::model;
use anyhow::Result;

pub struct SeaOrmRoleRepository {
    db: DatabaseConnection,
}

impl SeaOrmRoleRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl RoleRepository for SeaOrmRoleRepository {
    async fn create(&self, r: &Role) -> Result<()> {
        let active_model = model::ActiveModel {
            id: Set(r.id),
            name: Set(r.name.clone()),
            description: Set(r.description.clone()),
            created_at: Set(r.created_at.into()),
        };
        active_model.insert(&self.db).await?;
        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Role>> {
        let model = RoleEntity::find_by_id(id).one(&self.db).await?;
        Ok(model.map(|m| Role {
            id: m.id,
            name: m.name,
            description: m.description,
            created_at: m.created_at.into(),
        }))
    }

    async fn find_by_name(&self, name: &str) -> Result<Option<Role>> {
        let model = RoleEntity::find()
            .filter(model::Column::Name.eq(name))
            .one(&self.db)
            .await?;
        Ok(model.map(|m| Role {
            id: m.id,
            name: m.name,
            description: m.description,
            created_at: m.created_at.into(),
        }))
    }

    async fn list(&self) -> Result<Vec<Role>> {
        let models = RoleEntity::find().all(&self.db).await?;
        Ok(models.into_iter().map(|m| Role {
            id: m.id,
            name: m.name,
            description: m.description,
            created_at: m.created_at.into(),
        }).collect())
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        RoleEntity::delete_by_id(id).exec(&self.db).await?;
        Ok(())
    }
}
