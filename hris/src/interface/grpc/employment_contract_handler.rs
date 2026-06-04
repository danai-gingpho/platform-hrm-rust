use tonic::{Request, Response, Status};
use uuid::Uuid;
use std::sync::Arc;
use crate::application::employment_contract::service::EmploymentContractService;
use crate::application::employment_contract::dto::CreateEmploymentContractRequest as DtoCreateRequest;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use std::str::FromStr;

pub mod hris_proto {
    tonic::include_proto!("hris");
}

use hris_proto::employment_contract_service_server::EmploymentContractService as EmploymentContractGrpcService;
pub use hris_proto::employment_contract_service_server::EmploymentContractServiceServer;
use hris_proto::{GetEmploymentContractRequest, CreateEmploymentContractRequest, EmploymentContractResponse};

pub struct EmploymentContractGrpcHandler {
    service: Arc<EmploymentContractService>,
}

impl EmploymentContractGrpcHandler {
    pub fn new(service: Arc<EmploymentContractService>) -> Self {
        Self { service }
    }
}

#[tonic::async_trait]
impl EmploymentContractGrpcService for EmploymentContractGrpcHandler {
    async fn get_employment_contract(
        &self,
        request: Request<GetEmploymentContractRequest>,
    ) -> Result<Response<EmploymentContractResponse>, Status> {
        let id = Uuid::parse_str(&request.into_inner().id)
            .map_err(|_| Status::invalid_argument("Invalid UUID"))?;

        let contract = self.service.get_contract_by_id(id).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(EmploymentContractResponse {
            id: contract.id.to_string(),
            employment_id: contract.employment_id.to_string(),
            contract_no: contract.contract_no,
            contract_type: contract.contract_type,
            start_date: contract.start_date.to_string(),
            end_date: contract.end_date.map(|d| d.to_string()).unwrap_or_default(),
            signed_date: contract.signed_date.map(|d| d.to_string()).unwrap_or_default(),
            basic_salary: contract.basic_salary.to_string(),
            salary_type: contract.salary_type,
            currency: contract.currency,
            document_url: contract.document_url,
        }))
    }

    async fn create_employment_contract(
        &self,
        request: Request<CreateEmploymentContractRequest>,
    ) -> Result<Response<EmploymentContractResponse>, Status> {
        let req = request.into_inner();
        
        let employment_id = Uuid::parse_str(&req.employment_id)
            .map_err(|_| Status::invalid_argument("Invalid employment_id UUID"))?;
        
        let start_date = NaiveDate::parse_from_str(&req.start_date, "%Y-%m-%d")
            .map_err(|_| Status::invalid_argument("Invalid start_date format, expected YYYY-MM-DD"))?;
        
        let end_date = if req.end_date.is_empty() {
            None
        } else {
            Some(NaiveDate::parse_from_str(&req.end_date, "%Y-%m-%d")
                .map_err(|_| Status::invalid_argument("Invalid end_date format, expected YYYY-MM-DD"))?)
        };

        let signed_date = if req.signed_date.is_empty() {
            None
        } else {
            Some(NaiveDate::parse_from_str(&req.signed_date, "%Y-%m-%d")
                .map_err(|_| Status::invalid_argument("Invalid signed_date format, expected YYYY-MM-DD"))?)
        };

        let basic_salary = Decimal::from_str(&req.basic_salary)
            .map_err(|_| Status::invalid_argument("Invalid basic_salary format"))?;

        let dto = DtoCreateRequest {
            employment_id,
            contract_no: if req.contract_no.is_empty() { None } else { Some(req.contract_no) },
            contract_type: req.contract_type,
            start_date,
            end_date,
            signed_date,
            basic_salary,
            salary_type: req.salary_type,
            currency: req.currency,
            document_url: req.document_url,
        };

        let contract = self.service.create_contract(dto).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(EmploymentContractResponse {
            id: contract.id.to_string(),
            employment_id: contract.employment_id.to_string(),
            contract_no: contract.contract_no,
            contract_type: contract.contract_type,
            start_date: contract.start_date.to_string(),
            end_date: contract.end_date.map(|d| d.to_string()).unwrap_or_default(),
            signed_date: contract.signed_date.map(|d| d.to_string()).unwrap_or_default(),
            basic_salary: contract.basic_salary.to_string(),
            salary_type: contract.salary_type,
            currency: contract.currency,
            document_url: contract.document_url,
        }))
    }
}
