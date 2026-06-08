use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "approval_workflow")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub module: String,
    pub legal_entity_id: Uuid,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crate::domain::legal_entity::Entity",
        from = "Column::LegalEntityId",
        to = "crate::domain::legal_entity::Column::Id"
    )]
    LegalEntity,
    #[sea_orm(has_many = "crate::domain::approval_step::Entity")]
    ApprovalStep,
}

impl Related<crate::domain::legal_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LegalEntity.def()
    }
}

impl Related<crate::domain::approval_step::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ApprovalStep.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}