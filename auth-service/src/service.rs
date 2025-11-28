use tonic::{Request, Response, Status, transport::Channel};

use crate::auth::{ self, auth_server::Auth };
use crate::domain::token::{Payload, generate_access_token, generate_refresh_token};
use crate::users::{CreateUserRequest, GetUserRequest};
use crate::users::users_client::UsersClient;

use crate::domain::password::{hash_password, verify_password};

#[derive(Debug)]
pub struct AuthService {
    users_client: UsersClient<Channel>
}

impl AuthService {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let users_client = UsersClient::connect("http://127.0.0.1:50052").await?;
        Ok(Self { users_client })
    }
}

#[tonic::async_trait]
impl Auth for AuthService {
    async fn sign_up(
        &self,
        request: Request<auth::SignUpRequest>,
    ) -> Result<Response<auth::SignUpResponse>, Status> {
        let request = request.into_inner();

        let password_hash = match hash_password(request.password) {
            Ok(hash) => hash,
            Err(e) => {
                println!("Error occured: {}", e);
                return Err(Status::internal(e));
            }
        };

        let mut users_service = self.users_client.clone();
        let is_user_exists = users_service.get_user(GetUserRequest {
            email: String::clone(&request.email)
        }).await;

        if let Ok(_u) = is_user_exists {
            println!("User with this email already exists");
            return Err(Status::already_exists("User with this email already exists."));
        }

        let mut users_service = self.users_client.clone();
        let response = users_service.create_user(
            CreateUserRequest {
                username: request.username,
                email: request.email,
                password: password_hash,
            }
        ).await;

        let created_user = match response {
            Ok(u) => u.into_inner(),
            Err(e) => {
                println!("Error sign up.");
                return Err(e);
            }
        };

        let claims = Payload {
            sub: created_user.id,
            username: created_user.username.clone(),
        };

        let access_token = generate_access_token(claims.clone())
            .map_err(|_| {
                println!("Token generation error.");
                Status::internal("Failed to generate access token")
            })?;
        let refresh_token = generate_refresh_token(claims.clone())
            .map_err(|_| {
                println!("Token generation error.");
                Status::internal("Failed to generate refresh token")
            })?;

        let response = auth::SignUpResponse {
                user: Some(auth::User {
                    id: created_user.id,
                    username: created_user.username,
                    email: created_user.email,
                }),
                access_token,
        };

        Ok(Response::new(response))
    }

    async fn sign_in(
        &self,
        request: Request<auth::SignInRequest>,
    ) -> Result<Response<auth::SignInResponse>, Status> {

        let request = request.into_inner();

        let mut users_service = self.users_client.clone();
        let response = users_service.get_user(GetUserRequest {
            email: request.email,
        }).await;

        let user = match response {
            Err(_status) => return Err(Status::unauthenticated("User with this email not found")),
            Ok(u) => u.into_inner(),
        };

        if let Err(_e) = verify_password(request.password, user.password.clone()) {
            return Err(Status::unauthenticated("Wrong password."));
        }

        let claims = Payload {
            sub: user.id,
            username: user.username.clone(),
        };

        let access_token = generate_access_token(claims.clone())
            .map_err(|_| {
                println!("Token generation error.");
                Status::internal("Failed to generate access token")
            })?;
        let refresh_token = generate_refresh_token(claims.clone())
            .map_err(|_| {
                println!("Token generation error.");
                Status::internal("Failed to generate refresh token")
            })?;

        let response = auth::SignInResponse {
                user: Some(auth::User {
                    id: user.id,
                    username: user.username,
                    email: user.email,
                }),
                access_token,
        };

        Ok(Response::new(response))
    }
}