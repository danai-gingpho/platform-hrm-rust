use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use rust_decimal::Decimal;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "leave_balance")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub employee_id: Uuid,
    pub leave_type_id: Uuid,
    pub year: i32,
    pub entitled_days: Decimal,
    pub used_days: Decimal,
    pub remaining_days: Decimal,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crate::domain::employee::Entity",
        from = "Column::EmployeeId",
        to = "crate::domain::employee::Column::Id"
    )]
    Employee,
    #[sea_orm(
        belongs_to = "crate::domain::leave_type::Entity",
        from = "Column::LeaveTypeId",
        to = "crate::domain::leave_type::Column::Id"
    )]
    LeaveType,
}

impl Related<crate::domain::employee::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Employee.def()
    }
}

impl Related<crate::domain::leave_type::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LeaveType.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}