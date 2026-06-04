use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "allowance_type")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub code: String,
    pub name: String,
    pub taxable: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "crate::domain::employee_allowance::Entity")]
    EmployeeAllowance,
}

impl Related<crate::domain::employee_allowance::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EmployeeAllowance.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}