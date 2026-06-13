use tonic::{Request, Response, Status};
use uuid::Uuid;
use std::sync::Arc;
use crate::application::salary_structure::service::SalaryStructureService;
use crate::application::salary_structure::dto::{
    CreateSalaryStructureRequest as DtoCreateRequest, UpdateSalaryStructureRequest as DtoUpdateRequest,
};
use crate::domain::shared::dtos::PaginationQuery;

pub mod hris_proto {
    tonic::include_proto!("hris");
}

use hris_proto::salary_structure_service_server::SalaryStructureService as SalaryStructureGrpcService;
pub use hris_proto::salary_structure_service_server::SalaryStructureServiceServer;
use hris_proto::{
    GetSalaryStructureRequest, CreateSalaryStructureRequest, SalaryStructureResponse,
    UpdateSalaryStructureRequest, DeleteSalaryStructureRequest, ListSalaryStructuresRequest, ListSalaryStructuresResponse, Empty
};

pub struct SalaryStructureGrpcHandler {
    service: Arc<SalaryStructureService>,
}

impl SalaryStructureGrpcHandler {
    pub fn new(service: Arc<SalaryStructureService>) -> Self {
        Self { service }
    }
}

fn map_salary_structure_to_response(structure: crate::domain::salary_structure::entity::Model) -> SalaryStructureResponse {
    SalaryStructureResponse {
        id: structure.id.to_string(),
        code: structure.code,
        name: structure.name,
        legal_entity_id: structure.legal_entity_id.to_string(),
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

        Ok(Response::new(map_salary_structure_to_response(structure)))
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

        Ok(Response::new(map_salary_structure_to_response(structure)))
    }

    async fn update_salary_structure(
        &self,
        request: Request<UpdateSalaryStructureRequest>,
    ) -> Result<Response<SalaryStructureResponse>, Status> {
        let req = request.into_inner();
        let id = Uuid::parse_str(&req.id)
            .map_err(|_| Status::invalid_argument("Invalid UUID"))?;

        let dto = DtoUpdateRequest {
            name: req.name,
            legal_entity_id: None,
        };

        let structure = self.service.update_salary_structure(id, dto).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(map_salary_structure_to_response(structure)))
    }

    async fn delete_salary_structure(
        &self,
        request: Request<DeleteSalaryStructureRequest>,
    ) -> Result<Response<Empty>, Status> {
        let id = Uuid::parse_str(&request.into_inner().id)
            .map_err(|_| Status::invalid_argument("Invalid UUID"))?;

        self.service.delete_salary_structure(id).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(Empty {}))
    }

    async fn list_salary_structures(
        &self,
        request: Request<ListSalaryStructuresRequest>,
    ) -> Result<Response<ListSalaryStructuresResponse>, Status> {
        let req = request.into_inner();
        let query = PaginationQuery {
            page: Some(req.page as u64),
            limit: Some(req.per_page as u64),
            search: req.legal_entity_id, // Filter by legal_entity_id via search
        };

        let response = self.service.get_all_salary_structures(query).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(ListSalaryStructuresResponse {
            items: response.data.into_iter().map(map_salary_structure_to_response).collect(),
            total_items: response.total as u32,
            total_pages: response.total_pages as u32,
            current_page: response.page as u32,
            per_page: response.limit as u32,
        }))
    }
}
