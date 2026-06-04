use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use rust_decimal::Decimal;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "payroll_item")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub payroll_run_id: Uuid,
    pub item_code: String,
    pub item_name: String,
    pub item_category: String,
    pub quantity: Decimal,
    pub rate: Decimal,
    pub amount: Decimal,
    pub taxable: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crate::domain::payroll_run::Entity",
        from = "Column::PayrollRunId",
        to = "crate::domain::payroll_run::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    PayrollRun,
}

impl Related<crate::domain::payroll_run::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PayrollRun.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}