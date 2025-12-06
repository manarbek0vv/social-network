use tonic::{ transport::Server };
use crate::config::Config;
use crate::proto::auth::auth_server::{ AuthServer };
use crate::service::AuthService;

pub mod service;
pub mod domain;
pub mod proto;
pub mod config;
pub mod error;
pub mod validation;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let config = Config::from_env();

    let addr = config.microservice_url.parse()?;

    let service = AuthService::new(
        config.users_service_url
    ).await?;

    println!("Auth service listening on {}", addr);

    Server::builder()
        .add_service(AuthServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}