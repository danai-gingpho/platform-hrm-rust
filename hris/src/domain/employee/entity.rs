use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "employee")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub employee_no: String,
    pub citizen_id: String,
    pub passport_no: String,
    pub title: String,
    pub first_name_th: String,
    pub last_name_th: String,
    pub first_name_en: String,
    pub last_name_en: String,
    pub gender: String,
    pub birth_date: ChronoDate,
    pub marital_status: String,
    pub nationality: String,
    pub religion: String,
    pub phone: String,
    pub email: String,
    #[sea_orm(column_type = "Text")]
    pub current_address: String,
    #[sea_orm(column_type = "Text")]
    pub permanent_address: String,
    pub emergency_contact_name: String,
    pub emergency_contact_phone: String,
    pub blood_group: String,
    #[sea_orm(column_type = "Text")]
    pub photo_url: String,
    pub status: String,
    pub created_at: ChronoDateTimeWithTimeZone,
    pub updated_at: ChronoDateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "crate::domain::employment::Entity")]
    Employment,
}

impl Related<crate::domain::employment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Employment.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}