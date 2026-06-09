use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub created_at: DateTime<Utc>,
}

impl Permission {
    pub fn new(name: String, code: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            code,
            created_at: Utc::now(),
        }
    }
}
