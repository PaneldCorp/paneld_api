use sqlx::postgres::PgPool;

use crate::models::author::Author;

pub struct AuthorRepo
{
    pool: PgPool
}

impl AuthorRepo
{
    pub fn new(pool: PgPool) -> Self
    {
        Self { pool }
    }

    pub async fn get_all_authors(&self) -> Result<Vec<Author>, sqlx::Error>
    {
        sqlx::query_as("
            Select * from authors;
        ")
            .fetch_all(&self.pool)
            .await
    }
}
