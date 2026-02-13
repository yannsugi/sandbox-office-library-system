use actix_web::{HttpResponse, ResponseError};
use presenter_book::ErrorJdto;
use thiserror::Error;

pub use controller_utils::AppContext;

#[derive(Debug, Error)]
pub enum UserControllerError {
    #[error("Validation error: {0}")]
    ValidationError(#[from] anyhow::Error),
}

impl ResponseError for UserControllerError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UserControllerError::ValidationError(e) => {
                HttpResponse::BadRequest().json(ErrorJdto {
                    code: "VALIDATION_ERROR".to_string(),
                    message: e.to_string(),
                })
            }
        }
    }
}

pub mod create;
pub use create::create;
