use tonic::{Request, Response, Status};
use uuid::Uuid;
use std::sync::Arc;
use crate::application::employment::service::EmploymentService;
use crate::application::employment::dto::{
    CreateEmploymentRequest as DtoCreateRequest, UpdateEmploymentRequest as DtoUpdateRequest,
};
use crate::domain::shared::dtos::PaginationQuery;
use chrono::NaiveDate;

pub mod hris_proto {
    tonic::include_proto!("hris");
}

use hris_proto::employment_service_server::EmploymentService as EmploymentGrpcService;
pub use hris_proto::employment_service_server::EmploymentServiceServer;
use hris_proto::{
    GetEmploymentRequest, CreateEmploymentRequest, EmploymentResponse,
    UpdateEmploymentRequest, DeleteEmploymentRequest, ListEmploymentsRequest, ListEmploymentsResponse, Empty
};

pub struct EmploymentGrpcHandler {
    service: Arc<EmploymentService>,
}

impl EmploymentGrpcHandler {
    pub fn new(service: Arc<EmploymentService>) -> Self {
        Self { service }
    }
}

fn map_employment_to_response(employment: crate::domain::employment::entity::Model) -> EmploymentResponse {
    EmploymentResponse {
        id: employment.id.to_string(),
        employee_id: employment.employee_id.to_string(),
        legal_entity_id: employment.legal_entity_id.to_string(),
        branch_id: employment.branch_id.to_string(),
        department_id: employment.department_id.to_string(),
        position_id: employment.position_id.to_string(),
        manager_employee_id: employment.manager_employee_id.map(|id| id.to_string()),
        employment_type: employment.employment_type,
        employment_status: employment.employment_status,
        hire_date: employment.hire_date.to_string(),
        probation_end_date: employment.probation_end_date.map(|d| d.to_string()),
        resignation_date: employment.resignation_date.map(|d| d.to_string()),
        last_working_date: employment.last_working_date.map(|d| d.to_string()),
        payroll_group: Some(employment.payroll_group),
        work_location: Some(employment.work_location),
        created_at: employment.created_at.to_string(),
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

        Ok(Response::new(map_employment_to_response(employment)))
    }

    async fn create_employment(
        &self,
        request: Request<CreateEmploymentRequest>,
    ) -> Result<Response<EmploymentResponse>, Status> {
        let req = request.into_inner();
        
        let employee_id = Uuid::parse_str(&req.employee_id)
            .map_err(|_| Status::invalid_argument("Invalid employee_id UUID"))?;
        let legal_entity_id = Uuid::parse_str(&req.legal_entity_id)
            .map_err(|_| Status::invalid_argument("Invalid legal_entity_id UUID"))?;
        let branch_id = Uuid::parse_str(&req.branch_id)
            .map_err(|_| Status::invalid_argument("Invalid branch_id UUID"))?;
        let department_id = Uuid::parse_str(&req.department_id)
            .map_err(|_| Status::invalid_argument("Invalid department_id UUID"))?;
        let position_id = Uuid::parse_str(&req.position_id)
            .map_err(|_| Status::invalid_argument("Invalid position_id UUID"))?;
        
        let manager_employee_id = if let Some(ref id_str) = req.manager_employee_id {
            Some(Uuid::parse_str(id_str)
                .map_err(|_| Status::invalid_argument("Invalid manager_employee_id UUID"))?)
        } else {
            None
        };

        let hire_date = NaiveDate::parse_from_str(&req.hire_date, "%Y-%m-%d")
            .map_err(|_| Status::invalid_argument("Invalid hire_date format, expected YYYY-MM-DD"))?;
        
        let probation_end_date = if let Some(ref date_str) = req.probation_end_date {
            Some(NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
                .map_err(|_| Status::invalid_argument("Invalid probation_end_date format, expected YYYY-MM-DD"))?)
        } else {
            None
        };

        let dto = DtoCreateRequest {
            employee_id,
            legal_entity_id,
            branch_id,
            department_id,
            position_id,
            manager_employee_id,
            employment_type: req.employment_type,
            employment_status: req.employment_status,
            hire_date,
            probation_end_date,
            payroll_group: req.payroll_group.unwrap_or_default(),
            work_location: req.work_location.unwrap_or_default(),
        };

        let employment = self.service.create_employment(dto).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(map_employment_to_response(employment)))
    }

    async fn update_employment(
        &self,
        request: Request<UpdateEmploymentRequest>,
    ) -> Result<Response<EmploymentResponse>, Status> {
        let req = request.into_inner();
        let id = Uuid::parse_str(&req.id)
            .map_err(|_| Status::invalid_argument("Invalid UUID"))?;

        let branch_id = if let Some(ref id_str) = req.branch_id {
            Some(Uuid::parse_str(id_str).map_err(|_| Status::invalid_argument("Invalid branch_id UUID"))?)
        } else {
            None
        };

        let department_id = if let Some(ref id_str) = req.department_id {
            Some(Uuid::parse_str(id_str).map_err(|_| Status::invalid_argument("Invalid department_id UUID"))?)
        } else {
            None
        };

        let position_id = if let Some(ref id_str) = req.position_id {
            Some(Uuid::parse_str(id_str).map_err(|_| Status::invalid_argument("Invalid position_id UUID"))?)
        } else {
            None
        };

        let manager_employee_id = if let Some(ref id_str) = req.manager_employee_id {
            Some(Uuid::parse_str(id_str).map_err(|_| Status::invalid_argument("Invalid manager_employee_id UUID"))?)
        } else {
            None
        };

        let hire_date = if let Some(ref date_str) = req.hire_date {
            Some(NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
                .map_err(|_| Status::invalid_argument("Invalid hire_date format, expected YYYY-MM-DD"))?)
        } else {
            None
        };

        let probation_end_date = if let Some(ref date_str) = req.probation_end_date {
            Some(NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
                .map_err(|_| Status::invalid_argument("Invalid probation_end_date format, expected YYYY-MM-DD"))?)
        } else {
            None
        };

        let resignation_date = if let Some(ref date_str) = req.resignation_date {
            Some(NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
                .map_err(|_| Status::invalid_argument("Invalid resignation_date format, expected YYYY-MM-DD"))?)
        } else {
            None
        };

        let last_working_date = if let Some(ref date_str) = req.last_working_date {
            Some(NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
                .map_err(|_| Status::invalid_argument("Invalid last_working_date format, expected YYYY-MM-DD"))?)
        } else {
            None
        };

        let dto = DtoUpdateRequest {
            legal_entity_id: None,
            branch_id,
            department_id,
            position_id,
            manager_employee_id,
            employment_type: req.employment_type,
            employment_status: req.employment_status,
            hire_date,
            probation_end_date,
            resignation_date,
            last_working_date,
            payroll_group: req.payroll_group,
            work_location: req.work_location,
        };

        let employment = self.service.update_employment(id, dto).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(map_employment_to_response(employment)))
    }

    async fn delete_employment(
        &self,
        request: Request<DeleteEmploymentRequest>,
    ) -> Result<Response<Empty>, Status> {
        let id = Uuid::parse_str(&request.into_inner().id)
            .map_err(|_| Status::invalid_argument("Invalid UUID"))?;

        self.service.delete_employment(id).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(Empty {}))
    }

    async fn list_employments(
        &self,
        request: Request<ListEmploymentsRequest>,
    ) -> Result<Response<ListEmploymentsResponse>, Status> {
        let req = request.into_inner();
        let query = PaginationQuery {
            page: Some(req.page as u64),
            limit: Some(req.per_page as u64),
            search: req.employee_id, // Filter by employee_id via search
        };

        let response = self.service.get_all_employments(query).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(ListEmploymentsResponse {
            items: response.data.into_iter().map(map_employment_to_response).collect(),
            total_items: response.total as u32,
            total_pages: response.total_pages as u32,
            current_page: response.page as u32,
            per_page: response.limit as u32,
        }))
    }
}
