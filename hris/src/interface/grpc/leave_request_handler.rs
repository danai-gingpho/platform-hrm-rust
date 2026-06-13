use tonic::{Request, Response, Status};
use uuid::Uuid;
use std::sync::Arc;
use crate::application::leave_request::service::LeaveRequestService;
use crate::application::leave_request::dto::{
    CreateLeaveRequestRequest as DtoCreateRequest, UpdateLeaveRequestRequest as DtoUpdateRequest,
};
use crate::domain::shared::dtos::PaginationQuery;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use std::str::FromStr;

pub mod hris_proto {
    tonic::include_proto!("hris");
}

use hris_proto::leave_request_service_server::LeaveRequestService as LeaveRequestGrpcService;
pub use hris_proto::leave_request_service_server::LeaveRequestServiceServer;
use hris_proto::{
    GetLeaveRequestRequest, CreateLeaveRequestRequest, LeaveRequestResponse,
    UpdateLeaveRequestRequest, DeleteLeaveRequestRequest, ListLeaveRequestsRequest, ListLeaveRequestsResponse, Empty
};

pub struct LeaveRequestGrpcHandler {
    service: Arc<LeaveRequestService>,
}

impl LeaveRequestGrpcHandler {
    pub fn new(service: Arc<LeaveRequestService>) -> Self {
        Self { service }
    }
}

fn map_leave_request_to_response(lr: crate::domain::leave_request::entity::Model) -> LeaveRequestResponse {
    LeaveRequestResponse {
        id: lr.id.to_string(),
        employee_id: lr.employee_id.to_string(),
        leave_type_id: lr.leave_type_id.to_string(),
        start_date: lr.start_date.to_string(),
        end_date: lr.end_date.to_string(),
        total_days: lr.total_days.to_string(),
        reason: lr.reason,
        status: lr.status,
        approved_by: lr.approved_by.map(|id| id.to_string()),
        approved_at: lr.approved_at.map(|d| d.to_string()),
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

        Ok(Response::new(map_leave_request_to_response(leave_request)))
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

        Ok(Response::new(map_leave_request_to_response(leave_request)))
    }

    async fn update_leave_request(
        &self,
        request: Request<UpdateLeaveRequestRequest>,
    ) -> Result<Response<LeaveRequestResponse>, Status> {
        let req = request.into_inner();
        let id = Uuid::parse_str(&req.id)
            .map_err(|_| Status::invalid_argument("Invalid UUID"))?;

        let approved_by = if let Some(ref val) = req.approved_by {
            Some(Uuid::parse_str(val).map_err(|_| Status::invalid_argument("Invalid approved_by UUID"))?)
        } else {
            None
        };

        let dto = DtoUpdateRequest {
            status: req.status,
            approved_by,
        };

        let leave_request = self.service.update_leave_request(id, dto).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(map_leave_request_to_response(leave_request)))
    }

    async fn delete_leave_request(
        &self,
        request: Request<DeleteLeaveRequestRequest>,
    ) -> Result<Response<Empty>, Status> {
        let id = Uuid::parse_str(&request.into_inner().id)
            .map_err(|_| Status::invalid_argument("Invalid UUID"))?;

        self.service.delete_leave_request(id).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(Empty {}))
    }

    async fn list_leave_requests(
        &self,
        request: Request<ListLeaveRequestsRequest>,
    ) -> Result<Response<ListLeaveRequestsResponse>, Status> {
        let req = request.into_inner();
        let query = PaginationQuery {
            page: Some(req.page as u64),
            limit: Some(req.per_page as u64),
            search: req.employee_id, // Filter by employee_id via search
        };

        let response = self.service.get_all_leave_requests(query).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(ListLeaveRequestsResponse {
            items: response.data.into_iter().map(map_leave_request_to_response).collect(),
            total_items: response.total as u32,
            total_pages: response.total_pages as u32,
            current_page: response.page as u32,
            per_page: response.limit as u32,
        }))
    }
}
