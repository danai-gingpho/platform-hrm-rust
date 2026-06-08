use std::sync::Arc;

use crate::domain::auth::claims::AuthContext;
use crate::domain::errors::GatewayResult;
use crate::infrastructure::keycloak::verifier::JwtVerifier;

pub async fn execute(verifier: Arc<JwtVerifier>, bearer: &str) -> GatewayResult<AuthContext> {
    verifier.verify(bearer).await
}
