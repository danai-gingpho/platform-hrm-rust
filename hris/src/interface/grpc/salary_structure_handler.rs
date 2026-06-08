use tonic::{Request, Response, Status};
use uuid::Uuid;
use std::sync::Arc;
use crate::application::salary_structure::service::SalaryStructureService;
use crate::application::salary_structure::dto::CreateSalaryStructureRequest as DtoCreateRequest;

pub mod hris_proto {
    tonic::include_proto!("hris");
}

use hris_proto::salary_structure_service_server::SalaryStructureService as SalaryStructureGrpcService;
pub use hris_proto::salary_structure_service_server::SalaryStructureServiceServer;
use hris_proto::{GetSalaryStructureRequest, CreateSalaryStructureRequest, SalaryStructureResponse};

pub struct SalaryStructureGrpcHandler {
    service: Arc<SalaryStructureService>,
}

impl SalaryStructureGrpcHandler {
    pub fn new(service: Arc<SalaryStructureService>) -> Self {
        Self { service }
    }
}

#[tonic::async_trait]
impl SalaryStructureGrpcService for SalaryStructureGrpcHandler {
    async fn get_salary_structure(
        &self,
        request: Request<GetSalaryStructureRequest>,
    ) -> Result<Response<SalaryStructureResponse>, Status> {
        let id = Uuid::parse_str(&request.into_inner().id)
            .map_err(|_| Status::invalid_argument("Invalid UUID"))?;

        let structure = self.service.get_salary_structure_by_id(id).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(SalaryStructureResponse {
            id: structure.id.to_string(),
            code: structure.code,
            name: structure.name,
            legal_entity_id: structure.legal_entity_id.to_string(),
        }))
    }

    async fn create_salary_structure(
        &self,
        request: Request<CreateSalaryStructureRequest>,
    ) -> Result<Response<SalaryStructureResponse>, Status> {
        let req = request.into_inner();
        let dto = DtoCreateRequest {
            code: if req.code.is_empty() { None } else { Some(req.code) },
            name: req.name,
            legal_entity_id: Uuid::parse_str(&req.legal_entity_id).map_err(|_| Status::invalid_argument("Invalid legal_entity_id"))?,
        };

        let structure = self.service.create_salary_structure(dto).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(SalaryStructureResponse {
            id: structure.id.to_string(),
            code: structure.code,
            name: structure.name,
            legal_entity_id: structure.legal_entity_id.to_string(),
        }))
    }
}
