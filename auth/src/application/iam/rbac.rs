use crate::domain::role::entity::ActiveModel as RoleActiveModel;
use crate::domain::permission::entity::ActiveModel as PermissionActiveModel;
use crate::domain::role_permission::entity::ActiveModel as RolePermActiveModel;
use crate::domain::user_role::entity::ActiveModel as UserRoleActiveModel;
use crate::proto::{CreateRoleRequest, CreatePermissionRequest, AssignRoleRequest, AssignPermissionRequest};
use sea_orm::{DatabaseConnection, ActiveModelTrait, Set};
use uuid::Uuid;
use chrono::Utc;
use std::sync::Arc;

pub struct RbacService {
    db: Arc<DatabaseConnection>,
}

impl RbacService {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    pub async fn create_role(&self, req: CreateRoleRequest) -> anyhow::Result<crate::domain::role::entity::Model> {
        let role = RoleActiveModel {
            id: Set(Uuid::new_v4()),
            name: Set(req.name),
            description: Set(Some(req.description)),
            created_at: Set(Utc::now().into()),
            updated_at: Set(Utc::now().into()),
            ..Default::default()
        };
        Ok(role.insert(self.db.as_ref()).await?)
    }

    pub async fn create_permission(&self, req: CreatePermissionRequest) -> anyhow::Result<crate::domain::permission::entity::Model> {
        let perm = PermissionActiveModel {
            id: Set(Uuid::new_v4()),
            code: Set(req.code),
            name: Set(req.name),
            created_at: Set(Utc::now().into()),
            updated_at: Set(Utc::now().into()),
            ..Default::default()
        };
        Ok(perm.insert(self.db.as_ref()).await?)
    }

    pub async fn assign_role_to_user(&self, req: AssignRoleRequest) -> anyhow::Result<()> {
        let mapping = UserRoleActiveModel {
            user_id: Set(Uuid::parse_str(&req.user_id)?),
            role_id: Set(Uuid::parse_str(&req.role_id)?),
        };
        mapping.insert(self.db.as_ref()).await?;
        Ok(())
    }

    pub async fn assign_permission_to_role(&self, req: AssignPermissionRequest) -> anyhow::Result<()> {
        let mapping = RolePermActiveModel {
            role_id: Set(Uuid::parse_str(&req.role_id)?),
            permission_id: Set(Uuid::parse_str(&req.permission_id)?),
        };
        mapping.insert(self.db.as_ref()).await?;
        Ok(())
    }
}
