use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use chrono::{NaiveDate, DateTime, FixedOffset};
use crate::domain::employee::entity::Model as EmployeeModel;
use validator::Validate;

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct EmployeeResponse {
    pub id: Uuid,
    pub employee_no: String,
    pub citizen_id: String,
    pub passport_no: String,
    pub title: String,
    pub first_name_th: String,
    pub last_name_th: String,
    pub first_name_en: String,
    pub last_name_en: String,
    pub gender: String,
    pub birth_date: NaiveDate,
    pub marital_status: String,
    pub nationality: String,
    pub religion: String,
    pub phone: String,
    pub email: String,
    pub current_address: String,
    pub permanent_address: String,
    pub emergency_contact_name: String,
    pub emergency_contact_phone: String,
    pub blood_group: String,
    pub photo_url: String,
    pub status: String,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}

#[derive(Deserialize, ToSchema, Validate, Clone, Debug)]
pub struct CreateEmployeeRequest {
    #[validate(length(min = 2, max = 50))]
    pub employee_no: Option<String>,
    pub citizen_id: String,
    pub passport_no: String,
    pub title: String,
    pub first_name_th: String,
    pub last_name_th: String,
    pub first_name_en: String,
    pub last_name_en: String,
    pub gender: String,
    pub birth_date: NaiveDate,
    pub marital_status: String,
    pub nationality: String,
    pub religion: String,
    pub phone: String,
    #[validate(email)]
    pub email: String,
    pub current_address: String,
    pub permanent_address: String,
    pub emergency_contact_name: String,
    pub emergency_contact_phone: String,
    pub blood_group: String,
    pub photo_url: String,
}

#[derive(Deserialize, ToSchema, Clone, Debug)]
pub struct UpdateEmployeeRequest {
    pub citizen_id: Option<String>,
    pub passport_no: Option<String>,
    pub title: Option<String>,
    pub first_name_th: Option<String>,
    pub last_name_th: Option<String>,
    pub first_name_en: Option<String>,
    pub last_name_en: Option<String>,
    pub gender: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub marital_status: Option<String>,
    pub nationality: Option<String>,
    pub religion: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub current_address: Option<String>,
    pub permanent_address: Option<String>,
    pub emergency_contact_name: Option<String>,
    pub emergency_contact_phone: Option<String>,
    pub blood_group: Option<String>,
    pub photo_url: Option<String>,
    pub status: Option<String>,
}

impl From<EmployeeModel> for EmployeeResponse {
    fn from(model: EmployeeModel) -> Self {
        Self {
            id: model.id,
            employee_no: model.employee_no,
            citizen_id: model.citizen_id,
            passport_no: model.passport_no,
            title: model.title,
            first_name_th: model.first_name_th,
            last_name_th: model.last_name_th,
            first_name_en: model.first_name_en,
            last_name_en: model.last_name_en,
            gender: model.gender,
            birth_date: model.birth_date,
            marital_status: model.marital_status,
            nationality: model.nationality,
            religion: model.religion,
            phone: model.phone,
            email: model.email,
            current_address: model.current_address,
            permanent_address: model.permanent_address,
            emergency_contact_name: model.emergency_contact_name,
            emergency_contact_phone: model.emergency_contact_phone,
            blood_group: model.blood_group,
            photo_url: model.photo_url,
            status: model.status,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
