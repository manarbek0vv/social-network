use tonic::{ Code, Request, Response, Status, transport::Channel};
use thiserror::Error;

use crate::auth::{ self, auth_server::Auth };
use crate::domain::token::{Payload, generate_access_token, generate_refresh_token};
use crate::users::{CreateUserRequest, GetUserRequest};
use crate::users::users_client::UsersClient;


use crate::domain::password::{hash_password, verify_password};

#[derive(Debug)]
pub struct AuthService {
    users_client: UsersClient<Channel>
}

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("User already exists")]
    UserAlreadyExists,

    #[error("user service internal error")]
    UserServiceInternal,

    // #[error("Password hashing error")]
    // PasswordHashingError,
}

fn map_users_status_to_auth_error(status: Status) -> AuthError {
    match status.code() {
        Code::NotFound => AuthError::InvalidCredentials,
        Code::AlreadyExists => AuthError::UserAlreadyExists,
        _ => AuthError::UserServiceInternal,
    }
}

fn map_auth_error_to_status(error: AuthError) -> Status {
    match error {
        AuthError::InvalidCredentials => Status::unauthenticated("invalid credentials"),
        AuthError::UserAlreadyExists => Status::already_exists("User with this email exists"),
        AuthError::UserServiceInternal => Status::internal("Auth service internal error"),
    }
}

impl AuthService {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let users_client = 
            UsersClient::connect("http://127.0.0.1:50052").await?;
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
                return Err(Status::internal("error on password hashing"));
            }
        };

        let mut users_service = self.users_client.clone();
        let response = users_service.create_user(
            CreateUserRequest {
                username: request.username,
                email: request.email,
                password: password_hash,
            }
        ).await
        .map_err(map_users_status_to_auth_error)
        .map_err(map_auth_error_to_status)?
        .into_inner();

        let claims = Payload {
            sub: response.id,
            username: response.username.clone(),
        };

        let access_token = generate_access_token(claims.clone())
            .map_err(|_| {
                println!("Token generation error.");
                Status::internal("Failed to generate access token")
            })?;
        let _refresh_token = generate_refresh_token(claims.clone())
            .map_err(|_| {
                println!("Token generation error.");
                Status::internal("Failed to generate refresh token")
            })?;

        let response = auth::SignUpResponse {
                user: Some(auth::User {
                    id: response.id,
                    username: response.username,
                    email: response.email,
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

        let user = response
            .map_err(map_users_status_to_auth_error)
            .map_err(map_auth_error_to_status)?
            .into_inner();

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
        let _refresh_token = generate_refresh_token(claims.clone())
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