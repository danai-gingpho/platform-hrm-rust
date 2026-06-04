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
use crate::domain::department::entity::Model as DepartmentModel;
use crate::domain::department::repository::DepartmentRepository;
pub use model::Column;
pub use model::Entity;

pub struct SeaOrmDepartmentRepository {
    db: DatabaseConnection,
}

impl SeaOrmDepartmentRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}


#[async_trait]
impl DepartmentRepository for SeaOrmDepartmentRepository {
    async fn find_by_id(&self, id: Uuid) -> AppResult<DepartmentModel> {
        let model = Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(AppError::NotFound)?;

        Ok(model)
    }

    async fn create(&self, department: DepartmentModel) -> AppResult<DepartmentModel> {
        let active_model: model::ActiveModel = department.into_active_model();
        let saved = active_model.insert(&self.db).await?;
        Ok(saved)
    }

    async fn find_all(&self, query: PaginationQuery) -> AppResult<PaginatedResponse<DepartmentModel>> {
        let page = query.page.unwrap_or(1);
        let limit = query.limit.unwrap_or(10);
        let offset = (page - 1) * limit;
        
        let mut stmt = Entity::find();
        
        if let Some(ref search_term) = query.search {
            if !search_term.trim().is_empty() {
                let pattern = format!("%{}%", search_term);
                stmt = stmt.filter(
                    sea_orm::Condition::any()
                        .add(model::Column::Name.into_expr().ilike(pattern.clone()))
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

    async fn delete(&self, id: Uuid) -> AppResult<()> {
        let result = Entity::delete_by_id(id).exec(&self.db).await?;
        if result.rows_affected == 0 {
            return Err(AppError::NotFound);
        }
        Ok(())
    }

    async fn update(&self, department: DepartmentModel) -> AppResult<DepartmentModel> {
        let active_model: model::ActiveModel = department.into_active_model();
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
