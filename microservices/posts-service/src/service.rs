use tonic::{Request, Response, Status};
use crate::{proto::proto::posts::{CreatePostRequest, CreatePostResponse, DeletePostRequest, DeletePostResponse, GetPostRequest, GetPostResponse, GetPostsRequest, GetPostsResponse, UpdatePostRequest, UpdatePostResponse, posts_server::Posts}, repository::PostsRepository};

#[derive(Debug)]
pub struct PostsService {
    repository: PostsRepository
}

impl PostsService {
    pub fn new(repository: PostsRepository) -> Self {
        Self { repository }
    }
}

#[tonic::async_trait]
impl Posts for PostsService {
    async fn get_post(
        &self,
        request: Request<GetPostRequest>,
    ) -> Result<Response<GetPostResponse>, Status> {
        let request = &request.into_inner();

        let post = self.repository.get_post(request)
            .await.map_err(|_| Status::not_found("post with this id not found"))?;

        let response = GetPostResponse {
            post: Some(post.into())
        };

        Ok(Response::new(response))
    }

    async fn get_posts(
        &self,
        _request: Request<GetPostsRequest>,
    ) -> Result<Response<GetPostsResponse>, Status> {
        let posts = self.repository.get_posts()
            .await.map_err(|_| Status::internal("Error on getting posts"))?;

        let response = GetPostsResponse {
            posts: posts.into_iter().map(Into::into).collect()
        };

        Ok(Response::new(response))
    }

    async fn create_post(
        &self,
        request: Request<CreatePostRequest>,
    ) -> Result<Response<CreatePostResponse>, Status> {
        let request = &request.into_inner();

        let created_post = self.repository.create_post(request)
            .await.map_err(|_| Status::internal("Error on creating post"))?;

        let response = CreatePostResponse {
            post: Some(created_post.into())
        };

        Ok(Response::new(response))
    }

    async fn update_post(
        &self,
        request: Request<UpdatePostRequest>,
    ) -> Result<Response<UpdatePostResponse>, Status> {
        let request = &request.into_inner();

        let updated_post = self.repository.update_post(request)
            .await.map_err(|_| Status::internal("Error on updating post"))?;

        let response = UpdatePostResponse {
            post: Some(updated_post.into())
        };

        Ok(Response::new(response))
    }

    async fn delete_post(
        &self,
        request: Request<DeletePostRequest>,
    ) -> Result<Response<DeletePostResponse>, Status> {
        let request = &request.into_inner();

        let deleted_post = self.repository.delete_post(request)
            .await.map_err(|_| Status::internal("Error on deleting post"))?;

        let response = DeletePostResponse {
            post: Some(deleted_post.into())
        };

        Ok(Response::new(response))
    }
}