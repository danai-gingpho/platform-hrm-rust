pub mod staff;
pub mod role;
pub mod permission;
pub mod user_role;
pub mod role_permission;
pub mod seeder;

use sea_orm::{Database, DatabaseConnection};
use std::env;

pub async fn create_connection() -> Result<DatabaseConnection, sea_orm::DbErr> {
    let database_url = env::var("OWNER_DATABASE_URL").expect("OWNER_DATABASE_URL must be set");
    Database::connect(database_url).await
}
