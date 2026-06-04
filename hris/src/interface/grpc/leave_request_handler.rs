use tonic::{Request, Response, Status};
use uuid::Uuid;
use std::sync::Arc;
use crate::application::leave_request::service::LeaveRequestService;
use crate::application::leave_request::dto::CreateLeaveRequestRequest as DtoCreateRequest;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use std::str::FromStr;

pub mod hris_proto {
    tonic::include_proto!("hris");
}

use hris_proto::leave_request_service_server::LeaveRequestService as LeaveRequestGrpcService;
pub use hris_proto::leave_request_service_server::LeaveRequestServiceServer;
use hris_proto::{GetLeaveRequestRequest, CreateLeaveRequestRequest, LeaveRequestResponse};

pub struct LeaveRequestGrpcHandler {
    service: Arc<LeaveRequestService>,
}

impl LeaveRequestGrpcHandler {
    pub fn new(service: Arc<LeaveRequestService>) -> Self {
        Self { service }
    }
}

#[tonic::async_trait]
impl LeaveRequestGrpcService for LeaveRequestGrpcHandler {
    async fn get_leave_request(
        &self,
        request: Request<GetLeaveRequestRequest>,
    ) -> Result<Response<LeaveRequestResponse>, Status> {
        let id = Uuid::parse_str(&request.into_inner().id)
            .map_err(|_| Status::invalid_argument("Invalid UUID"))?;

        let leave_request = self.service.get_leave_request_by_id(id).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(LeaveRequestResponse {
            id: leave_request.id.to_string(),
            employee_id: leave_request.employee_id.to_string(),
            leave_type_id: leave_request.leave_type_id.to_string(),
            start_date: leave_request.start_date.to_string(),
            end_date: leave_request.end_date.to_string(),
            total_days: leave_request.total_days.to_string(),
            reason: leave_request.reason,
            status: leave_request.status,
            approved_by: leave_request.approved_by.map(|id| id.to_string()).unwrap_or_default(),
            approved_at: leave_request.approved_at.map(|d| d.to_string()).unwrap_or_default(),
        }))
    }

    async fn create_leave_request(
        &self,
        request: Request<CreateLeaveRequestRequest>,
    ) -> Result<Response<LeaveRequestResponse>, Status> {
        let req = request.into_inner();
        
        let employee_id = Uuid::parse_str(&req.employee_id)
            .map_err(|_| Status::invalid_argument("Invalid employee_id UUID"))?;
        let leave_type_id = Uuid::parse_str(&req.leave_type_id)
            .map_err(|_| Status::invalid_argument("Invalid leave_type_id UUID"))?;
        
        let start_date = NaiveDate::parse_from_str(&req.start_date, "%Y-%m-%d")
            .map_err(|_| Status::invalid_argument("Invalid start_date format, expected YYYY-MM-DD"))?;
        let end_date = NaiveDate::parse_from_str(&req.end_date, "%Y-%m-%d")
            .map_err(|_| Status::invalid_argument("Invalid end_date format, expected YYYY-MM-DD"))?;
        
        let total_days = Decimal::from_str(&req.total_days)
            .map_err(|_| Status::invalid_argument("Invalid total_days format"))?;

        let dto = DtoCreateRequest {
            employee_id,
            leave_type_id,
            start_date,
            end_date,
            total_days,
            reason: req.reason,
        };

        let leave_request = self.service.create_leave_request(dto).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(LeaveRequestResponse {
            id: leave_request.id.to_string(),
            employee_id: leave_request.employee_id.to_string(),
            leave_type_id: leave_request.leave_type_id.to_string(),
            start_date: leave_request.start_date.to_string(),
            end_date: leave_request.end_date.to_string(),
            total_days: leave_request.total_days.to_string(),
            reason: leave_request.reason,
            status: leave_request.status,
            approved_by: leave_request.approved_by.map(|id| id.to_string()).unwrap_or_default(),
            approved_at: leave_request.approved_at.map(|d| d.to_string()).unwrap_or_default(),
        }))
    }
}
