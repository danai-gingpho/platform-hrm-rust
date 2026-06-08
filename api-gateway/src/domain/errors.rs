use thiserror::Error;

#[derive(Debug, Error)]
pub enum GatewayError {
    #[error("missing or malformed Authorization header")]
    MissingToken,

    #[error("invalid token: {0}")]
    InvalidToken(String),

    #[error("token expired")]
    TokenExpired,

    #[error("forbidden: {0}")]
    Forbidden(String),

    #[error("no route matches {method} {path}")]
    RouteNotFound { method: String, path: String },

    #[error("upstream not configured: {0}")]
    UnknownUpstream(String),

    #[error("upstream error: {0}")]
    Upstream(String),

    #[error("upstream timeout")]
    UpstreamTimeout,

    #[error("jwks fetch failed: {0}")]
    JwksFetch(String),

    #[error("internal: {0}")]
    Internal(#[from] anyhow::Error),
}

pub type GatewayResult<T> = Result<T, GatewayError>;
