use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;
use crate::domain::social_security_rate::entity::Model as SocialSecurityRate;
use crate::domain::social_security_rate::repository::SocialSecurityRateRepository;
use crate::domain::errors::{AppError, AppResult};
use crate::domain::shared::dtos::{PaginationQuery, PaginatedResponse};
use crate::application::social_security_rate::dto::{CreateSocialSecurityRateRequest, UpdateSocialSecurityRateRequest};

pub struct SocialSecurityRateService {
    repository: Arc<dyn SocialSecurityRateRepository>,
}

impl SocialSecurityRateService {
    pub fn new(repository: Arc<dyn SocialSecurityRateRepository>) -> Self {
        Self { repository }
    }

    pub async fn get_all_rates(&self, query: PaginationQuery) -> AppResult<PaginatedResponse<SocialSecurityRate>> {
        self.repository.find_all(query).await
    }

    pub async fn get_rate_by_id(&self, id: Uuid) -> AppResult<SocialSecurityRate> {
        self.repository.find_by_id(id).await
    }

    pub async fn create_rate(&self, req: CreateSocialSecurityRateRequest) -> AppResult<SocialSecurityRate> {
        req.validate().map_err(|e| AppError::Validation(e.to_string()))?;
        let rate = SocialSecurityRate {
            id: Uuid::new_v4(),
            effective_date: req.effective_date,
            employee_percent: req.employee_percent,
            employer_percent: req.employer_percent,
            max_salary: req.max_salary,
        };
        self.repository.create(rate).await
    }

    pub async fn update_rate(&self, id: Uuid, req: UpdateSocialSecurityRateRequest) -> AppResult<SocialSecurityRate> {
        let mut rate = self.repository.find_by_id(id).await?;
        if let Some(val) = req.effective_date { rate.effective_date = val; }
        if let Some(val) = req.employee_percent { rate.employee_percent = val; }
        if let Some(val) = req.employer_percent { rate.employer_percent = val; }
        if let Some(val) = req.max_salary { rate.max_salary = val; }
        self.repository.update(rate).await
    }

    pub async fn delete_rate(&self, id: Uuid) -> AppResult<()> {
        self.repository.delete(id).await
    }
}
