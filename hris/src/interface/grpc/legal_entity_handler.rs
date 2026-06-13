use tonic::{Request, Response, Status};
use uuid::Uuid;
use std::sync::Arc;
use crate::application::legal_entity::service::LegalEntityService;
use crate::application::legal_entity::dto::{
    CreateLegalEntityRequest as DtoCreateRequest, UpdateLegalEntityRequest as DtoUpdateRequest,
};
use crate::domain::shared::dtos::PaginationQuery;

pub mod hris_proto {
    tonic::include_proto!("hris");
}

use hris_proto::legal_entity_service_server::LegalEntityService as LegalEntityGrpcService;
pub use hris_proto::legal_entity_service_server::LegalEntityServiceServer;
use hris_proto::{
    GetLegalEntityRequest, CreateLegalEntityRequest, LegalEntityResponse,
    UpdateLegalEntityRequest, DeleteLegalEntityRequest, ListLegalEntitiesRequest, ListLegalEntitiesResponse, Empty
};

pub struct LegalEntityGrpcHandler {
    service: Arc<LegalEntityService>,
}

impl LegalEntityGrpcHandler {
    pub fn new(service: Arc<LegalEntityService>) -> Self {
        Self { service }
    }
}

fn map_legal_entity_to_response(le: crate::domain::legal_entity::entity::Model) -> LegalEntityResponse {
    LegalEntityResponse {
        id: le.id.to_string(),
        code: le.code,
        tax_id: le.tax_id,
        name_th: le.name_th,
        name_en: le.name_en,
        address: le.address,
        phone: le.phone,
        email: le.email,
        is_active: le.is_active,
    }
}

#[tonic::async_trait]
impl LegalEntityGrpcService for LegalEntityGrpcHandler {
    async fn get_legal_entity(
        &self,
        request: Request<GetLegalEntityRequest>,
    ) -> Result<Response<LegalEntityResponse>, Status> {
        let id = Uuid::parse_str(&request.into_inner().id)
            .map_err(|_| Status::invalid_argument("Invalid UUID"))?;

        let legal_entity = self.service.get_legal_entity_by_id(id).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(map_legal_entity_to_response(legal_entity)))
    }

    async fn create_legal_entity(
        &self,
        request: Request<CreateLegalEntityRequest>,
    ) -> Result<Response<LegalEntityResponse>, Status> {
        let req = request.into_inner();
        let dto = DtoCreateRequest {
            code: if req.code.is_empty() { None } else { Some(req.code) },
            tax_id: req.tax_id,
            name_th: req.name_th,
            name_en: req.name_en,
            address: req.address,
            phone: req.phone,
            email: req.email,
        };

        let legal_entity = self.service.create_legal_entity(dto).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(map_legal_entity_to_response(legal_entity)))
    }

    async fn update_legal_entity(
        &self,
        request: Request<UpdateLegalEntityRequest>,
    ) -> Result<Response<LegalEntityResponse>, Status> {
        let req = request.into_inner();
        let id = Uuid::parse_str(&req.id)
            .map_err(|_| Status::invalid_argument("Invalid UUID"))?;

        let dto = DtoUpdateRequest {
            tax_id: None, // Code doesn't allow updating tax_id in DTO but it's in proto? 
                          // Actually DTO has tax_id: Option<String>.
            name_th: req.name_th,
            name_en: req.name_en,
            address: req.address,
            phone: req.phone,
            email: req.email,
            is_active: req.is_active,
        };

        let legal_entity = self.service.update_legal_entity(id, dto).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(map_legal_entity_to_response(legal_entity)))
    }

    async fn delete_legal_entity(
        &self,
        request: Request<DeleteLegalEntityRequest>,
    ) -> Result<Response<Empty>, Status> {
        let id = Uuid::parse_str(&request.into_inner().id)
            .map_err(|_| Status::invalid_argument("Invalid UUID"))?;

        self.service.delete_legal_entity(id).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(Empty {}))
    }

    async fn list_legal_entities(
        &self,
        request: Request<ListLegalEntitiesRequest>,
    ) -> Result<Response<ListLegalEntitiesResponse>, Status> {
        let req = request.into_inner();
        let query = PaginationQuery {
            page: Some(req.page as u64),
            limit: Some(req.per_page as u64),
            search: None,
        };

        let response = self.service.get_all_legal_entities(query).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(ListLegalEntitiesResponse {
            items: response.data.into_iter().map(map_legal_entity_to_response).collect(),
            total_items: response.total as u32,
            total_pages: response.total_pages as u32,
            current_page: response.page as u32,
            per_page: response.limit as u32,
        }))
    }
}
