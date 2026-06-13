use tonic::{Request, Response, Status};
use uuid::Uuid;
use std::sync::Arc;
use crate::application::tax_rate::service::TaxRateService;
use crate::application::tax_rate::dto::{
    CreateTaxRateRequest as DtoCreateRequest, UpdateTaxRateRequest as DtoUpdateRequest,
};
use crate::domain::shared::dtos::PaginationQuery;
use rust_decimal::Decimal;
use std::str::FromStr;

pub mod hris_proto {
    tonic::include_proto!("hris");
}

use hris_proto::tax_rate_service_server::TaxRateService as TaxRateGrpcService;
pub use hris_proto::tax_rate_service_server::TaxRateServiceServer;
use hris_proto::{
    GetTaxRateRequest, CreateTaxRateRequest, TaxRateResponse,
    UpdateTaxRateRequest, DeleteTaxRateRequest, ListTaxRatesRequest, ListTaxRatesResponse, Empty
};

pub struct TaxRateGrpcHandler {
    service: Arc<TaxRateService>,
}

impl TaxRateGrpcHandler {
    pub fn new(service: Arc<TaxRateService>) -> Self {
        Self { service }
    }
}

fn map_tax_rate_to_response(rate: crate::domain::tax_rate::entity::Model) -> TaxRateResponse {
    TaxRateResponse {
        id: rate.id.to_string(),
        year: rate.year,
        min_income: rate.min_income.to_string(),
        max_income: rate.max_income.to_string(),
        tax_percent: rate.tax_percent.to_string(),
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

        Ok(Response::new(map_tax_rate_to_response(rate)))
    }

    async fn create_tax_rate(
        &self,
        request: Request<CreateTaxRateRequest>,
    ) -> Result<Response<TaxRateResponse>, Status> {
        let req = request.into_inner();
        let dto = DtoCreateRequest {
            year: req.year,
            min_income: Decimal::from_str(&req.min_income).map_err(|_| Status::invalid_argument("Invalid min_income"))?,
            max_income: Decimal::from_str(&req.max_income).map_err(|_| Status::invalid_argument("Invalid max_income"))?,
            tax_percent: Decimal::from_str(&req.tax_percent).map_err(|_| Status::invalid_argument("Invalid tax_percent"))?,
        };

        let rate = self.service.create_rate(dto).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(map_tax_rate_to_response(rate)))
    }

    async fn update_tax_rate(
        &self,
        request: Request<UpdateTaxRateRequest>,
    ) -> Result<Response<TaxRateResponse>, Status> {
        let req = request.into_inner();
        let id = Uuid::parse_str(&req.id)
            .map_err(|_| Status::invalid_argument("Invalid UUID"))?;

        let min_income = if let Some(ref val) = req.min_income {
            Some(Decimal::from_str(val).map_err(|_| Status::invalid_argument("Invalid min_income"))?)
        } else {
            None
        };

        let max_income = if let Some(ref val) = req.max_income {
            Some(Decimal::from_str(val).map_err(|_| Status::invalid_argument("Invalid max_income"))?)
        } else {
            None
        };

        let tax_percent = if let Some(ref val) = req.tax_percent {
            Some(Decimal::from_str(val).map_err(|_| Status::invalid_argument("Invalid tax_percent"))?)
        } else {
            None
        };

        let dto = DtoUpdateRequest {
            year: req.year.and_then(|v| if v == 0 { None } else { Some(v) }),
            min_income,
            max_income,
            tax_percent,
        };

        let rate = self.service.update_rate(id, dto).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(map_tax_rate_to_response(rate)))
    }

    async fn delete_tax_rate(
        &self,
        request: Request<DeleteTaxRateRequest>,
    ) -> Result<Response<Empty>, Status> {
        let id = Uuid::parse_str(&request.into_inner().id)
            .map_err(|_| Status::invalid_argument("Invalid UUID"))?;

        self.service.delete_rate(id).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(Empty {}))
    }

    async fn list_tax_rates(
        &self,
        request: Request<ListTaxRatesRequest>,
    ) -> Result<Response<ListTaxRatesResponse>, Status> {
        let req = request.into_inner();
        let query = PaginationQuery {
            page: Some(req.page as u64),
            limit: Some(req.per_page as u64),
            search: None,
        };

        let response = self.service.get_all_rates(query).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(ListTaxRatesResponse {
            items: response.data.into_iter().map(map_tax_rate_to_response).collect(),
            total_items: response.total as u32,
            total_pages: response.total_pages as u32,
            current_page: response.page as u32,
            per_page: response.limit as u32,
        }))
    }
}
