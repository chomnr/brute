use std::fmt;

use actix_web::{HttpResponse, ResponseError};

#[derive(Debug)]
pub enum BruteResponeError {
    ValidationError(String),
    InternalError(String),
    BadRequest(String),
}

impl fmt::Display for BruteResponeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for BruteResponeError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            BruteResponeError::ValidationError(_) => actix_web::http::StatusCode::BAD_REQUEST,
            BruteResponeError::InternalError(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            BruteResponeError::BadRequest(_) => actix_web::http::StatusCode::BAD_REQUEST,

        }
    }

    fn error_response(&self) -> HttpResponse {
        match self {
            BruteResponeError::ValidationError(msg) => HttpResponse::BadRequest().body(msg.clone()),
            BruteResponeError::InternalError(msg) => HttpResponse::InternalServerError().body(msg.clone()),
            BruteResponeError::BadRequest(msg) => HttpResponse::BadRequest().body(msg.clone()),
        }
    }
}
