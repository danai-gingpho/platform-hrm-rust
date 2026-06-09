use async_trait::async_trait;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, ActiveModelTrait, Set};
use uuid::Uuid;
use crate::domain::staff::{Staff, StaffRepository};
use crate::infrastructure::db::staff::model::Entity as StaffEntity;
use crate::infrastructure::db::staff::model;
use anyhow::Result;

pub struct SeaOrmStaffRepository {
    db: DatabaseConnection,
}

impl SeaOrmStaffRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl StaffRepository for SeaOrmStaffRepository {
    async fn create(&self, staff: &Staff) -> Result<()> {
        let active_model = model::ActiveModel {
            id: Set(staff.id),
            email: Set(staff.email.clone()),
            password_hash: Set(staff.password_hash.clone()),
            first_name: Set(staff.first_name.clone()),
            last_name: Set(staff.last_name.clone()),
            is_active: Set(staff.is_active),
            created_at: Set(staff.created_at.into()),
            updated_at: Set(staff.updated_at.into()),
        };
        active_model.insert(&self.db).await?;
        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Staff>> {
        let model = StaffEntity::find_by_id(id).one(&self.db).await?;
        Ok(model.map(|m| Staff {
            id: m.id,
            email: m.email,
            password_hash: m.password_hash,
            first_name: m.first_name,
            last_name: m.last_name,
            is_active: m.is_active,
            created_at: m.created_at.into(),
            updated_at: m.updated_at.into(),
        }))
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<Staff>> {
        let model = StaffEntity::find()
            .filter(model::Column::Email.eq(email))
            .one(&self.db)
            .await?;
        Ok(model.map(|m| Staff {
            id: m.id,
            email: m.email,
            password_hash: m.password_hash,
            first_name: m.first_name,
            last_name: m.last_name,
            is_active: m.is_active,
            created_at: m.created_at.into(),
            updated_at: m.updated_at.into(),
        }))
    }

    async fn update(&self, staff: &Staff) -> Result<()> {
        let model = StaffEntity::find_by_id(staff.id).one(&self.db).await?;
        if let Some(m) = model {
            let mut active_model: model::ActiveModel = m.into();
            active_model.email = Set(staff.email.clone());
            active_model.password_hash = Set(staff.password_hash.clone());
            active_model.first_name = Set(staff.first_name.clone());
            active_model.last_name = Set(staff.last_name.clone());
            active_model.is_active = Set(staff.is_active);
            active_model.updated_at = Set(staff.updated_at.into());
            active_model.update(&self.db).await?;
        }
        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        StaffEntity::delete_by_id(id).exec(&self.db).await?;
        Ok(())
    }
}
