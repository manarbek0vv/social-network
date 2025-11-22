use tonic::{ transport::Server, transport::Channel, Request, Response, Status };
use auth::auth_server::{ Auth, AuthServer };

pub mod auth {
    tonic::include_proto!("auth");
}

pub mod users {
    tonic::include_proto!("users");
}

use users::users_client::UsersClient;
use users::CreateUserRequest;

#[derive(Debug)]
pub struct AuthService {
    users_client: UsersClient<Channel>
}

impl AuthService {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let users_client = UsersClient::connect("http://[::1]:50052").await?;
        Ok(Self { users_client })
    }
}

#[tonic::async_trait]
impl Auth for AuthService {
    async fn sign_up(
        &self,
        request: Request<auth::User>,
    ) -> Result<Response<auth::SignUpResponse>, Status> {
        let request = request.into_inner();

        let mut users_service = self.users_client.clone();

        let response = users_service.create_user(
            CreateUserRequest {
                username: request.username,
                email: request.email,
                password: request.password,
            }
        ).await?
        .into_inner();

        let created_user = response;

        let response = auth::SignUpResponse {
                user: Some(auth::User {
                    username: created_user.username,
                    email: created_user.email,
                    password: created_user.password,
                }),
                access_token: "HFG87HFDG8FDHFD5GFDH8".to_owned(),
        };

        Ok(Response::new(response))
    }

    async fn sign_in(
        &self,
        request: Request<auth::User>,
    ) -> Result<Response<auth::SignInResponse>, Status> {

        let request = request.into_inner();

        let response = auth::SignInResponse {
                user: Some(auth::User {
                    username: request.username,
                    email: request.email,
                    password: request.password,
                }),
                access_token: "HFG87HFDG8FDHFD5GFDH8".to_owned(),
        };

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let svc = AuthService::new().await?;

    println!("Auth service listening on {}", addr);

    Server::builder()
        .add_service(AuthServer::new(svc))
        .serve(addr)
        .await?;

    Ok(())
}