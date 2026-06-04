use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
    http::{HeaderMap, StatusCode},
};
use uuid::Uuid;
use crate::domain::shared::context::TenantContext;

pub async fn extract_tenant_context(
    headers: HeaderMap,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let tenant_id = headers.get("x-tenant-id")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| Uuid::parse_str(s).ok());

    let company_id = headers.get("x-company-id")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| Uuid::parse_str(s).ok());

    let context = TenantContext {
        tenant_id,
        company_id,
    };

    req.extensions_mut().insert(context);

    Ok(next.run(req).await)
}
