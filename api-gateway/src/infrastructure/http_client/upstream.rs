use std::collections::HashMap;
use std::time::Duration;

use crate::domain::errors::{GatewayError, GatewayResult};

#[derive(Clone)]
pub struct UpstreamRegistry {
    targets: HashMap<String, String>, // name → base URL (no trailing /)
    pub client: reqwest::Client,
}

impl UpstreamRegistry {
    pub fn new(targets: HashMap<String, String>, request_timeout_ms: u64) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_millis(request_timeout_ms))
            .build()
            .expect("reqwest client");
        let normalized = targets
            .into_iter()
            .map(|(k, v)| (k, v.trim_end_matches('/').to_owned()))
            .collect();
        Self {
            targets: normalized,
            client,
        }
    }

    pub fn target(&self, name: &str) -> GatewayResult<&str> {
        self.targets
            .get(name)
            .map(|s| s.as_str())
            .ok_or_else(|| GatewayError::UnknownUpstream(name.to_owned()))
    }
}
