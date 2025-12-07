use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Post with this id not found")]
    PostNotFound,
}