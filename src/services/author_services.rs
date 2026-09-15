use crate::{
    models::author::Author,
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
}
