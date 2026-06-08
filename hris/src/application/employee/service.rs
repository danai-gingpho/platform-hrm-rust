use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;
use crate::domain::employee::entity::Model as Employee;
use crate::domain::employee::repository::EmployeeRepository;
use crate::domain::errors::{AppError, AppResult};
use crate::domain::shared::dtos::{PaginationQuery, PaginatedResponse};
use crate::application::employee::dto::{CreateEmployeeRequest, UpdateEmployeeRequest};
use crate::utils::code_generator::CodeGenerator;
use chrono::Utc;

pub struct EmployeeService {
    repository: Arc<dyn EmployeeRepository>,
}

impl EmployeeService {
    pub fn new(repository: Arc<dyn EmployeeRepository>) -> Self {
        Self { repository }
    }

    pub async fn get_all_employees(&self, query: PaginationQuery) -> AppResult<PaginatedResponse<Employee>> {
        self.repository.find_all(query).await
    }

    pub async fn get_employee_by_id(&self, id: Uuid) -> AppResult<Employee> {
        self.repository.find_by_id(id).await
    }

    pub async fn create_employee(&self, req: CreateEmployeeRequest) -> AppResult<Employee> {
        req.validate().map_err(|e| AppError::Validation(e.to_string()))?;
        
        let employee_no = match req.employee_no {
            Some(no) => no,
            None => {
                let last_no = self.repository.find_latest_code().await?;
                CodeGenerator::generate("EMP", last_no)
            }
        };

        let employee = Employee {
            id: Uuid::new_v4(),
            employee_no,
            citizen_id: req.citizen_id,
            passport_no: req.passport_no,
            title: req.title,
            first_name_th: req.first_name_th,
            last_name_th: req.last_name_th,
            first_name_en: req.first_name_en,
            last_name_en: req.last_name_en,
            gender: req.gender,
            birth_date: req.birth_date,
            marital_status: req.marital_status,
            nationality: req.nationality,
            religion: req.religion,
            phone: req.phone,
            email: req.email,
            current_address: req.current_address,
            permanent_address: req.permanent_address,
            emergency_contact_name: req.emergency_contact_name,
            emergency_contact_phone: req.emergency_contact_phone,
            blood_group: req.blood_group,
            photo_url: req.photo_url,
            status: "Active".to_string(),
            created_at: Utc::now().into(),
            updated_at: Utc::now().into(),
        };
        
        self.repository.create(employee).await
    }

    pub async fn update_employee(&self, id: Uuid, req: UpdateEmployeeRequest) -> AppResult<Employee> {
        let mut employee = self.repository.find_by_id(id).await?;
        
        if let Some(val) = req.citizen_id { employee.citizen_id = val; }
        if let Some(val) = req.passport_no { employee.passport_no = val; }
        if let Some(val) = req.title { employee.title = val; }
        if let Some(val) = req.first_name_th { employee.first_name_th = val; }
        if let Some(val) = req.last_name_th { employee.last_name_th = val; }
        if let Some(val) = req.first_name_en { employee.first_name_en = val; }
        if let Some(val) = req.last_name_en { employee.last_name_en = val; }
        if let Some(val) = req.gender { employee.gender = val; }
        if let Some(val) = req.birth_date { employee.birth_date = val; }
        if let Some(val) = req.marital_status { employee.marital_status = val; }
        if let Some(val) = req.nationality { employee.nationality = val; }
        if let Some(val) = req.religion { employee.religion = val; }
        if let Some(val) = req.phone { employee.phone = val; }
        if let Some(val) = req.email { employee.email = val; }
        if let Some(val) = req.current_address { employee.current_address = val; }
        if let Some(val) = req.permanent_address { employee.permanent_address = val; }
        if let Some(val) = req.emergency_contact_name { employee.emergency_contact_name = val; }
        if let Some(val) = req.emergency_contact_phone { employee.emergency_contact_phone = val; }
        if let Some(val) = req.blood_group { employee.blood_group = val; }
        if let Some(val) = req.photo_url { employee.photo_url = val; }
        if let Some(val) = req.status { employee.status = val; }
        
        employee.updated_at = Utc::now().into();
        
        self.repository.update(employee).await
    }

    pub async fn delete_employee(&self, id: Uuid) -> AppResult<()> {
        self.repository.delete(id).await
    }
}
