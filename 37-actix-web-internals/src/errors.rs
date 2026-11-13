//! Ошибки домена → HTTP-ответы через ResponseError.
use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("заголовок X-Client-Id обязателен")]
    MissingClientId,
    #[error("некорректный X-Client-Id: {0}")]
    BadClientId(String),
    #[error("тело слишком большое: {0} байт, лимит {1}")]
    TooLarge(usize, usize),
    #[error("внутренняя ошибка")]
    Internal,
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::MissingClientId | ApiError::BadClientId(_) => StatusCode::BAD_REQUEST,
            ApiError::TooLarge(..) => StatusCode::PAYLOAD_TOO_LARGE,
            ApiError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(json!({
            "error": self.to_string(),
            "code": self.status_code().as_u16(),
        }))
    }
}
