use tonic::{Request, Response, Status};
use uuid::Uuid;
use std::sync::Arc;
use crate::application::allowance_type::service::AllowanceTypeService;
use crate::application::allowance_type::dto::CreateAllowanceTypeRequest as DtoCreateRequest;
use crate::application::allowance_type::dto::UpdateAllowanceTypeRequest as DtoUpdateRequest;
use crate::domain::shared::dtos::PaginationQuery;

pub mod hris_proto {
    tonic::include_proto!("hris");
}

use hris_proto::allowance_type_service_server::AllowanceTypeService as AllowanceTypeGrpcService;
pub use hris_proto::allowance_type_service_server::AllowanceTypeServiceServer;
use hris_proto::{
    GetAllowanceTypeRequest, CreateAllowanceTypeRequest, AllowanceTypeResponse,
    UpdateAllowanceTypeRequest, DeleteAllowanceTypeRequest, ListAllowanceTypesRequest,
    ListAllowanceTypesResponse, Empty
};

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

    async fn update_allowance_type(
        &self,
        request: Request<UpdateAllowanceTypeRequest>,
    ) -> Result<Response<AllowanceTypeResponse>, Status> {
        let req = request.into_inner();
        let id = Uuid::parse_str(&req.id)
            .map_err(|_| Status::invalid_argument("Invalid UUID"))?;

        let dto = DtoUpdateRequest {
            name: if req.name.is_empty() { None } else { Some(req.name) },
            taxable: req.taxable,
        };

        let allowance_type = self.service.update_allowance_type(id, dto).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(AllowanceTypeResponse {
            id: allowance_type.id.to_string(),
            code: allowance_type.code,
            name: allowance_type.name,
            taxable: allowance_type.taxable,
        }))
    }

    async fn delete_allowance_type(
        &self,
        request: Request<DeleteAllowanceTypeRequest>,
    ) -> Result<Response<Empty>, Status> {
        let id = Uuid::parse_str(&request.into_inner().id)
            .map_err(|_| Status::invalid_argument("Invalid UUID"))?;

        self.service.delete_allowance_type(id).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(Empty {}))   
    }

    async fn list_allowance_types(
        &self,
        request: Request<ListAllowanceTypesRequest>,
    ) -> Result<Response<ListAllowanceTypesResponse>, Status> {
        let req = request.into_inner();
        let query = PaginationQuery {
            page: Some(req.page as u64),
            limit: Some(req.per_page as u64),
            search: None,
        };

        let paginated = self.service.get_all_allowance_types(query).await
            .map_err(|e| Status::internal(e.to_string()))?;

        let items = paginated.data.into_iter().map(|at| AllowanceTypeResponse {
            id: at.id.to_string(),
            code: at.code,
            name: at.name,
            taxable: at.taxable,
        }).collect();

        Ok(Response::new(ListAllowanceTypesResponse {
            items,
            total_items: paginated.total as u32,
            total_pages: paginated.total_pages as u32,
            current_page: paginated.page as u32,
            per_page: paginated.limit as u32,
        }))
    }
}
