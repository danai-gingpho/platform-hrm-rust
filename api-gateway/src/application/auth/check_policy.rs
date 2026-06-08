// use crate::domain::auth::claims::AuthContext;
use crate::domain::auth::claims::AuthContext;
use crate::domain::auth::policy::AccessPolicy;
use crate::domain::errors::{GatewayError, GatewayResult};

pub fn execute(policy: &AccessPolicy, ctx: Option<&AuthContext>) -> GatewayResult<()> {
    if policy.public {
        return Ok(());
    }
    let ctx = ctx.ok_or(GatewayError::MissingToken)?;

    if !policy.required_roles.is_empty() && !ctx.has_any_role(policy.required_roles.iter()) {
        return Err(GatewayError::Forbidden(format!(
            "requires any of roles: {}",
            policy.required_roles.join(",")
        )));
    }
    if !policy.required_scopes.is_empty() && !ctx.has_all_scopes(policy.required_scopes.iter()) {
        return Err(GatewayError::Forbidden(format!(
            "requires scopes: {}",
            policy.required_scopes.join(" ")
        )));
    }
    Ok(())
}
