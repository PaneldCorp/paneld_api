#[derive(thiserror::Error, Debug)]
#[error("...")]
pub enum DatabaseError {}

#[derive(thiserror::Error, Debug)]
#[error("...")]
pub enum AuthenticateError
{
    #[error("Wrong authentication credentials")]
    WrongCredentials,

    #[error("Failed to create authentication token")]
    TokenCreation,

    #[error("Invalid authentication credentials")]
    InvalidToken,
}

#[derive(thiserror::Error, Debug)]
#[error("Bad Request {message}")]
pub struct BadRequest {
    pub message : String
}

#[derive(thiserror::Error, Debug)]
#[error("{element} not found")]
pub struct NotFound {
    pub element: String
}
