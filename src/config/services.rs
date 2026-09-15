use sqlx::postgres::PgPool;

use crate::{
    repositories::author_repository::AuthorRepo,
    services::author_services::AuthorServices
};

pub struct Services
{
    pub authors: AuthorServices
}

impl Services
{
    pub fn new(pool: PgPool) -> Self
    {
        let author_repo = AuthorRepo::new(pool);
        let authors = AuthorServices::new(author_repo);

        Self { authors }
    }
}
