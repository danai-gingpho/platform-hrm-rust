use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use rust_decimal::Decimal;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "timesheet")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub employee_id: Uuid,
    pub work_date: ChronoDate,
    pub check_in: Option<ChronoDateTimeWithTimeZone>,  
    pub check_out: Option<ChronoDateTimeWithTimeZone>, 
    pub late_minutes: i32,
    pub early_leave_minutes: i32,
    pub absent: bool,
    pub work_hours: Decimal,      
    pub overtime_hours: Decimal,  
    pub status: String,
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
}

impl Related<crate::domain::employee::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Employee.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}