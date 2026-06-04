use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "department")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub company_id: Uuid,
    pub parent_id: Option<Uuid>, // บอสของแผนกนี้ (ถ้ามี)
    pub code: String,
    pub name: String,
    pub cost_center: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crate::domain::company::Entity",
        from = "Column::CompanyId",
        to = "crate::domain::company::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Company,
    // Self-referencing: แผนกนี้ขึ้นตรงกับ แผนกแม่
    #[sea_orm(
        belongs_to = "Entity",
        from = "Column::ParentId",
        to = "Column::Id",
        on_update = "Cascade",
        on_delete = "SetNull"
    )]
    Parent,
}

impl Related<crate::domain::company::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Company.def()
    }
}

impl Related<Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Parent.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}