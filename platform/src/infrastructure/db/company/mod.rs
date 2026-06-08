pub mod model;
use async_trait::async_trait;
use sea_orm::{DatabaseConnection, EntityTrait, ActiveModelTrait, IntoActiveModel, ColumnTrait, QueryFilter};
use uuid::Uuid;
use crate::domain::company::entity::Model as CompanyModel;
use crate::domain::company::repository::CompanyRepository;
pub use model::Entity;

pub struct SeaOrmCompanyRepository {
    db: DatabaseConnection,
}

impl SeaOrmCompanyRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl CompanyRepository for SeaOrmCompanyRepository {
    async fn create(&self, company: CompanyModel) -> anyhow::Result<CompanyModel> {
        let active_model = company.into_active_model();
        let saved = active_model.insert(&self.db).await?;
        Ok(saved)
    }

    async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<CompanyModel>> {
        let model = Entity::find_by_id(id).one(&self.db).await?;
        Ok(model)
    }

    async fn find_by_code(&self, code: &str) -> anyhow::Result<Option<CompanyModel>> {
        let model = Entity::find().filter(crate::domain::company::entity::Column::CompanyCode.eq(code)).one(&self.db).await?;
        Ok(model)
    }

    async fn update(&self, company: CompanyModel) -> anyhow::Result<CompanyModel> {
        let active_model = company.into_active_model();
        let updated = active_model.update(&self.db).await?;
        Ok(updated)
    }

    async fn delete(&self, id: Uuid) -> anyhow::Result<()> {
        Entity::delete_by_id(id).exec(&self.db).await?;
        Ok(())
    }
}
