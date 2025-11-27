#[derive(Debug)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
}