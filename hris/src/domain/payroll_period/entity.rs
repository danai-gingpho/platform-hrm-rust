use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{NaiveDate as ChronoDate, DateTime as ChronoDateTimeWithTimeZone};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "payroll_period")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub company_id: Uuid,
    pub period_code: String,
    pub start_date: ChronoDate,
    pub end_date: ChronoDate,
    pub payment_date: ChronoDate,
    pub status: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crate::domain::company::Entity",
        from = "Column::CompanyId",
        to = "crate::domain::company::Column::Id"
    )]
    Company,
    #[sea_orm(has_many = "crate::domain::payroll_run::Entity")]
    PayrollRun,
}

impl Related<crate::domain::company::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Company.def()
    }
}

impl Related<crate::domain::payroll_run::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PayrollRun.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}