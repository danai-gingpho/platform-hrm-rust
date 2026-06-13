pub mod model;
use async_trait::async_trait;
use sea_orm::sea_query::extension::postgres::PgExpr;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QuerySelect, QueryOrder, IntoActiveModel, TransactionTrait, Statement, ConnectionTrait
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
    async fn find_by_id(&self, tenant_id: &str, id: Uuid) -> AppResult<EmployeeModel> {
        let tenant_id = tenant_id.to_string();
        self.db.transaction::<_, EmployeeModel, AppError>(|txn| {
            Box::pin(async move {
                txn.execute(Statement::from_string(txn.get_database_backend(), format!("SET search_path TO {}", tenant_id))).await?;
                let model = Entity::find_by_id(id)
                    .one(txn)
                    .await?
                    .ok_or(AppError::NotFound)?;
                Ok(model)
            })
        })
        .await
        .map_err(|e| AppError::Database(e.to_string()))
    }

    async fn create(&self, tenant_id: &str, employee: EmployeeModel) -> AppResult<EmployeeModel> {
        let tenant_id = tenant_id.to_string();
        self.db.transaction::<_, EmployeeModel, AppError>(|txn| {
            Box::pin(async move {
                txn.execute(Statement::from_string(txn.get_database_backend(), format!("SET search_path TO {}", tenant_id))).await?;
                let active_model: model::ActiveModel = employee.into_active_model();
                let saved = active_model.insert(txn).await?;
                Ok(saved)
            })
        })
        .await
        .map_err(|e| AppError::Database(e.to_string()))
    }

    async fn find_all(&self, tenant_id: &str, query: PaginationQuery) -> AppResult<PaginatedResponse<EmployeeModel>> {
        let tenant_id = tenant_id.to_string();
        self.db.transaction::<_, PaginatedResponse<EmployeeModel>, AppError>(|txn| {
            Box::pin(async move {
                txn.execute(Statement::from_string(txn.get_database_backend(), format!("SET search_path TO {}", tenant_id))).await?;
                
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

                let total = stmt.clone().count(txn).await?;
                let total_pages = ((total as f64) / (limit as f64)).ceil() as u64;
                let models = stmt.offset(offset).limit(limit).all(txn).await?;

                Ok(PaginatedResponse {
                    data: models,
                    total,
                    page,
                    limit,
                    total_pages,
                })
            })
        })
        .await
        .map_err(|e| AppError::Database(e.to_string()))
    }

    async fn delete(&self, tenant_id: &str, id: Uuid) -> AppResult<()> {
        let tenant_id = tenant_id.to_string();
        self.db.transaction::<_, (), AppError>(|txn| {
            Box::pin(async move {
                txn.execute(Statement::from_string(txn.get_database_backend(), format!("SET search_path TO {}", tenant_id))).await?;
                let result = Entity::delete_by_id(id).exec(txn).await?;
                if result.rows_affected == 0 {
                    return Err(AppError::NotFound);
                }
                Ok(())
            })
        })
        .await
        .map_err(|e| AppError::Database(e.to_string()))
    }

    async fn update(&self, tenant_id: &str, employee: EmployeeModel) -> AppResult<EmployeeModel> {
        let tenant_id = tenant_id.to_string();
        self.db.transaction::<_, EmployeeModel, AppError>(|txn| {
            Box::pin(async move {
                txn.execute(Statement::from_string(txn.get_database_backend(), format!("SET search_path TO {}", tenant_id))).await?;
                let active_model: model::ActiveModel = employee.into_active_model();
                let updated = active_model.update(txn).await?;
                Ok(updated)
            })
        })
        .await
        .map_err(|e| AppError::Database(e.to_string()))
    }

    async fn find_latest_code(&self, tenant_id: &str) -> AppResult<Option<String>> {
        let tenant_id = tenant_id.to_string();
        self.db.transaction::<_, Option<String>, AppError>(|txn| {
            Box::pin(async move {
                txn.execute(Statement::from_string(txn.get_database_backend(), format!("SET search_path TO {}", tenant_id))).await?;
                let model = Entity::find()
                    .order_by_desc(model::Column::EmployeeNo)
                    .one(txn)
                    .await?;
                
                Ok(model.map(|m| m.employee_no))
            })
        })
        .await
        .map_err(|e| AppError::Database(e.to_string()))
    }
}
