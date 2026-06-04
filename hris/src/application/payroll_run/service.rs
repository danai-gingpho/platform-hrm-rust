use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;
use crate::domain::payroll_run::entity::Model as PayrollRun;
use crate::domain::payroll_run::repository::PayrollRunRepository;
use crate::domain::errors::{AppError, AppResult};
use crate::domain::shared::dtos::{PaginationQuery, PaginatedResponse};
use crate::application::payroll_run::dto::{CreatePayrollRunRequest, UpdatePayrollRunRequest};
use chrono::Utc;

pub struct PayrollRunService {
    repository: Arc<dyn PayrollRunRepository>,
}

impl PayrollRunService {
    pub fn new(repository: Arc<dyn PayrollRunRepository>) -> Self {
        Self { repository }
    }

    pub async fn get_all_payroll_runs(&self, query: PaginationQuery) -> AppResult<PaginatedResponse<PayrollRun>> {
        self.repository.find_all(query).await
    }

    pub async fn get_payroll_run_by_id(&self, id: Uuid) -> AppResult<PayrollRun> {
        self.repository.find_by_id(id).await
    }

    pub async fn create_payroll_run(&self, req: CreatePayrollRunRequest) -> AppResult<PayrollRun> {
        req.validate().map_err(|e| AppError::Validation(e.to_string()))?;
        
        let payroll_run = PayrollRun {
            id: Uuid::new_v4(),
            payroll_period_id: req.payroll_period_id,
            employee_id: req.employee_id,
            gross_income: req.gross_income,
            total_deduction: req.total_deduction,
            taxable_income: req.taxable_income,
            tax_amount: req.tax_amount,
            social_security: req.social_security,
            net_income: req.net_income,
            status: req.status,
            calculated_at: Utc::now().into(),
        };
        
        self.repository.create(payroll_run).await
    }

    pub async fn update_payroll_run(&self, id: Uuid, req: UpdatePayrollRunRequest) -> AppResult<PayrollRun> {
        let mut payroll_run = self.repository.find_by_id(id).await?;
        
        if let Some(gross_income) = req.gross_income { payroll_run.gross_income = gross_income; }
        if let Some(total_deduction) = req.total_deduction { payroll_run.total_deduction = total_deduction; }
        if let Some(taxable_income) = req.taxable_income { payroll_run.taxable_income = taxable_income; }
        if let Some(tax_amount) = req.tax_amount { payroll_run.tax_amount = tax_amount; }
        if let Some(social_security) = req.social_security { payroll_run.social_security = social_security; }
        if let Some(net_income) = req.net_income { payroll_run.net_income = net_income; }
        if let Some(status) = req.status { payroll_run.status = status; }
        
        self.repository.update(payroll_run).await
    }

    pub async fn delete_payroll_run(&self, id: Uuid) -> AppResult<()> {
        self.repository.delete(id).await
    }
}
