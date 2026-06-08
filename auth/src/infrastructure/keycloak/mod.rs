use serde::{Deserialize, Serialize};
use reqwest::Client;

pub struct KeycloakClient {
    base_url: String,
    admin_token: String,
    client: Client,
}

impl KeycloakClient {
    pub fn new(base_url: String, admin_token: String) -> Self {
        Self {
            base_url,
            admin_token,
            client: Client::new(),
        }
    }

    pub async fn create_realm(&self, realm_name: &str) -> anyhow::Result<()> {
        let url = format!("{}/admin/realms", self.base_url);
        let body = serde_json::json!({
            "realm": realm_name,
            "enabled": true,
            "displayName": realm_name,
        });

        self.client.post(url)
            .bearer_auth(&self.admin_token)
            .json(&body)
            .send()
            .await?;
        
        Ok(())
    }

    pub async fn create_user(&self, realm: &str, email: &str, first_name: &str, last_name: &str) -> anyhow::Result<String> {
        let url = format!("{}/admin/realms/{}/users", self.base_url, realm);
        let body = serde_json::json!({
            "username": email,
            "email": email,
            "firstName": first_name,
            "lastName": last_name,
            "enabled": true,
        });

        let resp = self.client.post(url)
            .bearer_auth(&self.admin_token)
            .json(&body)
            .send()
            .await?;

        // Extract ID from Location header
        let location = resp.headers().get("Location")
            .ok_or_else(|| anyhow::anyhow!("No Location header in user creation response"))?
            .to_str()?;
        
        let id = location.split('/').last().ok_or_else(|| anyhow::anyhow!("Invalid Location header"))?;
        
        Ok(id.to_string())
    }
}
