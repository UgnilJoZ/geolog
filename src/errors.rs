use actix_web::error::ResponseError;
use actix_web::http::StatusCode;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Database error {0:?}")]
    Database(#[from] sqlx::Error),
    #[error("Web error {0:?}")]
    Web(#[from] actix_web::Error),
    #[error("User was not unauthenticated")]
    Unauthenticated,
    #[error("User {user} is not allowed to access the resource {resource}")]
    Forbidden { user: String, resource: PathBuf },
    #[error("Error decoding base64 string: {0}")]
    Base64DecodingError(base64::DecodeError),
    #[error("Database pool not found")]
    DatabaseNotPresent,
    #[error("Authorization header had unexpected format. Expected: `Token …`")]
    AuthHeaderMalformed,
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        use Error::*;
        match self {
            Unauthenticated => StatusCode::UNAUTHORIZED,
            Forbidden { .. } => StatusCode::FORBIDDEN,
            AuthHeaderMalformed | Base64DecodingError(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
