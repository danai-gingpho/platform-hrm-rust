use crate::domain::owner::staff::{Staff, StaffRepository};
use anyhow::Result;
use std::sync::Arc;
use uuid::Uuid;
use bcrypt::{hash, DEFAULT_COST};

pub struct StaffService {
    staff_repo: Arc<dyn StaffRepository>,
}

impl StaffService {
    pub fn new(staff_repo: Arc<dyn StaffRepository>) -> Self {
        Self { staff_repo }
    }

    pub async fn create_staff(
        &self,
        email: String,
        password: String,
        first_name: Option<String>,
        last_name: Option<String>,
    ) -> Result<Staff> {
        let password_hash = hash(password, DEFAULT_COST)?;
        let staff = Staff::new(email, password_hash, first_name, last_name);
        self.staff_repo.create(&staff).await?;
        Ok(staff)
    }

    pub async fn get_staff(&self, id: Uuid) -> Result<Option<Staff>> {
        self.staff_repo.find_by_id(id).await
    }
}
