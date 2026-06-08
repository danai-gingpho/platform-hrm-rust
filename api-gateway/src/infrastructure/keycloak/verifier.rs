use jsonwebtoken::{decode, decode_header, Validation};

use super::jwks::JwksCache;
use crate::domain::auth::claims::{AuthContext, KeycloakClaims};
use crate::domain::errors::{GatewayError, GatewayResult};

pub struct JwtVerifier {
    jwks: JwksCache,
    issuer: String,
    audience: String,
    client_id: String,
}

impl JwtVerifier {
    pub fn new(
        jwks: JwksCache,
        issuer: impl Into<String>,
        audience: impl Into<String>,
        client_id: impl Into<String>,
    ) -> Self {
        Self {
            jwks,
            issuer: issuer.into(),
            audience: audience.into(),
            client_id: client_id.into(),
        }
    }

    pub async fn verify(&self, bearer: &str) -> GatewayResult<AuthContext> {
        let header = decode_header(bearer)
            .map_err(|e| GatewayError::InvalidToken(format!("header: {e}")))?;
        let kid = header
            .kid
            .ok_or_else(|| GatewayError::InvalidToken("missing kid".into()))?;

        let key = self.jwks.get(&kid).await?;

        let mut validation = Validation::new(header.alg);
        validation.set_issuer(&[&self.issuer]);
        validation.set_audience(&[&self.audience]);

        let data = decode::<KeycloakClaims>(bearer, &key, &validation).map_err(|e| {
            use jsonwebtoken::errors::ErrorKind;
            match e.kind() {
                ErrorKind::ExpiredSignature => GatewayError::TokenExpired,
                _ => GatewayError::InvalidToken(e.to_string()),
            }
        })?;

        Ok(AuthContext::from_claims(data.claims, &self.client_id))
    }
}
