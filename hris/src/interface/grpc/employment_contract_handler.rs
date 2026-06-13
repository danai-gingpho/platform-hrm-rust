use tonic::{Request, Response, Status};
use uuid::Uuid;
use std::sync::Arc;
use crate::application::employment_contract::service::EmploymentContractService;
use crate::application::employment_contract::dto::{
    CreateEmploymentContractRequest as DtoCreateRequest, UpdateEmploymentContractRequest as DtoUpdateRequest,
};
use crate::domain::shared::dtos::PaginationQuery;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use std::str::FromStr;

pub mod hris_proto {
    tonic::include_proto!("hris");
}

use hris_proto::employment_contract_service_server::EmploymentContractService as EmploymentContractGrpcService;
pub use hris_proto::employment_contract_service_server::EmploymentContractServiceServer;
use hris_proto::{
    GetEmploymentContractRequest, CreateEmploymentContractRequest, EmploymentContractResponse,
    UpdateEmploymentContractRequest, DeleteEmploymentContractRequest, ListEmploymentContractsRequest,
    ListEmploymentContractsResponse, Empty
};

pub struct EmploymentContractGrpcHandler {
    service: Arc<EmploymentContractService>,
}

impl EmploymentContractGrpcHandler {
    pub fn new(service: Arc<EmploymentContractService>) -> Self {
        Self { service }
    }
}

fn map_contract_to_response(contract: crate::domain::employment_contract::entity::Model) -> EmploymentContractResponse {
    EmploymentContractResponse {
        id: contract.id.to_string(),
        employment_id: contract.employment_id.to_string(),
        contract_no: contract.contract_no,
        contract_type: contract.contract_type,
        start_date: contract.start_date.to_string(),
        end_date: contract.end_date.map(|d| d.to_string()),
        signed_date: contract.signed_date.map(|d| d.to_string()),
        basic_salary: contract.basic_salary.to_string(),
        salary_type: contract.salary_type,
        currency: contract.currency,
        document_url: contract.document_url,
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

        Ok(Response::new(map_contract_to_response(contract)))
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
        
        let end_date = if let Some(ref date_str) = req.end_date {
            Some(NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
                .map_err(|_| Status::invalid_argument("Invalid end_date format, expected YYYY-MM-DD"))?)
        } else {
            None
        };

        let signed_date = if let Some(ref date_str) = req.signed_date {
            Some(NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
                .map_err(|_| Status::invalid_argument("Invalid signed_date format, expected YYYY-MM-DD"))?)
        } else {
            None
        };

        let basic_salary = Decimal::from_str(&req.basic_salary)
            .map_err(|_| Status::invalid_argument("Invalid basic_salary format"))?;

        let dto = DtoCreateRequest {
            employment_id,
            contract_no: req.contract_no,
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

        Ok(Response::new(map_contract_to_response(contract)))
    }

    async fn update_employment_contract(
        &self,
        request: Request<UpdateEmploymentContractRequest>,
    ) -> Result<Response<EmploymentContractResponse>, Status> {
        let req = request.into_inner();
        let id = Uuid::parse_str(&req.id)
            .map_err(|_| Status::invalid_argument("Invalid UUID"))?;

        let start_date = if let Some(ref date_str) = req.start_date {
            Some(NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
                .map_err(|_| Status::invalid_argument("Invalid start_date format, expected YYYY-MM-DD"))?)
        } else {
            None
        };

        let end_date = if let Some(ref date_str) = req.end_date {
            Some(NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
                .map_err(|_| Status::invalid_argument("Invalid end_date format, expected YYYY-MM-DD"))?)
        } else {
            None
        };

        let signed_date = if let Some(ref date_str) = req.signed_date {
            Some(NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
                .map_err(|_| Status::invalid_argument("Invalid signed_date format, expected YYYY-MM-DD"))?)
        } else {
            None
        };

        let basic_salary = if let Some(ref amt_str) = req.basic_salary {
            Some(Decimal::from_str(amt_str)
                .map_err(|_| Status::invalid_argument("Invalid basic_salary format"))?)
        } else {
            None
        };

        let dto = DtoUpdateRequest {
            contract_type: req.contract_type,
            start_date,
            end_date,
            signed_date,
            basic_salary,
            salary_type: req.salary_type,
            currency: req.currency,
            document_url: req.document_url,
        };

        let contract = self.service.update_contract(id, dto).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(map_contract_to_response(contract)))
    }

    async fn delete_employment_contract(
        &self,
        request: Request<DeleteEmploymentContractRequest>,
    ) -> Result<Response<Empty>, Status> {
        let id = Uuid::parse_str(&request.into_inner().id)
            .map_err(|_| Status::invalid_argument("Invalid UUID"))?;

        self.service.delete_contract(id).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(Empty {}))
    }

    async fn list_employment_contracts(
        &self,
        request: Request<ListEmploymentContractsRequest>,
    ) -> Result<Response<ListEmploymentContractsResponse>, Status> {
        let req = request.into_inner();
        let query = PaginationQuery {
            page: Some(req.page as u64),
            limit: Some(req.per_page as u64),
            search: req.employment_id, // Using search for filtering by employment_id
        };

        let response = self.service.get_all_contracts(query).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(ListEmploymentContractsResponse {
            items: response.data.into_iter().map(map_contract_to_response).collect(),
            total_items: response.total as u32,
            total_pages: response.total_pages as u32,
            current_page: response.page as u32,
            per_page: response.limit as u32,
        }))
    }
}
