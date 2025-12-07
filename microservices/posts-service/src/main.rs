use std::net::SocketAddr;

use sqlx::postgres::PgPoolOptions;
use tonic::transport::Server;

use crate::config::Config;
use crate::proto::proto::posts::posts_server::{ PostsServer };
use crate::repository::PostsRepository;
use crate::service::PostsService;

pub mod config;
pub mod service;
pub mod proto;
pub mod repository;
pub mod error;
pub mod model;
pub mod domain;
pub mod validation;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let config = Config::from_env();

    let addr: SocketAddr = config.microservice_url.parse()?;

    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;

    let repository = PostsRepository::new(db);
    let service = PostsService::new(repository);

    println!("Posts service listening on {}", addr);

    Server::builder()
        .add_service(PostsServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}