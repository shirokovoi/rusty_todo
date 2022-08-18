use actix_web::error::ResponseError;
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
}

impl ResponseError for Error {
    fn status_code(&self) -> actix_web::http::StatusCode {
        unimplemented!();
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        unimplemented!();
    }
}
