use actix_web::{error::ResponseError, HttpResponse};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to perform IO operation")]
    Io {
        #[from]
        source: std::io::Error,
    },
    #[error("An error occured while parse toml")]
    TomlParseError {
        #[from]
        source: toml::de::Error,
    },
    #[error("Got error while access db")]
    SqlxError {
        #[from]
        source: sqlx::Error,
    },
    #[error("Wrong logger configuration")]
    LoggerError {
        #[from]
        source: flexi_logger::FlexiLoggerError,
    },
    #[error("Failed to apply migrations")]
    MigrateError {
        #[from]
        source: sqlx::migrate::MigrateError,
    },
    #[error("Failed while work with bcrypt hash")]
    BcryptError {
        #[from]
        source: bcrypt::BcryptError,
    },
    #[error("User with given username already exists")]
    UsernameAlreadyExists,
}

impl ResponseError for Error {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match &self {
            Error::UsernameAlreadyExists => actix_web::http::StatusCode::BAD_REQUEST,
            _ => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        unimplemented!();
    }
}
