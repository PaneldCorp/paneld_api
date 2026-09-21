use sqlx::postgres::PgPool;
use uuid::Uuid;

use crate::models::author::{Author, NewAuthor};

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

    pub async fn get_author_by_id(&self, id: Uuid) -> Result<Author, sqlx::Error>
    {
        sqlx::query_as("
            Select * from authors where id = $1;
        ")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    pub async fn get_author_by_name(&self, name: String) -> Result<Author, sqlx::Error>
    {
        sqlx::query_as("
            Select * from authors where name = $1;
        ")
            .bind(name)
            .fetch_one(&self.pool)
            .await
    }

    pub async fn new_author(&self, new_author: NewAuthor) -> Result<Author, sqlx::Error>
    {
        let uuid = uuid::Uuid::new_v4();
        let now = chrono::Utc::now().date_naive();

        let result = sqlx::query("
            Insert into authors values (
                $1,
                $2,
                $3,
                $4,
                $5,
                $6,
                $7,
                $8
            );
        ")
            .bind(uuid)
            .bind(&new_author.japanese_name)
            .bind(&new_author.name)
            .bind(&new_author.birthday)
            .bind(&new_author.website)
            .bind(&new_author.description)
            .bind(now)
            .bind(now)
            .execute(&self.pool)
            .await;

        match result
        {
            Ok(_) => Ok(Author::from_new_author(new_author, uuid, now)),
            Err(err) => Err(err)
        }
    }
}
