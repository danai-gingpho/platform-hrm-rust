use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "employment")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub employee_id: Uuid,
    pub legal_entity_id: Uuid,
    pub branch_id: Uuid,
    pub department_id: Uuid,
    pub position_id: Uuid,
    pub manager_employee_id: Option<Uuid>, // หัวหน้าสายงาน สามารถเป็น Null ได้กรณีเบอร์ 1 ขององค์กร
    pub employment_type: String,
    pub employment_status: String,
    pub hire_date: ChronoDate,
    pub probation_end_date: Option<ChronoDate>,
    pub resignation_date: Option<ChronoDate>,
    pub last_working_date: Option<ChronoDate>,
    pub payroll_group: String,
    pub work_location: String,
    pub created_at: ChronoDateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "crate::domain::employee::Entity", from = "Column::EmployeeId", to = "crate::domain::employee::Column::Id")]
    Employee,
    #[sea_orm(belongs_to = "crate::domain::legal_entity::Entity", from = "Column::LegalEntityId", to = "crate::domain::legal_entity::Column::Id")]
    LegalEntity,
    #[sea_orm(belongs_to = "crate::domain::branch::Entity", from = "Column::BranchId", to = "crate::domain::branch::Column::Id")]
    Branch,
    #[sea_orm(belongs_to = "crate::domain::department::Entity", from = "Column::DepartmentId", to = "crate::domain::department::Column::Id")]
    Department,
    #[sea_orm(belongs_to = "crate::domain::position::Entity", from = "Column::PositionId", to = "crate::domain::position::Column::Id")]
    Position,
    // โยงหาพนักงานคนอื่นที่เป็น Manager
    #[sea_orm(belongs_to = "crate::domain::employee::Entity", from = "Column::ManagerEmployeeId", to = "crate::domain::employee::Column::Id")]
    Manager,
    #[sea_orm(has_many = "crate::domain::employment_contract::Entity")]
    Contract,
}

impl Related<crate::domain::employee::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Employee.def()
    }
}
impl Related<crate::domain::legal_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LegalEntity.def()
    }
}
impl Related<crate::domain::branch::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Branch.def()
    }
}
impl Related<crate::domain::department::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Department.def()
    }
}
impl Related<crate::domain::position::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Position.def()
    }
}
impl Related<crate::domain::employment_contract::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Contract.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}