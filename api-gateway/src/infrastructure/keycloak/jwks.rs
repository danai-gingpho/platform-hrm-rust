use std::sync::Arc;
use std::time::Duration;

use jsonwebtoken::{jwk::JwkSet, DecodingKey};
use moka::future::Cache;
use serde::Deserialize;

use crate::domain::errors::{GatewayError, GatewayResult};

/// kid → DecodingKey, with TTL refresh.
#[derive(Clone)]
pub struct JwksCache {
    keys: Cache<String, Arc<DecodingKey>>,
    jwks_url: String,
    http: reqwest::Client,
}

#[derive(Debug, Deserialize)]
struct RawJwks {
    keys: Vec<serde_json::Value>,
}

impl JwksCache {
    pub fn new(jwks_url: impl Into<String>, ttl_secs: u64) -> Self {
        Self {
            keys: Cache::builder()
                .time_to_live(Duration::from_secs(ttl_secs))
                .max_capacity(64)
                .build(),
            jwks_url: jwks_url.into(),
            http: reqwest::Client::builder()
                .timeout(Duration::from_secs(5))
                .build()
                .expect("reqwest client"),
        }
    }

    pub async fn get(&self, kid: &str) -> GatewayResult<Arc<DecodingKey>> {
        if let Some(k) = self.keys.get(kid).await {
            return Ok(k);
        }
        // Cache miss → refresh full JWKS.
        self.refresh().await?;
        self.keys
            .get(kid)
            .await
            .ok_or_else(|| GatewayError::InvalidToken(format!("unknown kid '{kid}'")))
    }

    pub async fn refresh(&self) -> GatewayResult<()> {
        let raw: RawJwks = self
            .http
            .get(&self.jwks_url)
            .send()
            .await
            .map_err(|e| GatewayError::JwksFetch(e.to_string()))?
            .error_for_status()
            .map_err(|e| GatewayError::JwksFetch(e.to_string()))?
            .json()
            .await
            .map_err(|e| GatewayError::JwksFetch(e.to_string()))?;

        // Keycloak returns JWKs whose JSON we can map straight into jsonwebtoken's JwkSet.
        let set: JwkSet = serde_json::from_value(serde_json::json!({ "keys": raw.keys }))
            .map_err(|e| GatewayError::JwksFetch(format!("parse jwks: {e}")))?;

        let mut count = 0usize;
        for jwk in &set.keys {
            let kid = match jwk.common.key_id.clone() {
                Some(k) => k,
                None => continue,
            };
            let key = DecodingKey::from_jwk(jwk)
                .map_err(|e| GatewayError::JwksFetch(format!("decode jwk {kid}: {e}")))?;
            self.keys.insert(kid, Arc::new(key)).await;
            count += 1;
        }
        tracing::debug!(loaded = count, "refreshed JWKS");
        Ok(())
    }
}
