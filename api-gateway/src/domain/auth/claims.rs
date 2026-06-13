use std::collections::HashSet;

use serde::{Deserialize, Serialize};

/// Subset of Keycloak's access-token payload that the gateway cares about.
#[derive(Debug, Clone, Deserialize)]
pub struct KeycloakClaims {
    pub sub: String,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub preferred_username: Option<String>,
    #[serde(default)]
    pub realm_access: Option<RealmAccess>,
    #[serde(default)]
    pub resource_access: Option<std::collections::HashMap<String, ClientAccess>>,
    #[serde(default)]
    pub scope: Option<String>,
    pub exp: i64,
    pub iat: Option<i64>,
    pub iss: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RealmAccess {
    #[serde(default)]
    pub roles: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ClientAccess {
    #[serde(default)]
    pub roles: Vec<String>,
}

/// What downstream code (handlers, authz layer) sees after auth succeeds.
#[derive(Debug, Clone, Serialize)]
pub struct AuthContext {
    pub sub: String,
    pub email: Option<String>,
    pub username: Option<String>,
    pub realm_roles: HashSet<String>,
    pub client_roles: HashSet<String>,
    pub scopes: HashSet<String>,
}

impl AuthContext {
    pub fn from_claims(claims: KeycloakClaims, gateway_client_id: &str) -> Self {
        let realm_roles: HashSet<String> = claims
            .realm_access
            .map(|r| r.roles.into_iter().collect())
            .unwrap_or_default();

        let client_roles: HashSet<String> = claims
            .resource_access
            .and_then(|map| map.get(gateway_client_id).cloned())
            .map(|c| c.roles.into_iter().collect())
            .unwrap_or_default();

        let scopes: HashSet<String> = claims
            .scope
            .map(|s| s.split_whitespace().map(str::to_owned).collect())
            .unwrap_or_default();

        Self {
            sub: claims.sub,
            email: claims.email,
            username: claims.preferred_username,
            tenant_id: claims.tenant_id,
            realm_roles,
            client_roles,
            scopes,
        }
    }

    pub fn has_any_role<'a, I: IntoIterator<Item = &'a String>>(&self, roles: I) -> bool {
        roles
            .into_iter()
            .any(|r| self.realm_roles.contains(r) || self.client_roles.contains(r))
    }

    pub fn has_all_scopes<'a, I: IntoIterator<Item = &'a String>>(&self, scopes: I) -> bool {
        scopes.into_iter().all(|s| self.scopes.contains(s))
    }

    pub fn roles_csv(&self) -> String {
        let mut all: Vec<&String> = self
            .realm_roles
            .iter()
            .chain(self.client_roles.iter())
            .collect();
        all.sort();
        all.dedup();
        all.into_iter().cloned().collect::<Vec<_>>().join(",")
    }
}
in(",")
    }
}
