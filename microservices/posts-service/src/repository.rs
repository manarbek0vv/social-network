use sqlx::{Pool, Postgres};

#[derive(Debug, Clone)]
pub struct PostsRepository {
    pub db: Pool<Postgres>
}

impl PostsRepository {
    pub fn new(db: Pool<Postgres>) -> Self {
        Self { db }
    }
}