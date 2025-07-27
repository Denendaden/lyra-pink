use lyssg::{error::*, ssg::*};

use std::{error::Error, fmt};

use actix_web::{error, http::{header::ContentType, StatusCode}, HttpResponse};

// u16 contains HTTP response status code
#[derive(Debug)]
pub struct HttpError(u16);

impl Error for HttpError {}

impl From<LyError> for HttpError {
    fn from(e: LyError) -> Self {
        HttpError(e.http_code())
    }
}

impl error::ResponseError for HttpError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(
                match LyWebpage::from_file("templates/error.html") {
                    Ok(lw) => lw
                        .fill_with_str("error", &self.to_string())
                        .contents,
                    Err(_) => self.to_string(),
                }
            )
    }

    fn status_code(&self) -> StatusCode {
        StatusCode::from_u16(self.0).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -- {}", self.0, match self.0 {
            404 => "file not found",
            500 => "internal server error",
            _ => "unknown error"
        })
    }
}
