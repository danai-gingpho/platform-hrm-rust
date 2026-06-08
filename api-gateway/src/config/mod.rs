use std::path::Path;

use serde::Deserialize;

use crate::domain::auth::policy::RouteConfig;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub keycloak: KeycloakConfig,
    pub upstream: Vec<UpstreamConfig>,
    pub route: Vec<RouteConfig>,
    pub grpc: GrpcConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GrpcConfig {
    pub auth_url: String,
    pub hris_url: String,
    pub platform_url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub addr: String,
    #[serde(default = "default_timeout_ms")]
    pub timeout_ms: u64,
    #[serde(default = "default_rate_limit_rps")]
    pub rate_limit_rps: u64,
    #[serde(default = "default_rate_limit_burst")]
    pub rate_limit_burst: u32,
}

fn default_timeout_ms() -> u64 {
    30_000
}

fn default_rate_limit_rps() -> u64 {
    100
}

fn default_rate_limit_burst() -> u32 {
    200
}

#[derive(Debug, Deserialize, Clone)]
pub struct KeycloakConfig {
    pub issuer: String,
    pub audience: String,
    pub jwks_url: String,
    #[serde(default = "default_jwks_ttl")]
    pub jwks_cache_ttl_secs: u64,
    pub client_id: String,
}

fn default_jwks_ttl() -> u64 {
    3600
}

#[derive(Debug, Deserialize, Clone)]
pub struct UpstreamConfig {
    pub name: String,
    pub target: String,
    #[serde(default)]
    pub strip_prefix: String,
}

impl Config {
    pub fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let raw = std::fs::read_to_string(path.as_ref())
            .map_err(|e| anyhow::anyhow!("read config {:?}: {e}", path.as_ref()))?;
        let cfg: Self = toml::from_str(&raw).map_err(|e| anyhow::anyhow!("parse config: {e}"))?;
        Ok(cfg)
    }
}
