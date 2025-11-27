use tonic::{ Request, Response, Status };
use crate::repository::UsersRepository;
use crate::structs::NewUser;
use crate::users::users_server::Users;
use crate::users::{CreateUserRequest, GetUserRequest, User};

#[derive(Debug, Clone)]
pub struct UsersService {
    repository: UsersRepository
}

impl UsersService {
    pub fn new(repository: UsersRepository) -> Self {
        Self { repository }
    }
}

#[tonic::async_trait]
impl Users for UsersService {
    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<User>, Status> {
        let request = request.into_inner();

        println!("Getting user with username: {}", request.email);

        let response =
            self.repository.get_user_by_email(request.email).await;

        let user = match response {
            Ok(Some(u)) => u,
            Ok(None) => {
                println!("User not found.");
                return Err(Status::not_found("User with this username not found."));
            },
            Err(_e) => {
                println!("Something went wrong.");
                return Err(Status::internal("Internal server error."));
            },
        };

        let response = User {
                id: user.id,
                username: user.username,
                email: user.email,
                password: user.password,
        };

        Ok(Response::new(response))
    }

    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<User>, Status> {
        let request = request.into_inner();

        println!("Creating user with credentials: {:?}", request);

        let request_for_repo = NewUser {
            username: request.username,
            email: request.email,
            password: request.password,
        };

        let created_user =
            self.repository.create_user(request_for_repo).await;

        let user = match created_user {
            Ok(u) => u,
            Err(_e) => {
                println!("Error on creating user.");
                return Err(Status::internal("Internal server error."));
            }
        };

        let response = User {
                id: user.id,
                username: user.username,
                email: user.email,
                password: user.password,
        };

        Ok(Response::new(response))
    }
}