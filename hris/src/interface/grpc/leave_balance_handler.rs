use tonic::{Request, Response, Status};
use uuid::Uuid;
use std::sync::Arc;
use crate::application::leave_balance::service::LeaveBalanceService;
use crate::application::leave_balance::dto::CreateLeaveBalanceRequest as DtoCreateRequest;
use rust_decimal::Decimal;
use std::str::FromStr;

pub mod hris_proto {
    tonic::include_proto!("hris");
}

use hris_proto::leave_balance_service_server::LeaveBalanceService as LeaveBalanceGrpcService;
pub use hris_proto::leave_balance_service_server::LeaveBalanceServiceServer;
use hris_proto::{GetLeaveBalanceRequest, CreateLeaveBalanceRequest, LeaveBalanceResponse};

pub struct LeaveBalanceGrpcHandler {
    service: Arc<LeaveBalanceService>,
}

impl LeaveBalanceGrpcHandler {
    pub fn new(service: Arc<LeaveBalanceService>) -> Self {
        Self { service }
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

        Ok(Response::new(LeaveBalanceResponse {
            id: leave_balance.id.to_string(),
            employee_id: leave_balance.employee_id.to_string(),
            leave_type_id: leave_balance.leave_type_id.to_string(),
            year: leave_balance.year,
            entitled_days: leave_balance.entitled_days.to_string(),
            used_days: leave_balance.used_days.to_string(),
            remaining_days: leave_balance.remaining_days.to_string(),
        }))
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

        Ok(Response::new(LeaveBalanceResponse {
            id: leave_balance.id.to_string(),
            employee_id: leave_balance.employee_id.to_string(),
            leave_type_id: leave_balance.leave_type_id.to_string(),
            year: leave_balance.year,
            entitled_days: leave_balance.entitled_days.to_string(),
            used_days: leave_balance.used_days.to_string(),
            remaining_days: leave_balance.remaining_days.to_string(),
        }))
    }
}
