use tonic::{Request, Response, Status};
use uuid::Uuid;
use std::sync::Arc;
use crate::application::branch::service::BranchService;
use crate::application::branch::dto::{
    CreateBranchRequest as DtoCreateRequest, UpdateBranchRequest as DtoUpdateRequest,
};

pub mod hris_proto {
    tonic::include_proto!("hris");
}

use hris_proto::branch_service_server::BranchService as BranchGrpcService;
pub use hris_proto::branch_service_server::BranchServiceServer;
use hris_proto::{
    GetBranchRequest, CreateBranchRequest, BranchResponse,
    UpdateBranchRequest, DeleteBranchRequest, ListBranchesRequest, ListBranchesResponse, Empty
};
use crate::domain::shared::dtos::PaginationQuery;

pub struct BranchGrpcHandler {
    service: Arc<BranchService>,
}

impl BranchGrpcHandler {
    pub fn new(service: Arc<BranchService>) -> Self {
        Self { service }
    }
}

fn map_branch_to_response(branch: crate::domain::branch::entity::Model) -> BranchResponse {
    BranchResponse {
        id: branch.id.to_string(),
        legal_entity_id: branch.legal_entity_id.to_string(),
        code: branch.code,
        name: branch.name,
        timezone: branch.timezone,
        address: branch.address,
        created_at: branch.created_at.to_rfc3339(),
    }
}

#[tonic::async_trait]
impl BranchGrpcService for BranchGrpcHandler {
    async fn get_branch(
        &self,
        request: Request<GetBranchRequest>,
    ) -> Result<Response<BranchResponse>, Status> {
        let id = Uuid::parse_str(&request.into_inner().id)
            .map_err(|_| Status::invalid_argument("Invalid UUID"))?;

        let branch = self.service.get_branch_by_id(id).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(map_branch_to_response(branch)))
    }

    async fn create_branch(
        &self,
        request: Request<CreateBranchRequest>,
    ) -> Result<Response<BranchResponse>, Status> {
        let req = request.into_inner();
        let legal_entity_id = Uuid::parse_str(&req.legal_entity_id)
            .map_err(|_| Status::invalid_argument("Invalid legal_entity_id UUID"))?;

        let dto = DtoCreateRequest {
            legal_entity_id,
            code: req.code.and_then(|c| if c.is_empty() { None } else { Some(c) }),
            name: req.name,
            timezone: req.timezone,
            address: req.address,
        };

        let branch = self.service.create_branch(dto).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(map_branch_to_response(branch)))
    }

    async fn update_branch(
        &self,
        request: Request<UpdateBranchRequest>,
    ) -> Result<Response<BranchResponse>, Status> {
        let req = request.into_inner();
        let id = Uuid::parse_str(&req.id)
            .map_err(|_| Status::invalid_argument("Invalid UUID"))?;

        let dto = DtoUpdateRequest {
            name: req.name,
            timezone: req.timezone,
            address: req.address,
        };

        let branch = self.service.update_branch(id, dto).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(map_branch_to_response(branch)))
    }   

    async fn delete_branch(
        &self,
        request: Request<DeleteBranchRequest>,
    ) -> Result<Response<Empty>, Status> {
        let id = Uuid::parse_str(&request.into_inner().id)
            .map_err(|_| Status::invalid_argument("Invalid UUID"))?;

        self.service.delete_branch(id).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(Empty {}))
    }

    async fn list_branches(
        &self,
        request: Request<ListBranchesRequest>,
    ) -> Result<Response<ListBranchesResponse>, Status> {
        let req = request.into_inner();
        let query = PaginationQuery {
            page: Some(req.page as u64),
            limit: Some(req.per_page as u64),
            search: None,
        };

        let paginated = self.service.get_all_branches(query).await
            .map_err(|e| Status::internal(e.to_string()))?;

        let items = paginated.data.into_iter().map(map_branch_to_response).collect();

        Ok(Response::new(ListBranchesResponse {
            items,
            total_items: paginated.total as u32,
            total_pages: paginated.total_pages as u32,
            current_page: paginated.page as u32,
            per_page: paginated.limit as u32,
        }))
    }
}
