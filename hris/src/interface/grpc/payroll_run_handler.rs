use tonic::{Request, Response, Status};
use uuid::Uuid;
use std::sync::Arc;
use crate::application::payroll_run::service::PayrollRunService;
use crate::application::payroll_run::dto::CreatePayrollRunRequest as DtoCreateRequest;

pub mod hris_proto {
    tonic::include_proto!("hris");
}

use hris_proto::payroll_run_service_server::PayrollRunService as PayrollRunGrpcService;
pub use hris_proto::payroll_run_service_server::PayrollRunServiceServer;
use hris_proto::{GetPayrollRunRequest, CreatePayrollRunRequest, PayrollRunResponse};

pub struct PayrollRunGrpcHandler {
    service: Arc<PayrollRunService>,
}

impl PayrollRunGrpcHandler {
    pub fn new(service: Arc<PayrollRunService>) -> Self {
        Self { service }
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

        Ok(Response::new(PayrollRunResponse {
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
            calculated_at: run.calculated_at.to_string(),
        }))
    }

    async fn create_payroll_run(
        &self,
        request: Request<CreatePayrollRunRequest>,
    ) -> Result<Response<PayrollRunResponse>, Status> {
        let req = request.into_inner();
        let dto = DtoCreateRequest {
            payroll_period_id: Uuid::parse_str(&req.payroll_period_id).map_err(|_| Status::invalid_argument("Invalid payroll_period_id"))?,
            employee_id: Uuid::parse_str(&req.employee_id).map_err(|_| Status::invalid_argument("Invalid employee_id"))?,
            gross_income: req.gross_income.parse().map_err(|_| Status::invalid_argument("Invalid gross_income"))?,
            total_deduction: req.total_deduction.parse().map_err(|_| Status::invalid_argument("Invalid total_deduction"))?,
            taxable_income: req.taxable_income.parse().map_err(|_| Status::invalid_argument("Invalid taxable_income"))?,
            tax_amount: req.tax_amount.parse().map_err(|_| Status::invalid_argument("Invalid tax_amount"))?,
            social_security: req.social_security.parse().map_err(|_| Status::invalid_argument("Invalid social_security"))?,
            net_income: req.net_income.parse().map_err(|_| Status::invalid_argument("Invalid net_income"))?,
            status: req.status,
        };

        let run = self.service.create_payroll_run(dto).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(PayrollRunResponse {
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
            calculated_at: run.calculated_at.to_string(),
        }))
    }
}
