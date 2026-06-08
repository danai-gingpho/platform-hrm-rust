use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
    http::{StatusCode},
};
use std::sync::Arc;
use sea_orm::{DatabaseConnection, ConnectionTrait, Statement, DatabaseBackend};

pub async fn company_middleware(
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let company_id = req.headers().get("x-company-id")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    // Logic to get schema_name from company_id (from Central DB)
    // For simplicity, let's assume we get the schema name directly or from a cache
    let schema_name = if let Some(ref id) = company_id {
        // In real app: fetch from DB or Cache
        format!("company_{}", id) 
    } else {
        "public".to_string()
    };

    let db = req.extensions().get::<Arc<DatabaseConnection>>()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    // Switch schema
    let sql = format!("SET search_path TO {}, public", schema_name);
    db.execute(Statement::from_string(DatabaseBackend::Postgres, sql))
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(next.run(req).await)
}
