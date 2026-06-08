use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
    http::{StatusCode, header},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company_id: Option<String>, // Custom claim from Keycloak
    exp: usize,
}

pub async fn auth_middleware(
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = req.headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok());

    if let Some(auth_token) = auth_header {
        if let Some(_token) = auth_token.strip_prefix("Bearer ") {
            // In real app: validate with Keycloak public key
            // let decoding_key = DecodingKey::from_secret("secret".as_ref());
            // let token_data = decode::<Claims>(token, &decoding_key, &Validation::new(Algorithm::HS256))
            //     .map_err(|_| StatusCode::UNAUTHORIZED)?;
            
            // req.extensions_mut().insert(token_data.claims);
            return Ok(next.run(req).await);
        }
    }

    // For demonstration, we allow requests without token but in production this should be strict
    Ok(next.run(req).await)
}
