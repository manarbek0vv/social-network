extern crate prost_types;

pub mod proto {
    pub mod posts {
        tonic::include_proto!("posts");
    }
}