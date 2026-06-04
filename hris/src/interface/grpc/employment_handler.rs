use tonic::{Request, Response, Status};
use uuid::Uuid;
use std::sync::Arc;
use crate::application::employment::service::EmploymentService;
use crate::application::employment::dto::CreateEmploymentRequest as DtoCreateRequest;
use chrono::NaiveDate;

pub mod hris_proto {
    tonic::include_proto!("hris");
}

use hris_proto::employment_service_server::EmploymentService as EmploymentGrpcService;
pub use hris_proto::employment_service_server::EmploymentServiceServer;
use hris_proto::{GetEmploymentRequest, CreateEmploymentRequest, EmploymentResponse};

pub struct EmploymentGrpcHandler {
    service: Arc<EmploymentService>,
}

impl EmploymentGrpcHandler {
    pub fn new(service: Arc<EmploymentService>) -> Self {
        Self { service }
    }
}

#[tonic::async_trait]
impl EmploymentGrpcService for EmploymentGrpcHandler {
    async fn get_employment(
        &self,
        request: Request<GetEmploymentRequest>,
    ) -> Result<Response<EmploymentResponse>, Status> {
        let id = Uuid::parse_str(&request.into_inner().id)
            .map_err(|_| Status::invalid_argument("Invalid UUID"))?;

        let employment = self.service.get_employment_by_id(id).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(EmploymentResponse {
            id: employment.id.to_string(),
            employee_id: employment.employee_id.to_string(),
            company_id: employment.company_id.to_string(),
            branch_id: employment.branch_id.to_string(),
            department_id: employment.department_id.to_string(),
            position_id: employment.position_id.to_string(),
            manager_employee_id: employment.manager_employee_id.map(|id| id.to_string()).unwrap_or_default(),
            employment_type: employment.employment_type,
            employment_status: employment.employment_status,
            hire_date: employment.hire_date.to_string(),
            probation_end_date: employment.probation_end_date.map(|d| d.to_string()).unwrap_or_default(),
            resignation_date: employment.resignation_date.map(|d| d.to_string()).unwrap_or_default(),
            last_working_date: employment.last_working_date.map(|d| d.to_string()).unwrap_or_default(),
            payroll_group: employment.payroll_group,
            work_location: employment.work_location,
            created_at: employment.created_at.to_string(),
        }))
    }

    async fn create_employment(
        &self,
        request: Request<CreateEmploymentRequest>,
    ) -> Result<Response<EmploymentResponse>, Status> {
        let req = request.into_inner();
        
        let employee_id = Uuid::parse_str(&req.employee_id)
            .map_err(|_| Status::invalid_argument("Invalid employee_id UUID"))?;
        let company_id = Uuid::parse_str(&req.company_id)
            .map_err(|_| Status::invalid_argument("Invalid company_id UUID"))?;
        let branch_id = Uuid::parse_str(&req.branch_id)
            .map_err(|_| Status::invalid_argument("Invalid branch_id UUID"))?;
        let department_id = Uuid::parse_str(&req.department_id)
            .map_err(|_| Status::invalid_argument("Invalid department_id UUID"))?;
        let position_id = Uuid::parse_str(&req.position_id)
            .map_err(|_| Status::invalid_argument("Invalid position_id UUID"))?;
        
        let manager_employee_id = if req.manager_employee_id.is_empty() {
            None
        } else {
            Some(Uuid::parse_str(&req.manager_employee_id)
                .map_err(|_| Status::invalid_argument("Invalid manager_employee_id UUID"))?)
        };

        let hire_date = NaiveDate::parse_from_str(&req.hire_date, "%Y-%m-%d")
            .map_err(|_| Status::invalid_argument("Invalid hire_date format, expected YYYY-MM-DD"))?;
        
        let probation_end_date = if req.probation_end_date.is_empty() {
            None
        } else {
            Some(NaiveDate::parse_from_str(&req.probation_end_date, "%Y-%m-%d")
                .map_err(|_| Status::invalid_argument("Invalid probation_end_date format, expected YYYY-MM-DD"))?)
        };

        let dto = DtoCreateRequest {
            employee_id,
            company_id,
            branch_id,
            department_id,
            position_id,
            manager_employee_id,
            employment_type: req.employment_type,
            employment_status: req.employment_status,
            hire_date,
            probation_end_date,
            payroll_group: req.payroll_group,
            work_location: req.work_location,
        };

        let employment = self.service.create_employment(dto).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(EmploymentResponse {
            id: employment.id.to_string(),
            employee_id: employment.employee_id.to_string(),
            company_id: employment.company_id.to_string(),
            branch_id: employment.branch_id.to_string(),
            department_id: employment.department_id.to_string(),
            position_id: employment.position_id.to_string(),
            manager_employee_id: employment.manager_employee_id.map(|id| id.to_string()).unwrap_or_default(),
            employment_type: employment.employment_type,
            employment_status: employment.employment_status,
            hire_date: employment.hire_date.to_string(),
            probation_end_date: employment.probation_end_date.map(|d| d.to_string()).unwrap_or_default(),
            resignation_date: employment.resignation_date.map(|d| d.to_string()).unwrap_or_default(),
            last_working_date: employment.last_working_date.map(|d| d.to_string()).unwrap_or_default(),
            payroll_group: employment.payroll_group,
            work_location: employment.work_location,
            created_at: employment.created_at.to_string(),
        }))
    }
}
