use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

use crate::domain::errors::GatewayError;

impl IntoResponse for GatewayError {
    fn into_response(self) -> Response {
        let (status, msg) = match &self {
            GatewayError::MissingToken => (StatusCode::UNAUTHORIZED, self.to_string()),
            GatewayError::InvalidToken(_) => (StatusCode::UNAUTHORIZED, self.to_string()),
            GatewayError::TokenExpired => (StatusCode::UNAUTHORIZED, self.to_string()),
            GatewayError::Forbidden(_) => (StatusCode::FORBIDDEN, self.to_string()),
            GatewayError::RouteNotFound { .. } => (StatusCode::NOT_FOUND, self.to_string()),
            GatewayError::UnknownUpstream(_) => (StatusCode::BAD_GATEWAY, self.to_string()),
            GatewayError::Upstream(_) => (StatusCode::BAD_GATEWAY, self.to_string()),
            GatewayError::UpstreamTimeout => (StatusCode::GATEWAY_TIMEOUT, self.to_string()),
            GatewayError::JwksFetch(_) => {
                tracing::error!(error = %self, "jwks fetch failed");
                (
                    StatusCode::SERVICE_UNAVAILABLE,
                    "auth temporarily unavailable".to_owned(),
                )
            }
            GatewayError::Internal(e) => {
                tracing::error!(error = ?e, "gateway internal error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal error".to_owned(),
                )
            }
        };
        (status, Json(json!({ "error": msg }))).into_response()
    }
}
