use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateShiftRequest {
    #[validate(length(min = 1))]
    pub code: Option<String>,
    #[validate(length(min = 1))]
    pub name: String,
    pub start_time: String, // "HH:MM:SS"
    pub end_time: String,
    pub break_minutes: i32,
    pub late_grace_minutes: i32,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateShiftRequest {
    pub name: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub break_minutes: Option<i32>,
    pub late_grace_minutes: Option<i32>,
}
