use tonic::{Request, Response, Status};
use uuid::Uuid;
use std::sync::Arc;
use crate::application::position::service::PositionService;
use crate::application::position::dto::CreatePositionRequest as DtoCreateRequest;

pub mod hris_proto {
    tonic::include_proto!("hris");
}

use hris_proto::position_service_server::PositionService as PositionGrpcService;
pub use hris_proto::position_service_server::PositionServiceServer;
use hris_proto::{GetPositionRequest, CreatePositionRequest, PositionResponse};

pub struct PositionGrpcHandler {
    service: Arc<PositionService>,
}

impl PositionGrpcHandler {
    pub fn new(service: Arc<PositionService>) -> Self {
        Self { service }
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

        Ok(Response::new(PositionResponse {
            id: position.id.to_string(),
            code: position.code,
            name: position.name,
            level: position.level,
            job_grade: position.job_grade,
        }))
    }

    async fn create_position(
        &self,
        request: Request<CreatePositionRequest>,
    ) -> Result<Response<PositionResponse>, Status> {
        let req = request.into_inner();
        
        let dto = DtoCreateRequest {
            code: if req.code.is_empty() { None } else { Some(req.code) },
            name: req.name,
            level: req.level,
            job_grade: req.job_grade,
        };

        let position = self.service.create_position(dto).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(PositionResponse {
            id: position.id.to_string(),
            code: position.code,
            name: position.name,
            level: position.level,
            job_grade: position.job_grade,
        }))
    }
}
