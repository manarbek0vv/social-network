use sqlx::postgres::PgPoolOptions;
use sqlx::prelude::FromRow;
use sqlx::{Pool, Postgres};
use std::env;
use tonic::{ transport::Server, Request, Response, Status };
use users::users_server::{ Users, UsersServer };
use users::{ User, GetUserRequest, CreateUserRequest };

pub mod users {
    tonic::include_proto!("users");
}

#[derive(Debug)]
pub struct UsersService {
    db: Pool<Postgres>
}

impl UsersService {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let db = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;

        Ok(Self { db })
    }
}

#[derive(Debug, FromRow)]
struct DbUser {
    id: i32,
    username: String,
    email: String,
    password: String,
}

#[tonic::async_trait]
impl Users for UsersService {
    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<User>, Status> {
        let request = request.into_inner();

        println!("Getting user with username: {}", request.username);

        let row = sqlx::query_as::<_, DbUser>("
            SELECT id, username, email, password
            FROM users
            WHERE username = $1
        ").bind(request.username)
        .fetch_one(&self.db)
        .await
        .map_err(|e| {
            match e {
                sqlx::Error::RowNotFound => Status::not_found("User not found"),
                _ => {
                    eprintln!("DB error on get_user: {e}");
                    Status::internal("database error")
                }
            }
        })?;

        let response = User {
                id: row.id as u32,
                username: row.username,
                email: row.email,
                password: row.password,
        };

        Ok(Response::new(response))
    }

    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<User>, Status> {
        let request = request.into_inner();

        println!("Creating user with credentials: {:?}", request);

        let row = sqlx::query_as::<_, DbUser>("
            INSERT INTO users (username, email, password)
            VALUES ($1, $2, $3)
            RETURNING id, username, email, password
        ")
        .bind(request.username)
        .bind(request.email)
        .bind(request.password)
        .fetch_one(&self.db)
        .await
        .map_err(|e| {
            eprintln!("DB error on create_user: {e}");
            Status::internal("Database error")
        })?;

        let response = User {
                id: 1,
                username: row.username,
                email: row.email,
                password: row.password,
        };

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let address = "[::1]:50052".parse()?;
    let database_url = env::var("DATABASE_URL")?;
    let svc = UsersService::new(&database_url).await?;

    println!("Users service listening on {}", address);

    Server::builder()
        .add_service(UsersServer::new(svc))
        .serve(address)
        .await?;

    Ok(())
}