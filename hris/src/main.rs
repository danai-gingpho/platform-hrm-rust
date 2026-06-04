mod domain;
mod application;
mod infrastructure;
mod interface;
mod utils;

use std::sync::Arc;
use std::net::SocketAddr;
use axum::{Router, routing::get, Json};
use tonic::transport::Server;
use crate::infrastructure::db::init_db;

// Repositories
use crate::infrastructure::db::company::SeaOrmCompanyRepository;
use crate::infrastructure::db::employee::SeaOrmEmployeeRepository;
use crate::infrastructure::db::department::SeaOrmDepartmentRepository;
use crate::infrastructure::db::branch::SeaOrmBranchRepository;
use crate::infrastructure::db::position::SeaOrmPositionRepository;
use crate::infrastructure::db::payroll_item::SeaOrmPayrollItemRepository;
use crate::infrastructure::db::payroll_period::SeaOrmPayrollPeriodRepository;
use crate::infrastructure::db::payroll_run::SeaOrmPayrollRunRepository;
use crate::infrastructure::db::salary_structure::SeaOrmSalaryStructureRepository;
use crate::infrastructure::db::salary_structure_item::SeaOrmSalaryStructureItemRepository;
use crate::infrastructure::db::tax_rate::SeaOrmTaxRateRepository;
use crate::infrastructure::db::social_security_rate::SeaOrmSocialSecurityRateRepository;
use crate::infrastructure::db::allowance_type::SeaOrmAllowanceTypeRepository;
use crate::infrastructure::db::leave_type::SeaOrmLeaveTypeRepository;
use crate::infrastructure::db::shift::SeaOrmShiftRepository;
use crate::infrastructure::db::employment::SeaOrmEmploymentRepository;
use crate::infrastructure::db::employment_contract::SeaOrmEmploymentContractRepository;
use crate::infrastructure::db::leave_request::SeaOrmLeaveRequestRepository;
use crate::infrastructure::db::leave_balance::SeaOrmLeaveBalanceRepository;
use crate::infrastructure::db::attendance_log::SeaOrmAttendanceLogRepository;
use crate::infrastructure::db::employee_allowance::SeaOrmEmployeeAllowanceRepository;

// Services
use crate::application::company::service::CompanyService;
use crate::application::employee::service::EmployeeService;
use crate::application::department::service::DepartmentService;
use crate::application::branch::service::BranchService;
use crate::application::position::service::PositionService;
use crate::application::payroll_item::service::PayrollItemService;
use crate::application::payroll_period::service::PayrollPeriodService;
use crate::application::payroll_run::service::PayrollRunService;
use crate::application::salary_structure::service::SalaryStructureService;
use crate::application::salary_structure_item::service::SalaryStructureItemService;
use crate::application::tax_rate::service::TaxRateService;
use crate::application::social_security_rate::service::SocialSecurityRateService;
use crate::application::allowance_type::service::AllowanceTypeService;
use crate::application::leave_type::service::LeaveTypeService;
use crate::application::shift::service::ShiftService;
use crate::application::employment::service::EmploymentService;
use crate::application::employment_contract::service::EmploymentContractService;
use crate::application::leave_request::service::LeaveRequestService;
use crate::application::leave_balance::service::LeaveBalanceService;
use crate::application::attendance_log::service::AttendanceLogService;
use crate::application::employee_allowance::service::EmployeeAllowanceService;

// gRPC Handlers
use crate::interface::grpc::company_handler::{CompanyGrpcHandler, CompanyServiceServer};
use crate::interface::grpc::employee_handler::{EmployeeGrpcHandler, EmployeeServiceServer};
use crate::interface::grpc::department_handler::{DepartmentGrpcHandler, DepartmentServiceServer};
use crate::interface::grpc::branch_handler::{BranchGrpcHandler, BranchServiceServer};
use crate::interface::grpc::position_handler::{PositionGrpcHandler, PositionServiceServer};
use crate::interface::grpc::payroll_item_handler::{PayrollItemGrpcHandler, PayrollItemServiceServer};
use crate::interface::grpc::payroll_period_handler::{PayrollPeriodGrpcHandler, PayrollPeriodServiceServer};
use crate::interface::grpc::payroll_run_handler::{PayrollRunGrpcHandler, PayrollRunServiceServer};
use crate::interface::grpc::salary_structure_handler::{SalaryStructureGrpcHandler, SalaryStructureServiceServer};
use crate::interface::grpc::salary_structure_item_handler::{SalaryStructureItemGrpcHandler, SalaryStructureItemServiceServer};
use crate::interface::grpc::tax_rate_handler::{TaxRateGrpcHandler, TaxRateServiceServer};
use crate::interface::grpc::social_security_rate_handler::{SocialSecurityRateGrpcHandler, SocialSecurityRateServiceServer};
use crate::interface::grpc::allowance_type_handler::{AllowanceTypeGrpcHandler, AllowanceTypeServiceServer};
use crate::interface::grpc::leave_type_handler::{LeaveTypeGrpcHandler, LeaveTypeServiceServer};
use crate::interface::grpc::shift_handler::{ShiftGrpcHandler, ShiftServiceServer};
use crate::interface::grpc::employment_handler::{EmploymentGrpcHandler, EmploymentServiceServer};
use crate::interface::grpc::employment_contract_handler::{EmploymentContractGrpcHandler, EmploymentContractServiceServer};
use crate::interface::grpc::leave_request_handler::{LeaveRequestGrpcHandler, LeaveRequestServiceServer};
use crate::interface::grpc::leave_balance_handler::{LeaveBalanceGrpcHandler, LeaveBalanceServiceServer};
use crate::interface::grpc::attendance_log_handler::{AttendanceLogGrpcHandler, AttendanceLogServiceServer};
use crate::interface::grpc::employee_allowance_handler::{EmployeeAllowanceGrpcHandler, EmployeeAllowanceServiceServer};

use crate::interface::http;

use crate::domain::shared::dtos::PaginationQuery;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Initialize Database
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/hris".to_string());
    let db = init_db(&database_url).await.expect("Failed to connect to database");

    // 2. Initialize Repositories and Services
    let company_repo = Arc::new(SeaOrmCompanyRepository::new(db.clone()));
    let company_service = Arc::new(CompanyService::new(company_repo));

    let employee_repo = Arc::new(SeaOrmEmployeeRepository::new(db.clone()));
    let employee_service = Arc::new(EmployeeService::new(employee_repo));

    let department_repo = Arc::new(SeaOrmDepartmentRepository::new(db.clone()));
    let department_service = Arc::new(DepartmentService::new(department_repo));

    let branch_repo = Arc::new(SeaOrmBranchRepository::new(db.clone()));
    let branch_service = Arc::new(BranchService::new(branch_repo));

    let position_repo = Arc::new(SeaOrmPositionRepository::new(db.clone()));
    let position_service = Arc::new(PositionService::new(position_repo));

    let payroll_item_repo = Arc::new(SeaOrmPayrollItemRepository::new(db.clone()));
    let payroll_item_service = Arc::new(PayrollItemService::new(payroll_item_repo));

    let payroll_period_repo = Arc::new(SeaOrmPayrollPeriodRepository::new(db.clone()));
    let payroll_period_service = Arc::new(PayrollPeriodService::new(payroll_period_repo));

    let payroll_run_repo = Arc::new(SeaOrmPayrollRunRepository::new(db.clone()));
    let payroll_run_service = Arc::new(PayrollRunService::new(payroll_run_repo));

    let salary_structure_repo = Arc::new(SeaOrmSalaryStructureRepository::new(db.clone()));
    let salary_structure_service = Arc::new(SalaryStructureService::new(salary_structure_repo));

    let salary_structure_item_repo = Arc::new(SeaOrmSalaryStructureItemRepository::new(db.clone()));
    let salary_structure_item_service = Arc::new(SalaryStructureItemService::new(salary_structure_item_repo));

    let tax_rate_repo = Arc::new(SeaOrmTaxRateRepository::new(db.clone()));
    let tax_rate_service = Arc::new(TaxRateService::new(tax_rate_repo));

    let social_security_rate_repo = Arc::new(SeaOrmSocialSecurityRateRepository::new(db.clone()));
    let social_security_rate_service = Arc::new(SocialSecurityRateService::new(social_security_rate_repo));

    let allowance_type_repo = Arc::new(SeaOrmAllowanceTypeRepository::new(db.clone()));
    let allowance_type_service = Arc::new(AllowanceTypeService::new(allowance_type_repo));

    let leave_type_repo = Arc::new(SeaOrmLeaveTypeRepository::new(db.clone()));
    let leave_type_service = Arc::new(LeaveTypeService::new(leave_type_repo));

    let shift_repo = Arc::new(SeaOrmShiftRepository::new(db.clone()));
    let shift_service = Arc::new(ShiftService::new(shift_repo));

    let employment_repo = Arc::new(SeaOrmEmploymentRepository::new(db.clone()));
    let employment_service = Arc::new(EmploymentService::new(employment_repo));

    let employment_contract_repo = Arc::new(SeaOrmEmploymentContractRepository::new(db.clone()));
    let employment_contract_service = Arc::new(EmploymentContractService::new(employment_contract_repo));

    let leave_request_repo = Arc::new(SeaOrmLeaveRequestRepository::new(db.clone()));
    let leave_request_service = Arc::new(LeaveRequestService::new(leave_request_repo));

    let leave_balance_repo = Arc::new(SeaOrmLeaveBalanceRepository::new(db.clone()));
    let leave_balance_service = Arc::new(LeaveBalanceService::new(leave_balance_repo));

    let attendance_log_repo = Arc::new(SeaOrmAttendanceLogRepository::new(db.clone()));
    let attendance_log_service = Arc::new(AttendanceLogService::new(attendance_log_repo));

    let employee_allowance_repo = Arc::new(SeaOrmEmployeeAllowanceRepository::new(db.clone()));
    let employee_allowance_service = Arc::new(EmployeeAllowanceService::new(employee_allowance_repo));

    // 3. gRPC Setup
    let grpc_router = Server::builder()
        .add_service(CompanyServiceServer::new(CompanyGrpcHandler::new(company_service.clone())))
        .add_service(EmployeeServiceServer::new(EmployeeGrpcHandler::new(employee_service.clone())))
        .add_service(DepartmentServiceServer::new(DepartmentGrpcHandler::new(department_service.clone())))
        .add_service(BranchServiceServer::new(BranchGrpcHandler::new(branch_service.clone())))
        .add_service(PositionServiceServer::new(PositionGrpcHandler::new(position_service.clone())))
        .add_service(PayrollItemServiceServer::new(PayrollItemGrpcHandler::new(payroll_item_service.clone())))
        .add_service(PayrollPeriodServiceServer::new(PayrollPeriodGrpcHandler::new(payroll_period_service.clone())))
        .add_service(PayrollRunServiceServer::new(PayrollRunGrpcHandler::new(payroll_run_service.clone())))
        .add_service(SalaryStructureServiceServer::new(SalaryStructureGrpcHandler::new(salary_structure_service.clone())))
        .add_service(SalaryStructureItemServiceServer::new(SalaryStructureItemGrpcHandler::new(salary_structure_item_service.clone())))
        .add_service(TaxRateServiceServer::new(TaxRateGrpcHandler::new(tax_rate_service.clone())))
        .add_service(SocialSecurityRateServiceServer::new(SocialSecurityRateGrpcHandler::new(social_security_rate_service.clone())))
        .add_service(AllowanceTypeServiceServer::new(AllowanceTypeGrpcHandler::new(allowance_type_service.clone())))
        .add_service(LeaveTypeServiceServer::new(LeaveTypeGrpcHandler::new(leave_type_service.clone())))
        .add_service(ShiftServiceServer::new(ShiftGrpcHandler::new(shift_service.clone())))
        .add_service(EmploymentServiceServer::new(EmploymentGrpcHandler::new(employment_service.clone())))
        .add_service(EmploymentContractServiceServer::new(EmploymentContractGrpcHandler::new(employment_contract_service.clone())))
        .add_service(LeaveRequestServiceServer::new(LeaveRequestGrpcHandler::new(leave_request_service.clone())))
        .add_service(LeaveBalanceServiceServer::new(LeaveBalanceGrpcHandler::new(leave_balance_service.clone())))
        .add_service(AttendanceLogServiceServer::new(AttendanceLogGrpcHandler::new(attendance_log_service.clone())))
        .add_service(EmployeeAllowanceServiceServer::new(EmployeeAllowanceGrpcHandler::new(employee_allowance_service.clone())))
        .into_router();


    // 4. REST Setup (Axum)
    let rest_router = Router::new()
        .route("/health", get(|| async { "OK" }))
        .nest("/companies", http::company_handler::router(company_service.clone()))
        .nest("/employees", http::employee_handler::router(employee_service.clone()))
        .nest("/departments", http::department_handler::router(department_service.clone()))
        .nest("/branches", http::branch_handler::router(branch_service.clone()))
        .nest("/positions", http::position_handler::router(position_service.clone()))
        .nest("/payroll-items", http::payroll_item_handler::router(payroll_item_service.clone()))
        .nest("/payroll-periods", http::payroll_period_handler::router(payroll_period_service.clone()))
        .nest("/payroll-runs", http::payroll_run_handler::router(payroll_run_service.clone()))
        .nest("/salary-structures", http::salary_structure_handler::router(salary_structure_service.clone()))
        .nest("/salary-structure-items", http::salary_structure_item_handler::router(salary_structure_item_service.clone()))
        .nest("/tax-rates", http::tax_rate_handler::router(tax_rate_service.clone()))
        .nest("/social-security-rates", http::social_security_rate_handler::router(social_security_rate_service.clone()))
        .nest("/allowance-types", http::allowance_type_handler::router(allowance_type_service.clone()))
        .nest("/leave-types", http::leave_type_handler::router(leave_type_service.clone()))
        .nest("/shifts", http::shift_handler::router(shift_service.clone()))
        .nest("/employments", http::employment_handler::router(employment_service.clone()))
        .nest("/employment-contracts", http::employment_contract_handler::router(employment_contract_service.clone()))
        .nest("/leave-requests", http::leave_request_handler::router(leave_request_service.clone()))
        .nest("/leave-balances", http::leave_balance_handler::router(leave_balance_service.clone()))
        .nest("/attendance-logs", http::attendance_log_handler::router(attendance_log_service.clone()))
        .nest("/employee-allowances", http::employee_allowance_handler::router(employee_allowance_service.clone()))
        .layer(axum::middleware::from_fn(http::middleware::extract_tenant_context))
        .layer(TraceLayer::new_for_http());

    // 5. Multiplexing logic
    let combined_router = rest_router.fallback_service(grpc_router);

    // 6. Start Server
    let addr = SocketAddr::from(([0, 0, 0, 0], 50051));
    println!("Multiplexed HRIS Server listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, combined_router).await?;

    Ok(())
}
