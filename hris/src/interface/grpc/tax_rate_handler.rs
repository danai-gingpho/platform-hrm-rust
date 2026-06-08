use tonic::{Request, Response, Status};
use uuid::Uuid;
use std::sync::Arc;
use crate::application::tax_rate::service::TaxRateService;
use crate::application::tax_rate::dto::CreateTaxRateRequest as DtoCreateRequest;

pub mod hris_proto {
    tonic::include_proto!("hris");
}

use hris_proto::tax_rate_service_server::TaxRateService as TaxRateGrpcService;
pub use hris_proto::tax_rate_service_server::TaxRateServiceServer;
use hris_proto::{GetTaxRateRequest, CreateTaxRateRequest, TaxRateResponse};

pub struct TaxRateGrpcHandler {
    service: Arc<TaxRateService>,
}

impl TaxRateGrpcHandler {
    pub fn new(service: Arc<TaxRateService>) -> Self {
        Self { service }
    }
}

#[tonic::async_trait]
impl TaxRateGrpcService for TaxRateGrpcHandler {
    async fn get_tax_rate(
        &self,
        request: Request<GetTaxRateRequest>,
    ) -> Result<Response<TaxRateResponse>, Status> {
        let id = Uuid::parse_str(&request.into_inner().id)
            .map_err(|_| Status::invalid_argument("Invalid UUID"))?;

        let rate = self.service.get_rate_by_id(id).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(TaxRateResponse {
            id: rate.id.to_string(),
            year: rate.year,
            min_income: rate.min_income.to_string(),
            max_income: rate.max_income.to_string(),
            tax_percent: rate.tax_percent.to_string(),
        }))
    }

    async fn create_tax_rate(
        &self,
        request: Request<CreateTaxRateRequest>,
    ) -> Result<Response<TaxRateResponse>, Status> {
        let req = request.into_inner();
        let dto = DtoCreateRequest {
            year: req.year,
            min_income: req.min_income.parse().map_err(|_| Status::invalid_argument("Invalid min_income"))?,
            max_income: req.max_income.parse().map_err(|_| Status::invalid_argument("Invalid max_income"))?,
            tax_percent: req.tax_percent.parse().map_err(|_| Status::invalid_argument("Invalid tax_percent"))?,
        };

        let rate = self.service.create_rate(dto).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(TaxRateResponse {
            id: rate.id.to_string(),
            year: rate.year,
            min_income: rate.min_income.to_string(),
            max_income: rate.max_income.to_string(),
            tax_percent: rate.tax_percent.to_string(),
        }))
    }
}
