use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "salary_structure_item")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub salary_structure_id: Uuid,
    pub code: String,
    pub name: String,
    pub item_type: String,
    pub calculation_type: String,
    pub taxable: bool,
    pub sso_calculatable: bool,
    pub sequence: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crate::domain::salary_structure::Entity",
        from = "Column::SalaryStructureId",
        to = "crate::domain::salary_structure::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    SalaryStructure,
}

impl Related<crate::domain::salary_structure::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SalaryStructure.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}