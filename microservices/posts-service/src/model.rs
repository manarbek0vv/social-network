use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;
use crate::proto::proto::posts::{CreatePostRequest, Post as ProtoPost};
use crate::{domain::time::datetime_to_timestamp};

#[derive(Debug, FromRow)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub user_id: i64,
    pub created_at: DateTime<Utc>,
}

impl From<Post> for ProtoPost {
    fn from(post: Post) -> ProtoPost {
        ProtoPost {
            id: post.id,
            title: post.title,
            description: post.description,
            user_id: post.user_id,
            created_at: datetime_to_timestamp(post.created_at),
        }
    }
}
