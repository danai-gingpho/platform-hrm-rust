pub mod proto {
    tonic::include_proto!("auth");
}

pub mod interface;
pub mod infrastructure;
pub mod application;
pub mod domain;
