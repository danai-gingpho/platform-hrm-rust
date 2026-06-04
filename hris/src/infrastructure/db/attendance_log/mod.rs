pub mod model;
use async_trait::async_trait;
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, EntityTrait, PaginatorTrait, 
    QuerySelect, IntoActiveModel, Set, ColumnTrait, QueryFilter, QueryOrder
};
use crate::domain::errors::{AppError, AppResult};
use crate::domain::shared::dtos::{PaginatedResponse, PaginationQuery};
use crate::domain::attendance_log::entity::Model as AttendanceLogModel;
use crate::domain::attendance_log::repository::AttendanceLogRepository;
use crate::domain::shared::context::TenantContext;
pub use model::Column;
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
    async fn find_by_id(&self, ctx: &TenantContext, id: i64) -> AppResult<AttendanceLogModel> {
        let tenant_id = ctx.tenant_id.ok_or(AppError::MissingTenantContext)?;

        let model = Entity::find_by_id(id)
            .filter(model::Column::TenantId.eq(tenant_id))
            .one(&self.db)
            .await?
            .ok_or(AppError::NotFound)?;

        Ok(model)
    }

    async fn create(&self, ctx: &TenantContext, mut attendance_log: AttendanceLogModel) -> AppResult<AttendanceLogModel> {
        let tenant_id = ctx.tenant_id.ok_or(AppError::MissingTenantContext)?;
        attendance_log.tenant_id = tenant_id;

        let mut active_model: model::ActiveModel = attendance_log.into_active_model();
        active_model.id = sea_orm::ActiveValue::NotSet; // Ensure ID is not set for auto-increment
        let saved = active_model.insert(&self.db).await?;
        Ok(saved)
    }

    async fn find_all(&self, ctx: &TenantContext, query: PaginationQuery) -> AppResult<PaginatedResponse<AttendanceLogModel>> {
        let tenant_id = ctx.tenant_id.ok_or(AppError::MissingTenantContext)?;

        let page = query.page.unwrap_or(1);
        let limit = query.limit.unwrap_or(10);
        let offset = (page - 1) * limit;
        
        let stmt = Entity::find()
            .filter(model::Column::TenantId.eq(tenant_id));
        
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

    async fn delete(&self, ctx: &TenantContext, id: i64) -> AppResult<()> {
        let tenant_id = ctx.tenant_id.ok_or(AppError::MissingTenantContext)?;

        let result = Entity::delete_by_id(id)
            .filter(model::Column::TenantId.eq(tenant_id))
            .exec(&self.db)
            .await?;
        if result.rows_affected == 0 {
            return Err(AppError::NotFound);
        }
        Ok(())
    }

    async fn update(&self, ctx: &TenantContext, mut attendance_log: AttendanceLogModel) -> AppResult<AttendanceLogModel> {
        let tenant_id = ctx.tenant_id.ok_or(AppError::MissingTenantContext)?;
        attendance_log.tenant_id = tenant_id;

        let active_model: model::ActiveModel = attendance_log.into_active_model();
        let updated = active_model.update(&self.db).await?;
        Ok(updated)
    }

    async fn find_latest_code(&self, _ctx: &TenantContext) -> AppResult<Option<String>> {
        Ok(None)
    }
}
