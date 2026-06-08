use crate::domain::company::entity::ActiveModel as CompanyActiveModel;
use crate::domain::user::entity::ActiveModel as UserActiveModel;
use crate::domain::company_user::entity::ActiveModel as CompanyUserActiveModel;
use crate::proto::auth::auth_service_client::AuthServiceClient;
use crate::proto::auth::{CreateRealmRequest, CreateUserRequest as AuthCreateUserRequest};
use crate::proto::{CreateCompanyRequest, CreateUserRequest};
use sea_orm::{DatabaseConnection, EntityTrait, ActiveModelTrait, Set, TransactionTrait, ConnectionTrait};
use uuid::Uuid;
use chrono::Utc;
use std::sync::Arc;
use tonic::transport::Channel;

pub struct PlatformService {
    db: Arc<DatabaseConnection>,
    auth_client: AuthServiceClient<Channel>,
}

impl PlatformService {
    pub fn new(db: Arc<DatabaseConnection>, auth_client: AuthServiceClient<Channel>) -> Self {
        Self { db, auth_client }
    }

    pub async fn create_company(&self, req: CreateCompanyRequest) -> anyhow::Result<crate::domain::company::entity::Model> {
        let id = Uuid::new_v4();
        let schema_name = format!("company_{}", req.code.to_lowercase());

        // 1. Call Auth Service to Create Realm
        let mut client = self.auth_client.clone();
        client.create_realm(CreateRealmRequest {
            realm_name: req.code.clone(),
        }).await.map_err(|e| anyhow::anyhow!("Auth service error: {}", e.message()))?;

        // 2. Create schema in DB
        let sql = format!("CREATE SCHEMA IF NOT EXISTS {}", schema_name);
        self.db.execute(sea_orm::Statement::from_string(
            sea_orm::DatabaseBackend::Postgres,
            sql,
        )).await?;

        // 3. Save to central DB with credentials
        // In real app: generate unique db user/pass and grant permissions
        let db_username = format!("user_{}", req.code.to_lowercase());
        let db_password = Uuid::new_v4().to_string(); // Placeholder password

        let company = CompanyActiveModel {
            id: Set(id),
            company_name: Set(req.name),
            company_code: Set(req.code.clone()),
            schema_name: Set(schema_name),
            status: Set("active".to_string()),
            keycloak_realm: Set(Some(req.code)),
            db_username: Set(Some(db_username)),
            db_password: Set(Some(db_password)),
            db_host: Set(Some("localhost".to_string())),
            db_name: Set(Some("hrm_companies".to_string())),
            created_at: Set(Utc::now().into()),
            updated_at: Set(Utc::now().into()),
            ..Default::default()
        };

        let saved = company.insert(self.db.as_ref()).await?;
        Ok(saved)
    }

    pub async fn get_company_db_credentials(&self, company_id: Uuid) -> anyhow::Result<crate::domain::company::entity::Model> {
        let company = crate::domain::company::entity::Entity::find_by_id(company_id)
            .one(self.db.as_ref())
            .await?
            .ok_or_else(|| anyhow::anyhow!("Company not found"))?;
        Ok(company)
    }

    pub async fn create_user(&self, req: CreateUserRequest) -> anyhow::Result<crate::domain::user::entity::Model> {
        let company_id = Uuid::parse_str(&req.company_id)?;
        
        // Find company to get realm
        let company = crate::domain::company::entity::Entity::find_by_id(company_id)
            .one(self.db.as_ref())
            .await?
            .ok_or_else(|| anyhow::anyhow!("Company not found"))?;

        let realm = company.keycloak_realm.ok_or_else(|| anyhow::anyhow!("Company has no realm"))?;

        // 1. Call Auth Service to Create User
        let mut client = self.auth_client.clone();
        let auth_resp = client.create_user(AuthCreateUserRequest {
            email: req.email.clone(),
            first_name: req.first_name.clone(),
            last_name: req.last_name.clone(),
            realm,
        }).await.map_err(|e| anyhow::anyhow!("Auth service error: {}", e.message()))?;
        
        let keycloak_id = auth_resp.into_inner().keycloak_id;

        // 2. Save user and mapping in DB (Transaction)
        let txn = self.db.begin().await?;

        let user_id = Uuid::new_v4();
        let user = UserActiveModel {
            id: Set(user_id),
            email: Set(req.email),
            first_name: Set(req.first_name),
            last_name: Set(req.last_name),
            keycloak_id: Set(Some(keycloak_id)),
            created_at: Set(Utc::now().into()),
            updated_at: Set(Utc::now().into()),
            ..Default::default()
        };
        let saved_user = user.insert(&txn).await?;

        let mapping = CompanyUserActiveModel {
            company_id: Set(company_id),
            user_id: Set(user_id),
        };
        mapping.insert(&txn).await?;

        txn.commit().await?;

        Ok(saved_user)
    }
}
