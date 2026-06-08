use sea_orm::{DbErr, ConnectionTrait, Schema};
use crate::domain::company::entity::Entity as Company;
use crate::domain::user::entity::Entity as User;
use crate::domain::company_user::entity::Entity as CompanyUser;

pub async fn run_migrations(db: &impl ConnectionTrait) -> Result<(), DbErr> {
    let builder = db.get_database_backend();
    let schema = Schema::new(builder);

    // Create companies table
    let mut stmt = schema.create_table_from_entity(Company);
    stmt.if_not_exists();
    db.execute(builder.build(&stmt)).await?;

    // Create users table
    let mut stmt = schema.create_table_from_entity(User);
    stmt.if_not_exists();
    db.execute(builder.build(&stmt)).await?;

    // Create company_users mapping table
    let mut stmt = schema.create_table_from_entity(CompanyUser);
    stmt.if_not_exists();
    db.execute(builder.build(&stmt)).await?;

    println!("Migrations applied successfully.");
    Ok(())
}
