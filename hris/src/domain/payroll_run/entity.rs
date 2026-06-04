use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use rust_decimal::Decimal;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "payroll_run")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub payroll_period_id: Uuid,
    pub employee_id: Uuid,
    pub gross_income: Decimal,
    pub total_deduction: Decimal,
    pub taxable_income: Decimal,
    pub tax_amount: Decimal,
    pub social_security: Decimal,
    pub net_income: Decimal,
    pub status: String,
    pub calculated_at: ChronoDateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crate::domain::payroll_period::Entity",
        from = "Column::PayrollPeriodId",
        to = "crate::domain::payroll_period::Column::Id"
    )]
    PayrollPeriod,
    #[sea_orm(
        belongs_to = "crate::domain::employee::Entity",
        from = "Column::EmployeeId",
        to = "crate::domain::employee::Column::Id"
    )]
    Employee,
    #[sea_orm(has_many = "crate::domain::payroll_item::Entity")]
    PayrollItem,
}

impl Related<crate::domain::payroll_period::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PayrollPeriod.def()
    }
}

impl Related<crate::domain::employee::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Employee.def()
    }
}

impl Related<crate::domain::payroll_item::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PayrollItem.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}