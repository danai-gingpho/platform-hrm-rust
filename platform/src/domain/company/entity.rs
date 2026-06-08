use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "companies")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub company_name: String,
    #[sea_orm(unique)]
    pub company_code: String,
    #[sea_orm(unique)]
    pub schema_name: String,
    pub keycloak_realm: Option<String>,
    pub status: String,
    
    // DB Credentials for Isolation
    pub db_username: Option<String>,
    pub db_password: Option<String>,
    pub db_host: Option<String>,
    pub db_name: Option<String>,

    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "crate::domain::company_user::entity::Entity")]
    CompanyUsers,
}

impl Related<crate::domain::company_user::entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CompanyUsers.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
