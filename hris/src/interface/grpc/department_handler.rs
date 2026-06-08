use tonic::{Request, Response, Status};
use uuid::Uuid;
use std::sync::Arc;
use crate::application::department::service::DepartmentService;
use crate::application::department::dto::CreateDepartmentRequest as DtoCreateRequest;

pub mod hris_proto {
    tonic::include_proto!("hris");
}

use hris_proto::department_service_server::DepartmentService as DepartmentGrpcService;
pub use hris_proto::department_service_server::DepartmentServiceServer;
use hris_proto::{GetDepartmentRequest, CreateDepartmentRequest, DepartmentResponse};

pub struct DepartmentGrpcHandler {
    service: Arc<DepartmentService>,
}

impl DepartmentGrpcHandler {
    pub fn new(service: Arc<DepartmentService>) -> Self {
        Self { service }
    }
}

#[tonic::async_trait]
impl DepartmentGrpcService for DepartmentGrpcHandler {
    async fn get_department(
        &self,
        request: Request<GetDepartmentRequest>,
    ) -> Result<Response<DepartmentResponse>, Status> {
        let id = Uuid::parse_str(&request.into_inner().id)
            .map_err(|_| Status::invalid_argument("Invalid UUID"))?;

        let department = self.service.get_department_by_id(id).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(DepartmentResponse {
            id: department.id.to_string(),
            legal_entity_id: department.legal_entity_id.to_string(),
            parent_id: department.parent_id.map(|id| id.to_string()).unwrap_or_default(),
            code: department.code,
            name: department.name,
            cost_center: department.cost_center,
        }))
    }

    async fn create_department(
        &self,
        request: Request<CreateDepartmentRequest>,
    ) -> Result<Response<DepartmentResponse>, Status> {
        let req = request.into_inner();
        
        let legal_entity_id = Uuid::parse_str(&req.legal_entity_id)
            .map_err(|_| Status::invalid_argument("Invalid legal_entity_id UUID"))?;
            
        let parent_id = if req.parent_id.is_empty() {
            None
        } else {
            Some(Uuid::parse_str(&req.parent_id)
                .map_err(|_| Status::invalid_argument("Invalid parent_id UUID"))?)
        };

        let dto = DtoCreateRequest {
            legal_entity_id,
            parent_id,
            code: if req.code.is_empty() { None } else { Some(req.code) },
            name: req.name,
            cost_center: req.cost_center,
        };

        let department = self.service.create_department(dto).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(DepartmentResponse {
            id: department.id.to_string(),
            legal_entity_id: department.legal_entity_id.to_string(),
            parent_id: department.parent_id.map(|id| id.to_string()).unwrap_or_default(),
            code: department.code,
            name: department.name,
            cost_center: department.cost_center,
        }))
    }
}
