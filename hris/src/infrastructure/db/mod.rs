pub mod company;
pub mod employee;
pub mod department;
pub mod branch;
pub mod position;
pub mod payroll_item;
pub mod payroll_period;
pub mod payroll_run;
pub mod salary_structure;
pub mod salary_structure_item;
pub mod tax_rate;
pub mod social_security_rate;
pub mod allowance_type;
pub mod leave_type;
pub mod shift;
pub mod employment;
pub mod employment_contract;
pub mod leave_request;
pub mod leave_balance;
pub mod attendance_log;
pub mod employee_allowance;
use sea_orm::{Database, DatabaseConnection};
use crate::domain::errors::AppResult;

pub async fn init_db(database_url: &str) -> AppResult<DatabaseConnection> {
    let db = Database::connect(database_url).await?;
    Ok(db)
}
