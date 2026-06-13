use tonic::{Request, Response, Status};
use crate::proto::owner::owner_auth_service_server::OwnerAuthService;
use crate::proto::owner::*;
use crate::application::owner::auth::AuthService;
use crate::application::owner::staff::StaffService;
use std::sync::Arc;

pub struct AuthHandler {
    auth_service: Arc<AuthService>,
    staff_service: Arc<StaffService>,
}

impl AuthHandler {
    pub fn new(auth_service: Arc<AuthService>, staff_service: Arc<StaffService>) -> Self {
        Self {
            auth_service,
            staff_service,
        }
    }
}

#[tonic::async_trait]
impl OwnerAuthService for AuthHandler {
    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<AuthResponse>, Status> {
        let req = request.into_inner();
        match self.auth_service.login(&req.email, &req.password).await {
            Ok((token, staff)) => Ok(Response::new(AuthResponse {
                access_token: token,
                refresh_token: "".to_string(), // TODO
                staff: Some(StaffResponse {
                    id: staff.id.to_string(),
                    email: staff.email,
                    first_name: staff.first_name.unwrap_or_default(),
                    last_name: staff.last_name.unwrap_or_default(),
                    is_active: staff.is_active,
                }),
            })),
            Err(e) => Err(Status::unauthenticated(e.to_string())),
        }
    }

    async fn validate_token(
        &self,
        _request: Request<ValidateTokenRequest>,
    ) -> Result<Response<ValidateTokenResponse>, Status> {
        // TODO: Implement token validation logic
        Ok(Response::new(ValidateTokenResponse::default()))
    }

    async fn create_staff(
        &self,
        request: Request<CreateStaffRequest>,
    ) -> Result<Response<StaffResponse>, Status> {
        let req = request.into_inner();
        match self.staff_service.create_staff(
            req.email,
            req.password,
            Some(req.first_name),
            Some(req.last_name),
        ).await {
            Ok(staff) => Ok(Response::new(StaffResponse {
                id: staff.id.to_string(),
                email: staff.email,
                first_name: staff.first_name.unwrap_or_default(),
                last_name: staff.last_name.unwrap_or_default(),
                is_active: staff.is_active,
            })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn get_staff(
        &self,
        request: Request<GetStaffRequest>,
    ) -> Result<Response<StaffResponse>, Status> {
        let id = uuid::Uuid::parse_str(&request.into_inner().id)
            .map_err(|_| Status::invalid_argument("Invalid ID format"))?;
        
        match self.staff_service.get_staff(id).await {
            Ok(Some(staff)) => Ok(Response::new(StaffResponse {
                id: staff.id.to_string(),
                email: staff.email,
                first_name: staff.first_name.unwrap_or_default(),
                last_name: staff.last_name.unwrap_or_default(),
                is_active: staff.is_active,
            })),
            Ok(None) => Err(Status::not_found("Staff not found")),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn create_role(
        &self,
        _request: Request<CreateRoleRequest>,
    ) -> Result<Response<RoleResponse>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    async fn create_permission(
        &self,
        _request: Request<CreatePermissionRequest>,
    ) -> Result<Response<PermissionResponse>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    async fn assign_role_to_staff(
        &self,
        _request: Request<AssignRoleRequest>,
    ) -> Result<Response<ActionResponse>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    async fn assign_permission_to_role(
        &self,
        _request: Request<AssignPermissionRequest>,
    ) -> Result<Response<ActionResponse>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    async fn get_system_status(
        &self,
        _request: Request<SystemStatusRequest>,
    ) -> Result<Response<SystemStatusResponse>, Status> {
        Ok(Response::new(SystemStatusResponse {
            status: "ok".to_string(),
            active_tenants: 0, // TODO: calculate from platform db
            version: "0.1.0".to_string(),
        }))
    }
}
