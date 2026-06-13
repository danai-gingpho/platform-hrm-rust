use tonic::{Request, Response, Status};
use uuid::Uuid;
use std::sync::Arc;
use crate::application::position::service::PositionService;
use crate::application::position::dto::{
    CreatePositionRequest as DtoCreateRequest, UpdatePositionRequest as DtoUpdateRequest,
};
use crate::domain::shared::dtos::PaginationQuery;

pub mod hris_proto {
    tonic::include_proto!("hris");
}

use hris_proto::position_service_server::PositionService as PositionGrpcService;
pub use hris_proto::position_service_server::PositionServiceServer;
use hris_proto::{
    GetPositionRequest, CreatePositionRequest, PositionResponse,
    UpdatePositionRequest, DeletePositionRequest, ListPositionsRequest, ListPositionsResponse, Empty
};

pub struct PositionGrpcHandler {
    service: Arc<PositionService>,
}

impl PositionGrpcHandler {
    pub fn new(service: Arc<PositionService>) -> Self {
        Self { service }
    }
}

fn map_position_to_response(position: crate::domain::position::entity::Model) -> PositionResponse {
    PositionResponse {
        id: position.id.to_string(),
        code: position.code,
        name: position.name,
        level: position.level,
        job_grade: position.job_grade,
    }
}

#[tonic::async_trait]
impl PositionGrpcService for PositionGrpcHandler {
    async fn get_position(
        &self,
        request: Request<GetPositionRequest>,
    ) -> Result<Response<PositionResponse>, Status> {
        let id = Uuid::parse_str(&request.into_inner().id)
            .map_err(|_| Status::invalid_argument("Invalid UUID"))?;

        let position = self.service.get_position_by_id(id).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(map_position_to_response(position)))
    }

    async fn create_position(
        &self,
        request: Request<CreatePositionRequest>,
    ) -> Result<Response<PositionResponse>, Status> {
        let req = request.into_inner();
        
        let dto = DtoCreateRequest {
            code: req.code.and_then(|c| if c.is_empty() { None } else { Some(c) }),
            name: req.name,
            level: req.level,
            job_grade: req.job_grade,
        };

        let position = self.service.create_position(dto).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(map_position_to_response(position)))
    }

    async fn update_position(
        &self,
        request: Request<UpdatePositionRequest>,
    ) -> Result<Response<PositionResponse>, Status> {
        let req = request.into_inner();
        let id = Uuid::parse_str(&req.id)
            .map_err(|_| Status::invalid_argument("Invalid UUID"))?;

        let dto = DtoUpdateRequest {
            name: req.name,
            level: req.level,
            job_grade: req.job_grade,
        };

        let position = self.service.update_position(id, dto).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(map_position_to_response(position)))
    }

    async fn delete_position(
        &self,
        request: Request<DeletePositionRequest>,
    ) -> Result<Response<Empty>, Status> {
        let id = Uuid::parse_str(&request.into_inner().id)
            .map_err(|_| Status::invalid_argument("Invalid UUID"))?;

        self.service.delete_position(id).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(Empty {}))
    }

    async fn list_positions(
        &self,
        request: Request<ListPositionsRequest>,
    ) -> Result<Response<ListPositionsResponse>, Status> {
        let req = request.into_inner();
        let query = PaginationQuery {
            page: Some(req.page as u64),
            limit: Some(req.per_page as u64),
            search: None,
        };

        let response = self.service.get_all_positions(query).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(ListPositionsResponse {
            items: response.data.into_iter().map(map_position_to_response).collect(),
            total_items: response.total as u32,
            total_pages: response.total_pages as u32,
            current_page: response.page as u32,
            per_page: response.limit as u32,
        }))
    }
}
