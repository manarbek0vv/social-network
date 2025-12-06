use tonic::{Request, Response, Status};
use crate::{proto::posts::{CreatePostRequest, CreatePostResponse, DeletePostRequest, DeletePostResponse, GetPostRequest, GetPostResponse, GetPostsRequest, GetPostsResponse, Post, UpdatePostRequest, UpdatePostResponse, posts_server::Posts}, repository::PostsRepository};

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
        let _request = request.into_inner();

        let response = GetPostResponse {
            post: Some(Post {
                        id: 1,
                        title: String::from("Social network"),
                        description: String::from("Developing an IT startup"),
                        user_id: 123432,
                        created_at: 1243254353,
                })
        };

        Ok(Response::new(response))
    }

    async fn get_posts(
        &self,
        _request: Request<GetPostsRequest>,
    ) -> Result<Response<GetPostsResponse>, Status> {

        let response = GetPostsResponse {
            posts: vec![
                Post {
                    id: 1,
                    title: String::from("Social network"),
                    description: String::from("Developing an IT startup"),
                    user_id: 123432,
                    created_at: 1243254353,
                }
            ]
        };

        Ok(Response::new(response))
    }

    async fn create_post(
        &self,
        _request: Request<CreatePostRequest>,
    ) -> Result<Response<CreatePostResponse>, Status> {
        let response = CreatePostResponse {
            post: Some(Post {
                        id: 1,
                        title: String::from("Social network"),
                        description: String::from("Developing an IT startup"),
                        user_id: 123432,
                        created_at: 1243254353,
                })
        };

        Ok(Response::new(response))
    }

    async fn update_post(
        &self,
        _request: Request<UpdatePostRequest>,
    ) -> Result<Response<UpdatePostResponse>, Status> {
        let response = UpdatePostResponse {
            post: Some(Post {
                        id: 1,
                        title: String::from("Social network"),
                        description: String::from("Developing an IT startup"),
                        user_id: 123432,
                        created_at: 1243254353,
                })
        };

        Ok(Response::new(response))
    }

    async fn delete_post(
        &self,
        _request: Request<DeletePostRequest>,
    ) -> Result<Response<DeletePostResponse>, Status> {
        let response = DeletePostResponse {
            post: Some(Post {
                        id: 1,
                        title: String::from("Social network"),
                        description: String::from("Developing an IT startup"),
                        user_id: 123432,
                        created_at: 1243254353,
                })
        };

        Ok(Response::new(response))
    }
}