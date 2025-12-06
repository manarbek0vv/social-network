use serde::{Deserialize, Serialize};


// ---------- Sign Up ----------
#[derive(Serialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct SignUpRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct SignUpResponse {
    pub user: User,
    pub access_token: String,
}

// ---------- Sign In ----------

#[derive(Deserialize)]
pub struct SignInRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct SignInResponse {
    pub user: User,
    pub access_token: String,
}