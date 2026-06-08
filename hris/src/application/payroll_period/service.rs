use crate::utils::code_generator::CodeGenerator;
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;
use crate::domain::payroll_period::entity::Model as PayrollPeriod;
use crate::domain::payroll_period::repository::PayrollPeriodRepository;
use crate::domain::errors::{AppError, AppResult};
use crate::domain::shared::dtos::{PaginationQuery, PaginatedResponse};
use crate::application::payroll_period::dto::{CreatePayrollPeriodRequest, UpdatePayrollPeriodRequest};

pub struct PayrollPeriodService {
    repository: Arc<dyn PayrollPeriodRepository>,
}

impl PayrollPeriodService {
    pub fn new(repository: Arc<dyn PayrollPeriodRepository>) -> Self {
        Self { repository }
    }

    pub async fn get_all_payroll_periods(&self, query: PaginationQuery) -> AppResult<PaginatedResponse<PayrollPeriod>> {
        self.repository.find_all(query).await
    }

    pub async fn get_payroll_period_by_id(&self, id: Uuid) -> AppResult<PayrollPeriod> {
        self.repository.find_by_id(id).await
    }

    pub async fn create_payroll_period(&self, req: CreatePayrollPeriodRequest) -> AppResult<PayrollPeriod> {
        req.validate().map_err(|e| AppError::Validation(e.to_string()))?;
        
        let period_code = match req.period_code {
            Some(c) => c,
            None => {
                let last_code = self.repository.find_latest_code().await?;
                CodeGenerator::generate("PRD", last_code)
            }
        };

        let payroll_period = PayrollPeriod {
            id: Uuid::new_v4(),
            legal_entity_id: req.legal_entity_id,
            period_code,
            start_date: req.start_date,
            end_date: req.end_date,
            payment_date: req.payment_date,
            status: req.status,
        };
        
        self.repository.create(payroll_period).await
    }

    pub async fn update_payroll_period(&self, id: Uuid, req: UpdatePayrollPeriodRequest) -> AppResult<PayrollPeriod> {
        let mut payroll_period = self.repository.find_by_id(id).await?;
        
        if let Some(val) = req.period_code { payroll_period.period_code = val; }
        if let Some(val) = req.start_date { payroll_period.start_date = val; }
        if let Some(val) = req.end_date { payroll_period.end_date = val; }
        if let Some(val) = req.payment_date { payroll_period.payment_date = val; }
        if let Some(val) = req.status { payroll_period.status = val; }
        
        self.repository.update(payroll_period).await
    }

    pub async fn delete_payroll_period(&self, id: Uuid) -> AppResult<()> {
        self.repository.delete(id).await
    }
}
