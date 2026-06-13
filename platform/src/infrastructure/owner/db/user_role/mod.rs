pub mod model;
use async_trait::async_trait;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, ActiveModelTrait, Set};
use uuid::Uuid;
use crate::domain::owner::user_role::StaffRoleRepository;
use crate::infrastructure::owner::db::user_role::model::Entity as StaffRoleEntity;
use anyhow::Result;

pub struct SeaOrmStaffRoleRepository {
    db: DatabaseConnection,
}

impl SeaOrmStaffRoleRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl StaffRoleRepository for SeaOrmStaffRoleRepository {
    async fn assign(&self, staff_id: Uuid, role_id: Uuid) -> Result<()> {
        let active_model = model::ActiveModel {
            staff_id: Set(staff_id),
            role_id: Set(role_id),
        };
        active_model.insert(&self.db).await?;
        Ok(())
    }

    async fn remove(&self, staff_id: Uuid, role_id: Uuid) -> Result<()> {
        StaffRoleEntity::delete_by_id((staff_id, role_id)).exec(&self.db).await?;
        Ok(())
    }

    async fn find_roles_by_staff_id(&self, staff_id: Uuid) -> Result<Vec<Uuid>> {
        let models = StaffRoleEntity::find()
            .filter(model::Column::StaffId.eq(staff_id))
            .all(&self.db)
            .await?;
        Ok(models.into_iter().map(|m| m.role_id).collect())
    }
}
