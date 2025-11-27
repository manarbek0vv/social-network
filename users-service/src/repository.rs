use sqlx::{ Pool, Postgres };
use crate::structs::{NewUser, User};

#[derive(Clone, Debug)]
pub struct UsersRepository {
    db: Pool<Postgres>
}

impl UsersRepository {
    pub fn new(db: Pool<Postgres>) -> Self {
        Self { db }
    }

    pub async fn get_user_by_email(
        &self,
        email: String,
    ) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as!(User, "
            SELECT id, username, email, password
            FROM users
            WHERE email = $1
        ", email)
        .fetch_optional(&self.db)
        .await
    }

    pub async fn create_user(
        &self,
        data: NewUser,
    ) -> Result<User, sqlx::error::Error> {
        sqlx::query_as!(User, "
            INSERT INTO users (username, email, password)
            VALUES ($1, $2, $3)
            RETURNING id, username, email, password
        ",
        data.username,
        data.email,
        data.password,
        )
        .fetch_one(&self.db)
        .await
    }
}