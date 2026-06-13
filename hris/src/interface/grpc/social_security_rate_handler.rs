use tonic::{Request, Response, Status};
use uuid::Uuid;
use std::sync::Arc;
use crate::application::social_security_rate::service::SocialSecurityRateService;
use crate::application::social_security_rate::dto::{
    CreateSocialSecurityRateRequest as DtoCreateRequest, UpdateSocialSecurityRateRequest as DtoUpdateRequest,
};
use crate::domain::shared::dtos::PaginationQuery;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use std::str::FromStr;

pub mod hris_proto {
    tonic::include_proto!("hris");
}

use hris_proto::social_security_rate_service_server::SocialSecurityRateService as SocialSecurityRateGrpcService;
pub use hris_proto::social_security_rate_service_server::SocialSecurityRateServiceServer;
use hris_proto::{
    GetSocialSecurityRateRequest, CreateSocialSecurityRateRequest, SocialSecurityRateResponse,
    UpdateSocialSecurityRateRequest, DeleteSocialSecurityRateRequest, ListSocialSecurityRatesRequest, ListSocialSecurityRatesResponse, Empty
};

pub struct SocialSecurityRateGrpcHandler {
    service: Arc<SocialSecurityRateService>,
}

impl SocialSecurityRateGrpcHandler {
    pub fn new(service: Arc<SocialSecurityRateService>) -> Self {
        Self { service }
    }
}

fn map_social_security_rate_to_response(rate: crate::domain::social_security_rate::entity::Model) -> SocialSecurityRateResponse {
    SocialSecurityRateResponse {
        id: rate.id.to_string(),
        effective_date: rate.effective_date.to_string(),
        employee_percent: rate.employee_percent.to_string(),
        employer_percent: rate.employer_percent.to_string(),
        max_salary: rate.max_salary.to_string(),
    }
}

#[tonic::async_trait]
impl SocialSecurityRateGrpcService for SocialSecurityRateGrpcHandler {
    async fn get_social_security_rate(
        &self,
        request: Request<GetSocialSecurityRateRequest>,
    ) -> Result<Response<SocialSecurityRateResponse>, Status> {
        let id = Uuid::parse_str(&request.into_inner().id)
            .map_err(|_| Status::invalid_argument("Invalid UUID"))?;

        let rate = self.service.get_rate_by_id(id).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(map_social_security_rate_to_response(rate)))
    }

    async fn create_social_security_rate(
        &self,
        request: Request<CreateSocialSecurityRateRequest>,
    ) -> Result<Response<SocialSecurityRateResponse>, Status> {
        let req = request.into_inner();
        let dto = DtoCreateRequest {
            effective_date: NaiveDate::parse_from_str(&req.effective_date, "%Y-%m-%d").map_err(|_| Status::invalid_argument("Invalid effective_date"))?,
            employee_percent: Decimal::from_str(&req.employee_percent).map_err(|_| Status::invalid_argument("Invalid employee_percent"))?,
            employer_percent: Decimal::from_str(&req.employer_percent).map_err(|_| Status::invalid_argument("Invalid employer_percent"))?,
            max_salary: Decimal::from_str(&req.max_salary).map_err(|_| Status::invalid_argument("Invalid max_salary"))?,
        };

        let rate = self.service.create_rate(dto).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(map_social_security_rate_to_response(rate)))
    }

    async fn update_social_security_rate(
        &self,
        request: Request<UpdateSocialSecurityRateRequest>,
    ) -> Result<Response<SocialSecurityRateResponse>, Status> {
        let req = request.into_inner();
        let id = Uuid::parse_str(&req.id)
            .map_err(|_| Status::invalid_argument("Invalid UUID"))?;

        let effective_date = if let Some(ref val) = req.effective_date {
            Some(NaiveDate::parse_from_str(val, "%Y-%m-%d").map_err(|_| Status::invalid_argument("Invalid effective_date"))?)
        } else {
            None
        };

        let employee_percent = if let Some(ref val) = req.employee_percent {
            Some(Decimal::from_str(val).map_err(|_| Status::invalid_argument("Invalid employee_percent"))?)
        } else {
            None
        };

        let employer_percent = if let Some(ref val) = req.employer_percent {
            Some(Decimal::from_str(val).map_err(|_| Status::invalid_argument("Invalid employer_percent"))?)
        } else {
            None
        };

        let max_salary = if let Some(ref val) = req.max_salary {
            Some(Decimal::from_str(val).map_err(|_| Status::invalid_argument("Invalid max_salary"))?)
        } else {
            None
        };

        let dto = DtoUpdateRequest {
            effective_date,
            employee_percent,
            employer_percent,
            max_salary,
        };

        let rate = self.service.update_rate(id, dto).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(map_social_security_rate_to_response(rate)))
    }

    async fn delete_social_security_rate(
        &self,
        request: Request<DeleteSocialSecurityRateRequest>,
    ) -> Result<Response<Empty>, Status> {
        let id = Uuid::parse_str(&request.into_inner().id)
            .map_err(|_| Status::invalid_argument("Invalid UUID"))?;

        self.service.delete_rate(id).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(Empty {}))
    }

    async fn list_social_security_rates(
        &self,
        request: Request<ListSocialSecurityRatesRequest>,
    ) -> Result<Response<ListSocialSecurityRatesResponse>, Status> {
        let req = request.into_inner();
        let query = PaginationQuery {
            page: Some(req.page as u64),
            limit: Some(req.per_page as u64),
            search: None,
        };

        let response = self.service.get_all_rates(query).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(ListSocialSecurityRatesResponse {
            items: response.data.into_iter().map(map_social_security_rate_to_response).collect(),
            total_items: response.total as u32,
            total_pages: response.total_pages as u32,
            current_page: response.page as u32,
            per_page: response.limit as u32,
        }))
    }
}
