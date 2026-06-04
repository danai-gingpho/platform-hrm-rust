use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "position")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub code: String,
    pub name: String,
    pub level: i32,
    pub job_grade: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "crate::domain::employment::Entity")]
    Employment,
}

impl Related<crate::domain::employment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Employment.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}