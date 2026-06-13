use tonic::{Request, Response, Status};
use uuid::Uuid;
use std::sync::Arc;
use crate::application::shift::service::ShiftService;
use crate::application::shift::dto::{
    CreateShiftRequest as DtoCreateRequest, UpdateShiftRequest as DtoUpdateRequest,
};
use crate::domain::shared::dtos::PaginationQuery;

pub mod hris_proto {
    tonic::include_proto!("hris");
}

use hris_proto::shift_service_server::ShiftService as ShiftGrpcService;
pub use hris_proto::shift_service_server::ShiftServiceServer;
use hris_proto::{
    GetShiftRequest, CreateShiftRequest, ShiftResponse,
    UpdateShiftRequest, DeleteShiftRequest, ListShiftsRequest, ListShiftsResponse, Empty
};

pub struct ShiftGrpcHandler {
    service: Arc<ShiftService>,
}

impl ShiftGrpcHandler {
    pub fn new(service: Arc<ShiftService>) -> Self {
        Self { service }
    }
}

fn map_shift_to_response(shift: crate::domain::shift::entity::Model) -> ShiftResponse {
    ShiftResponse {
        id: shift.id.to_string(),
        code: shift.code,
        name: shift.name,
        start_time: shift.start_time.to_string(),
        end_time: shift.end_time.to_string(),
        break_minutes: shift.break_minutes,
        late_grace_minutes: shift.late_grace_minutes,
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

        Ok(Response::new(map_shift_to_response(shift)))
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

        Ok(Response::new(map_shift_to_response(shift)))
    }

    async fn update_shift(
        &self,
        request: Request<UpdateShiftRequest>,
    ) -> Result<Response<ShiftResponse>, Status> {
        let req = request.into_inner();
        let id = Uuid::parse_str(&req.id)
            .map_err(|_| Status::invalid_argument("Invalid UUID"))?;

        let dto = DtoUpdateRequest {
            name: req.name,
            start_time: req.start_time,
            end_time: req.end_time,
            break_minutes: req.break_minutes.and_then(|v| if v == 0 { None } else { Some(v) }),
            late_grace_minutes: req.late_grace_minutes.and_then(|v| if v == 0 { None } else { Some(v) }),
        };

        let shift = self.service.update_shift(id, dto).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(map_shift_to_response(shift)))
    }

    async fn delete_shift(
        &self,
        request: Request<DeleteShiftRequest>,
    ) -> Result<Response<Empty>, Status> {
        let id = Uuid::parse_str(&request.into_inner().id)
            .map_err(|_| Status::invalid_argument("Invalid UUID"))?;

        self.service.delete_shift(id).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(Empty {}))
    }

    async fn list_shifts(
        &self,
        request: Request<ListShiftsRequest>,
    ) -> Result<Response<ListShiftsResponse>, Status> {
        let req = request.into_inner();
        let query = PaginationQuery {
            page: Some(req.page as u64),
            limit: Some(req.per_page as u64),
            search: None,
        };

        let response = self.service.get_all_shifts(query).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(ListShiftsResponse {
            items: response.data.into_iter().map(map_shift_to_response).collect(),
            total_items: response.total as u32,
            total_pages: response.total_pages as u32,
            current_page: response.page as u32,
            per_page: response.limit as u32,
        }))
    }
}
