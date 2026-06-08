use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
};

pub async fn extract_tenant_context(
    req: Request,
    next: Next,
) -> Result<Response, axum::http::StatusCode> {
    // Middleware remains as a placeholder or for future logic (e.g., Schema switching)
    // but no longer injects TenantContext extension as it's not used by services.
    Ok(next.run(req).await)
}
