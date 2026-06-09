use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Staff {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Staff {
    pub fn new(email: String, password_hash: String, first_name: Option<String>, last_name: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            email,
            password_hash,
            first_name,
            last_name,
            is_active: true,
            created_at: now,
            updated_at: now,
        }
    }
}
