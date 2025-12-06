use actix_web::{App, HttpServer, middleware::Logger, web };

use crate::{config::Config, state::AppState};
use crate::routes::{ auth::auth_routes, health::health_routes };

pub mod state;
pub mod config;
pub mod proto;
pub mod dto;

pub mod routes;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    env_logger::init();

    let config = Config::from_env();
    let state = AppState::new(&config)
        .await.map_err(|_| {
            println!("Application state error");
            std::io::Error::new(
                std::io::ErrorKind::ConnectionRefused,
                "Failed to connect to microservices"
            )
        }).unwrap();

    HttpServer::new(move || {
        App::new()
        .wrap(Logger::default())
        .app_data(web::Data::new(state.clone()))
        .service(
            web::scope("/api")
                .service(health_routes())
                .service(auth_routes())
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}