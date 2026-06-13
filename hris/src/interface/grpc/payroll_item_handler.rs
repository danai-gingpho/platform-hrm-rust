use tonic::{Request, Response, Status};
use uuid::Uuid;
use std::sync::Arc;
use crate::application::payroll_item::service::PayrollItemService;
use crate::application::payroll_item::dto::{
    CreatePayrollItemRequest as DtoCreateRequest, UpdatePayrollItemRequest as DtoUpdateRequest,
};
use crate::domain::shared::dtos::PaginationQuery;
use rust_decimal::Decimal;
use std::str::FromStr;

pub mod hris_proto {
    tonic::include_proto!("hris");
}

use hris_proto::payroll_item_service_server::PayrollItemService as PayrollItemGrpcService;
pub use hris_proto::payroll_item_service_server::PayrollItemServiceServer;
use hris_proto::{
    GetPayrollItemRequest, CreatePayrollItemRequest, PayrollItemResponse,
    UpdatePayrollItemRequest, DeletePayrollItemRequest, ListPayrollItemsRequest, ListPayrollItemsResponse, Empty
};

pub struct PayrollItemGrpcHandler {
    service: Arc<PayrollItemService>,
}

impl PayrollItemGrpcHandler {
    pub fn new(service: Arc<PayrollItemService>) -> Self {
        Self { service }
    }
}

fn map_payroll_item_to_response(item: crate::domain::payroll_item::entity::Model) -> PayrollItemResponse {
    PayrollItemResponse {
        id: item.id.to_string(),
        payroll_run_id: item.payroll_run_id.to_string(),
        item_code: item.item_code,
        item_name: item.item_name,
        item_category: item.item_category,
        quantity: item.quantity.to_string(),
        rate: item.rate.to_string(),
        amount: item.amount.to_string(),
        taxable: item.taxable,
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

        Ok(Response::new(map_payroll_item_to_response(item)))
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
            quantity: Decimal::from_str(&req.quantity).map_err(|_| Status::invalid_argument("Invalid quantity"))?,
            rate: Decimal::from_str(&req.rate).map_err(|_| Status::invalid_argument("Invalid rate"))?,
            amount: Decimal::from_str(&req.amount).map_err(|_| Status::invalid_argument("Invalid amount"))?,
            taxable: req.taxable,
        };

        let item = self.service.create_payroll_item(dto).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(map_payroll_item_to_response(item)))
    }

    async fn update_payroll_item(
        &self,
        request: Request<UpdatePayrollItemRequest>,
    ) -> Result<Response<PayrollItemResponse>, Status> {
        let req = request.into_inner();
        let id = Uuid::parse_str(&req.id)
            .map_err(|_| Status::invalid_argument("Invalid UUID"))?;

        let quantity = if let Some(ref val) = req.quantity {
            Some(Decimal::from_str(val).map_err(|_| Status::invalid_argument("Invalid quantity"))?)
        } else {
            None
        };

        let rate = if let Some(ref val) = req.rate {
            Some(Decimal::from_str(val).map_err(|_| Status::invalid_argument("Invalid rate"))?)
        } else {
            None
        };

        let amount = if let Some(ref val) = req.amount {
            Some(Decimal::from_str(val).map_err(|_| Status::invalid_argument("Invalid amount"))?)
        } else {
            None
        };

        let dto = DtoUpdateRequest {
            item_code: None, // DTO doesn't support updating item_code? Actually it does.
            item_name: req.item_name,
            item_category: None,
            quantity,
            rate,
            amount,
            taxable: req.taxable,
        };

        let item = self.service.update_payroll_item(id, dto).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(map_payroll_item_to_response(item)))
    }

    async fn delete_payroll_item(
        &self,
        request: Request<DeletePayrollItemRequest>,
    ) -> Result<Response<Empty>, Status> {
        let id = Uuid::parse_str(&request.into_inner().id)
            .map_err(|_| Status::invalid_argument("Invalid UUID"))?;

        self.service.delete_payroll_item(id).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(Empty {}))
    }

    async fn list_payroll_items(
        &self,
        request: Request<ListPayrollItemsRequest>,
    ) -> Result<Response<ListPayrollItemsResponse>, Status> {
        let req = request.into_inner();
        let query = PaginationQuery {
            page: Some(req.page as u64),
            limit: Some(req.per_page as u64),
            search: req.payroll_run_id, // Filter by payroll_run_id via search
        };

        let response = self.service.get_all_payroll_items(query).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(ListPayrollItemsResponse {
            items: response.data.into_iter().map(map_payroll_item_to_response).collect(),
            total_items: response.total as u32,
            total_pages: response.total_pages as u32,
            current_page: response.page as u32,
            per_page: response.limit as u32,
        }))
    }
}
