use tonic::{Request, Response, Status};
use crate::proto::{
    platform_service_server::PlatformService as ProtoPlatformService,
    CreateCompanyRequest, CompanyResponse,
    CreateUserRequest, UserResponse,
    CreateRoleRequest, RoleResponse,
    CreatePermissionRequest, PermissionResponse,
    AssignRoleRequest, AssignRoleResponse,
    AssignPermissionRequest, AssignPermissionResponse,
};
use crate::application::iam::service::PlatformService;
use crate::application::iam::rbac::RbacService;
use std::sync::Arc;
use uuid::Uuid;

pub struct PlatformServiceImpl {
    platform_service: Arc<PlatformService>,
    rbac_service: Arc<RbacService>,
}

impl PlatformServiceImpl {
    pub fn new(platform_service: Arc<PlatformService>, rbac_service: Arc<RbacService>) -> Self {
        Self { 
            platform_service,
            rbac_service,
        }
    }
}

#[tonic::async_trait]
impl ProtoPlatformService for PlatformServiceImpl {
    async fn create_company(&self, request: Request<CreateCompanyRequest>) -> Result<Response<CompanyResponse>, Status> {
        let saved = self.platform_service.create_company(request.into_inner()).await
            .map_err(|e| Status::internal(e.to_string()))?;
        Ok(Response::new(CompanyResponse {
            id: saved.id.to_string(),
            name: saved.company_name,
            code: saved.company_code,
            schema_name: saved.schema_name,
        }))
    }

    async fn get_company_db_credentials(&self, request: Request<crate::proto::GetCompanyDbCredentialsRequest>) -> Result<Response<crate::proto::CompanyDbCredentialsResponse>, Status> {
        let req = request.into_inner();
        let company_id = Uuid::parse_str(&req.company_id).map_err(|_| Status::invalid_argument("Invalid UUID"))?;

        let company = self.platform_service.get_company_db_credentials(company_id).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(crate::proto::CompanyDbCredentialsResponse {
            username: company.db_username.unwrap_or_default(),
            password: company.db_password.unwrap_or_default(),
            host: company.db_host.unwrap_or_default(),
            db_name: company.db_name.unwrap_or_default(),
            schema_name: company.schema_name,
        }))
    }

    async fn create_user(&self, request: Request<CreateUserRequest>) -> Result<Response<UserResponse>, Status> {
        let saved = self.platform_service.create_user(request.into_inner()).await
            .map_err(|e| Status::internal(e.to_string()))?;
        Ok(Response::new(UserResponse {
            id: saved.id.to_string(),
            email: saved.email,
            keycloak_id: saved.keycloak_id.unwrap_or_default(),
        }))
    }

    async fn create_role(&self, request: Request<CreateRoleRequest>) -> Result<Response<RoleResponse>, Status> {
        let saved = self.rbac_service.create_role(request.into_inner()).await
            .map_err(|e| Status::internal(e.to_string()))?;
        Ok(Response::new(RoleResponse {
            id: saved.id.to_string(),
            name: saved.name,
        }))
    }

    async fn create_permission(&self, request: Request<CreatePermissionRequest>) -> Result<Response<PermissionResponse>, Status> {
        let saved = self.rbac_service.create_permission(request.into_inner()).await
            .map_err(|e| Status::internal(e.to_string()))?;
        Ok(Response::new(PermissionResponse {
            id: saved.id.to_string(),
            name: saved.name,
            code: saved.code,
        }))
    }

    async fn assign_role_to_user(&self, request: Request<AssignRoleRequest>) -> Result<Response<AssignRoleResponse>, Status> {
        self.rbac_service.assign_role_to_user(request.into_inner()).await
            .map_err(|e| Status::internal(e.to_string()))?;
        Ok(Response::new(AssignRoleResponse { success: true }))
    }

    async fn assign_permission_to_role(&self, request: Request<AssignPermissionRequest>) -> Result<Response<AssignPermissionResponse>, Status> {
        self.rbac_service.assign_permission_to_role(request.into_inner()).await
            .map_err(|e| Status::internal(e.to_string()))?;
        Ok(Response::new(AssignPermissionResponse { success: true }))
    }
}
