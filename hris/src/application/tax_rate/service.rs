use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;
use crate::domain::tax_rate::entity::Model as TaxRate;
use crate::domain::tax_rate::repository::TaxRateRepository;
use crate::domain::errors::{AppError, AppResult};
use crate::domain::shared::dtos::{PaginationQuery, PaginatedResponse};
use crate::application::tax_rate::dto::{CreateTaxRateRequest, UpdateTaxRateRequest};

pub struct TaxRateService {
    repository: Arc<dyn TaxRateRepository>,
}

impl TaxRateService {
    pub fn new(repository: Arc<dyn TaxRateRepository>) -> Self {
        Self { repository }
    }

    pub async fn get_all_rates(&self, query: PaginationQuery) -> AppResult<PaginatedResponse<TaxRate>> {
        self.repository.find_all(query).await
    }

    pub async fn get_rate_by_id(&self, id: Uuid) -> AppResult<TaxRate> {
        self.repository.find_by_id(id).await
    }

    pub async fn create_rate(&self, req: CreateTaxRateRequest) -> AppResult<TaxRate> {
        req.validate().map_err(|e| AppError::Validation(e.to_string()))?;
        let rate = TaxRate {
            id: Uuid::new_v4(),
            year: req.year,
            min_income: req.min_income,
            max_income: req.max_income,
            tax_percent: req.tax_percent,
        };
        self.repository.create(rate).await
    }

    pub async fn update_rate(&self, id: Uuid, req: UpdateTaxRateRequest) -> AppResult<TaxRate> {
        let mut rate = self.repository.find_by_id(id).await?;
        if let Some(val) = req.year { rate.year = val; }
        if let Some(val) = req.min_income { rate.min_income = val; }
        if let Some(val) = req.max_income { rate.max_income = val; }
        if let Some(val) = req.tax_percent { rate.tax_percent = val; }
        self.repository.update(rate).await
    }

    pub async fn delete_rate(&self, id: Uuid) -> AppResult<()> {
        self.repository.delete(id).await
    }
}
