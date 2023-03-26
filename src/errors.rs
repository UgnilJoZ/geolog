use actix_web::error::ResponseError;
use actix_web::http::StatusCode;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Database error {0:?}")]
    DbError(#[from] sqlx::Error),
    #[error("Actix error {0:?}")]
    ActixError(#[from] actix_web::Error),
    #[error("User was not unauthenticated")]
    Unauthenticated,
    #[error("User {user} is not allowed to access the resource {resource}")]
    Forbidden {
        user: String,
        resource: PathBuf,
    },
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::Unauthenticated => StatusCode::UNAUTHORIZED,
            Error::Forbidden{..} => StatusCode::FORBIDDEN,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}