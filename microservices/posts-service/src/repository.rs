use sqlx::{Pool, Postgres};

use crate::model::{Post};
use crate::proto::proto::posts::{CreatePostRequest, DeletePostRequest, GetPostRequest, UpdatePostRequest};
use crate::{error::RepositoryError};

#[derive(Debug, Clone)]
pub struct PostsRepository {
    pub db: Pool<Postgres>
}

impl PostsRepository {
    pub fn new(db: Pool<Postgres>) -> Self {
        Self { db }
    }

    pub async fn get_post(
        &self,
        value: &GetPostRequest,
    ) -> Result<Post, RepositoryError> {
        let id = value.id;

        let result = sqlx::query_as!(
            Post,
        r#"
            SELECT * FROM posts
            WHERE id = $1
        "#, id
        ).fetch_optional(&self.db)
        .await?;

        result.ok_or(RepositoryError::PostNotFound)
    }

    pub async fn get_posts(
        &self
    ) -> Result<Vec<Post>, RepositoryError> {
        let result = sqlx::query_as!(
            Post,
        r#"
            SELECT * FROM posts
        "#
        ).fetch_all(&self.db)
        .await?;

        Ok(result)
    }

    pub async fn create_post(
        &self,
        value: &CreatePostRequest,
    ) -> Result<Post, RepositoryError> {
        let CreatePostRequest { title, description, user_id } = value;

        sqlx::query_as!(
            Post,
        r#"
            INSERT INTO posts (title, description, user_id)
            VALUES ($1, $2, $3)
            RETURNING id, title, description, user_id, created_at
        "#, title, description, user_id
        ).fetch_one(&self.db)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e))
    }

    pub async fn update_post(
        &self,
        value: &UpdatePostRequest,
    ) -> Result<Post, RepositoryError> {
        let UpdatePostRequest { id, title, description } = value;

        sqlx::query_as!(
            Post,
        r#"
            UPDATE posts
            SET title = $1, description = $2
            WHERE id = $3
            RETURNING id, title, description, user_id, created_at
        "#, title, description, id
        ).fetch_one(&self.db)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e))
    }

    pub async fn delete_post(
        &self,
        value: &DeletePostRequest,
    ) -> Result<Post, RepositoryError> {
        let id = value.id;

        sqlx::query_as!(
            Post,
        r#"
            DELETE FROM posts
            WHERE id = $1
            RETURNING id, title, description, user_id, created_at
        "#, id
        ).fetch_one(&self.db)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e))
    }
}