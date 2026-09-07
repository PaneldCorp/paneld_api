use dotenv::dotenv;
use std::env;

pub struct Database
{
    host: String,
    port: String,
}

impl Database
{
    pub fn init() -> Self
    {
        dotenv().ok();
        let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

        Self { host, port }
    }

    pub fn get_binding(&self) -> String
    {
        format!("{}:{}", self.host, self.port)
    }
}
