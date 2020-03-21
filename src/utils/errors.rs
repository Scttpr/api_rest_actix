use actix_web::{error, HttpResponse, dev, http};
use failure::Fail;
use log::error;

#[derive(Fail, Debug)]
pub enum HttpError {
    #[fail(display = "not found")]
    NotFound,
    #[fail(display = "bad data")]
    BadClientData,
    #[fail(display = "internal error")]
    InternalError,
    #[fail(display = "timeout")]
    Timeout,
    #[fail(display = "forbidden")]
    Forbidden,
}

impl error::ResponseError for HttpError {
    fn error_response(&self) -> HttpResponse {
        error!("error");
        dev::HttpResponseBuilder::new(self.status_code())
            .set_header(http::header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(self.to_string())
    }

    fn status_code(&self) -> http::StatusCode {
        match *self {
            HttpError::BadClientData => http::StatusCode::BAD_REQUEST,
            HttpError::NotFound => http::StatusCode::NOT_FOUND,
            HttpError::InternalError => http::StatusCode::INTERNAL_SERVER_ERROR,
            HttpError::Timeout => http::StatusCode::GATEWAY_TIMEOUT,
            HttpError::Forbidden => http::StatusCode::FORBIDDEN,
        }
    }
}

pub mod error_messages {
    pub fn not_found (element: &str) -> String {
        let message = String::from(element) + " not found";
        message
    }
}
