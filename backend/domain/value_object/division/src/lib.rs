use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DivisionId(i32);

impl DivisionId {
    pub fn new(value: i32) -> Result<Self, DivisionIdError> {
        if value <= 0 {
            return Err(DivisionIdError::InvalidId);
        }
        Ok(Self(value))
    }

    pub fn value(&self) -> i32 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DivisionName(String);

impl DivisionName {
    pub fn new(value: String) -> Result<Self, DivisionNameError> {
        if value.trim().is_empty() {
            return Err(DivisionNameError::Empty);
        }
        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Error)]
pub enum DivisionIdError {
    #[error("Division ID must be positive")]
    InvalidId,
}

#[derive(Debug, Error)]
pub enum DivisionNameError {
    #[error("Division name cannot be empty")]
    Empty,
}
