pub mod model;
use async_trait::async_trait;
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, EntityTrait, PaginatorTrait, IntoActiveModel, QuerySelect, QueryOrder
};
use uuid::Uuid;

use crate::domain::errors::{AppError, AppResult};
use crate::domain::shared::dtos::{PaginatedResponse, PaginationQuery};
use crate::domain::salary_structure::entity::Model as SalaryStructureModel;
use crate::domain::salary_structure::repository::SalaryStructureRepository;
pub use model::Column;
pub use model::Entity;

pub struct SeaOrmSalaryStructureRepository {
    db: DatabaseConnection,
}

impl SeaOrmSalaryStructureRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl SalaryStructureRepository for SeaOrmSalaryStructureRepository {
    async fn find_by_id(&self, id: Uuid) -> AppResult<SalaryStructureModel> {
        let model = Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(AppError::NotFound)?;

        Ok(model)
    }

    async fn create(&self, salary_structure: SalaryStructureModel) -> AppResult<SalaryStructureModel> {
        let active_model: model::ActiveModel = salary_structure.into_active_model();
        let saved = active_model.insert(&self.db).await?;
        Ok(saved)
    }

    async fn find_all(&self, query: PaginationQuery) -> AppResult<PaginatedResponse<SalaryStructureModel>> {
        let page = query.page.unwrap_or(1);
        let limit = query.limit.unwrap_or(10);
        let offset = (page - 1) * limit;
        
        let stmt = Entity::find();
        
        let total = stmt.clone().count(&self.db).await?;
        let total_pages = ((total as f64) / (limit as f64)).ceil() as u64;
        let models = stmt.offset(offset).limit(limit).all(&self.db).await?;

        Ok(PaginatedResponse {
            data: models,
            total,
            page,
            limit,
            total_pages,
        })
    }

    async fn delete(&self, id: Uuid) -> AppResult<()> {
        let result = Entity::delete_by_id(id).exec(&self.db).await?;
        if result.rows_affected == 0 {
            return Err(AppError::NotFound);
        }
        Ok(())
    }

    async fn find_latest_code(&self) -> AppResult<Option<String>> {
        let model = Entity::find()
            .order_by_desc(model::Column::Code)
            .one(&self.db)
            .await?;
        Ok(model.map(|m| m.code))
    }

    async fn update(&self, salary_structure: SalaryStructureModel) -> AppResult<SalaryStructureModel> {
        let active_model: model::ActiveModel = salary_structure.into_active_model();
        let updated = active_model.update(&self.db).await?;
        Ok(updated)
    }
}
