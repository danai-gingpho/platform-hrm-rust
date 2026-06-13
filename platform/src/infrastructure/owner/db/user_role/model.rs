use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "staff_roles")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub staff_id: Uuid,
    #[sea_orm(primary_key, auto_increment = false)]
    pub role_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
