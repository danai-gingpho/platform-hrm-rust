use tonic::{Request, Response, Status};
use uuid::Uuid;
use std::sync::Arc;
use crate::application::legal_entity::service::LegalEntityService;
use crate::application::legal_entity::dto::CreateLegalEntityRequest as DtoCreateRequest;

pub mod hris_proto {
    tonic::include_proto!("hris");
}

use hris_proto::legal_entity_service_server::LegalEntityService as LegalEntityGrpcService;
pub use hris_proto::legal_entity_service_server::LegalEntityServiceServer;
use hris_proto::{GetLegalEntityRequest, CreateLegalEntityRequest, LegalEntityResponse};

pub struct LegalEntityGrpcHandler {
    service: Arc<LegalEntityService>,
}

impl LegalEntityGrpcHandler {
    pub fn new(service: Arc<LegalEntityService>) -> Self {
        Self { service }
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

        Ok(Response::new(LegalEntityResponse {
            id: legal_entity.id.to_string(),
            code: legal_entity.code,
            tax_id: legal_entity.tax_id,
            name_th: legal_entity.name_th,
            name_en: legal_entity.name_en,
            address: legal_entity.address,
            phone: legal_entity.phone,
            email: legal_entity.email,
            is_active: legal_entity.is_active,
        }))
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

        Ok(Response::new(LegalEntityResponse {
            id: legal_entity.id.to_string(),
            code: legal_entity.code,
            tax_id: legal_entity.tax_id,
            name_th: legal_entity.name_th,
            name_en: legal_entity.name_en,
            address: legal_entity.address,
            phone: legal_entity.phone,
            email: legal_entity.email,
            is_active: legal_entity.is_active,
        }))
    }
}
