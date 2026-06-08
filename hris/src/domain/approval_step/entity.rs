use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "approval_step")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub workflow_id: Uuid,
    pub sequence: i32,
    pub approver_type: String,
    pub approver_employee_id: Option<Uuid>, // Null ได้ถ้าเป็นแบบ Dynamic Type (เช่นสายหัวหน้างานโดยตรง)
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crate::domain::approval_workflow::Entity",
        from = "Column::WorkflowId",
        to = "crate::domain::approval_workflow::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    ApprovalWorkflow,
    #[sea_orm(
        belongs_to = "crate::domain::employee::Entity",
        from = "Column::ApproverEmployeeId",
        to = "crate::domain::employee::Column::Id"
    )]
    ApproverEmployee,
}

impl Related<crate::domain::approval_workflow::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ApprovalWorkflow.def()
    }
}

impl Related<crate::domain::employee::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ApproverEmployee.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}