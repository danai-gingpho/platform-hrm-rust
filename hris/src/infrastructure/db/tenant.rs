use std::cell::RefCell;
use sea_orm::{DatabaseConnection, TransactionTrait, Statement, ConnectionTrait, DatabaseBackend, DbErr};
use crate::domain::errors::{AppError, AppResult};

// Task-local storage for Tenant ID
tokio::task_local! {
    pub static TENANT_ID: RefCell<Option<String>>;
}

pub struct TenantDatabase {
    inner: DatabaseConnection,
}

impl TenantDatabase {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { inner: conn }
    }

    /// Executes a database operation within a transaction that has the search_path set to the current tenant.
    pub async fn with_tenant_schema<F, T, Fut>(&self, op: F) -> AppResult<T>
    where
        F: FnOnce(DatabaseConnection) -> Fut,
        Fut: std::future::Future<Output = Result<T, DbErr>>,
    {
        // Get tenant_id from task-local
        let tenant_id = TENANT_ID.with(|id| id.borrow().clone())
            .ok_or_else(|| AppError::Internal("Missing tenant context in database operation".to_string()))?;

        // Start a transaction
        let txn = self.inner.begin().await?;

        // Set search_path
        let sql = format!("SET search_path TO \"{}\"", tenant_id);
        txn.execute(Statement::from_string(DatabaseBackend::Postgres, sql)).await?;

        // Run the operation
        // Note: We're passing the transaction as the connection
        let res = op(txn.into_connection()).await?;

        Ok(res)
    }

    /// Provides direct access to the connection if needed (bypass tenant isolation)
    pub fn get_connection(&self) -> &DatabaseConnection {
        &self.inner
    }
}
