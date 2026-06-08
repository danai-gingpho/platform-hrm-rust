pub mod handlers;

use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use crate::application::iam::service::PlatformService;
use crate::application::iam::rbac::RbacService;

#[derive(Clone)]
pub struct AppState {
    pub platform_service: Arc<PlatformService>,
    pub rbac_service: Arc<RbacService>,
}

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/companies", post(handlers::company_handler::create_company))
        .with_state(state.platform_service.clone())
        .nest("/iam", iam_router(state))
}

fn iam_router(state: AppState) -> Router {
    Router::new()
        .route("/users", post(handlers::iam_handler::create_user))
        .route("/roles", post(handlers::iam_handler::create_role))
        .route("/permissions", post(handlers::iam_handler::create_permission))
        .route("/assign-role", post(handlers::iam_handler::assign_role))
        .route("/assign-permission", post(handlers::iam_handler::assign_permission))
        .with_state((state.platform_service, state.rbac_service))
}
