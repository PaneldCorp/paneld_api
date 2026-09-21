use uuid::Uuid;

use crate::{
    models::author::{NewAuthor, Author},
    repositories::author_repository::AuthorRepo,
    config::error::Error
};

pub struct AuthorServices
{
    repo: AuthorRepo
}

impl AuthorServices
{
    pub fn new(repo: AuthorRepo) -> Self
    {
        Self { repo }
    }

    pub async fn get_all_authors(&self) -> Result<Vec<Author>, Error>
    {
        let authors = self.repo.get_all_authors().await;

        match authors
        {
            Ok(authors) => Ok(authors),
            Err(err) => Err(Error::from_sqlx_error(err))
        }
    }

    pub async fn get_author_by_id(&self, id: Uuid) -> Result<Vec<Author>, Error>
    {
        let author = self.repo.get_author_by_id(id).await;

        match author
        {
            Ok(author) => Ok(author),
            Err(err) => Err(Error::from_sqlx_error(err))
        }
    }

    pub async fn get_author_by_name(&self, name: String) -> Result<Vec<Author>, Error>
    {
        let author = self.repo.get_author_by_name(name).await;

        match author
        {
            Ok(author) => Ok(author),
            Err(err) => Err(Error::from_sqlx_error(err))
        }
    }

    pub async fn new_author(&self, new_author: NewAuthor) -> Result<Author, Error>
    {
        let new_author = self.repo.new_author(new_author).await;

        match new_author 
        {
            Ok(author) => Ok(author),
            Err(err) => Err(Error::from_sqlx_error(err))
        }
    }
}
