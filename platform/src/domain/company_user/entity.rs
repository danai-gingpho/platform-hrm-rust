use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "company_users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub company_id: Uuid,
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crate::domain::company::entity::Entity",
        from = "Column::CompanyId",
        to = "crate::domain::company::entity::Column::Id"
    )]
    Company,
    #[sea_orm(
        belongs_to = "crate::domain::user::entity::Entity",
        from = "Column::UserId",
        to = "crate::domain::user::entity::Column::Id"
    )]
    User,
}

impl Related<crate::domain::company::entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Company.def()
    }
}

impl Related<crate::domain::user::entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}


impl ActiveModelBehavior for ActiveModel {}
