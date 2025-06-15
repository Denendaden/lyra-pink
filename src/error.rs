use crate::ssg::*;

use std::fmt;

use actix_web::{error, http::{header::ContentType, StatusCode}, HttpResponse, ResponseError};
use derive_more::{Error};

#[derive(Debug, Error)]
pub enum LyError {
    NotFound,
    InternalServerError,
}

impl error::ResponseError for LyError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(
                match LyWebpage::read_file("www/error.html") {
                    Ok(lw) => lw
                        .fill_template("error", &self.to_string())
                        .contents,
                    Err(_) => self.to_string(),
                }
            )
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl fmt::Display for LyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -- {}", self.status_code().as_str(), match self {
            LyError::NotFound => "file not found",
            LyError::InternalServerError => "internal server error",
        })
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
