use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
pub use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "attendance_log")]
pub struct Model {
    // bigint PK ใน PostgreSQL จะ map เป็น i64 ใน Rust และเปิด auto_increment ไว้อัตโนมัติ
    #[sea_orm(primary_key)]
    pub id: i64,
    pub tenant_id: Uuid,
    pub employee_id: Uuid,
    pub device_id: String,
    pub punch_time: ChronoDateTimeWithTimeZone,
    pub punch_type: String,
    // สำหรับ jsonb จะใช้ Json ของ sea_orm ซึ่งเป็น wrapper ของ serde_json::Value
    pub raw_payload: Json, 
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crate::domain::employee::Entity",
        from = "Column::EmployeeId",
        to = "crate::domain::employee::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Employee,
}

impl Related<crate::domain::employee::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Employee.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}