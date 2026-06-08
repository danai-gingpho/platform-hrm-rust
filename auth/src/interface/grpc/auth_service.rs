use crate::proto::{
    auth_service_server::AuthService as ProtoAuthService,
    CreateRealmRequest, CreateRealmResponse,
    CreateUserRequest, UserResponse,
    CreateRoleRequest, RoleResponse,
    CreatePermissionRequest, PermissionResponse,
    AssignRoleRequest, AssignRoleResponse,
    AssignPermissionRequest, AssignPermissionResponse,
};
use crate::infrastructure::keycloak::KeycloakClient;
use crate::application::iam::rbac::RbacService;
use tonic::{Request, Response, Status};
use std::sync::Arc;

pub struct AuthServiceImpl {
    keycloak: Arc<KeycloakClient>,
    rbac_service: Arc<RbacService>,
}

impl AuthServiceImpl {
    pub fn new(keycloak: Arc<KeycloakClient>, rbac_service: Arc<RbacService>) -> Self {
        Self { keycloak, rbac_service }
    }
}

#[tonic::async_trait]
impl ProtoAuthService for AuthServiceImpl {
    async fn create_realm(&self, request: Request<CreateRealmRequest>) -> Result<Response<CreateRealmResponse>, Status> {
        let req = request.into_inner();
        self.keycloak.create_realm(&req.realm_name).await
            .map_err(|e| Status::internal(e.to_string()))?;
        Ok(Response::new(CreateRealmResponse { success: true }))
    }

    async fn create_user(&self, request: Request<CreateUserRequest>) -> Result<Response<UserResponse>, Status> {
        let req = request.into_inner();
        let keycloak_id = self.keycloak.create_user(&req.realm, &req.email, &req.first_name, &req.last_name).await
            .map_err(|e| Status::internal(e.to_string()))?;
        
        // In real app: also save to auth-service's own DB for local profile/cache
        Ok(Response::new(UserResponse {
            id: "".to_string(), // This would come from local DB
            email: req.email,
            keycloak_id,
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
