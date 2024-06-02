use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

pub struct AppError {
    code: StatusCode,
    message: String,
}

// AppError is different during `DEBUG` compilation and `RELEASE`
// This was made for better debug use and a securer release

impl AppError {
    
    #[cfg(debug_assertions)]
    pub fn new(code: StatusCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into()
        }
    }

    #[cfg(not(debug_assertions))]
    pub fn new(code: StatusCode, message: impl Into<String>) -> Self {
        Self {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Internal server error".into()
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (
            self.code,
            Json(ResponseMessage { message: self.message })
        ).into_response()
    }
}

#[derive(Serialize)]
struct ResponseMessage {
    message: String
}