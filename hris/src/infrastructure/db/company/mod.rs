pub mod model;
use async_trait::async_trait;
use sea_orm::sea_query::extension::postgres::PgExpr;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QuerySelect, QueryOrder, Set, IntoActiveModel, ModelTrait
};
use uuid::Uuid;

use crate::domain::errors::{AppError, AppResult};
use crate::domain::shared::dtos::{PaginatedResponse, PaginationQuery};
use crate::domain::company::entity::Model as CompanyModel;
use crate::domain::company::repository::CompanyRepository;
pub use model::Column;
pub use model::Entity;

pub struct SeaOrmCompanyRepository {
    db: DatabaseConnection,
}

impl SeaOrmCompanyRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}


use crate::domain::shared::context::TenantContext;

#[async_trait]
impl CompanyRepository for SeaOrmCompanyRepository {
    async fn find_by_id(&self, ctx: &TenantContext, id: Uuid) -> AppResult<CompanyModel> {
        let tenant_id = ctx.tenant_id.ok_or(AppError::MissingTenantContext)?;
        
        let model = Entity::find_by_id(id)
            .filter(model::Column::TenantId.eq(tenant_id))
            .one(&self.db)
            .await?
            .ok_or(AppError::NotFound)?;

        Ok(model)
    }

    async fn create(&self, ctx: &TenantContext, mut company: CompanyModel) -> AppResult<CompanyModel> {
        let tenant_id = ctx.tenant_id.ok_or(AppError::MissingTenantContext)?;
        company.tenant_id = tenant_id;

        let active_model: model::ActiveModel = company.into_active_model();
        let saved = active_model.insert(&self.db).await?;
        Ok(saved)
    }

    async fn find_all(&self, ctx: &TenantContext, query: PaginationQuery) -> AppResult<PaginatedResponse<CompanyModel>> {
        let tenant_id = ctx.tenant_id.ok_or(AppError::MissingTenantContext)?;
        
        let page = query.page.unwrap_or(1);
        let limit = query.limit.unwrap_or(10);
        let offset = (page - 1) * limit;
        
        let mut stmt = Entity::find()
            .filter(model::Column::TenantId.eq(tenant_id));
        
        if let Some(ref search_term) = query.search {
            if !search_term.trim().is_empty() {
                let pattern = format!("%{}%", search_term);
                stmt = stmt.filter(
                    sea_orm::Condition::any()
                        .add(model::Column::NameEn.into_expr().ilike(pattern.clone()))
                        .add(model::Column::NameTh.into_expr().ilike(pattern.clone()))
                        .add(model::Column::Code.into_expr().ilike(pattern.clone()))
                );
            }
        }

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

    async fn find_by_user_id(&self, ctx: &TenantContext, _user_id: Uuid, query: PaginationQuery) -> AppResult<PaginatedResponse<CompanyModel>> {
        self.find_all(ctx, query).await
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

    async fn update(&self, ctx: &TenantContext, mut company: CompanyModel) -> AppResult<CompanyModel> {
        let tenant_id = ctx.tenant_id.ok_or(AppError::MissingTenantContext)?;
        company.tenant_id = tenant_id;

        let active_model: model::ActiveModel = company.into_active_model();
        let updated = active_model.update(&self.db).await?;
        Ok(updated)
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
}
