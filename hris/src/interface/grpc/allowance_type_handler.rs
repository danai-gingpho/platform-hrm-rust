use tonic::{Request, Response, Status};
use uuid::Uuid;
use std::sync::Arc;
use crate::application::allowance_type::service::AllowanceTypeService;
use crate::application::allowance_type::dto::CreateAllowanceTypeRequest as DtoCreateRequest;

pub mod hris_proto {
    tonic::include_proto!("hris");
}

use hris_proto::allowance_type_service_server::AllowanceTypeService as AllowanceTypeGrpcService;
pub use hris_proto::allowance_type_service_server::AllowanceTypeServiceServer;
use hris_proto::{GetAllowanceTypeRequest, CreateAllowanceTypeRequest, AllowanceTypeResponse};

pub struct AllowanceTypeGrpcHandler {
    service: Arc<AllowanceTypeService>,
}

impl AllowanceTypeGrpcHandler {
    pub fn new(service: Arc<AllowanceTypeService>) -> Self {
        Self { service }
    }
}

#[tonic::async_trait]
impl AllowanceTypeGrpcService for AllowanceTypeGrpcHandler {
    async fn get_allowance_type(
        &self,
        request: Request<GetAllowanceTypeRequest>,
    ) -> Result<Response<AllowanceTypeResponse>, Status> {
        let id = Uuid::parse_str(&request.into_inner().id)
            .map_err(|_| Status::invalid_argument("Invalid UUID"))?;

        let allowance_type = self.service.get_allowance_type_by_id(id).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(AllowanceTypeResponse {
            id: allowance_type.id.to_string(),
            code: allowance_type.code,
            name: allowance_type.name,
            taxable: allowance_type.taxable,
        }))
    }

    async fn create_allowance_type(
        &self,
        request: Request<CreateAllowanceTypeRequest>,
    ) -> Result<Response<AllowanceTypeResponse>, Status> {
        let req = request.into_inner();
        let dto = DtoCreateRequest {
            code: if req.code.is_empty() { None } else { Some(req.code) },
            name: req.name,
            taxable: req.taxable,
        };

        let allowance_type = self.service.create_allowance_type(dto).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(AllowanceTypeResponse {
            id: allowance_type.id.to_string(),
            code: allowance_type.code,
            name: allowance_type.name,
            taxable: allowance_type.taxable,
        }))
    }
}
