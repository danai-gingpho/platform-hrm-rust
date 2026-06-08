use tonic::{Request, Status};

pub fn extract_tenant_context<T>(_req: &Request<T>) -> Result<(), Status> {
    Ok(())
}
