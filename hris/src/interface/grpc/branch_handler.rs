use tonic::{Request, Response, Status};
use uuid::Uuid;
use std::sync::Arc;
use crate::application::branch::service::BranchService;
use crate::application::branch::dto::CreateBranchRequest as DtoCreateRequest;

pub mod hris_proto {
    tonic::include_proto!("hris");
}

use hris_proto::branch_service_server::BranchService as BranchGrpcService;
pub use hris_proto::branch_service_server::BranchServiceServer;
use hris_proto::{GetBranchRequest, CreateBranchRequest, BranchResponse};

pub struct BranchGrpcHandler {
    service: Arc<BranchService>,
}

impl BranchGrpcHandler {
    pub fn new(service: Arc<BranchService>) -> Self {
        Self { service }
    }
}

#[tonic::async_trait]
impl BranchGrpcService for BranchGrpcHandler {
    async fn get_branch(
        &self,
        request: Request<GetBranchRequest>,
    ) -> Result<Response<BranchResponse>, Status> {
        let ctx = crate::interface::grpc::metadata::extract_tenant_context(&request)?;
        let id = Uuid::parse_str(&request.into_inner().id)
            .map_err(|_| Status::invalid_argument("Invalid UUID"))?;

        let branch = self.service.get_branch_by_id(&ctx, id).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(BranchResponse {
            id: branch.id.to_string(),
            company_id: branch.company_id.to_string(),
            code: branch.code,
            name: branch.name,
            timezone: branch.timezone,
            address: branch.address,
        }))
    }

    async fn create_branch(
        &self,
        request: Request<CreateBranchRequest>,
    ) -> Result<Response<BranchResponse>, Status> {
        let ctx = crate::interface::grpc::metadata::extract_tenant_context(&request)?;
        let req = request.into_inner();
        let company_id = Uuid::parse_str(&req.company_id)
            .map_err(|_| Status::invalid_argument("Invalid company_id UUID"))?;

        let dto = DtoCreateRequest {
            company_id,
            code: if req.code.is_empty() { None } else { Some(req.code) },
            name: req.name,
            timezone: req.timezone,
            address: req.address,
        };

        let branch = self.service.create_branch(&ctx, dto).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(BranchResponse {
            id: branch.id.to_string(),
            company_id: branch.company_id.to_string(),
            code: branch.code,
            name: branch.name,
            timezone: branch.timezone,
            address: branch.address,
        }))
    }
}
