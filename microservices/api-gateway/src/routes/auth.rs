use actix_web::{HttpResponse, Result, Scope, post, web};
use tonic::Code;

use crate::{proto::auth, state::AppState};
use crate::dto::auth_dto::{ User, SignUpRequest, SignUpResponse, SignInRequest, SignInResponse };

pub fn auth_routes() -> Scope {
    web::scope("/auth")
        .service(sign_up)
        .service(sign_in)

}

#[post("/sign-up")]
async fn sign_up(
    state: web::Data<AppState>,
    payload: web::Json<SignUpRequest>
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
        .map_err(|error| {
            match error.code() {
                Code::AlreadyExists => actix_web::error::ErrorConflict(error.message().to_string()),
                _ => actix_web::error::ErrorInternalServerError(error.message().to_string()),
            }
        })?
        .into_inner();

    let http_response = SignUpResponse {
        user: User {
            id: response.user.as_ref()
                .map(|u| u.id)
                .unwrap_or_default(),
            username: response.user.as_ref()
                .map(|u| u.username.clone())
                .unwrap_or_default(),
            email: response.user.as_ref()
                .map(|u| u.email.clone())
                .unwrap_or_default(),
        },
        access_token: response.access_token,
    };

    Ok(HttpResponse::Ok().json(http_response))
}


#[post("/sign-in")]
async fn sign_in(
    state: web::Data<AppState>,
    data: web::Json<SignInRequest>,
) -> Result<HttpResponse> {
    let mut client = state.auth_client.clone();

    let request = auth::SignInRequest {
        email: data.email.clone(),
        password: data.password.clone(),
    };

    let response = client
        .sign_in(tonic::Request::new(request))
        .await
        .map_err(|error| {
            match error.code() {
                Code::Unauthenticated => {
                    actix_web::error::ErrorUnauthorized(error.message().to_string())
                },
                _ => actix_web::error::ErrorInternalServerError(error.message().to_string())
            }
        })?
        .into_inner();
    
    let http_response = SignInResponse {
        user: User {
            id: response.user.as_ref()
                .map(|u| u.id)
                .unwrap_or_default(),
            username: response.user.as_ref()
                .map(|u| u.username.clone())
                .unwrap_or_default(),
            email: response.user.as_ref()
                .map(|u| u.email.clone())
                .unwrap_or_default(),
        },
        access_token: response.access_token,
    };

    Ok(HttpResponse::Ok().json(http_response))
}