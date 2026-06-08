use tonic::{Request, Response, Status};
use uuid::Uuid;
use std::sync::Arc;
use crate::application::employee_allowance::service::EmployeeAllowanceService;
use crate::application::employee_allowance::dto::CreateEmployeeAllowanceRequest as DtoCreateRequest;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use std::str::FromStr;

pub mod hris_proto {
    tonic::include_proto!("hris");
}

use hris_proto::employee_allowance_service_server::EmployeeAllowanceService as EmployeeAllowanceGrpcService;
pub use hris_proto::employee_allowance_service_server::EmployeeAllowanceServiceServer;
use hris_proto::{GetEmployeeAllowanceRequest, CreateEmployeeAllowanceRequest, EmployeeAllowanceResponse};

pub struct EmployeeAllowanceGrpcHandler {
    service: Arc<EmployeeAllowanceService>,
}

impl EmployeeAllowanceGrpcHandler {
    pub fn new(service: Arc<EmployeeAllowanceService>) -> Self {
        Self { service }
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

        Ok(Response::new(EmployeeAllowanceResponse {
            id: allowance.id.to_string(),
            employee_id: allowance.employee_id.to_string(),
            allowance_type_id: allowance.allowance_type_id.to_string(),
            amount: allowance.amount.to_string(),
            effective_date: allowance.effective_date.to_string(),
            end_date: allowance.end_date.map(|d| d.to_string()).unwrap_or_default(),
        }))
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
        
        let end_date = if req.end_date.is_empty() {
            None
        } else {
            Some(NaiveDate::parse_from_str(&req.end_date, "%Y-%m-%d")
                .map_err(|_| Status::invalid_argument("Invalid end_date format, expected YYYY-MM-DD"))?)
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

        Ok(Response::new(EmployeeAllowanceResponse {
            id: allowance.id.to_string(),
            employee_id: allowance.employee_id.to_string(),
            allowance_type_id: allowance.allowance_type_id.to_string(),
            amount: allowance.amount.to_string(),
            effective_date: allowance.effective_date.to_string(),
            end_date: allowance.end_date.map(|d| d.to_string()).unwrap_or_default(),
        }))
    }
}
