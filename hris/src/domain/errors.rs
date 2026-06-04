use sea_orm::DbErr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("not found")]
    NotFound,

    #[error("validation error: {0}")]
    Validation(String),

    #[error("conflict: {0}")]
    Conflict(String),

    #[error("bad request: {0}")]
    BadRequest(String),

    #[error("unauthorized")]
    Unauthorized,

    #[error("missing tenant context")]
    MissingTenantContext,

    #[error("tenant not found")]
    TenantNotFound,

    #[error("internal error: {0}")]
    Internal(String),

    #[error("database error: {0}")]
    Db(#[from] DbErr),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type AppResult<T> = Result<T, AppError>;
