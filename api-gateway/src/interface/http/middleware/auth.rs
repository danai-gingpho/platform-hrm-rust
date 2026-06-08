use axum::{
    extract::Request,
    http::header,
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::application::auth::{check_policy, verify_token};
use crate::domain::auth::claims::AuthContext;
use crate::domain::auth::policy::RouteRule;
use crate::domain::errors::{GatewayError, GatewayResult};
use crate::interface::http::state::AppState;

#[derive(Clone)]
pub struct MatchedRoute(pub RouteRule);

/// Combined route-resolution + auth + authz middleware.
///
/// Order:
///   1. Find matching RouteRule (method + path); 404 if none.
///   2. If policy is public → skip token verification.
///   3. Else extract Bearer token, verify against Keycloak JWKS, build AuthContext.
///   4. check_policy(AuthContext, policy) → 403 if not allowed.
///   5. Insert MatchedRoute + AuthContext into request extensions for downstream
///      (the proxy handler).
pub async fn middleware(state: AppState, req: Request, next: Next) -> Response {
    match run(state, req, next).await {
        Ok(r) => r,
        Err(e) => e.into_response(),
    }
}

async fn run(state: AppState, mut req: Request, next: Next) -> Result<Response, GatewayError> {
    let method = req.method().clone();
    let path = req.uri().path().to_owned();

    let rule = state
        .routes
        .iter()
        .find(|r| r.matches(&method, &path))
        .cloned()
        .ok_or_else(|| GatewayError::RouteNotFound {
            method: method.to_string(),
            path: path.clone(),
        })?;

    let auth_ctx: Option<AuthContext> = if rule.policy.public {
        None
    } else {
        let bearer = extract_bearer(&req)?;
        Some(verify_token::execute(state.verifier.clone(), &bearer).await?)
    };

    check_policy::execute(&rule.policy, auth_ctx.as_ref())?;

    if let Some(ctx) = auth_ctx.clone() {
        req.extensions_mut().insert(ctx);
    }
    req.extensions_mut().insert(MatchedRoute(rule));

    Ok(next.run(req).await)
}

fn extract_bearer(req: &Request) -> GatewayResult<String> {
    let header_val = req
        .headers()
        .get(header::AUTHORIZATION)
        .ok_or(GatewayError::MissingToken)?
        .to_str()
        .map_err(|_| GatewayError::MissingToken)?;

    let bearer = header_val
        .strip_prefix("Bearer ")
        .or_else(|| header_val.strip_prefix("bearer "))
        .ok_or(GatewayError::MissingToken)?
        .trim();

    if bearer.is_empty() {
        return Err(GatewayError::MissingToken);
    }
    Ok(bearer.to_owned())
}
