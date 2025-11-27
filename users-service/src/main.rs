use sqlx::postgres::PgPoolOptions;
use std::env;
use std::net::SocketAddr;
use tonic::transport::Server;
use users::users_server::{ UsersServer };

use crate::repository::UsersRepository;
use crate::service::UsersService;

pub mod service;
pub mod repository;
pub mod structs;

pub mod users {
    tonic::include_proto!("users");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    // Users service URL
    let address: SocketAddr = env::var("USERS_SERVICE")
        .unwrap_or_else(|_e| "127.0.0.1:50051".to_string())
        .parse()?;

    // Database reaching
    let database_url = env::var("DATABASE_URL")?;
    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let repository = UsersRepository::new(db);
    let svc = UsersService::new(repository);

    println!("Users service listening on {}", address);

    Server::builder()
        .add_service(UsersServer::new(svc))
        .serve(address)
        .await?;

    Ok(())
}