use actix_web::{App, HttpResponse, HttpServer, Result, get, post, web::{self}};
use serde::{Deserialize, Serialize};
use tonic::transport::Channel;

pub mod auth {
    tonic::include_proto!("auth");
}
pub mod users {
    tonic::include_proto!("users");
}

use auth::auth_client::AuthClient;
use users::users_client::UsersClient;

#[derive(Clone)]
struct AppState {
    auth_client: AuthClient<Channel>,
    users_client: UsersClient<Channel>,
}

async fn new_state() -> Result<AppState, Box<dyn std::error::Error>> {
    let auth_channel = Channel::from_static("http://127.0.0.1:50051")   
        .connect()
        .await?;
    let users_channel = Channel::from_static("http://127.0.0.1:50052")
        .connect()
        .await?;
    Ok(AppState {
        auth_client: AuthClient::new(auth_channel),
        users_client: UsersClient::new(users_channel),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let state = new_state()
        .await
        .expect("Failed to connect gRPC services");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(health)
            .service(sign_up)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}

#[derive(Deserialize)]
struct SignUpJson {
    username: String,
    email: String,
    password: String,
}
#[derive(Serialize)]
struct SignUpResponse {
    username: String,
    email: String,
    access_token: String,
}

#[get("/health")]
async fn health() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json("API Gateway is working"))
}

#[post("/sign-up")]
async fn sign_up(
    state: web::Data<AppState>,
    payload: web::Json<SignUpJson>
) -> Result<HttpResponse> {
    let mut client = state.auth_client.clone();

    let request = auth::SignUpRequest {
        username: payload.username.clone(),
        email: payload.email.clone(),
        password: payload.password.clone(),
    };

    let response = client
        .sign_up(tonic::Request::new(request))
        .await
        .map_err(|e| {
            eprintln!("gRPC error on sign_up: {e}");
            actix_web::error::ErrorInternalServerError("Auth Service error: sign up")
        })?
        .into_inner();

    let http_response = SignUpResponse {
        username: response.user.as_ref()
            .map(|u| u.username.clone())
            .unwrap_or_default(),
        email: response.user.as_ref()
            .map(|u| u.email.clone())
            .unwrap_or_default(),
        access_token: response.access_token,
    };

    Ok(HttpResponse::Ok().json(http_response))
}

#[derive(Deserialize)]
struct SignInJSON {
    email: String,
    password: String,
}

#[derive(Serialize)]
struct SignInResponse {
    id: i64,
    username: String,
    email: String,
}

#[post("sign-in")]
async fn sign_in(
    state: web::Data<AppState>,
    data: web::Json<SignInJSON>,
) -> Result<HttpResponse> {
    let mut client = state.auth_client.clone();

    let request = auth::SignInRequest {
        email: data.email.clone(),
        password: data.password.clone(),
    };

    let response = client
        .sign_in(tonic::Request::new(request))
        .await
        .map_err(|e| {
            eprintln!("gRPC error on sign_in: {e}");
            actix_web::error::ErrorInternalServerError("Auth Service error: sign up")
        })?
        .into_inner();
    
    let http_response = SignInResponse {
        id: response.user.as_ref()
            .map(|u| u.id.clone())
            .unwrap_or_default(),
        username: response.user.as_ref()
            .map(|u| u.username.clone())
            .unwrap_or_default(),
        email: response.user.as_ref()
            .map(|u| u.email.clone())
            .unwrap_or_default(),
    };

    Ok(HttpResponse::Ok().json(http_response))
}