use actix_web::{error, http::{header::ContentType, StatusCode}, HttpResponse};

use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum LyError {
    NotFound,
    InternalServerError,
}

impl error::ResponseError for LyError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<std::io::Error> for LyError {
    fn from(err: std::io::Error) -> Self {
        match err.kind() {
            std::io::ErrorKind::NotFound => Self::NotFound,
            _ => Self::InternalServerError,
        }
    }
}
