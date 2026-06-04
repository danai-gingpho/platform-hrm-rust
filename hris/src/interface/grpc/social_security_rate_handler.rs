use tonic::{Request, Response, Status};
use uuid::Uuid;
use std::sync::Arc;
use crate::application::social_security_rate::service::SocialSecurityRateService;
use crate::application::social_security_rate::dto::CreateSocialSecurityRateRequest as DtoCreateRequest;

pub mod hris_proto {
    tonic::include_proto!("hris");
}

use hris_proto::social_security_rate_service_server::SocialSecurityRateService as SocialSecurityRateGrpcService;
pub use hris_proto::social_security_rate_service_server::SocialSecurityRateServiceServer;
use hris_proto::{GetSocialSecurityRateRequest, CreateSocialSecurityRateRequest, SocialSecurityRateResponse};

pub struct SocialSecurityRateGrpcHandler {
    service: Arc<SocialSecurityRateService>,
}

impl SocialSecurityRateGrpcHandler {
    pub fn new(service: Arc<SocialSecurityRateService>) -> Self {
        Self { service }
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

        Ok(Response::new(SocialSecurityRateResponse {
            id: rate.id.to_string(),
            effective_date: rate.effective_date.to_string(),
            employee_percent: rate.employee_percent.to_string(),
            employer_percent: rate.employer_percent.to_string(),
            max_salary: rate.max_salary.to_string(),
        }))
    }

    async fn create_social_security_rate(
        &self,
        request: Request<CreateSocialSecurityRateRequest>,
    ) -> Result<Response<SocialSecurityRateResponse>, Status> {
        let req = request.into_inner();
        let dto = DtoCreateRequest {
            effective_date: chrono::NaiveDate::parse_from_str(&req.effective_date, "%Y-%m-%d").map_err(|_| Status::invalid_argument("Invalid effective_date"))?,
            employee_percent: req.employee_percent.parse().map_err(|_| Status::invalid_argument("Invalid employee_percent"))?,
            employer_percent: req.employer_percent.parse().map_err(|_| Status::invalid_argument("Invalid employer_percent"))?,
            max_salary: req.max_salary.parse().map_err(|_| Status::invalid_argument("Invalid max_salary"))?,
        };

        let rate = self.service.create_rate(dto).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(SocialSecurityRateResponse {
            id: rate.id.to_string(),
            effective_date: rate.effective_date.to_string(),
            employee_percent: rate.employee_percent.to_string(),
            employer_percent: rate.employer_percent.to_string(),
            max_salary: rate.max_salary.to_string(),
        }))
    }
}
