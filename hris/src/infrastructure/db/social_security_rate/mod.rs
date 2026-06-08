pub mod model;
use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, PaginatorTrait, IntoActiveModel, QuerySelect};
use uuid::Uuid;

use crate::domain::errors::{AppError, AppResult};
use crate::domain::shared::dtos::{PaginatedResponse, PaginationQuery};
use crate::domain::social_security_rate::entity::Model as SocialSecurityRateModel;
use crate::domain::social_security_rate::repository::SocialSecurityRateRepository;
pub use model::Entity;

pub struct SeaOrmSocialSecurityRateRepository {
    db: DatabaseConnection,
}

impl SeaOrmSocialSecurityRateRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl SocialSecurityRateRepository for SeaOrmSocialSecurityRateRepository {
    async fn find_by_id(&self, id: Uuid) -> AppResult<SocialSecurityRateModel> {
        Entity::find_by_id(id).one(&self.db).await?.ok_or(AppError::NotFound)
    }

    async fn create(&self, rate: SocialSecurityRateModel) -> AppResult<SocialSecurityRateModel> {
        let active_model = rate.into_active_model();
        Ok(active_model.insert(&self.db).await?)
    }

    async fn find_all(&self, query: PaginationQuery) -> AppResult<PaginatedResponse<SocialSecurityRateModel>> {
        let page = query.page.unwrap_or(1);
        let limit = query.limit.unwrap_or(10);
        let offset = (page - 1) * limit;
        let stmt = Entity::find();
        let total = stmt.clone().count(&self.db).await?;
        let models = stmt.offset(offset).limit(limit).all(&self.db).await?;
        Ok(PaginatedResponse { data: models, total, page, limit, total_pages: (total as f64 / limit as f64).ceil() as u64 })
    }

    async fn delete(&self, id: Uuid) -> AppResult<()> {
        let res = Entity::delete_by_id(id).exec(&self.db).await?;
        if res.rows_affected == 0 { return Err(AppError::NotFound); }
        Ok(())
    }

    async fn find_latest_code(&self) -> AppResult<Option<String>> {
        Ok(None)
    }

    async fn update(&self, rate: SocialSecurityRateModel) -> AppResult<SocialSecurityRateModel> {
        let active_model = rate.into_active_model();
        Ok(active_model.update(&self.db).await?)
    }
}
