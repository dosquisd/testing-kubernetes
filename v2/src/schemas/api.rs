use serde::Serialize;
use actix_web::http::StatusCode;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub message: String,
    pub status_code: u16,
}

impl ErrorResponse {
    pub fn get_status_code(&self) -> StatusCode {
        match self.status_code {
            400 => StatusCode::BAD_REQUEST,
            404 => StatusCode::NOT_FOUND,
            500 => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::INTERNAL_SERVER_ERROR, // Default to 500 for any other status code
        }
    }
}
