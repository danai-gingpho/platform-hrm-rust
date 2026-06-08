use tonic::{Request, Response, Status};
use uuid::Uuid;
use std::sync::Arc;
use crate::application::leave_type::service::LeaveTypeService;
use crate::application::leave_type::dto::CreateLeaveTypeRequest as DtoCreateRequest;

pub mod hris_proto {
    tonic::include_proto!("hris");
}

use hris_proto::leave_type_service_server::LeaveTypeService as LeaveTypeGrpcService;
pub use hris_proto::leave_type_service_server::LeaveTypeServiceServer;
use hris_proto::{GetLeaveTypeRequest, CreateLeaveTypeRequest, LeaveTypeResponse};

pub struct LeaveTypeGrpcHandler {
    service: Arc<LeaveTypeService>,
}

impl LeaveTypeGrpcHandler {
    pub fn new(service: Arc<LeaveTypeService>) -> Self {
        Self { service }
    }
}

#[tonic::async_trait]
impl LeaveTypeGrpcService for LeaveTypeGrpcHandler {
    async fn get_leave_type(
        &self,
        request: Request<GetLeaveTypeRequest>,
    ) -> Result<Response<LeaveTypeResponse>, Status> {
        let id = Uuid::parse_str(&request.into_inner().id)
            .map_err(|_| Status::invalid_argument("Invalid UUID"))?;

        let leave_type = self.service.get_leave_type_by_id(id).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(LeaveTypeResponse {
            id: leave_type.id.to_string(),
            code: leave_type.code,
            name: leave_type.name,
            paid: leave_type.paid,
            max_days_per_year: leave_type.max_days_per_year.to_string(),
            gender_restriction: leave_type.gender_restriction,
        }))
    }

    async fn create_leave_type(
        &self,
        request: Request<CreateLeaveTypeRequest>,
    ) -> Result<Response<LeaveTypeResponse>, Status> {
        let req = request.into_inner();
        let dto = DtoCreateRequest {
            code: if req.code.is_empty() { None } else { Some(req.code) },
            name: req.name,
            paid: req.paid,
            max_days_per_year: req.max_days_per_year.parse().map_err(|_| Status::invalid_argument("Invalid max_days_per_year"))?,
            gender_restriction: req.gender_restriction,
        };

        let leave_type = self.service.create_leave_type(dto).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(LeaveTypeResponse {
            id: leave_type.id.to_string(),
            code: leave_type.code,
            name: leave_type.name,
            paid: leave_type.paid,
            max_days_per_year: leave_type.max_days_per_year.to_string(),
            gender_restriction: leave_type.gender_restriction,
        }))
    }
}
