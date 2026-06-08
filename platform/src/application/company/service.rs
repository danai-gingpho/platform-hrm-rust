use crate::domain::company::entity::Model as CompanyModel;
use sea_orm::{DatabaseConnection, ActiveModelTrait, Set, ConnectionTrait};
use uuid::Uuid;
use chrono::Utc;
use crate::proto::CreateCompanyRequest;

pub struct CompanyService {
    db: std::sync::Arc<DatabaseConnection>,
}

impl CompanyService {
    pub fn new(db: std::sync::Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    pub async fn create_company(&self, req: CreateCompanyRequest) -> anyhow::Result<CompanyModel> {
        use crate::domain::company::entity::ActiveModel;

        let id = Uuid::new_v4();
        let schema_name = format!("company_{}", req.code.to_lowercase());

        // 1. Create schema in DB
        let sql = format!("CREATE SCHEMA IF NOT EXISTS {}", schema_name);
        self.db.execute(sea_orm::Statement::from_string(
            sea_orm::DatabaseBackend::Postgres,
            sql,
        )).await?;

        // 2. Save company info to Central DB (public schema)
        let company = ActiveModel {
            id: Set(id),
            company_name: Set(req.name),
            company_code: Set(req.code),
            schema_name: Set(schema_name),
            status: Set("active".to_string()),
            created_at: Set(Utc::now().into()),
            updated_at: Set(Utc::now().into()),
            ..Default::default()
        };

        let saved = company.insert(self.db.as_ref()).await?;
        Ok(saved)
    }
}
