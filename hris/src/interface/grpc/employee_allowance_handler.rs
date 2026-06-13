use tonic::{Request, Response, Status};
use uuid::Uuid;
use std::sync::Arc;
use crate::application::employee_allowance::service::EmployeeAllowanceService;
use crate::application::employee_allowance::dto::{
    CreateEmployeeAllowanceRequest as DtoCreateRequest, UpdateEmployeeAllowanceRequest as DtoUpdateRequest,
};
use crate::domain::shared::dtos::PaginationQuery;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use std::str::FromStr;

pub mod hris_proto {
    tonic::include_proto!("hris");
}

use hris_proto::employee_allowance_service_server::EmployeeAllowanceService as EmployeeAllowanceGrpcService;
pub use hris_proto::employee_allowance_service_server::EmployeeAllowanceServiceServer;
use hris_proto::{
    GetEmployeeAllowanceRequest, CreateEmployeeAllowanceRequest, EmployeeAllowanceResponse,
    UpdateEmployeeAllowanceRequest, DeleteEmployeeAllowanceRequest, ListEmployeeAllowancesRequest,
    ListEmployeeAllowancesResponse, Empty
};

pub struct EmployeeAllowanceGrpcHandler {
    service: Arc<EmployeeAllowanceService>,
}

impl EmployeeAllowanceGrpcHandler {
    pub fn new(service: Arc<EmployeeAllowanceService>) -> Self {
        Self { service }
    }
}

fn map_allowance_to_response(allowance: crate::domain::employee_allowance::entity::Model) -> EmployeeAllowanceResponse {
    EmployeeAllowanceResponse {
        id: allowance.id.to_string(),
        employee_id: allowance.employee_id.to_string(),
        allowance_type_id: allowance.allowance_type_id.to_string(),
        amount: allowance.amount.to_string(),
        effective_date: allowance.effective_date.to_string(),
        end_date: allowance.end_date.map(|d| d.to_string()),
    }
}

#[tonic::async_trait]
impl EmployeeAllowanceGrpcService for EmployeeAllowanceGrpcHandler {
    async fn get_employee_allowance(
        &self,
        request: Request<GetEmployeeAllowanceRequest>,
    ) -> Result<Response<EmployeeAllowanceResponse>, Status> {
        let id = Uuid::parse_str(&request.into_inner().id)
            .map_err(|_| Status::invalid_argument("Invalid UUID"))?;

        let allowance = self.service.get_allowance_by_id(id).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(map_allowance_to_response(allowance)))
    }

    async fn create_employee_allowance(
        &self,
        request: Request<CreateEmployeeAllowanceRequest>,
    ) -> Result<Response<EmployeeAllowanceResponse>, Status> {
        let req = request.into_inner();
        
        let employee_id = Uuid::parse_str(&req.employee_id)
            .map_err(|_| Status::invalid_argument("Invalid employee_id UUID"))?;
        let allowance_type_id = Uuid::parse_str(&req.allowance_type_id)
            .map_err(|_| Status::invalid_argument("Invalid allowance_type_id UUID"))?;
        
        let amount = Decimal::from_str(&req.amount)
            .map_err(|_| Status::invalid_argument("Invalid amount format"))?;
        
        let effective_date = NaiveDate::parse_from_str(&req.effective_date, "%Y-%m-%d")
            .map_err(|_| Status::invalid_argument("Invalid effective_date format, expected YYYY-MM-DD"))?;
        
        let end_date = if let Some(ref date_str) = req.end_date {
            Some(NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
                .map_err(|_| Status::invalid_argument("Invalid end_date format, expected YYYY-MM-DD"))?)
        } else {
            None
        };

        let dto = DtoCreateRequest {
            employee_id,
            allowance_type_id,
            amount,
            effective_date,
            end_date,
        };

        let allowance = self.service.create_allowance(dto).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(map_allowance_to_response(allowance)))
    }

    async fn update_employee_allowance(
        &self,
        request: Request<UpdateEmployeeAllowanceRequest>,
    ) -> Result<Response<EmployeeAllowanceResponse>, Status> {
        let req = request.into_inner();
        let id = Uuid::parse_str(&req.id)
            .map_err(|_| Status::invalid_argument("Invalid UUID"))?;

        let amount = if let Some(ref amt_str) = req.amount {
            Some(Decimal::from_str(amt_str)
                .map_err(|_| Status::invalid_argument("Invalid amount format"))?)
        } else {
            None
        };

        let effective_date = if let Some(ref date_str) = req.effective_date {
            Some(NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
                .map_err(|_| Status::invalid_argument("Invalid effective_date format, expected YYYY-MM-DD"))?)
        } else {
            None
        };

        let end_date = if let Some(ref date_str) = req.end_date {
            Some(NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
                .map_err(|_| Status::invalid_argument("Invalid end_date format, expected YYYY-MM-DD"))?)
        } else {
            None
        };

        let dto = DtoUpdateRequest {
            amount,
            effective_date,
            end_date,
        };

        let allowance = self.service.update_allowance(id, dto).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(map_allowance_to_response(allowance)))
    }

    async fn delete_employee_allowance(
        &self,
        request: Request<DeleteEmployeeAllowanceRequest>,
    ) -> Result<Response<Empty>, Status> {
        let id = Uuid::parse_str(&request.into_inner().id)
            .map_err(|_| Status::invalid_argument("Invalid UUID"))?;

        self.service.delete_allowance(id).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(Empty {}))
    }

    async fn list_employee_allowances(
        &self,
        request: Request<ListEmployeeAllowancesRequest>,
    ) -> Result<Response<ListEmployeeAllowancesResponse>, Status> {
        let req = request.into_inner();
        let query = PaginationQuery {
            page: Some(req.page as u64),
            limit: Some(req.per_page as u64),
            search: req.employee_id, // Using search field for employee_id filtering as a quick fix or adjust DTO
        };

        let response = self.service.get_all_allowances(query).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(ListEmployeeAllowancesResponse {
            items: response.data.into_iter().map(map_allowance_to_response).collect(),
            total_items: response.total as u32,
            total_pages: response.total_pages as u32,
            current_page: response.page as u32,
            per_page: response.limit as u32,
        }))
    }
}
