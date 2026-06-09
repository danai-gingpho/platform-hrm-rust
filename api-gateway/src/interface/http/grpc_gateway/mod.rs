pub mod auth;
pub mod hris;
pub mod platform;
pub mod owner;

use axum::Router;
use crate::interface::http::state::AppState;

pub fn routes(state: AppState) -> Router<AppState> {
    Router::new()
        .nest("/api/v1/auth", auth::routes(state.clone()))
        .nest("/api/v1/platform", platform::routes(state.clone()))
        .nest("/api/v1/owner", owner::routes(state.clone()))
        .nest("/api/v1/hris", hris::routes(state))
}
