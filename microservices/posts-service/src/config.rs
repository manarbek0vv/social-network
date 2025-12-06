use std::env;

pub struct Config {
    pub microservice_url: String,
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        let microservice_url = env::var("POSTS_SERVICE")
                .unwrap_or_else(|_| "127.0.0.1:50053".to_string());
        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/social-network".to_string());

        Self {
            microservice_url,
            database_url,
        }
    }

    pub fn ensure_http_prefix(addr: &str) -> String {
        if addr.starts_with("http://") || addr.starts_with("https://") {
            addr.to_string()
        } else {
            format!("http://{}", addr)
        }
    }
}