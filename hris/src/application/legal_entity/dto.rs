use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use chrono::{DateTime, FixedOffset};
use crate::domain::legal_entity::entity::Model as LegalEntityModel;
use validator::Validate;

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct LegalEntityResponse {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub id: Uuid,
    
    #[schema(example = "LE001")]
    pub code: String,
    
    #[schema(example = "0105560000000")]
    pub tax_id: String,
    
    #[schema(example = "บริษัท เทค จำกัด")]
    pub name_th: String,
    
    #[schema(example = "Tech Company Co., Ltd.")]
    pub name_en: String,
    
    #[schema(example = "123 อาคาร เอ ชั้น 10 ถ.สุขุมวิท กรุงเทพฯ 10110")]
    pub address: String,
    
    #[schema(example = "021234567")]
    pub phone: String,
    
    #[schema(example = "contact@legal_entity.com")]
    pub email: String,
    
    #[schema(example = true)]
    pub is_active: bool,
    
    #[schema(example = "2026-05-29T16:00:00+07:00")]
    pub created_at: DateTime<FixedOffset>,
    
    #[schema(example = "2026-05-29T16:00:00+07:00")]
    pub updated_at: DateTime<FixedOffset>,
}

/// DTO สำหรับรับข้อมูลตอนสร้าง LegalEntity ใหม่ (ไม่ต้องส่ง id, created_at, updated_at มา)
#[derive(Deserialize, ToSchema, Validate, Clone, Debug)]
pub struct CreateLegalEntityRequest {
    #[validate(length(min = 2, max = 50, message = "Code ต้องมีตั้งแต่ 2 ถึง 50 ตัวอักษร"))]
    #[schema(example = "LE001")]
    pub code: Option<String>,
    
    #[validate(length(equal = 13, message = "Tax ID ต้องมี 13 หลัก"))]
    #[schema(example = "0105560000000")]
    pub tax_id: String,
    
    #[validate(length(min = 1, message = "กรุณากรอกชื่อภาษาไทย"))]
    #[schema(example = "บริษัท เทค จำกัด")]
    pub name_th: String,
    
    #[validate(length(min = 1, message = "กรุณากรอกชื่อภาษาอังกฤษ"))]
    #[schema(example = "Tech Company Co., Ltd.")]
    pub name_en: String,
    
    #[schema(example = "123 อาคาร เอ...")]
    pub address: String,
    
    #[schema(example = "021234567")]
    pub phone: String,
    
    #[validate(email(message = "รูปแบบ Email ไม่ถูกต้อง"))] // ตรวจสอบว่าเป็น email จริงไหม
    #[schema(example = "contact@legal_entity.com")]
    pub email: String,
}
/// DTO สำหรับรับข้อมูลตอนอัปเดตแก้ไข LegalEntity (ห้ามแก้ code และเปิดให้เลือกอัปเดตเฉพาะบางฟิลด์ได้)
#[derive(Deserialize, ToSchema, Clone, Debug)]
pub struct UpdateLegalEntityRequest {
    #[schema(example = "0105560000000")]
    pub tax_id: Option<String>,
    
    #[schema(example = "บริษัท เทค จำกัด (มหาชน)")]
    pub name_th: Option<String>,
    
    #[schema(example = "Tech Company Public Co., Ltd.")]
    pub name_en: Option<String>,
    
    pub address: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub is_active: Option<bool>,
}

// --- แถมฟังก์ชันสำหรับแปลงจาก Sea-ORM Model ไปเป็น LegalEntityResponse DTO ได้ง่ายๆ ---
impl From<LegalEntityModel> for LegalEntityResponse {
    fn from(model: LegalEntityModel) -> Self {
        Self {
            id: model.id,
            code: model.code,
            tax_id: model.tax_id,
            name_th: model.name_th,
            name_en: model.name_en,
            address: model.address,
            phone: model.phone,
            email: model.email,
            is_active: model.is_active,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
