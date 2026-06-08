use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "branch")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub legal_entity_id: Uuid,
    pub code: String, // varchar(50)
    pub name: String, // varchar(255)
    pub timezone: String, // varchar(100)
    #[sea_orm(column_type = "Text")]
    pub address: String, // text
    pub created_at: DateTimeWithTimeZone, // timestamptz (DateTime<FixedOffset>)
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // กำหนดความสัมพันธ์เป็น BelongsTo ไปยังตาราง LegalEntity (เนื่องจาก legal_entity_id เป็น FK)
    #[sea_orm(
        belongs_to = "crate::domain::legal_entity::Entity",
        from = "Column::LegalEntityId",
        to = "crate::domain::legal_entity::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    LegalEntity,
}

// Implement Related trait เพื่อให้คิวรีเชื่อมโยงกันได้ง่ายขึ้น (เช่น Branch::find().find_with_related(LegalEntity))
impl Related<crate::domain::legal_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LegalEntity.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}