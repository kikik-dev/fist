use dotenvy::dotenv;
use std::env;

pub struct Config {
    pub database_url: String,
    #[allow(dead_code)]
    pub jwt_secret: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok(); // Loads .env file
        Self {
            database_url: env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite::memory:".to_string()),
            jwt_secret: env::var("JWT_SECRET").unwrap_or_else(|_| "default-secret".to_string()),
        }
    }
}
