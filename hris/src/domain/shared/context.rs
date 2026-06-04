use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TenantContext {
    pub tenant_id: Option<Uuid>,
    pub company_id: Option<Uuid>,
}
