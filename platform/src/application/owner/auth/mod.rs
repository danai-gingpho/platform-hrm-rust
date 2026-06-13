use crate::domain::owner::staff::{Staff, StaffRepository};
use crate::domain::owner::user_role::StaffRoleRepository;
use crate::domain::owner::role_permission::RolePermissionRepository;
use crate::domain::owner::permission::PermissionRepository;
use anyhow::{Result, anyhow};
use bcrypt::verify;
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use chrono::{Utc, Duration};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    pub permissions: Vec<String>,
}

pub struct AuthService {
    staff_repo: Arc<dyn StaffRepository>,
    staff_role_repo: Arc<dyn StaffRoleRepository>,
    role_perm_repo: Arc<dyn RolePermissionRepository>,
    perm_repo: Arc<dyn PermissionRepository>,
    jwt_secret: String,
}

impl AuthService {
    pub fn new(
        staff_repo: Arc<dyn StaffRepository>,
        staff_role_repo: Arc<dyn StaffRoleRepository>,
        role_perm_repo: Arc<dyn RolePermissionRepository>,
        perm_repo: Arc<dyn PermissionRepository>,
        jwt_secret: String,
    ) -> Self {
        Self {
            staff_repo,
            staff_role_repo,
            role_perm_repo,
            perm_repo,
            jwt_secret,
        }
    }

    pub async fn login(&self, email: &str, password: &str) -> Result<(String, Staff)> {
        let staff = self.staff_repo.find_by_email(email).await?
            .ok_or_else(|| anyhow!("Invalid credentials"))?;

        if !staff.is_active {
            return Err(anyhow!("Account is disabled"));
        }

        if !verify(password, &staff.password_hash)? {
            return Err(anyhow!("Invalid credentials"));
        }

        // Get permissions
        let mut permissions = Vec::new();
        let role_ids = self.staff_role_repo.find_roles_by_staff_id(staff.id).await?;
        for role_id in role_ids {
            let perm_ids = self.role_perm_repo.find_permissions_by_role_id(role_id).await?;
            for perm_id in perm_ids {
                if let Some(perm) = self.perm_repo.find_by_id(perm_id).await? {
                    permissions.push(perm.code);
                }
            }
        }

        // Generate JWT
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .expect("valid timestamp")
            .timestamp();

        let claims = Claims {
            sub: staff.id.to_string(),
            exp: expiration as usize,
            iat: Utc::now().timestamp() as usize,
            permissions,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        )?;

        Ok((token, staff))
    }
}
