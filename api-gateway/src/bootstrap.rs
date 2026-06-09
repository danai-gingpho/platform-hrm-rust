use std::collections::HashMap;
use std::sync::Arc;

use crate::config::Config;
use crate::domain::auth::policy::RouteRule;
use crate::infrastructure::grpc::{
    auth::auth_service_client::AuthServiceClient,
    hris::{
        employee_service_client::EmployeeServiceClient,
        legal_entity_service_client::LegalEntityServiceClient,
        branch_service_client::BranchServiceClient,
    },
    platform::platform_service_client::PlatformServiceClient,
    owner::owner_auth_service_client::OwnerAuthServiceClient,
};
use crate::infrastructure::http_client::upstream::UpstreamRegistry;
use crate::infrastructure::keycloak::{jwks::JwksCache, verifier::JwtVerifier};
use crate::interface::http::state::AppState;
use tonic::transport::Channel;

pub async fn build_state(cfg: &Config) -> anyhow::Result<AppState> {
    // Keycloak JWKS + verifier
    let jwks = JwksCache::new(&cfg.keycloak.jwks_url, cfg.keycloak.jwks_cache_ttl_secs);
    // Warm the cache so the first request doesn't pay the JWKS fetch latency.
    if let Err(e) = jwks.refresh().await {
        tracing::warn!(error = %e, "initial JWKS warm-up failed; will retry on first auth request");
    }
    let verifier = Arc::new(JwtVerifier::new(
        jwks,
        &cfg.keycloak.issuer,
        &cfg.keycloak.audience,
        &cfg.keycloak.client_id,
    ));

    // Upstream registry
    let mut targets: HashMap<String, String> = HashMap::new();
    for u in &cfg.upstream {
        targets.insert(u.name.clone(), u.target.clone());
    }
    let upstreams = Arc::new(UpstreamRegistry::new(targets, cfg.server.timeout_ms));

    // Route table — compile RouteConfig → RouteRule, validate upstream names.
    let upstream_names: std::collections::HashSet<&str> =
        cfg.upstream.iter().map(|u| u.name.as_str()).collect();
    let mut routes = Vec::with_capacity(cfg.route.len());
    for rc in &cfg.route {
        if !upstream_names.contains(rc.upstream.as_str()) {
            anyhow::bail!(
                "route '{}' references unknown upstream '{}'",
                rc.pattern,
                rc.upstream
            );
        }
        routes.push(RouteRule::compile(rc.clone())?);
    }
    tracing::info!(count = routes.len(), "loaded route policies");

    // Initialize gRPC Clients
    let auth_channel = Channel::from_shared(cfg.grpc.auth_url.clone())?.connect().await?;
    let hris_channel = Channel::from_shared(cfg.grpc.hris_url.clone())?.connect().await?;
    let platform_channel = Channel::from_shared(cfg.grpc.platform_url.clone())?.connect().await?;
    let owner_channel = Channel::from_shared(cfg.grpc.owner_url.clone())?.connect().await?;

    Ok(AppState {
        verifier,
        upstreams,
        routes: Arc::new(routes),
        auth_client: AuthServiceClient::new(auth_channel),
        hris_employee_client: EmployeeServiceClient::new(hris_channel.clone()),
        hris_legal_entity_client: LegalEntityServiceClient::new(hris_channel.clone()),
        hris_branch_client: BranchServiceClient::new(hris_channel),
        platform_client: PlatformServiceClient::new(platform_channel),
        owner_client: OwnerAuthServiceClient::new(owner_channel),
    })
}
