use tonic::{Request, Response, Status};
use uuid::Uuid;
use std::sync::Arc;
use crate::application::employee::service::EmployeeService;
use crate::application::employee::dto::{
    CreateEmployeeRequest as DtoCreateRequest, UpdateEmployeeRequest as DtoUpdateRequest,
};
use crate::domain::shared::dtos::PaginationQuery;
use chrono::NaiveDate;

pub mod hris_proto {
    tonic::include_proto!("hris");
}

use hris_proto::employee_service_server::EmployeeService as EmployeeGrpcService;
pub use hris_proto::employee_service_server::EmployeeServiceServer;
use hris_proto::{
    CreateEmployeeRequest, DeleteEmployeeRequest, Empty, GetEmployeeRequest, ListEmployeesRequest,
    ListEmployeesResponse, UpdateEmployeeRequest, EmployeeResponse,
};

pub struct EmployeeGrpcHandler {
    service: Arc<EmployeeService>,
}

impl EmployeeGrpcHandler {
    pub fn new(service: Arc<EmployeeService>) -> Self {
        Self { service }
    }
}

fn map_employee_to_response(employee: crate::domain::employee::entity::Model) -> EmployeeResponse {
    EmployeeResponse {
        id: employee.id.to_string(),
        employee_no: employee.employee_no,
        citizen_id: employee.citizen_id,
        passport_no: employee.passport_no,
        title: employee.title,
        first_name_th: employee.first_name_th,
        last_name_th: employee.last_name_th,
        first_name_en: employee.first_name_en,
        last_name_en: employee.last_name_en,
        gender: employee.gender,
        birth_date: employee.birth_date.to_string(),
        marital_status: employee.marital_status,
        nationality: employee.nationality,
        religion: employee.religion,
        phone: employee.phone,
        email: employee.email,
        current_address: employee.current_address,
        permanent_address: employee.permanent_address,
        emergency_contact_name: employee.emergency_contact_name,
        emergency_contact_phone: employee.emergency_contact_phone,
        blood_group: employee.blood_group,
        photo_url: employee.photo_url,
        status: employee.status,
    }
}

#[tonic::async_trait]
impl EmployeeGrpcService for EmployeeGrpcHandler {
    async fn get_employee(
        &self,
        request: Request<GetEmployeeRequest>,
    ) -> Result<Response<EmployeeResponse>, Status> {
        let id = Uuid::parse_str(&request.into_inner().id)
            .map_err(|_| Status::invalid_argument("Invalid UUID"))?;

        let employee = self.service.get_employee_by_id(id).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(map_employee_to_response(employee)))
    }

    async fn create_employee(
        &self,
        request: Request<CreateEmployeeRequest>,
    ) -> Result<Response<EmployeeResponse>, Status> {
        let req = request.into_inner();
        
        let birth_date = NaiveDate::parse_from_str(&req.birth_date, "%Y-%m-%d")
            .map_err(|_| Status::invalid_argument("Invalid birth_date format, expected YYYY-MM-DD"))?;

        let dto = DtoCreateRequest {
            employee_no: if req.employee_no.is_empty() { None } else { Some(req.employee_no) },
            citizen_id: req.citizen_id,
            passport_no: req.passport_no,
            title: req.title,
            first_name_th: req.first_name_th,
            last_name_th: req.last_name_th,
            first_name_en: req.first_name_en,
            last_name_en: req.last_name_en,
            gender: req.gender,
            birth_date,
            marital_status: req.marital_status,
            nationality: req.nationality,
            religion: req.religion,
            phone: req.phone,
            email: req.email,
            current_address: req.current_address,
            permanent_address: req.permanent_address,
            emergency_contact_name: req.emergency_contact_name,
            emergency_contact_phone: req.emergency_contact_phone,
            blood_group: req.blood_group,
            photo_url: req.photo_url,
        };

        let employee = self.service.create_employee(dto).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(map_employee_to_response(employee)))
    }

    async fn update_employee(
        &self,
        request: Request<UpdateEmployeeRequest>,
    ) -> Result<Response<EmployeeResponse>, Status> {
        let req = request.into_inner();
        let id = Uuid::parse_str(&req.id)
            .map_err(|_| Status::invalid_argument("Invalid UUID"))?;

        let birth_date = if let Some(ref date_str) = req.birth_date {
            Some(NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
                .map_err(|_| Status::invalid_argument("Invalid birth_date format, expected YYYY-MM-DD"))?)
        } else {
            None
        };

        let dto = DtoUpdateRequest {
            citizen_id: req.citizen_id,
            passport_no: req.passport_no,
            title: req.title,
            first_name_th: req.first_name_th,
            last_name_th: req.last_name_th,
            first_name_en: req.first_name_en,
            last_name_en: req.last_name_en,
            gender: req.gender,
            birth_date,
            marital_status: req.marital_status,
            nationality: req.nationality,
            religion: req.religion,
            phone: req.phone,
            email: req.email,
            current_address: req.current_address,
            permanent_address: req.permanent_address,
            emergency_contact_name: req.emergency_contact_name,
            emergency_contact_phone: req.emergency_contact_phone,
            blood_group: req.blood_group,
            photo_url: req.photo_url,
            status: req.status,
        };

        let employee = self.service.update_employee(id, dto).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(map_employee_to_response(employee)))
    }

    async fn delete_employee(
        &self,
        request: Request<DeleteEmployeeRequest>,
    ) -> Result<Response<Empty>, Status> {
        let id = Uuid::parse_str(&request.into_inner().id)
            .map_err(|_| Status::invalid_argument("Invalid UUID"))?;

        self.service.delete_employee(id).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(Empty {}))
    }

    async fn list_employees(
        &self,
        request: Request<ListEmployeesRequest>,
    ) -> Result<Response<ListEmployeesResponse>, Status> {
        let req = request.into_inner();
        let query = PaginationQuery {
            page: Some(req.page as u64),
            per_page: Some(req.per_page as u64),
        };

        let response = self.service.get_all_employees(query).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(ListEmployeesResponse {
            items: response.items.into_iter().map(map_employee_to_response).collect(),
            total_items: response.total_items as u32,
            total_pages: response.total_pages as u32,
            current_page: response.current_page as u32,
            per_page: response.per_page as u32,
        }))
    }
}
