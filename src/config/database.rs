use dotenv::dotenv;
use std::{
    time::Duration,
    env
};
use sqlx::postgres::{PgPool, PgPoolOptions};

#[derive(Clone)]
pub struct Database
{
    host: String,
    port: String,
    pub pool: PgPool
}

impl Database
{
    pub async fn init() -> Self
    {
        dotenv().ok();
        let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
        let database_uri = env::var("DATABASE_URI").expect("Please enter DATABASE_URI to process a good connection."); // TODO: Change error message

        let pool = PgPoolOptions::new()
            .max_connections(10)
            .min_connections(1)
            .acquire_timeout(Duration::from_secs(5))
            .connect(&database_uri)
            .await
            .expect("Database connection failed.");

        Self { host, port, pool }
    }

    pub fn get_binding(&self) -> String
    {
        format!("{}:{}", self.host, self.port)
    }
}
