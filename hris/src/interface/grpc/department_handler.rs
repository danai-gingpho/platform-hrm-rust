use tonic::{Request, Response, Status};
use uuid::Uuid;
use std::sync::Arc;
use crate::application::department::service::DepartmentService;
use crate::application::department::dto::{
    CreateDepartmentRequest as DtoCreateRequest, UpdateDepartmentRequest as DtoUpdateRequest,
};
use crate::domain::shared::dtos::PaginationQuery;

pub mod hris_proto {
    tonic::include_proto!("hris");
}

use hris_proto::department_service_server::DepartmentService as DepartmentGrpcService;
pub use hris_proto::department_service_server::DepartmentServiceServer;
use hris_proto::{
    CreateDepartmentRequest, DeleteDepartmentRequest, Empty, GetDepartmentRequest,
    ListDepartmentsRequest, ListDepartmentsResponse, UpdateDepartmentRequest, DepartmentResponse,
};

pub struct DepartmentGrpcHandler {
    service: Arc<DepartmentService>,
}

impl DepartmentGrpcHandler {
    pub fn new(service: Arc<DepartmentService>) -> Self {
        Self { service }
    }
}

fn map_department_to_response(
    department: crate::domain::department::entity::Model,
) -> DepartmentResponse {
    DepartmentResponse {
        id: department.id.to_string(),
        legal_entity_id: department.legal_entity_id.to_string(),
        parent_id: department.parent_id.map(|id| id.to_string()),
        code: department.code,
        name: department.name,
        cost_center: department.cost_center,
    }
}

#[tonic::async_trait]
impl DepartmentGrpcService for DepartmentGrpcHandler {
    async fn get_department(
        &self,
        request: Request<GetDepartmentRequest>,
    ) -> Result<Response<DepartmentResponse>, Status> {
        let id = Uuid::parse_str(&request.into_inner().id)
            .map_err(|_| Status::invalid_argument("Invalid UUID"))?;

        let department = self.service.get_department_by_id(id).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(map_department_to_response(department)))
    }

    async fn create_department(
        &self,
        request: Request<CreateDepartmentRequest>,
    ) -> Result<Response<DepartmentResponse>, Status> {
        let req = request.into_inner();

        let legal_entity_id = Uuid::parse_str(&req.legal_entity_id)
            .map_err(|_| Status::invalid_argument("Invalid legal_entity_id UUID"))?;

        let parent_id = if let Some(ref id_str) = req.parent_id {
            Some(
                Uuid::parse_str(id_str)
                    .map_err(|_| Status::invalid_argument("Invalid parent_id UUID"))?,
            )
        } else {
            None
        };

        let dto = DtoCreateRequest {
            legal_entity_id,
            parent_id,
            code: req.code,
            name: req.name,
            cost_center: req.cost_center,
        };

        let department = self.service.create_department(dto).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(map_department_to_response(department)))
    }

    async fn update_department(
        &self,
        request: Request<UpdateDepartmentRequest>,
    ) -> Result<Response<DepartmentResponse>, Status> {
        let req = request.into_inner();
        let id = Uuid::parse_str(&req.id)
            .map_err(|_| Status::invalid_argument("Invalid UUID"))?;

        let parent_id = if let Some(ref id_str) = req.parent_id {
            Some(
                Uuid::parse_str(id_str)
                    .map_err(|_| Status::invalid_argument("Invalid parent_id UUID"))?,
            )
        } else {
            None
        };

        let dto = DtoUpdateRequest {
            parent_id,
            name: req.name,
            cost_center: req.cost_center,
        };

        let department = self.service.update_department(id, dto).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(map_department_to_response(department)))
    }

    async fn delete_department(
        &self,
        request: Request<DeleteDepartmentRequest>,
    ) -> Result<Response<Empty>, Status> {
        let id = Uuid::parse_str(&request.into_inner().id)
            .map_err(|_| Status::invalid_argument("Invalid UUID"))?;

        self.service.delete_department(id).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(Empty {}))
    }

    async fn list_departments(
        &self,
        request: Request<ListDepartmentsRequest>,
    ) -> Result<Response<ListDepartmentsResponse>, Status> {
        let req = request.into_inner();
        let query = PaginationQuery {
            page: Some(req.page as u64),
            per_page: Some(req.per_page as u64),
        };

        let response = self.service.get_all_departments(query).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(ListDepartmentsResponse {
            items: response
                .items
                .into_iter()
                .map(map_department_to_response)
                .collect(),
            total_items: response.total_items as u32,
            total_pages: response.total_pages as u32,
            current_page: response.current_page as u32,
            per_page: response.per_page as u32,
        }))
    }
}
