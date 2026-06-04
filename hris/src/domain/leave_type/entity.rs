use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use rust_decimal::Decimal;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "leave_type")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub paid: bool,
    pub max_days_per_year: Decimal,
    pub gender_restriction: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "crate::domain::leave_balance::Entity")]
    LeaveBalance,
    #[sea_orm(has_many = "crate::domain::leave_request::Entity")]
    LeaveRequest,
}

impl Related<crate::domain::leave_balance::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LeaveBalance.def()
    }
}

impl Related<crate::domain::leave_request::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LeaveRequest.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}