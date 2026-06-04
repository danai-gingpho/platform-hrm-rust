use tonic::{Request, Response, Status};
use uuid::Uuid;
use std::sync::Arc;
use crate::application::shift::service::ShiftService;
use crate::application::shift::dto::CreateShiftRequest as DtoCreateRequest;

pub mod hris_proto {
    tonic::include_proto!("hris");
}

use hris_proto::shift_service_server::ShiftService as ShiftGrpcService;
pub use hris_proto::shift_service_server::ShiftServiceServer;
use hris_proto::{GetShiftRequest, CreateShiftRequest, ShiftResponse};

pub struct ShiftGrpcHandler {
    service: Arc<ShiftService>,
}

impl ShiftGrpcHandler {
    pub fn new(service: Arc<ShiftService>) -> Self {
        Self { service }
    }
}

#[tonic::async_trait]
impl ShiftGrpcService for ShiftGrpcHandler {
    async fn get_shift(
        &self,
        request: Request<GetShiftRequest>,
    ) -> Result<Response<ShiftResponse>, Status> {
        let id = Uuid::parse_str(&request.into_inner().id)
            .map_err(|_| Status::invalid_argument("Invalid UUID"))?;

        let shift = self.service.get_shift_by_id(id).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(ShiftResponse {
            id: shift.id.to_string(),
            code: shift.code,
            name: shift.name,
            start_time: shift.start_time.to_string(),
            end_time: shift.end_time.to_string(),
            break_minutes: shift.break_minutes,
            late_grace_minutes: shift.late_grace_minutes,
        }))
    }

    async fn create_shift(
        &self,
        request: Request<CreateShiftRequest>,
    ) -> Result<Response<ShiftResponse>, Status> {
        let req = request.into_inner();
        let dto = DtoCreateRequest {
            code: if req.code.is_empty() { None } else { Some(req.code) },
            name: req.name,
            start_time: req.start_time,
            end_time: req.end_time,
            break_minutes: req.break_minutes,
            late_grace_minutes: req.late_grace_minutes,
        };

        let shift = self.service.create_shift(dto).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(ShiftResponse {
            id: shift.id.to_string(),
            code: shift.code,
            name: shift.name,
            start_time: shift.start_time.to_string(),
            end_time: shift.end_time.to_string(),
            break_minutes: shift.break_minutes,
            late_grace_minutes: shift.late_grace_minutes,
        }))
    }
}
