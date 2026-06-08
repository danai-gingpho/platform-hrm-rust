use std::sync::Arc;

use crate::domain::auth::policy::RouteRule;
use crate::infrastructure::grpc::{
    auth::auth_service_client::AuthServiceClient,
    hris::{
        employee_service_client::EmployeeServiceClient,
        legal_entity_service_client::LegalEntityServiceClient,
    },
    platform::platform_service_client::PlatformServiceClient,
};
use crate::infrastructure::http_client::upstream::UpstreamRegistry;
use crate::infrastructure::keycloak::verifier::JwtVerifier;
use tonic::transport::Channel;

#[derive(Clone)]
pub struct AppState {
    pub verifier: Arc<JwtVerifier>,
    pub upstreams: Arc<UpstreamRegistry>,
    pub routes: Arc<Vec<RouteRule>>,

    // gRPC Clients
    pub auth_client: AuthServiceClient<Channel>,
    pub hris_employee_client: EmployeeServiceClient<Channel>,
    pub hris_legal_entity_client: LegalEntityServiceClient<Channel>,
    pub platform_client: PlatformServiceClient<Channel>,
}
