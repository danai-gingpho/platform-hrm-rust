use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use rust_decimal::Decimal;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "employment_contract")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub employment_id: Uuid,
    pub contract_no: String,
    pub contract_type: String,
    pub start_date: ChronoDate,
    pub end_date: Option<ChronoDate>,
    pub signed_date: Option<ChronoDate>,
    
    // สำหรับ Numeric(18, 2) จะ map เข้าหาประเภทข้อมูล Decimal ของ Rust
    pub basic_salary: Decimal, 
    
    pub salary_type: String,
    pub currency: String,
    #[sea_orm(column_type = "Text")]
    pub document_url: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crate::domain::employment::Entity",
        from = "Column::EmploymentId",
        to = "crate::domain::employment::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Employment,
}

impl Related<crate::domain::employment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Employment.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}