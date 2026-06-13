use tonic::{Request, Response, Status};
use uuid::Uuid;
use std::sync::Arc;
use crate::application::salary_structure_item::service::SalaryStructureItemService;
use crate::application::salary_structure_item::dto::{
    CreateSalaryStructureItemRequest as DtoCreateRequest, UpdateSalaryStructureItemRequest as DtoUpdateRequest,
};
use crate::domain::shared::dtos::PaginationQuery;

pub mod hris_proto {
    tonic::include_proto!("hris");
}

use hris_proto::salary_structure_item_service_server::SalaryStructureItemService as SalaryStructureItemGrpcService;
pub use hris_proto::salary_structure_item_service_server::SalaryStructureItemServiceServer;
use hris_proto::{
    GetSalaryStructureItemRequest, CreateSalaryStructureItemRequest, SalaryStructureItemResponse,
    UpdateSalaryStructureItemRequest, DeleteSalaryStructureItemRequest, ListSalaryStructureItemsRequest, ListSalaryStructureItemsResponse, Empty
};

pub struct SalaryStructureItemGrpcHandler {
    service: Arc<SalaryStructureItemService>,
}

impl SalaryStructureItemGrpcHandler {
    pub fn new(service: Arc<SalaryStructureItemService>) -> Self {
        Self { service }
    }
}

fn map_salary_structure_item_to_response(item: crate::domain::salary_structure_item::entity::Model) -> SalaryStructureItemResponse {
    SalaryStructureItemResponse {
        id: item.id.to_string(),
        salary_structure_id: item.salary_structure_id.to_string(),
        code: item.code,
        name: item.name,
        item_type: item.item_type,
        calculation_type: item.calculation_type,
        taxable: item.taxable,
        sso_calculatable: item.sso_calculatable,
        sequence: item.sequence,
    }
}

#[tonic::async_trait]
impl SalaryStructureItemGrpcService for SalaryStructureItemGrpcHandler {
    async fn get_salary_structure_item(
        &self,
        request: Request<GetSalaryStructureItemRequest>,
    ) -> Result<Response<SalaryStructureItemResponse>, Status> {
        let id = Uuid::parse_str(&request.into_inner().id)
            .map_err(|_| Status::invalid_argument("Invalid UUID"))?;

        let item = self.service.get_item_by_id(id).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(map_salary_structure_item_to_response(item)))
    }

    async fn create_salary_structure_item(
        &self,
        request: Request<CreateSalaryStructureItemRequest>,
    ) -> Result<Response<SalaryStructureItemResponse>, Status> {
        let req = request.into_inner();
        let dto = DtoCreateRequest {
            salary_structure_id: Uuid::parse_str(&req.salary_structure_id).map_err(|_| Status::invalid_argument("Invalid salary_structure_id"))?,
            code: if req.code.is_empty() { None } else { Some(req.code) },
            name: req.name,
            item_type: req.item_type,
            calculation_type: req.calculation_type,
            taxable: req.taxable,
            sso_calculatable: req.sso_calculatable,
            sequence: req.sequence,
        };

        let item = self.service.create_item(dto).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(map_salary_structure_item_to_response(item)))
    }

    async fn update_salary_structure_item(
        &self,
        request: Request<UpdateSalaryStructureItemRequest>,
    ) -> Result<Response<SalaryStructureItemResponse>, Status> {
        let req = request.into_inner();
        let id = Uuid::parse_str(&req.id)
            .map_err(|_| Status::invalid_argument("Invalid UUID"))?;

        let dto = DtoUpdateRequest {
            code: None,
            name: req.name,
            item_type: req.item_type,
            calculation_type: req.calculation_type,
            taxable: req.taxable,
            sso_calculatable: req.sso_calculatable,
            sequence: req.sequence.and_then(|v| if v == 0 { None } else { Some(v) }),
        };

        let item = self.service.update_item(id, dto).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(map_salary_structure_item_to_response(item)))
    }

    async fn delete_salary_structure_item(
        &self,
        request: Request<DeleteSalaryStructureItemRequest>,
    ) -> Result<Response<Empty>, Status> {
        let id = Uuid::parse_str(&request.into_inner().id)
            .map_err(|_| Status::invalid_argument("Invalid UUID"))?;

        self.service.delete_item(id).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(Empty {}))
    }

    async fn list_salary_structure_items(
        &self,
        request: Request<ListSalaryStructureItemsRequest>,
    ) -> Result<Response<ListSalaryStructureItemsResponse>, Status> {
        let req = request.into_inner();
        let query = PaginationQuery {
            page: Some(req.page as u64),
            limit: Some(req.per_page as u64),
            search: req.salary_structure_id, // Filter by salary_structure_id via search
        };

        let response = self.service.get_all_items(query).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(ListSalaryStructureItemsResponse {
            items: response.data.into_iter().map(map_salary_structure_item_to_response).collect(),
            total_items: response.total as u32,
            total_pages: response.total_pages as u32,
            current_page: response.page as u32,
            per_page: response.limit as u32,
        }))
    }
}
