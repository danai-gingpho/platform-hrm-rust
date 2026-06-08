pub mod model;
use async_trait::async_trait;
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, EntityTrait, PaginatorTrait,
    IntoActiveModel, QuerySelect
};

use crate::domain::errors::{AppError, AppResult};
use crate::domain::shared::dtos::{PaginatedResponse, PaginationQuery};
use crate::domain::attendance_log::entity::Model as AttendanceLogModel;
use crate::domain::attendance_log::repository::AttendanceLogRepository;
pub use model::Entity;

pub struct SeaOrmAttendanceLogRepository {
    db: DatabaseConnection,
}

impl SeaOrmAttendanceLogRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl AttendanceLogRepository for SeaOrmAttendanceLogRepository {
    async fn find_by_id(&self, id: i64) -> AppResult<AttendanceLogModel> {
        let model = Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(AppError::NotFound)?;

        Ok(model)
    }

    async fn create(&self, attendance_log: AttendanceLogModel) -> AppResult<AttendanceLogModel> {
        let active_model: model::ActiveModel = attendance_log.into_active_model();
        let saved = active_model.insert(&self.db).await?;
        Ok(saved)
    }

    async fn find_all(&self, query: PaginationQuery) -> AppResult<PaginatedResponse<AttendanceLogModel>> {
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

    async fn delete(&self, id: i64) -> AppResult<()> {
        let result = Entity::delete_by_id(id)
            .exec(&self.db)
            .await?;
            
        if result.rows_affected == 0 {
            return Err(AppError::NotFound);
        }
        Ok(())
    }

    async fn update(&self, attendance_log: AttendanceLogModel) -> AppResult<AttendanceLogModel> {
        let active_model: model::ActiveModel = attendance_log.into_active_model();
        let updated = active_model.update(&self.db).await?;
        Ok(updated)
    }

    async fn find_latest_code(&self) -> AppResult<Option<String>> {
        // Attendance logs might not have a code generator in the same way
        Ok(None)
    }
}
