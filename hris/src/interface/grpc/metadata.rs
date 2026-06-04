use tonic::{Request, Status};
use uuid::Uuid;
use crate::domain::shared::context::TenantContext;

pub fn extract_tenant_context<T>(req: &Request<T>) -> Result<TenantContext, Status> {
    let tenant_id = req.metadata().get("x-tenant-id")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| Uuid::parse_str(s).ok());

    let company_id = req.metadata().get("x-company-id")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| Uuid::parse_str(s).ok());

    Ok(TenantContext {
        tenant_id,
        company_id,
    })
}
