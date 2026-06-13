use tonic::{Request, Response, Status};
use uuid::Uuid;
use std::sync::Arc;
use crate::application::payroll_run::service::PayrollRunService;
use crate::application::payroll_run::dto::{
    CreatePayrollRunRequest as DtoCreateRequest, UpdatePayrollRunRequest as DtoUpdateRequest,
};
use crate::domain::shared::dtos::PaginationQuery;
use rust_decimal::Decimal;
use std::str::FromStr;

pub mod hris_proto {
    tonic::include_proto!("hris");
}

use hris_proto::payroll_run_service_server::PayrollRunService as PayrollRunGrpcService;
pub use hris_proto::payroll_run_service_server::PayrollRunServiceServer;
use hris_proto::{
    GetPayrollRunRequest, CreatePayrollRunRequest, PayrollRunResponse,
    UpdatePayrollRunRequest, DeletePayrollRunRequest, ListPayrollRunsRequest, ListPayrollRunsResponse, Empty
};

pub struct PayrollRunGrpcHandler {
    service: Arc<PayrollRunService>,
}

impl PayrollRunGrpcHandler {
    pub fn new(service: Arc<PayrollRunService>) -> Self {
        Self { service }
    }
}

fn map_payroll_run_to_response(run: crate::domain::payroll_run::entity::Model) -> PayrollRunResponse {
    PayrollRunResponse {
        id: run.id.to_string(),
        payroll_period_id: run.payroll_period_id.to_string(),
        employee_id: run.employee_id.to_string(),
        gross_income: run.gross_income.to_string(),
        total_deduction: run.total_deduction.to_string(),
        taxable_income: run.taxable_income.to_string(),
        tax_amount: run.tax_amount.to_string(),
        social_security: run.social_security.to_string(),
        net_income: run.net_income.to_string(),
        status: run.status,
        calculated_at: run.calculated_at.to_rfc3339(),
    }
}

#[tonic::async_trait]
impl PayrollRunGrpcService for PayrollRunGrpcHandler {
    async fn get_payroll_run(
        &self,
        request: Request<GetPayrollRunRequest>,
    ) -> Result<Response<PayrollRunResponse>, Status> {
        let id = Uuid::parse_str(&request.into_inner().id)
            .map_err(|_| Status::invalid_argument("Invalid UUID"))?;

        let run = self.service.get_payroll_run_by_id(id).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(map_payroll_run_to_response(run)))
    }

    async fn create_payroll_run(
        &self,
        request: Request<CreatePayrollRunRequest>,
    ) -> Result<Response<PayrollRunResponse>, Status> {
        let req = request.into_inner();
        let dto = DtoCreateRequest {
            payroll_period_id: Uuid::parse_str(&req.payroll_period_id).map_err(|_| Status::invalid_argument("Invalid payroll_period_id"))?,
            employee_id: Uuid::parse_str(&req.employee_id).map_err(|_| Status::invalid_argument("Invalid employee_id"))?,
            gross_income: Decimal::from_str(&req.gross_income).map_err(|_| Status::invalid_argument("Invalid gross_income"))?,
            total_deduction: Decimal::from_str(&req.total_deduction).map_err(|_| Status::invalid_argument("Invalid total_deduction"))?,
            taxable_income: Decimal::from_str(&req.taxable_income).map_err(|_| Status::invalid_argument("Invalid taxable_income"))?,
            tax_amount: Decimal::from_str(&req.tax_amount).map_err(|_| Status::invalid_argument("Invalid tax_amount"))?,
            social_security: Decimal::from_str(&req.social_security).map_err(|_| Status::invalid_argument("Invalid social_security"))?,
            net_income: Decimal::from_str(&req.net_income).map_err(|_| Status::invalid_argument("Invalid net_income"))?,
            status: req.status,
        };

        let run = self.service.create_payroll_run(dto).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(map_payroll_run_to_response(run)))
    }

    async fn update_payroll_run(
        &self,
        request: Request<UpdatePayrollRunRequest>,
    ) -> Result<Response<PayrollRunResponse>, Status> {
        let req = request.into_inner();
        let id = Uuid::parse_str(&req.id)
            .map_err(|_| Status::invalid_argument("Invalid UUID"))?;

        let dto = DtoUpdateRequest {
            gross_income: req.gross_income.map(|v| Decimal::from_str(&v).unwrap_or_default()),
            total_deduction: req.total_deduction.map(|v| Decimal::from_str(&v).unwrap_or_default()),
            taxable_income: req.taxable_income.map(|v| Decimal::from_str(&v).unwrap_or_default()),
            tax_amount: req.tax_amount.map(|v| Decimal::from_str(&v).unwrap_or_default()),
            social_security: req.social_security.map(|v| Decimal::from_str(&v).unwrap_or_default()),
            net_income: req.net_income.map(|v| Decimal::from_str(&v).unwrap_or_default()),
            status: req.status,
        };

        let run = self.service.update_payroll_run(id, dto).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(map_payroll_run_to_response(run)))
    }

    async fn delete_payroll_run(
        &self,
        request: Request<DeletePayrollRunRequest>,
    ) -> Result<Response<Empty>, Status> {
        let id = Uuid::parse_str(&request.into_inner().id)
            .map_err(|_| Status::invalid_argument("Invalid UUID"))?;

        self.service.delete_payroll_run(id).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(Empty {}))
    }

    async fn list_payroll_runs(
        &self,
        request: Request<ListPayrollRunsRequest>,
    ) -> Result<Response<ListPayrollRunsResponse>, Status> {
        let req = request.into_inner();
        let query = PaginationQuery {
            page: Some(req.page as u64),
            limit: Some(req.per_page as u64),
            search: req.payroll_period_id, // Use search for filtering
        };

        let response = self.service.get_all_payroll_runs(query).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(ListPayrollRunsResponse {
            items: response.data.into_iter().map(map_payroll_run_to_response).collect(),
            total_items: response.total as u32,
            total_pages: response.total_pages as u32,
            current_page: response.page as u32,
            per_page: response.limit as u32,
        }))
    }
}
