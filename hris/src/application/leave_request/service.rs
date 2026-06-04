use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;
use crate::domain::leave_request::entity::Model as LeaveRequest;
// use crate::domain::leave_request::repository::LeaveRequestRepository;
use crate::domain::leave_request::repository::LeaveRequestRepository;
use crate::domain::errors::{AppError, AppResult};
use crate::domain::shared::dtos::{PaginationQuery, PaginatedResponse};
use crate::application::leave_request::dto::{CreateLeaveRequestRequest, UpdateLeaveRequestRequest};
use chrono::Utc;

pub struct LeaveRequestService {
    repository: Arc<dyn LeaveRequestRepository>,
}

impl LeaveRequestService {
    pub fn new(repository: Arc<dyn LeaveRequestRepository>) -> Self {
        Self { repository }
    }

    pub async fn get_all_leave_requests(&self, query: PaginationQuery) -> AppResult<PaginatedResponse<LeaveRequest>> {
        self.repository.find_all(query).await
    }

    pub async fn get_leave_request_by_id(&self, id: Uuid) -> AppResult<LeaveRequest> {
        self.repository.find_by_id(id).await
    }

    pub async fn create_leave_request(&self, req: CreateLeaveRequestRequest) -> AppResult<LeaveRequest> {
        req.validate().map_err(|e| AppError::Validation(e.to_string()))?;
        
        let leave_request = LeaveRequest {
            id: Uuid::new_v4(),
            employee_id: req.employee_id,
            leave_type_id: req.leave_type_id,
            start_date: req.start_date,
            end_date: req.end_date,
            total_days: req.total_days,
            reason: req.reason,
            status: "Pending".to_string(),
            approved_by: None,
            approved_at: None,
        };
        
        self.repository.create(leave_request).await
    }

    pub async fn update_leave_request(&self, id: Uuid, req: UpdateLeaveRequestRequest) -> AppResult<LeaveRequest> {
        let mut leave_request = self.repository.find_by_id(id).await?;
        
        if let Some(val) = req.status { 
            leave_request.status = val; 
            if leave_request.status == "Approved" {
                leave_request.approved_at = Some(Utc::now().into());
            }
        }
        if let Some(val) = req.approved_by { leave_request.approved_by = Some(val); }
        
        self.repository.update(leave_request).await
    }

    pub async fn delete_leave_request(&self, id: Uuid) -> AppResult<()> {
        self.repository.delete(id).await
    }
}
