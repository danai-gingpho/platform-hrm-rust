use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "role_permissions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub role_id: Uuid,
    #[sea_orm(primary_key, auto_increment = false)]
    pub permission_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crate::domain::role::entity::Entity",
        from = "Column::RoleId",
        to = "crate::domain::role::entity::Column::Id"
    )]
    Role,
    #[sea_orm(
        belongs_to = "crate::domain::permission::entity::Entity",
        from = "Column::PermissionId",
        to = "crate::domain::permission::entity::Column::Id"
    )]
    Permission,
}

impl Related<crate::domain::role::entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Role.def()
    }
}

impl Related<crate::domain::permission::entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Permission.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
