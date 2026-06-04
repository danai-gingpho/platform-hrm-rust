use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "salary_structure")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub company_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crate::domain::company::Entity",
        from = "Column::CompanyId",
        to = "crate::domain::company::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Company,
    #[sea_orm(has_many = "crate::domain::salary_structure_item::Entity")]
    SalaryStructureItem,
}

impl Related<crate::domain::company::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Company.def()
    }
}

impl Related<crate::domain::salary_structure_item::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SalaryStructureItem.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}