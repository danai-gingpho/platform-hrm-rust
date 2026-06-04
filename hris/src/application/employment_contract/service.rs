use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;
use crate::domain::employment_contract::entity::Model as Contract;
use crate::domain::employment_contract::repository::EmployeeContractRepository;
use crate::domain::errors::{AppError, AppResult};
use crate::domain::shared::dtos::{PaginationQuery, PaginatedResponse};
use crate::application::employment_contract::dto::{CreateEmploymentContractRequest, UpdateEmploymentContractRequest};
use crate::utils::code_generator::CodeGenerator;

pub struct EmploymentContractService {
    repository: Arc<dyn EmployeeContractRepository>,
}

impl EmploymentContractService {
    pub fn new(repository: Arc<dyn EmployeeContractRepository>) -> Self {
        Self { repository }
    }

    pub async fn get_all_contracts(&self, query: PaginationQuery) -> AppResult<PaginatedResponse<Contract>> {
        self.repository.find_all(query).await
    }

    pub async fn get_contract_by_id(&self, id: Uuid) -> AppResult<Contract> {
        self.repository.find_by_id(id).await
    }

    pub async fn create_contract(&self, req: CreateEmploymentContractRequest) -> AppResult<Contract> {
        req.validate().map_err(|e| AppError::Validation(e.to_string()))?;
        
        let contract_no = match req.contract_no {
            Some(no) => no,
            None => {
                let last_no = self.repository.find_latest_code().await?;
                CodeGenerator::generate("CNT", last_no)
            }
        };

        let contract = Contract {
            id: Uuid::new_v4(),
            employment_id: req.employment_id,
            contract_no,
            contract_type: req.contract_type,
            start_date: req.start_date,
            end_date: req.end_date,
            signed_date: req.signed_date,
            basic_salary: req.basic_salary,
            salary_type: req.salary_type,
            currency: req.currency,
            document_url: req.document_url,
        };
        
        self.repository.create(contract).await
    }

    pub async fn update_contract(&self, id: Uuid, req: UpdateEmploymentContractRequest) -> AppResult<Contract> {
        let mut contract = self.repository.find_by_id(id).await?;
        
        if let Some(val) = req.contract_type { contract.contract_type = val; }
        if let Some(val) = req.start_date { contract.start_date = val; }
        if let Some(val) = req.end_date { contract.end_date = Some(val); }
        if let Some(val) = req.signed_date { contract.signed_date = Some(val); }
        if let Some(val) = req.basic_salary { contract.basic_salary = val; }
        if let Some(val) = req.salary_type { contract.salary_type = val; }
        if let Some(val) = req.currency { contract.currency = val; }
        if let Some(val) = req.document_url { contract.document_url = val; }
        
        self.repository.update(contract).await
    }

    pub async fn delete_contract(&self, id: Uuid) -> AppResult<()> {
        self.repository.delete(id).await
    }
}
