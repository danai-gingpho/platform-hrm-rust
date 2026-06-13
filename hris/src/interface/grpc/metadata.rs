use tonic::{Request, Status};

pub fn extract_tenant_id<T>(req: &Request<T>) -> Result<String, Status> {
    req.metadata()
        .get("x-tenant-id")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .ok_or_else(|| Status::unauthenticated("Missing x-tenant-id metadata"))
}
