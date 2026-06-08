use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "legal_entity")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    

    #[sea_orm(unique)]
    pub code: String,
    
    pub tax_id: String,
    pub name_th: String,
    pub name_en: String,
    
    #[sea_orm(column_type = "Text")]
    pub address: String,
    
    pub phone: String,
    pub email: String,
    pub is_active: bool,
    pub created_at: ChronoDateTimeWithTimeZone,
    pub updated_at: ChronoDateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}