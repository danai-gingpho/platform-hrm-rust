use std::sync::Arc;
use sea_orm::DatabaseConnection;
use uuid::Uuid;
use crate::domain::staff::{Staff, StaffRepository};
use crate::domain::role::{Role, RoleRepository};
use crate::domain::permission::{Permission, PermissionRepository};
use crate::domain::user_role::StaffRoleRepository;
use crate::domain::role_permission::RolePermissionRepository;
use crate::application::staff::StaffService;
use crate::infrastructure::db::staff::SeaOrmStaffRepository;
use crate::infrastructure::db::role::SeaOrmRoleRepository;
use crate::infrastructure::db::permission::SeaOrmPermissionRepository;
use crate::infrastructure::db::user_role::SeaOrmStaffRoleRepository;
use crate::infrastructure::db::role_permission::SeaOrmRolePermissionRepository;

pub async fn seed(db: DatabaseConnection) -> anyhow::Result<()> {
    // 1. Initialize Repositories
    let staff_repo = Arc::new(SeaOrmStaffRepository::new(db.clone()));
    let role_repo = Arc::new(SeaOrmRoleRepository::new(db.clone()));
    let perm_repo = Arc::new(SeaOrmPermissionRepository::new(db.clone()));
    let staff_role_repo = Arc::new(SeaOrmStaffRoleRepository::new(db.clone()));
    let role_perm_repo = Arc::new(SeaOrmRolePermissionRepository::new(db.clone()));

    // 2. Create Permissions
    let perms = vec![
        ("Manage Staff", "manage_staff"),
        ("Manage Companies", "manage_companies"),
        ("View System Status", "view_system_status"),
    ];

    let mut perm_ids = Vec::new();
    for (name, code) in perms {
        if let Ok(None) = perm_repo.find_by_code(code).await {
            let p = Permission::new(name.to_string(), code.to_string());
            let id = p.id;
            perm_repo.create(&p).await?;
            perm_ids.push(id);
        } else if let Ok(Some(p)) = perm_repo.find_by_code(code).await {
            perm_ids.push(p.id);
        }
    }

    // 3. Create Super Admin Role
    let role_name = "Super Admin";
    let role_id = if let Ok(None) = role_repo.find_by_name(role_name).await {
        let r = Role::new(role_name.to_string(), Some("Full platform access".into()));
        let id = r.id;
        role_repo.create(&r).await?;
        id
    } else {
        role_repo.find_by_name(role_name).await?.unwrap().id
    };

    // 4. Link Permissions to Role
    for p_id in perm_ids {
        let existing = role_perm_repo.find_permissions_by_role_id(role_id).await?;
        if !existing.contains(&p_id) {
            role_perm_repo.assign(role_id, p_id).await?;
        }
    }

    // 5. Create Super Admin Staff using StaffService
    let admin_email = "admin@platform.com";
    if let Ok(None) = staff_repo.find_by_email(admin_email).await {
        let staff_service = StaffService::new(staff_repo.clone());
        let admin = staff_service.create_staff(
            admin_email.to_string(),
            "password123".to_string(),
            Some("System".into()),
            Some("Admin".into()),
        ).await?;

        // 6. Assign Role to Staff
        staff_role_repo.assign(admin.id, role_id).await?;
        println!("Seeded Super Admin user: {}", admin_email);
    }

    Ok(())
}
