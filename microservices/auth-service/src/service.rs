use tonic::{ Request, Response, Status, transport::Channel};

use crate::proto::auth::{ self, auth_server::Auth };
use crate::domain::token::{Payload, generate_tokens};
use crate::proto::users::{CreateUserRequest, GetUserRequest};
use crate::proto::users::users_client::UsersClient;
use crate::domain::password::{hash_password, verify_password};
use crate::validation::{validate_sign_in, validate_sign_up};

#[derive(Debug)]
pub struct AuthService {
    users_client: UsersClient<Channel>
}

impl AuthService {
    pub async fn new(
        users_service_url: String
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let users_client =
            UsersClient::connect(users_service_url).await?;
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

        let input = validate_sign_up(request)
            .map_err(|e| Status::invalid_argument(e))?;

        let password_hash = hash_password(input.password)
            .map_err(|_| Status::internal("error on password hasing"))?;

        let mut users_service = self.users_client.clone();
        let response = users_service.create_user(
            CreateUserRequest {
                username: input.username,
                email: input.email,
                password: password_hash,
            }
        ).await?
        .into_inner();

        let claims = Payload {
            sub: response.id,
            username: response.username.clone(),
        };

        // ---------- JWT Tokens ----------
        let (access_token, _refresh_token) = generate_tokens(claims)
            .map_err(|_| Status::internal("Error on generating tokens"))?;

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

        let input = validate_sign_in(request)
            .map_err(|e| Status::invalid_argument(e))?;

        let mut users_service = self.users_client.clone();
        let response = users_service.get_user(
            GetUserRequest {
                email: input.email,
        }).await;

        let user = response?.into_inner();

        verify_password(input.password, user.password)
            .map_err(|_| Status::unauthenticated("Wrong password."))?;

        let claims = Payload {
            sub: user.id,
            username: user.username.clone(),
        };

        // ---------- JWT tokens ----------
        let (access_token, _refresh_token) = generate_tokens(claims)
            .map_err(|_| Status::internal("Error on generating tokens"))?;

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