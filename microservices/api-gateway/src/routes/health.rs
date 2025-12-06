use actix_web::{HttpResponse, Result, Scope, get, web};

pub fn health_routes() -> Scope {
    web::scope("/health")
        .service(health)
}

#[get("")]
async fn health() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json("API Gateway is working"))
}