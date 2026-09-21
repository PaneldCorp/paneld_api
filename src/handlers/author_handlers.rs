use actix_web::{
    web::{
        Data,
        Path,
        Json
    },

    Responder 
};

use crate::{
    utils::log::{log, Level},
    models::author::{NewAuthor, Author},
    config::error::Error,
    AppState
};

pub async fn get_all_authors(data: Data<AppState>) -> impl Responder
{
    let request = data
        .services
        .authors
        .get_all_authors()
        .await;

    match request 
    {
        Ok(authors) => Error::OK.error_response::<Vec<Author>>(Some(authors)),
        Err(err) => {
            log(Level::Error, format!("All Authors request: {err}",).as_str());
            err.error_response::<String>(None)
        }
    }
}

pub async fn get_author_by_id(data: Data<AppState>, id: Path<Uuid>) -> impl Responder
{
    let request = data
        .services
        .authors
        .get_author_by_id(id)
        .await;

    match request 
    {
        Ok(author) => Error::OK.error_response::<Author>(Some(author)),
        Err(err) => {
            log(Level::Error, format!("Find Author by id request: {err}",).as_str());
            err.error_response::<String>(None)
        }
    }
}

pub async fn get_author_by_name(data: Data<AppState>, name: Path<String>) -> impl Responder
{
    let request = data
        .services
        .authors
        .get_author_by_name(name)
        .await;

    match request 
    {
        Ok(author) => Error::OK.error_response::<Author>(Some(author)),
        Err(err) => {
            log(Level::Error, format!("Find Author by id request: {err}",).as_str());
            err.error_response::<String>(None)
        }
    }
}

pub async fn new_author(data: Data<AppState>, new_author: Json<NewAuthor>) -> impl Responder
{
    let new_author = new_author.into_inner();

    let request = data
        .services
        .authors
        .new_author(new_author)
        .await;

    match request
    {
        Ok(author) => Error::OK.error_response::<Author>(Some(author)),
        Err(err) => {
            log(Level::Error, format!("New Author request: {err}",).as_str());
            err.error_response::<String>(None)
        }
    }
}
