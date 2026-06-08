pub mod model;
use async_trait::async_trait;
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, EntityTrait, PaginatorTrait,
    QueryOrder, IntoActiveModel, QuerySelect
};
use uuid::Uuid;

use crate::domain::errors::{AppError, AppResult};
use crate::domain::shared::dtos::{PaginatedResponse, PaginationQuery};
use crate::domain::legal_entity::entity::Model as LegalEntityModel;
use crate::domain::legal_entity::repository::LegalEntityRepository;
pub use model::Entity;

pub struct SeaOrmLegalEntityRepository {
    db: DatabaseConnection,
}

impl SeaOrmLegalEntityRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl LegalEntityRepository for SeaOrmLegalEntityRepository {
    async fn find_by_id(&self, id: Uuid) -> AppResult<LegalEntityModel> {
        let model = Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(AppError::NotFound)?;

        Ok(model)
    }

    async fn create(&self, legal_entity: LegalEntityModel) -> AppResult<LegalEntityModel> {
        let active_model: model::ActiveModel = legal_entity.into_active_model();
        let saved = active_model.insert(&self.db).await?;
        Ok(saved)
    }

    async fn find_all(&self, query: PaginationQuery) -> AppResult<PaginatedResponse<LegalEntityModel>> {
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

    async fn find_by_user_id(&self, _user_id: Uuid, query: PaginationQuery) -> AppResult<PaginatedResponse<LegalEntityModel>> {
        // Implementation for finding legal entities associated with a user
        // This might require a join table like user_legal_entity
        self.find_all(query).await
    }

    async fn delete(&self, id: Uuid) -> AppResult<()> {
        let result = Entity::delete_by_id(id)
            .exec(&self.db)
            .await?;
            
        if result.rows_affected == 0 {
            return Err(AppError::NotFound);
        }
        Ok(())
    }

    async fn update(&self, legal_entity: LegalEntityModel) -> AppResult<LegalEntityModel> {
        let active_model: model::ActiveModel = legal_entity.into_active_model();
        let updated = active_model.update(&self.db).await?;
        Ok(updated)
    }

    async fn find_latest_code(&self) -> AppResult<Option<String>> {
        let model = Entity::find()
            .order_by_desc(model::Column::Code)
            .one(&self.db)
            .await?;
        Ok(model.map(|m| m.code))
    }
}
