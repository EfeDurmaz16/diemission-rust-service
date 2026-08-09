use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub port: u16,
    pub node_api_base_url: String,
    pub node_api_username: String,
    pub node_api_password: String,
}

impl Config {
    pub fn from_env() -> Self {
        let _ = dotenvy::dotenv();

        Self {
            port: env::var("PORT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(8080),
            node_api_base_url: env::var("NODE_API_BASE_URL")
                .unwrap_or_else(|_| "http://localhost:5007".to_string())
                .trim_end_matches('/')
                .to_string(),
            node_api_username: require("NODE_API_USERNAME"),
            node_api_password: require("NODE_API_PASSWORD"),
        }
    }
}

/// Credentials have no safe default, so fail loudly at startup instead.
fn require(key: &str) -> String {
    env::var(key).unwrap_or_else(|_| panic!("{key} is not set. Run `cp .env.example .env` first."))
}
