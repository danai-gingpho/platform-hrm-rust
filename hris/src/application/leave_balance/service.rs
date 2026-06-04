use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;
use crate::domain::leave_balance::entity::Model as LeaveBalance;
use crate::domain::leave_balance::repository::LeaveBalanceRepository;
use crate::domain::errors::{AppError, AppResult};
use crate::domain::shared::dtos::{PaginationQuery, PaginatedResponse};
use crate::application::leave_balance::dto::{CreateLeaveBalanceRequest, UpdateLeaveBalanceRequest};

pub struct LeaveBalanceService {
    repository: Arc<dyn LeaveBalanceRepository>,
}

impl LeaveBalanceService {
    pub fn new(repository: Arc<dyn LeaveBalanceRepository>) -> Self {
        Self { repository }
    }

    pub async fn get_all_leave_balances(&self, query: PaginationQuery) -> AppResult<PaginatedResponse<LeaveBalance>> {
        self.repository.find_all(query).await
    }

    pub async fn get_leave_balance_by_id(&self, id: Uuid) -> AppResult<LeaveBalance> {
        self.repository.find_by_id(id).await
    }

    pub async fn create_leave_balance(&self, req: CreateLeaveBalanceRequest) -> AppResult<LeaveBalance> {
        req.validate().map_err(|e| AppError::Validation(e.to_string()))?;
        
        let leave_balance = LeaveBalance {
            id: Uuid::new_v4(),
            employee_id: req.employee_id,
            leave_type_id: req.leave_type_id,
            year: req.year,
            entitled_days: req.entitled_days,
            used_days: req.used_days,
            remaining_days: req.entitled_days - req.used_days,
        };
        
        self.repository.create(leave_balance).await
    }

    pub async fn update_leave_balance(&self, id: Uuid, req: UpdateLeaveBalanceRequest) -> AppResult<LeaveBalance> {
        let mut leave_balance = self.repository.find_by_id(id).await?;
        
        if let Some(val) = req.entitled_days { leave_balance.entitled_days = val; }
        if let Some(val) = req.used_days { leave_balance.used_days = val; }
        
        leave_balance.remaining_days = leave_balance.entitled_days - leave_balance.used_days;
        
        self.repository.update(leave_balance).await
    }

    pub async fn delete_leave_balance(&self, id: Uuid) -> AppResult<()> {
        self.repository.delete(id).await
    }
}
