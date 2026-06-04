pub mod model;
use async_trait::async_trait;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QuerySelect, QueryOrder, IntoActiveModel
};
use uuid::Uuid;

use crate::domain::errors::{AppError, AppResult};
use crate::domain::shared::dtos::{PaginatedResponse, PaginationQuery};
use crate::domain::allowance_type::entity::Model as AllowanceTypeModel;
use crate::domain::allowance_type::repository::AllowanceTypeRepository;
use crate::domain::shared::context::TenantContext;
pub use model::Column;
pub use model::Entity;

pub struct SeaOrmAllowanceTypeRepository {
    db: DatabaseConnection,
}

impl SeaOrmAllowanceTypeRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl AllowanceTypeRepository for SeaOrmAllowanceTypeRepository {
    async fn find_by_id(&self, ctx: &TenantContext, id: Uuid) -> AppResult<AllowanceTypeModel> {
        let tenant_id = ctx.tenant_id.ok_or(AppError::MissingTenantContext)?;
        
        let model = Entity::find_by_id(id)
            .filter(model::Column::TenantId.eq(tenant_id))
            .one(&self.db)
            .await?
            .ok_or(AppError::NotFound)?;

        Ok(model)
    }

    async fn create(&self, ctx: &TenantContext, mut allowance_type: AllowanceTypeModel) -> AppResult<AllowanceTypeModel> {
        let tenant_id = ctx.tenant_id.ok_or(AppError::MissingTenantContext)?;
        allowance_type.tenant_id = tenant_id;

        let active_model: model::ActiveModel = allowance_type.into_active_model();
        let saved = active_model.insert(&self.db).await?;
        Ok(saved)
    }

    async fn find_all(&self, ctx: &TenantContext, query: PaginationQuery) -> AppResult<PaginatedResponse<AllowanceTypeModel>> {
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

    async fn delete(&self, ctx: &TenantContext, id: Uuid) -> AppResult<()> {
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

    async fn find_latest_code(&self, ctx: &TenantContext) -> AppResult<Option<String>> {
        let tenant_id = ctx.tenant_id.ok_or(AppError::MissingTenantContext)?;

        let model = Entity::find()
            .filter(model::Column::TenantId.eq(tenant_id))
            .order_by_desc(model::Column::Code)
            .one(&self.db)
            .await?;
        Ok(model.map(|m| m.code))
    }

    async fn update(&self, ctx: &TenantContext, mut allowance_type: AllowanceTypeModel) -> AppResult<AllowanceTypeModel> {
        let tenant_id = ctx.tenant_id.ok_or(AppError::MissingTenantContext)?;
        allowance_type.tenant_id = tenant_id;

        let active_model: model::ActiveModel = allowance_type.into_active_model();
        let updated = active_model.update(&self.db).await?;
        Ok(updated)
    }
}
