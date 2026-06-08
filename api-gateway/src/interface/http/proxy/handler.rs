use axum::{
    body::{Body, Bytes},
    extract::State,
    http::{header, HeaderMap, HeaderName, HeaderValue, Request, StatusCode, Uri, Method},
    response::Response,
};

use crate::domain::auth::claims::AuthContext;
use crate::domain::errors::{GatewayError, GatewayResult};
use crate::interface::http::middleware::auth::MatchedRoute;
use crate::interface::http::state::AppState;

const HOP_BY_HOP: &[&str] = &[
    "connection",
    "keep-alive",
    "proxy-authenticate",
    "proxy-authorization",
    "te",
    "trailers",
    "transfer-encoding",
    "upgrade",
    "host",
];

pub async fn proxy(
    State(state): State<AppState>,
    req: Request<Body>,
) -> Result<Response, GatewayError> {
    let matched = req
        .extensions()
        .get::<MatchedRoute>()
        .cloned()
        .ok_or_else(|| {
            GatewayError::Internal(anyhow::anyhow!(
                "missing MatchedRoute (auth layer not run?)"
            ))
        })?
        .0;

    let upstream_base = state.upstreams.target(&matched.upstream)?.to_owned();

    let (mut parts, body) = req.into_parts();

    let path_and_query = parts
        .uri
        .path_and_query()
        .map(|pq| pq.as_str())
        .unwrap_or("/");
    let upstream_uri: Uri = format!("{upstream_base}{path_and_query}").parse().map_err(
        |e: axum::http::uri::InvalidUri| {
            GatewayError::Internal(anyhow::anyhow!("build upstream uri: {e}"))
        },
    )?;

    // Strip hop-by-hop and Authorization (don't leak the bearer downstream).
    sanitize_headers(&mut parts.headers);

    // Inject identity headers for the upstream service.
    if let Some(ctx) = parts.extensions.get::<AuthContext>() {
        insert_str(&mut parts.headers, "x-auth-subject", &ctx.sub);
        if let Some(u) = &ctx.username {
            insert_str(&mut parts.headers, "x-auth-username", u);
        }
        if let Some(e) = &ctx.email {
            insert_str(&mut parts.headers, "x-auth-email", e);
        }
        let roles = ctx.roles_csv();
        if !roles.is_empty() {
            insert_str(&mut parts.headers, "x-auth-roles", &roles);
        }
    }

    let mut attempts = 0;
    let max_attempts = 3;
    let mut body_opt = Some(body);

    loop {
        attempts += 1;

        // Since we are streaming the body, we can only retry if there is no body
        // or if it's the first attempt and we haven't consumed it yet.
        // For simplicity and safety with streaming, we only retry GET/HEAD/OPTIONS
        // which typically don't have bodies.
        let can_retry = matches!(
            parts.method,
            Method::GET | Method::HEAD | Method::OPTIONS
        );

        let mut req_builder = state
            .upstreams
            .client
            .request(parts.method.clone(), upstream_uri.to_string());
        for (name, value) in parts.headers.iter() {
            req_builder = req_builder.header(name.as_str(), value.as_bytes());
        }

        // We need to be careful: we can only consume 'body' once.
        // If we want to retry, we'd need to buffer it or not consume it.
        // For GET requests, the body is usually empty.
        let current_body = if let Some(b) = body_opt.take() {
            reqwest::Body::wrap_stream(b.into_data_stream())
        } else {
            reqwest::Body::from(Bytes::new())
        };

        let upstream_resp = req_builder.body(current_body).send().await;

        match upstream_resp {
            Ok(resp) => {
                let status = resp.status();
                if attempts < max_attempts
                    && can_retry
                    && (status == StatusCode::BAD_GATEWAY
                        || status == StatusCode::SERVICE_UNAVAILABLE
                        || status == StatusCode::GATEWAY_TIMEOUT)
                {
                    tracing::warn!(
                        method = %parts.method,
                        uri = %upstream_uri,
                        status = %status.as_u16(),
                        attempt = attempts,
                        "upstream transient error, retrying"
                    );
                    tokio::time::sleep(std::time::Duration::from_millis(100 * attempts)).await;
                    continue;
                }
                return build_response(resp).await;
            }
            Err(e)
                if attempts < max_attempts && can_retry && (e.is_timeout() || e.is_connect()) =>
            {
                tracing::warn!(
                    method = %parts.method,
                    uri = %upstream_uri,
                    error = %e,
                    attempt = attempts,
                    "upstream connection error, retrying"
                );
                tokio::time::sleep(std::time::Duration::from_millis(100 * attempts)).await;
                continue;
            }
            Err(e) => return Err(map_reqwest_err(e)),
        }
    }
}

fn sanitize_headers(headers: &mut HeaderMap) {
    headers.remove(header::AUTHORIZATION);
    for h in HOP_BY_HOP {
        headers.remove(*h);
    }
}

fn insert_str(headers: &mut HeaderMap, name: &'static str, value: &str) {
    if let Ok(v) = HeaderValue::from_str(value) {
        headers.insert(HeaderName::from_static(name), v);
    }
}

fn map_reqwest_err(e: reqwest::Error) -> GatewayError {
    if e.is_timeout() {
        GatewayError::UpstreamTimeout
    } else {
        GatewayError::Upstream(e.to_string())
    }
}

async fn build_response(resp: reqwest::Response) -> GatewayResult<Response> {
    let status = StatusCode::from_u16(resp.status().as_u16())
        .map_err(|e| GatewayError::Internal(anyhow::anyhow!("upstream status: {e}")))?;
    let headers = resp.headers().clone();

    let mut builder = Response::builder().status(status);
    for (name, value) in headers.iter() {
        let lower = name.as_str().to_ascii_lowercase();
        if HOP_BY_HOP.contains(&lower.as_str()) {
            continue;
        }
        builder = builder.header(name.as_str(), value.as_bytes());
    }

    let stream = resp.bytes_stream();
    builder
        .body(Body::from_stream(stream))
        .map_err(|e| GatewayError::Internal(anyhow::anyhow!("build response: {e}")))
}
