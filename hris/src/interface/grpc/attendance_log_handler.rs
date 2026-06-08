use tonic::{Request, Response, Status};
use uuid::Uuid;
use std::sync::Arc;
use crate::application::attendance_log::service::AttendanceLogService;
use crate::application::attendance_log::dto::CreateAttendanceLogRequest as DtoCreateRequest;
use crate::domain::shared::dtos::PaginationQuery;
use chrono::{DateTime, FixedOffset};
use serde_json::Value;

pub mod hris_proto {
    tonic::include_proto!("hris");
}

use hris_proto::attendance_log_service_server::AttendanceLogService as AttendanceLogGrpcService;
pub use hris_proto::attendance_log_service_server::AttendanceLogServiceServer;
use hris_proto::{
    GetAttendanceLogRequest, CreateAttendanceLogRequest, AttendanceLogResponse,
    DeleteAttendanceLogRequest, ListAttendanceLogsRequest, ListAttendanceLogsResponse, Empty
};

pub struct AttendanceLogGrpcHandler {
    service: Arc<AttendanceLogService>,
}

impl AttendanceLogGrpcHandler {
    pub fn new(service: Arc<AttendanceLogService>) -> Self {
        Self { service }
    }
}

#[tonic::async_trait]
impl AttendanceLogGrpcService for AttendanceLogGrpcHandler {
    async fn get_attendance_log(
        &self,
        request: Request<GetAttendanceLogRequest>,
    ) -> Result<Response<AttendanceLogResponse>, Status> {
        let id = request.into_inner().id;

        let log = self.service.get_log_by_id(id).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(AttendanceLogResponse {
            id: log.id,
            employee_id: log.employee_id.to_string(),
            device_id: log.device_id,
            punch_time: log.punch_time.to_string(),
            punch_type: log.punch_type,
            raw_payload: log.raw_payload.to_string(),
        }))
    }

    async fn create_attendance_log(
        &self,
        request: Request<CreateAttendanceLogRequest>,
    ) -> Result<Response<AttendanceLogResponse>, Status> {
        let req = request.into_inner();
        
        let employee_id = Uuid::parse_str(&req.employee_id)
            .map_err(|_| Status::invalid_argument("Invalid employee_id UUID"))?;
        
        let punch_time = DateTime::parse_from_rfc3339(&req.punch_time)
            .or_else(|_| DateTime::parse_from_str(&req.punch_time, "%Y-%m-%dT%H:%M:%S%.f%z"))
            .map_err(|_| Status::invalid_argument("Invalid punch_time format, expected RFC3339"))?
            .with_timezone(&FixedOffset::east_opt(0).unwrap());

        let raw_payload: Value = serde_json::from_str(&req.raw_payload)
            .map_err(|_| Status::invalid_argument("Invalid raw_payload JSON"))?;

        let dto = DtoCreateRequest {
            employee_id,
            device_id: req.device_id,
            punch_time,
            punch_type: req.punch_type,
            raw_payload,
        };

        let log = self.service.create_log(dto).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(AttendanceLogResponse {
            id: log.id,
            employee_id: log.employee_id.to_string(),
            device_id: log.device_id,
            punch_time: log.punch_time.to_string(),
            punch_type: log.punch_type,
            raw_payload: log.raw_payload.to_string(),
        }))
    }

    async fn delete_attendance_log(
        &self,
        request: Request<DeleteAttendanceLogRequest>,
    ) -> Result<Response<Empty>, Status> {
        let id = request.into_inner().id;

        self.service.delete_log(id).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(Empty {}))  
    }

    async fn list_attendance_logs(
        &self,
        request: Request<ListAttendanceLogsRequest>,
    ) -> Result<Response<ListAttendanceLogsResponse>, Status> {
        let req = request.into_inner();
        let query = PaginationQuery {
            page: Some(req.page as u64),
            limit: Some(req.per_page as u64),
            search: None,
        };

        let paginated = self.service.get_all_logs(query).await
            .map_err(|e| Status::internal(e.to_string()))?;

         let items = paginated.data.into_iter().map(|log| AttendanceLogResponse {
            id: log.id,
            employee_id: log.employee_id.to_string(),
            device_id: log.device_id,
            punch_time: log.punch_time.to_string(),
            punch_type: log.punch_type,
            raw_payload: log.raw_payload.to_string(),
        }).collect();

        Ok(Response::new(ListAttendanceLogsResponse {
            items,
            total_items: paginated.total as u32,
            total_pages: paginated.total_pages as u32,
            current_page: paginated.page as u32,
            per_page: paginated.limit as u32,
        }))
    }
}
