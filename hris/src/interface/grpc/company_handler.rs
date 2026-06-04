use tonic::{Request, Response, Status};
use uuid::Uuid;
use std::sync::Arc;
use crate::application::company::service::CompanyService;
use crate::application::company::dto::CreateCompanyRequest as DtoCreateRequest;

pub mod hris_proto {
    tonic::include_proto!("hris");
}

use hris_proto::company_service_server::CompanyService as CompanyGrpcService;
pub use hris_proto::company_service_server::CompanyServiceServer;
use hris_proto::{GetCompanyRequest, CreateCompanyRequest, CompanyResponse};

pub struct CompanyGrpcHandler {
    service: Arc<CompanyService>,
}

impl CompanyGrpcHandler {
    pub fn new(service: Arc<CompanyService>) -> Self {
        Self { service }
    }
}

#[tonic::async_trait]
impl CompanyGrpcService for CompanyGrpcHandler {
    async fn get_company(
        &self,
        request: Request<GetCompanyRequest>,
    ) -> Result<Response<CompanyResponse>, Status> {
        let ctx = crate::interface::grpc::metadata::extract_tenant_context(&request)?;
        let id = Uuid::parse_str(&request.into_inner().id)
            .map_err(|_| Status::invalid_argument("Invalid UUID"))?;

        let company = self.service.get_company_by_id(&ctx, id).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(CompanyResponse {
            id: company.id.to_string(),
            code: company.code,
            tax_id: company.tax_id,
            name_th: company.name_th,
            name_en: company.name_en,
            address: company.address,
            phone: company.phone,
            email: company.email,
            is_active: company.is_active,
        }))
    }

    async fn create_company(
        &self,
        request: Request<CreateCompanyRequest>,
    ) -> Result<Response<CompanyResponse>, Status> {
        let ctx = crate::interface::grpc::metadata::extract_tenant_context(&request)?;
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

        let company = self.service.create_company(&ctx, dto).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(CompanyResponse {
            id: company.id.to_string(),
            code: company.code,
            tax_id: company.tax_id,
            name_th: company.name_th,
            name_en: company.name_en,
            address: company.address,
            phone: company.phone,
            email: company.email,
            is_active: company.is_active,
        }))
    }
}
