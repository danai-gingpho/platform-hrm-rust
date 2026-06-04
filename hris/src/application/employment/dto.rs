use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use chrono::{NaiveDate, DateTime, FixedOffset};
use crate::domain::employment::entity::Model as EmploymentModel;
use validator::Validate;

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct EmploymentResponse {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub company_id: Uuid,
    pub branch_id: Uuid,
    pub department_id: Uuid,
    pub position_id: Uuid,
    pub manager_employee_id: Option<Uuid>,
    pub employment_type: String,
    pub employment_status: String,
    pub hire_date: NaiveDate,
    pub probation_end_date: Option<NaiveDate>,
    pub resignation_date: Option<NaiveDate>,
    pub last_working_date: Option<NaiveDate>,
    pub payroll_group: String,
    pub work_location: String,
    pub created_at: DateTime<FixedOffset>,
}

#[derive(Deserialize, ToSchema, Validate, Clone, Debug)]
pub struct CreateEmploymentRequest {
    pub employee_id: Uuid,
    pub company_id: Uuid,
    pub branch_id: Uuid,
    pub department_id: Uuid,
    pub position_id: Uuid,
    pub manager_employee_id: Option<Uuid>,
    pub employment_type: String,
    pub employment_status: String,
    pub hire_date: NaiveDate,
    pub probation_end_date: Option<NaiveDate>,
    pub payroll_group: String,
    pub work_location: String,
}

#[derive(Deserialize, ToSchema, Clone, Debug)]
pub struct UpdateEmploymentRequest {
    pub company_id: Option<Uuid>,
    pub branch_id: Option<Uuid>,
    pub department_id: Option<Uuid>,
    pub position_id: Option<Uuid>,
    pub manager_employee_id: Option<Uuid>,
    pub employment_type: Option<String>,
    pub employment_status: Option<String>,
    pub hire_date: Option<NaiveDate>,
    pub probation_end_date: Option<NaiveDate>,
    pub resignation_date: Option<NaiveDate>,
    pub last_working_date: Option<NaiveDate>,
    pub payroll_group: Option<String>,
    pub work_location: Option<String>,
}

impl From<EmploymentModel> for EmploymentResponse {
    fn from(model: EmploymentModel) -> Self {
        Self {
            id: model.id,
            employee_id: model.employee_id,
            company_id: model.company_id,
            branch_id: model.branch_id,
            department_id: model.department_id,
            position_id: model.position_id,
            manager_employee_id: model.manager_employee_id,
            employment_type: model.employment_type,
            employment_status: model.employment_status,
            hire_date: model.hire_date,
            probation_end_date: model.probation_end_date,
            resignation_date: model.resignation_date,
            last_working_date: model.last_working_date,
            payroll_group: model.payroll_group,
            work_location: model.work_location,
            created_at: model.created_at,
        }
    }
}
