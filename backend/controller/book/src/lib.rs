use actix_web::{HttpResponse, ResponseError};
use presenter_book::ErrorJdto;
use thiserror::Error;

pub use controller_utils::AppContext;

#[derive(Debug, Error)]
pub enum BookControllerError {
    #[error("Validation error: {0}")]
    ValidationError(#[from] anyhow::Error),
}

impl ResponseError for BookControllerError {
    fn error_response(&self) -> HttpResponse {
        match self {
            BookControllerError::ValidationError(e) => {
                HttpResponse::BadRequest().json(ErrorJdto {
                    code: "VALIDATION_ERROR".to_string(),
                    message: e.to_string(),
                })
            }
        }
    }
}

pub mod create;
pub mod find;
pub mod list;
pub mod borrow;
pub mod r#return;

pub use create::create;
pub use find::find;
pub use list::list;
pub use borrow::borrow;
pub use r#return::r#return;
