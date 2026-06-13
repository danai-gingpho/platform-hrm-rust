use tonic::{Request, Response, Status};
use uuid::Uuid;
use std::sync::Arc;
use crate::application::leave_balance::service::LeaveBalanceService;
use crate::application::leave_balance::dto::{
    CreateLeaveBalanceRequest as DtoCreateRequest, UpdateLeaveBalanceRequest as DtoUpdateRequest,
};
use crate::domain::shared::dtos::PaginationQuery;
use rust_decimal::Decimal;
use std::str::FromStr;

pub mod hris_proto {
    tonic::include_proto!("hris");
}

use hris_proto::leave_balance_service_server::LeaveBalanceService as LeaveBalanceGrpcService;
pub use hris_proto::leave_balance_service_server::LeaveBalanceServiceServer;
use hris_proto::{
    GetLeaveBalanceRequest, CreateLeaveBalanceRequest, LeaveBalanceResponse,
    UpdateLeaveBalanceRequest, DeleteLeaveBalanceRequest, ListLeaveBalancesRequest, ListLeaveBalancesResponse, Empty
};

pub struct LeaveBalanceGrpcHandler {
    service: Arc<LeaveBalanceService>,
}

impl LeaveBalanceGrpcHandler {
    pub fn new(service: Arc<LeaveBalanceService>) -> Self {
        Self { service }
    }
}

fn map_leave_balance_to_response(lb: crate::domain::leave_balance::entity::Model) -> LeaveBalanceResponse {
    LeaveBalanceResponse {
        id: lb.id.to_string(),
        employee_id: lb.employee_id.to_string(),
        leave_type_id: lb.leave_type_id.to_string(),
        year: lb.year,
        entitled_days: lb.entitled_days.to_string(),
        used_days: lb.used_days.to_string(),
        remaining_days: lb.remaining_days.to_string(),
    }
}

#[tonic::async_trait]
impl LeaveBalanceGrpcService for LeaveBalanceGrpcHandler {
    async fn get_leave_balance(
        &self,
        request: Request<GetLeaveBalanceRequest>,
    ) -> Result<Response<LeaveBalanceResponse>, Status> {
        let id = Uuid::parse_str(&request.into_inner().id)
            .map_err(|_| Status::invalid_argument("Invalid UUID"))?;

        let leave_balance = self.service.get_leave_balance_by_id(id).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(map_leave_balance_to_response(leave_balance)))
    }

    async fn create_leave_balance(
        &self,
        request: Request<CreateLeaveBalanceRequest>,
    ) -> Result<Response<LeaveBalanceResponse>, Status> {
        let req = request.into_inner();
        
        let employee_id = Uuid::parse_str(&req.employee_id)
            .map_err(|_| Status::invalid_argument("Invalid employee_id UUID"))?;
        let leave_type_id = Uuid::parse_str(&req.leave_type_id)
            .map_err(|_| Status::invalid_argument("Invalid leave_type_id UUID"))?;
        
        let entitled_days = Decimal::from_str(&req.entitled_days)
            .map_err(|_| Status::invalid_argument("Invalid entitled_days format"))?;
        let used_days = Decimal::from_str(&req.used_days)
            .map_err(|_| Status::invalid_argument("Invalid used_days format"))?;

        let dto = DtoCreateRequest {
            employee_id,
            leave_type_id,
            year: req.year,
            entitled_days,
            used_days,
        };

        let leave_balance = self.service.create_leave_balance(dto).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(map_leave_balance_to_response(leave_balance)))
    }

    async fn update_leave_balance(
        &self,
        request: Request<UpdateLeaveBalanceRequest>,
    ) -> Result<Response<LeaveBalanceResponse>, Status> {
        let req = request.into_inner();
        let id = Uuid::parse_str(&req.id)
            .map_err(|_| Status::invalid_argument("Invalid UUID"))?;

        let entitled_days = if let Some(ref val) = req.entitled_days {
            Some(Decimal::from_str(val).map_err(|_| Status::invalid_argument("Invalid entitled_days format"))?)
        } else {
            None
        };

        let used_days = if let Some(ref val) = req.used_days {
            Some(Decimal::from_str(val).map_err(|_| Status::invalid_argument("Invalid used_days format"))?)
        } else {
            None
        };

        let dto = DtoUpdateRequest {
            entitled_days,
            used_days,
        };

        let leave_balance = self.service.update_leave_balance(id, dto).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(map_leave_balance_to_response(leave_balance)))
    }

    async fn delete_leave_balance(
        &self,
        request: Request<DeleteLeaveBalanceRequest>,
    ) -> Result<Response<Empty>, Status> {
        let id = Uuid::parse_str(&request.into_inner().id)
            .map_err(|_| Status::invalid_argument("Invalid UUID"))?;

        self.service.delete_leave_balance(id).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(Empty {}))
    }

    async fn list_leave_balances(
        &self,
        request: Request<ListLeaveBalancesRequest>,
    ) -> Result<Response<ListLeaveBalancesResponse>, Status> {
        let req = request.into_inner();
        let query = PaginationQuery {
            page: Some(req.page as u64),
            limit: Some(req.per_page as u64),
            search: req.employee_id, // Filter by employee_id via search
        };

        let response = self.service.get_all_leave_balances(query).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(ListLeaveBalancesResponse {
            items: response.data.into_iter().map(map_leave_balance_to_response).collect(),
            total_items: response.total as u32,
            total_pages: response.total_pages as u32,
            current_page: response.page as u32,
            per_page: response.limit as u32,
        }))
    }
}
