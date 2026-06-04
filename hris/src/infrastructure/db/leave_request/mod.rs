pub mod model;
use async_trait::async_trait;
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, EntityTrait, PaginatorTrait, 
    QuerySelect, IntoActiveModel
};
use uuid::Uuid;

use crate::domain::errors::{AppError, AppResult};
use crate::domain::shared::dtos::{PaginatedResponse, PaginationQuery};
use crate::domain::leave_request::entity::Model as LeaveRequestModel;
use crate::domain::leave_request::repository::LeaveRequestRepository;
pub use model::Column;
pub use model::Entity;

pub struct SeaOrmLeaveRequestRepository {
    db: DatabaseConnection,
}

impl SeaOrmLeaveRequestRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl LeaveRequestRepository for SeaOrmLeaveRequestRepository {
    async fn find_by_id(&self, id: Uuid) -> AppResult<LeaveRequestModel> {
        let model = Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(AppError::NotFound)?;

        Ok(model)
    }

    async fn create(&self, leave_request: LeaveRequestModel) -> AppResult<LeaveRequestModel> {
        let active_model: model::ActiveModel = leave_request.into_active_model();
        let saved = active_model.insert(&self.db).await?;
        Ok(saved)
    }

    async fn find_all(&self, query: PaginationQuery) -> AppResult<PaginatedResponse<LeaveRequestModel>> {
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

    async fn update(&self, leave_request: LeaveRequestModel) -> AppResult<LeaveRequestModel> {
        let active_model: model::ActiveModel = leave_request.into_active_model();
        let updated = active_model.update(&self.db).await?;
        Ok(updated)
    }

    async fn find_latest_code(&self) -> AppResult<Option<String>> {
        Ok(None)
    }
}
