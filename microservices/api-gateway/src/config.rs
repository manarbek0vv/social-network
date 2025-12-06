use std::env;

pub struct Config {
    pub gateway_url: String,
    pub auth_service_url: String,
    pub users_service_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        let auth_service = env::var("AUTH_SERVICE")
            .unwrap_or_else(|_| "127.0.0.1:50051".to_string());
        let users_service = env::var("USERS_SERVICE")
            .unwrap_or_else(|_| "127.0.0.1:50052".to_string());
        
        Self {
            gateway_url: env::var("API_GATEWAY")
                .unwrap_or_else(|_| "127.0.0.1:8080".to_string()),
            auth_service_url: Self::ensure_http_prefix(&auth_service),
            users_service_url: Self::ensure_http_prefix(&users_service),
        }
    }
    
    fn ensure_http_prefix(addr: &str) -> String {
        if addr.starts_with("http://") || addr.starts_with("https://") {
            addr.to_string()
        } else {
            format!("http://{}", addr)
        }
    }
}
