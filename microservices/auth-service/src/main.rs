use std::env;

use tonic::{ transport::Server };
use auth::auth_server::{ AuthServer };
use crate::service::AuthService;

pub mod service;
pub mod repository;
pub mod domain;

pub mod auth {
    tonic::include_proto!("auth");
}
pub mod users {
    tonic::include_proto!("users");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let addr = env::var("AUTH_SERVICE")
        .unwrap_or_else(|_| "127.0.0.1:50051".to_string())
        .parse()?;
    let svc = AuthService::new().await?;

    println!("Auth service listening on {}", addr);

    Server::builder()
        .add_service(AuthServer::new(svc))
        .serve(addr)
        .await?;

    Ok(())
}