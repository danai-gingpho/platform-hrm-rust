use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;
use crate::domain::payroll_item::entity::Model as PayrollItem;
use crate::domain::payroll_item::repository::PayrollItemRepository;
use crate::domain::errors::{AppError, AppResult};
use crate::domain::shared::dtos::{PaginationQuery, PaginatedResponse};
use crate::application::payroll_item::dto::{CreatePayrollItemRequest, UpdatePayrollItemRequest};

pub struct PayrollItemService {
    repository: Arc<dyn PayrollItemRepository>,
}

impl PayrollItemService {
    pub fn new(repository: Arc<dyn PayrollItemRepository>) -> Self {
        Self { repository }
    }

    pub async fn get_all_payroll_items(&self, query: PaginationQuery) -> AppResult<PaginatedResponse<PayrollItem>> {
        self.repository.find_all(query).await
    }

    pub async fn get_payroll_item_by_id(&self, id: Uuid) -> AppResult<PayrollItem> {
        self.repository.find_by_id(id).await
    }

    pub async fn create_payroll_item(&self, req: CreatePayrollItemRequest) -> AppResult<PayrollItem> {
        req.validate().map_err(|e| AppError::Validation(e.to_string()))?;
        
        let payroll_item = PayrollItem {
            id: Uuid::new_v4(),
            payroll_run_id: req.payroll_run_id,
            item_code: req.item_code,
            item_name: req.item_name,
            item_category: req.item_category,
            quantity: req.quantity,
            rate: req.rate,
            amount: req.amount,
            taxable: req.taxable,
        };
        
        self.repository.create(payroll_item).await
    }

    pub async fn update_payroll_item(&self, id: Uuid, req: UpdatePayrollItemRequest) -> AppResult<PayrollItem> {
        let mut payroll_item = self.repository.find_by_id(id).await?;
        
        if let Some(item_code) = req.item_code { payroll_item.item_code = item_code; }
        if let Some(item_name) = req.item_name { payroll_item.item_name = item_name; }
        if let Some(item_category) = req.item_category { payroll_item.item_category = item_category; }
        if let Some(quantity) = req.quantity { payroll_item.quantity = quantity; }
        if let Some(rate) = req.rate { payroll_item.rate = rate; }
        if let Some(amount) = req.amount { payroll_item.amount = amount; }
        if let Some(taxable) = req.taxable { payroll_item.taxable = taxable; }
        
        self.repository.update(payroll_item).await
    }

    pub async fn delete_payroll_item(&self, id: Uuid) -> AppResult<()> {
        self.repository.delete(id).await
    }
}
