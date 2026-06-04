use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use rust_decimal::Decimal;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "employee_allowance")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub employee_id: Uuid,
    pub allowance_type_id: Uuid,
    pub amount: Decimal,
    pub effective_date: ChronoDate,
    pub end_date: Option<ChronoDate>,
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
        belongs_to = "crate::domain::allowance_type::Entity",
        from = "Column::AllowanceTypeId",
        to = "crate::domain::allowance_type::Column::Id"
    )]
    AllowanceType,
}

impl Related<crate::domain::employee::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Employee.def()
    }
}

impl Related<crate::domain::allowance_type::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AllowanceType.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}