pub mod model;
use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, PaginatorTrait, IntoActiveModel, QuerySelect, QueryOrder};
use uuid::Uuid;

use crate::domain::errors::{AppError, AppResult};
use crate::domain::shared::dtos::{PaginatedResponse, PaginationQuery};
use crate::domain::tax_rate::entity::Model as TaxRateModel;
use crate::domain::tax_rate::repository::TaxRateRepository;
pub use model::Column;
pub use model::Entity;

pub struct SeaOrmTaxRateRepository {
    db: DatabaseConnection,
}

impl SeaOrmTaxRateRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl TaxRateRepository for SeaOrmTaxRateRepository {
    async fn find_by_id(&self, id: Uuid) -> AppResult<TaxRateModel> {
        Entity::find_by_id(id).one(&self.db).await?.ok_or(AppError::NotFound)
    }

    async fn create(&self, tax_rate: TaxRateModel) -> AppResult<TaxRateModel> {
        let active_model = tax_rate.into_active_model();
        Ok(active_model.insert(&self.db).await?)
    }

    async fn find_all(&self, query: PaginationQuery) -> AppResult<PaginatedResponse<TaxRateModel>> {
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

    async fn update(&self, tax_rate: TaxRateModel) -> AppResult<TaxRateModel> {
        let active_model = tax_rate.into_active_model();
        Ok(active_model.update(&self.db).await?)
    }
}
