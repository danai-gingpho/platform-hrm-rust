use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "employee_shift")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub employee_id: Uuid,
    pub shift_id: Uuid,
    pub effective_date: ChronoDate,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crate::domain::employee::Entity",
        from = "Column::EmployeeId",
        to = "crate::domain::employee::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Employee,
    #[sea_orm(
        belongs_to = "crate::domain::shift::Entity",
        from = "Column::ShiftId",
        to = "crate::domain::shift::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Shift,
}

impl Related<crate::domain::employee::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Employee.def()
    }
}

impl Related<crate::domain::shift::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Shift.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}