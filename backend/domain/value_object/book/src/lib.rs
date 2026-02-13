use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BookId(i32);

impl BookId {
    pub fn new(value: i32) -> Result<Self, BookIdError> {
        if value <= 0 {
            return Err(BookIdError::InvalidId);
        }
        Ok(Self(value))
    }

    pub fn value(&self) -> i32 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BookTitle(String);

impl BookTitle {
    pub fn new(value: String) -> Result<Self, BookTitleError> {
        if value.trim().is_empty() {
            return Err(BookTitleError::Empty);
        }
        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Error)]
pub enum BookIdError {
    #[error("Book ID must be positive")]
    InvalidId,
}

#[derive(Debug, Error)]
pub enum BookTitleError {
    #[error("Book title cannot be empty")]
    Empty,
}
