use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "shift")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub start_time: ChronoTime,
    pub end_time: ChronoTime,
    pub break_minutes: i32,
    pub late_grace_minutes: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "crate::domain::employee_shift::Entity")]
    EmployeeShift,
}

impl Related<crate::domain::employee_shift::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EmployeeShift.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}