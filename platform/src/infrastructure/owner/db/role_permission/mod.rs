pub mod model;
use async_trait::async_trait;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, ActiveModelTrait, Set};
use uuid::Uuid;
use crate::domain::owner::role_permission::RolePermissionRepository;
use crate::infrastructure::owner::db::role_permission::model::Entity as RolePermissionEntity;
use anyhow::Result;

pub struct SeaOrmRolePermissionRepository {
    db: DatabaseConnection,
}

impl SeaOrmRolePermissionRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl RolePermissionRepository for SeaOrmRolePermissionRepository {
    async fn assign(&self, role_id: Uuid, permission_id: Uuid) -> Result<()> {
        let active_model = model::ActiveModel {
            role_id: Set(role_id),
            permission_id: Set(permission_id),
        };
        active_model.insert(&self.db).await?;
        Ok(())
    }

    async fn remove(&self, role_id: Uuid, permission_id: Uuid) -> Result<()> {
        RolePermissionEntity::delete_by_id((role_id, permission_id)).exec(&self.db).await?;
        Ok(())
    }

    async fn find_permissions_by_role_id(&self, role_id: Uuid) -> Result<Vec<Uuid>> {
        let models = RolePermissionEntity::find()
            .filter(model::Column::RoleId.eq(role_id))
            .all(&self.db)
            .await?;
        Ok(models.into_iter().map(|m| m.permission_id).collect())
    }
}
