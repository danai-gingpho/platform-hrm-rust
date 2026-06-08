use crate::proto::auth::auth_service_client::AuthServiceClient;
use crate::proto::auth::{
    CreateRoleRequest as AuthCreateRoleRequest,
    CreatePermissionRequest as AuthCreatePermissionRequest,
    AssignRoleRequest as AuthAssignRoleRequest,
    AssignPermissionRequest as AuthAssignPermissionRequest,
};
use crate::proto::{CreateRoleRequest, CreatePermissionRequest, AssignRoleRequest, AssignPermissionRequest};
use tonic::transport::Channel;

pub struct RbacService {
    auth_client: AuthServiceClient<Channel>,
}

impl RbacService {
    pub fn new(auth_client: AuthServiceClient<Channel>) -> Self {
        Self { auth_client }
    }

    pub async fn create_role(&self, req: CreateRoleRequest) -> anyhow::Result<crate::proto::RoleResponse> {
        let mut client = self.auth_client.clone();
        let resp = client.create_role(AuthCreateRoleRequest {
            name: req.name,
            description: req.description,
        }).await.map_err(|e| anyhow::anyhow!("Auth service error: {}", e.message()))?;
        
        let inner = resp.into_inner();
        Ok(crate::proto::RoleResponse {
            id: inner.id,
            name: inner.name,
        })
    }

    pub async fn create_permission(&self, req: CreatePermissionRequest) -> anyhow::Result<crate::proto::PermissionResponse> {
        let mut client = self.auth_client.clone();
        let resp = client.create_permission(AuthCreatePermissionRequest {
            name: req.name,
            code: req.code,
        }).await.map_err(|e| anyhow::anyhow!("Auth service error: {}", e.message()))?;
        
        let inner = resp.into_inner();
        Ok(crate::proto::PermissionResponse {
            id: inner.id,
            name: inner.name,
            code: inner.code,
        })
    }

    pub async fn assign_role_to_user(&self, req: AssignRoleRequest) -> anyhow::Result<()> {
        let mut client = self.auth_client.clone();
        client.assign_role_to_user(AuthAssignRoleRequest {
            user_id: req.user_id,
            role_id: req.role_id,
        }).await.map_err(|e| anyhow::anyhow!("Auth service error: {}", e.message()))?;
        Ok(())
    }

    pub async fn assign_permission_to_role(&self, req: AssignPermissionRequest) -> anyhow::Result<()> {
        let mut client = self.auth_client.clone();
        client.assign_permission_to_role(AuthAssignPermissionRequest {
            role_id: req.role_id,
            permission_id: req.permission_id,
        }).await.map_err(|e| anyhow::anyhow!("Auth service error: {}", e.message()))?;
        Ok(())
    }
}
