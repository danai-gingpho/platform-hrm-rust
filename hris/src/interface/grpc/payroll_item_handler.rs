use tonic::{Request, Response, Status};
use uuid::Uuid;
use std::sync::Arc;
use crate::application::payroll_item::service::PayrollItemService;
use crate::application::payroll_item::dto::CreatePayrollItemRequest as DtoCreateRequest;

pub mod hris_proto {
    tonic::include_proto!("hris");
}

use hris_proto::payroll_item_service_server::PayrollItemService as PayrollItemGrpcService;
pub use hris_proto::payroll_item_service_server::PayrollItemServiceServer;
use hris_proto::{GetPayrollItemRequest, CreatePayrollItemRequest, PayrollItemResponse};

pub struct PayrollItemGrpcHandler {
    service: Arc<PayrollItemService>,
}

impl PayrollItemGrpcHandler {
    pub fn new(service: Arc<PayrollItemService>) -> Self {
        Self { service }
    }
}

#[tonic::async_trait]
impl PayrollItemGrpcService for PayrollItemGrpcHandler {
    async fn get_payroll_item(
        &self,
        request: Request<GetPayrollItemRequest>,
    ) -> Result<Response<PayrollItemResponse>, Status> {
        let id = Uuid::parse_str(&request.into_inner().id)
            .map_err(|_| Status::invalid_argument("Invalid UUID"))?;

        let item = self.service.get_payroll_item_by_id(id).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(PayrollItemResponse {
            id: item.id.to_string(),
            payroll_run_id: item.payroll_run_id.to_string(),
            item_code: item.item_code,
            item_name: item.item_name,
            item_category: item.item_category,
            quantity: item.quantity.to_string(),
            rate: item.rate.to_string(),
            amount: item.amount.to_string(),
            taxable: item.taxable,
        }))
    }

    async fn create_payroll_item(
        &self,
        request: Request<CreatePayrollItemRequest>,
    ) -> Result<Response<PayrollItemResponse>, Status> {
        let req = request.into_inner();
        let dto = DtoCreateRequest {
            payroll_run_id: Uuid::parse_str(&req.payroll_run_id).map_err(|_| Status::invalid_argument("Invalid payroll_run_id"))?,
            item_code: req.item_code,
            item_name: req.item_name,
            item_category: req.item_category,
            quantity: req.quantity.parse().map_err(|_| Status::invalid_argument("Invalid quantity"))?,
            rate: req.rate.parse().map_err(|_| Status::invalid_argument("Invalid rate"))?,
            amount: req.amount.parse().map_err(|_| Status::invalid_argument("Invalid amount"))?,
            taxable: req.taxable,
        };

        let item = self.service.create_payroll_item(dto).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(PayrollItemResponse {
            id: item.id.to_string(),
            payroll_run_id: item.payroll_run_id.to_string(),
            item_code: item.item_code,
            item_name: item.item_name,
            item_category: item.item_category,
            quantity: item.quantity.to_string(),
            rate: item.rate.to_string(),
            amount: item.amount.to_string(),
            taxable: item.taxable,
        }))
    }
}
