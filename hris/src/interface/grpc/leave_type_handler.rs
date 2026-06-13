use tonic::{Request, Response, Status};
use uuid::Uuid;
use std::sync::Arc;
use crate::application::leave_type::service::LeaveTypeService;
use crate::application::leave_type::dto::{
    CreateLeaveTypeRequest as DtoCreateRequest, UpdateLeaveTypeRequest as DtoUpdateRequest,
};
use crate::domain::shared::dtos::PaginationQuery;
use rust_decimal::Decimal;
use std::str::FromStr;

pub mod hris_proto {
    tonic::include_proto!("hris");
}

use hris_proto::leave_type_service_server::LeaveTypeService as LeaveTypeGrpcService;
pub use hris_proto::leave_type_service_server::LeaveTypeServiceServer;
use hris_proto::{
    GetLeaveTypeRequest, CreateLeaveTypeRequest, LeaveTypeResponse,
    UpdateLeaveTypeRequest, DeleteLeaveTypeRequest, ListLeaveTypesRequest, ListLeaveTypesResponse, Empty
};

pub struct LeaveTypeGrpcHandler {
    service: Arc<LeaveTypeService>,
}

impl LeaveTypeGrpcHandler {
    pub fn new(service: Arc<LeaveTypeService>) -> Self {
        Self { service }
    }
}

fn map_leave_type_to_response(lt: crate::domain::leave_type::entity::Model) -> LeaveTypeResponse {
    LeaveTypeResponse {
        id: lt.id.to_string(),
        code: lt.code,
        name: lt.name,
        paid: lt.paid,
        max_days_per_year: lt.max_days_per_year.to_string(),
        gender_restriction: lt.gender_restriction,
    }
}

#[tonic::async_trait]
impl LeaveTypeGrpcService for LeaveTypeGrpcHandler {
    async fn get_leave_type(
        &self,
        request: Request<GetLeaveTypeRequest>,
    ) -> Result<Response<LeaveTypeResponse>, Status> {
        let id = Uuid::parse_str(&request.into_inner().id)
            .map_err(|_| Status::invalid_argument("Invalid UUID"))?;

        let leave_type = self.service.get_leave_type_by_id(id).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(map_leave_type_to_response(leave_type)))
    }

    async fn create_leave_type(
        &self,
        request: Request<CreateLeaveTypeRequest>,
    ) -> Result<Response<LeaveTypeResponse>, Status> {
        let req = request.into_inner();
        let dto = DtoCreateRequest {
            code: if req.code.is_empty() { None } else { Some(req.code) },
            name: req.name,
            paid: req.paid,
            max_days_per_year: Decimal::from_str(&req.max_days_per_year).map_err(|_| Status::invalid_argument("Invalid max_days_per_year"))?,
            gender_restriction: req.gender_restriction,
        };

        let leave_type = self.service.create_leave_type(dto).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(map_leave_type_to_response(leave_type)))
    }

    async fn update_leave_type(
        &self,
        request: Request<UpdateLeaveTypeRequest>,
    ) -> Result<Response<LeaveTypeResponse>, Status> {
        let req = request.into_inner();
        let id = Uuid::parse_str(&req.id)
            .map_err(|_| Status::invalid_argument("Invalid UUID"))?;

        let max_days_per_year = if let Some(ref val) = req.max_days_per_year {
            Some(Decimal::from_str(val).map_err(|_| Status::invalid_argument("Invalid max_days_per_year"))?)
        } else {
            None
        };

        let dto = DtoUpdateRequest {
            name: req.name,
            paid: req.paid,
            max_days_per_year,
            gender_restriction: req.gender_restriction,
        };

        let leave_type = self.service.update_leave_type(id, dto).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(map_leave_type_to_response(leave_type)))
    }

    async fn delete_leave_type(
        &self,
        request: Request<DeleteLeaveTypeRequest>,
    ) -> Result<Response<Empty>, Status> {
        let id = Uuid::parse_str(&request.into_inner().id)
            .map_err(|_| Status::invalid_argument("Invalid UUID"))?;

        self.service.delete_leave_type(id).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(Empty {}))
    }

    async fn list_leave_types(
        &self,
        request: Request<ListLeaveTypesRequest>,
    ) -> Result<Response<ListLeaveTypesResponse>, Status> {
        let req = request.into_inner();
        let query = PaginationQuery {
            page: Some(req.page as u64),
            limit: Some(req.per_page as u64),
            search: None,
        };

        let response = self.service.get_all_leave_types(query).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(ListLeaveTypesResponse {
            items: response.data.into_iter().map(map_leave_type_to_response).collect(),
            total_items: response.total as u32,
            total_pages: response.total_pages as u32,
            current_page: response.page as u32,
            per_page: response.limit as u32,
        }))
    }
}
