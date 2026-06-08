use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "audit_log")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64, // สำหรับ bigint PK
    pub module: String,
    pub record_id: Uuid,
    pub action: String,
    pub old_data: Option<Json>, // กรณี Insert จะไม่มี old_data
    pub new_data: Option<Json>, // กรณี Delete จะไม่มี new_data
    pub changed_by: Uuid,
    pub changed_at: ChronoDateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crate::domain::employee::Entity",
        from = "Column::ChangedBy",
        to = "crate::domain::employee::Column::Id"
    )]
    User,
}

impl Related<crate::domain::employee::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}