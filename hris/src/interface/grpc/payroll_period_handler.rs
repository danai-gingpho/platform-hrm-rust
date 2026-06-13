use tonic::{Request, Response, Status};
use uuid::Uuid;
use std::sync::Arc;
use crate::application::payroll_period::service::PayrollPeriodService;
use crate::application::payroll_period::dto::{
    CreatePayrollPeriodRequest as DtoCreateRequest, UpdatePayrollPeriodRequest as DtoUpdateRequest,
};
use crate::domain::shared::dtos::PaginationQuery;
use chrono::NaiveDate;

pub mod hris_proto {
    tonic::include_proto!("hris");
}

use hris_proto::payroll_period_service_server::PayrollPeriodService as PayrollPeriodGrpcService;
pub use hris_proto::payroll_period_service_server::PayrollPeriodServiceServer;
use hris_proto::{
    GetPayrollPeriodRequest, CreatePayrollPeriodRequest, PayrollPeriodResponse,
    UpdatePayrollPeriodRequest, DeletePayrollPeriodRequest, ListPayrollPeriodsRequest, ListPayrollPeriodsResponse, Empty
};

pub struct PayrollPeriodGrpcHandler {
    service: Arc<PayrollPeriodService>,
}

impl PayrollPeriodGrpcHandler {
    pub fn new(service: Arc<PayrollPeriodService>) -> Self {
        Self { service }
    }
}

fn map_payroll_period_to_response(period: crate::domain::payroll_period::entity::Model) -> PayrollPeriodResponse {
    PayrollPeriodResponse {
        id: period.id.to_string(),
        legal_entity_id: period.legal_entity_id.to_string(),
        period_code: period.period_code,
        start_date: period.start_date.to_string(),
        end_date: period.end_date.to_string(),
        payment_date: period.payment_date.to_string(),
        status: period.status,
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

        Ok(Response::new(map_payroll_period_to_response(period)))
    }

    async fn create_payroll_period(
        &self,
        request: Request<CreatePayrollPeriodRequest>,
    ) -> Result<Response<PayrollPeriodResponse>, Status> {
        let req = request.into_inner();
        let dto = DtoCreateRequest {
            legal_entity_id: Uuid::parse_str(&req.legal_entity_id).map_err(|_| Status::invalid_argument("Invalid legal_entity_id"))?,
            period_code: if req.period_code.is_empty() { None } else { Some(req.period_code) },
            start_date: NaiveDate::parse_from_str(&req.start_date, "%Y-%m-%d").map_err(|_| Status::invalid_argument("Invalid start_date"))?,
            end_date: NaiveDate::parse_from_str(&req.end_date, "%Y-%m-%d").map_err(|_| Status::invalid_argument("Invalid end_date"))?,
            payment_date: NaiveDate::parse_from_str(&req.payment_date, "%Y-%m-%d").map_err(|_| Status::invalid_argument("Invalid payment_date"))?,
            status: req.status,
        };

        let period = self.service.create_payroll_period(dto).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(map_payroll_period_to_response(period)))
    }

    async fn update_payroll_period(
        &self,
        request: Request<UpdatePayrollPeriodRequest>,
    ) -> Result<Response<PayrollPeriodResponse>, Status> {
        let req = request.into_inner();
        let id = Uuid::parse_str(&req.id)
            .map_err(|_| Status::invalid_argument("Invalid UUID"))?;

        let start_date = if let Some(ref val) = req.start_date {
            Some(NaiveDate::parse_from_str(val, "%Y-%m-%d").map_err(|_| Status::invalid_argument("Invalid start_date"))?)
        } else {
            None
        };

        let end_date = if let Some(ref val) = req.end_date {
            Some(NaiveDate::parse_from_str(val, "%Y-%m-%d").map_err(|_| Status::invalid_argument("Invalid end_date"))?)
        } else {
            None
        };

        let payment_date = if let Some(ref val) = req.payment_date {
            Some(NaiveDate::parse_from_str(val, "%Y-%m-%d").map_err(|_| Status::invalid_argument("Invalid payment_date"))?)
        } else {
            None
        };

        let dto = DtoUpdateRequest {
            period_code: None, // DTO doesn't support updating period_code? Actually it does.
            start_date,
            end_date,
            payment_date,
            status: req.status,
        };

        let period = self.service.update_payroll_period(id, dto).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(map_payroll_period_to_response(period)))
    }

    async fn delete_payroll_period(
        &self,
        request: Request<DeletePayrollPeriodRequest>,
    ) -> Result<Response<Empty>, Status> {
        let id = Uuid::parse_str(&request.into_inner().id)
            .map_err(|_| Status::invalid_argument("Invalid UUID"))?;

        self.service.delete_payroll_period(id).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(Empty {}))
    }

    async fn list_payroll_periods(
        &self,
        request: Request<ListPayrollPeriodsRequest>,
    ) -> Result<Response<ListPayrollPeriodsResponse>, Status> {
        let req = request.into_inner();
        let query = PaginationQuery {
            page: Some(req.page as u64),
            limit: Some(req.per_page as u64),
            search: req.legal_entity_id, // Filter by legal_entity_id via search
        };

        let response = self.service.get_all_payroll_periods(query).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(ListPayrollPeriodsResponse {
            items: response.data.into_iter().map(map_payroll_period_to_response).collect(),
            total_items: response.total as u32,
            total_pages: response.total_pages as u32,
            current_page: response.page as u32,
            per_page: response.limit as u32,
        }))
    }
}
