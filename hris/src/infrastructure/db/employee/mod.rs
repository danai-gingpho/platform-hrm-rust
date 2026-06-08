pub mod model;
use async_trait::async_trait;
use sea_orm::sea_query::extension::postgres::PgExpr;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QuerySelect, QueryOrder, IntoActiveModel
};
use uuid::Uuid;

use crate::domain::errors::{AppError, AppResult};
use crate::domain::shared::dtos::{PaginatedResponse, PaginationQuery};
use crate::domain::employee::entity::Model as EmployeeModel;
use crate::domain::employee::repository::EmployeeRepository;
pub use model::Entity;

pub struct SeaOrmEmployeeRepository {
    db: DatabaseConnection,
}

impl SeaOrmEmployeeRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}


#[async_trait]
impl EmployeeRepository for SeaOrmEmployeeRepository {
    async fn find_by_id(&self, id: Uuid) -> AppResult<EmployeeModel> {
        let model = Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(AppError::NotFound)?;

        Ok(model)
    }

    async fn create(&self, employee: EmployeeModel) -> AppResult<EmployeeModel> {
        let active_model: model::ActiveModel = employee.into_active_model();
        let saved = active_model.insert(&self.db).await?;
        Ok(saved)
    }

    async fn find_all(&self, query: PaginationQuery) -> AppResult<PaginatedResponse<EmployeeModel>> {
        let page = query.page.unwrap_or(1);
        let limit = query.limit.unwrap_or(10);
        let offset = (page - 1) * limit;
        
        let mut stmt = Entity::find();
        
        if let Some(ref search_term) = query.search {
            if !search_term.trim().is_empty() {
                let pattern = format!("%{}%", search_term);
                stmt = stmt.filter(
                    sea_orm::Condition::any()
                        .add(model::Column::FirstNameTh.into_expr().ilike(pattern.clone()))
                        .add(model::Column::LastNameTh.into_expr().ilike(pattern.clone()))
                        .add(model::Column::FirstNameEn.into_expr().ilike(pattern.clone()))
                        .add(model::Column::LastNameEn.into_expr().ilike(pattern.clone()))
                        .add(model::Column::EmployeeNo.into_expr().ilike(pattern.clone()))
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

    async fn update(&self, employee: EmployeeModel) -> AppResult<EmployeeModel> {
        let active_model: model::ActiveModel = employee.into_active_model();
        let updated = active_model.update(&self.db).await?;
        Ok(updated)
    }

    async fn find_latest_code(&self) -> AppResult<Option<String>> {
        let model = Entity::find()
            .order_by_desc(model::Column::EmployeeNo)
            .one(&self.db)
            .await?;
        
        Ok(model.map(|m| m.employee_no))
    }
}
