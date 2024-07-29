use std::{
    error::Error,
    fmt::{self},
};

use axum::{http::StatusCode, Json};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct BruteError {
    pub status_code: u16,
    pub message: String,
}

#[derive(Serialize)]
pub struct ErrorMessage {
    message: String,
}

impl BruteError {
    #[allow(dead_code)]
    pub fn to_response(&self) -> (StatusCode, Json<Self>) {
        let status_code = match self {
            BruteError { status_code, .. } => {
                StatusCode::from_u16(*status_code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
            }
        };
        (status_code, Json(self.clone()))
    }

    pub fn to_response_with_message_only(&self) -> (StatusCode, Json<ErrorMessage>) {
        let status_code = match self {
            BruteError { status_code, .. } => {
                StatusCode::from_u16(*status_code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
            }
        };

        let message = match self {
            BruteError { message, .. } => message.clone(),
        };

        (status_code, Json(ErrorMessage { message }))
    }
}

impl fmt::Display for BruteError {
    /// Formats the error message for display.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Status Code: {}, Message: {}",
            self.status_code, self.message
        )
    }
}

impl Error for BruteError {
    /// Returns the underlying cause of the error, if any.
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    /// Deprecated method to get a description of the error.
    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    /// Deprecated method to get the cause of the error.
    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}
