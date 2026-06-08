pub mod proto {
    tonic::include_proto!("hris");
    pub mod platform {
        tonic::include_proto!("platform");
    }
}

pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod interface;
pub mod utils;
