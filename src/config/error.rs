use super::error_kind::*;

use serde::{Serialize, Deserialize};
use actix_web::{
    http::StatusCode,
    HttpResponse,
};

#[derive(thiserror::Error, Debug)]
#[error("...")]
pub enum Error
{
    #[error("OK !")]
    OK,

    #[error("{0}")]
    Authenticate(#[from] AuthenticateError),

    #[error("{0}")]
    BadRequest(#[from] BadRequest),

    #[error("{0}")]
    NotFound(#[from] NotFound),

    #[error("{0}")]
    Database(#[from] DatabaseError),
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse<T: Serialize>
{
    code: u16,
    message: String,
    data: Option<T>
}

impl Error
{
    fn get_codes(&self) -> (StatusCode, u16)
    {
        match *self
        {
            // 2xx errors
            Error::OK => (StatusCode::OK, 200),

            // 4xx errors
            Error::BadRequest(_) => (StatusCode::BAD_REQUEST, 403),
            Error::NotFound(_) => (StatusCode::NOT_FOUND, 404),
            Error::Authenticate(AuthenticateError::WrongCredentials) => (StatusCode::UNAUTHORIZED, 401),
            Error::Authenticate(AuthenticateError::InvalidToken) => (StatusCode::UNAUTHORIZED, 401),

            // 5xx errors
            Error::Authenticate(AuthenticateError::TokenCreation) => (StatusCode::INTERNAL_SERVER_ERROR, 500),

            _ => (StatusCode::INTERNAL_SERVER_ERROR, 500)
        }
    }

    pub fn bad_request(message: String) -> Self
    {
        Error::BadRequest(BadRequest { message })
    }

    pub fn not_found(element: String) -> Self
    {
        Error::NotFound(NotFound { element })
    }

    pub fn error_response<T>(&self, data: Option<T>) -> HttpResponse
        where T: Serialize
    {
        let (status_code, code) = self.get_codes();
        let message = self.to_string();
        let error_response = ErrorResponse { code, message, data };
        HttpResponse::build(status_code).json(error_response)
    }

    pub fn from_sqlx_error(err: sqlx::Error) -> Self
    {
        match err
        {

            sqlx::Error::RowNotFound => Error::Database(DatabaseError::RowNotFound(err.to_string())),
            _ => Error::Database(DatabaseError::Unvalid(err.to_string()))
        }
    }
}
