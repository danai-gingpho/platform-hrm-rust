use tonic::{Request, Response, Status};
use uuid::Uuid;
use std::sync::Arc;
use crate::application::payroll_period::service::PayrollPeriodService;
use crate::application::payroll_period::dto::CreatePayrollPeriodRequest as DtoCreateRequest;

pub mod hris_proto {
    tonic::include_proto!("hris");
}

use hris_proto::payroll_period_service_server::PayrollPeriodService as PayrollPeriodGrpcService;
pub use hris_proto::payroll_period_service_server::PayrollPeriodServiceServer;
use hris_proto::{GetPayrollPeriodRequest, CreatePayrollPeriodRequest, PayrollPeriodResponse};

pub struct PayrollPeriodGrpcHandler {
    service: Arc<PayrollPeriodService>,
}

impl PayrollPeriodGrpcHandler {
    pub fn new(service: Arc<PayrollPeriodService>) -> Self {
        Self { service }
    }
}

#[tonic::async_trait]
impl PayrollPeriodGrpcService for PayrollPeriodGrpcHandler {
    async fn get_payroll_period(
        &self,
        request: Request<GetPayrollPeriodRequest>,
    ) -> Result<Response<PayrollPeriodResponse>, Status> {
        let id = Uuid::parse_str(&request.into_inner().id)
            .map_err(|_| Status::invalid_argument("Invalid UUID"))?;

        let period = self.service.get_payroll_period_by_id(id).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(PayrollPeriodResponse {
            id: period.id.to_string(),
            legal_entity_id: period.legal_entity_id.to_string(),
            period_code: period.period_code,
            start_date: period.start_date.to_string(),
            end_date: period.end_date.to_string(),
            payment_date: period.payment_date.to_string(),
            status: period.status,
        }))
    }

    async fn create_payroll_period(
        &self,
        request: Request<CreatePayrollPeriodRequest>,
    ) -> Result<Response<PayrollPeriodResponse>, Status> {
        let req = request.into_inner();
        let dto = DtoCreateRequest {
            legal_entity_id: Uuid::parse_str(&req.legal_entity_id).map_err(|_| Status::invalid_argument("Invalid legal_entity_id"))?,
            period_code: if req.period_code.is_empty() { None } else { Some(req.period_code) },
            start_date: chrono::NaiveDate::parse_from_str(&req.start_date, "%Y-%m-%d").map_err(|_| Status::invalid_argument("Invalid start_date"))?,
            end_date: chrono::NaiveDate::parse_from_str(&req.end_date, "%Y-%m-%d").map_err(|_| Status::invalid_argument("Invalid end_date"))?,
            payment_date: chrono::NaiveDate::parse_from_str(&req.payment_date, "%Y-%m-%d").map_err(|_| Status::invalid_argument("Invalid payment_date"))?,
            status: req.status,
        };

        let period = self.service.create_payroll_period(dto).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(PayrollPeriodResponse {
            id: period.id.to_string(),
            legal_entity_id: period.legal_entity_id.to_string(),
            period_code: period.period_code,
            start_date: period.start_date.to_string(),
            end_date: period.end_date.to_string(),
            payment_date: period.payment_date.to_string(),
            status: period.status,
        }))
    }
}
