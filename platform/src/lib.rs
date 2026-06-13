pub mod proto {
    tonic::include_proto!("platform");
    pub mod auth {
        tonic::include_proto!("auth");
    }
    pub mod owner {
        tonic::include_proto!("owner");
    }
}

pub mod interface;
pub mod domain;
pub mod application;
pub mod db;
pub mod infrastructure;
pub mod middleware;
