use tonic::transport::Channel;
use crate::config::Config;


// ------------- Including proto -------------
use crate::proto::auth::auth_client::AuthClient;
use crate::proto::users::users_client::UsersClient;


#[derive(Clone)]
pub struct AppState {
    pub auth_client: AuthClient<Channel>,
    pub users_client: UsersClient<Channel>,
}

impl AppState {
    pub async fn new(config: &Config) -> Result<Self, Box<dyn std::error::Error>> {
        let auth_channel = Channel::from_shared(config.auth_service_url.clone())?
                .connect().await?;
        let users_channel = Channel::from_shared(config.users_service_url.clone())?
            .connect().await?;

        let state = Self {
            auth_client: AuthClient::new(auth_channel),
            users_client: UsersClient::new(users_channel),
        };

        Ok(state)
    }
}