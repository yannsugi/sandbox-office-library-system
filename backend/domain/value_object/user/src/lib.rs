use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UserId(i32);

impl UserId {
    pub fn new(value: i32) -> Result<Self, UserIdError> {
        if value <= 0 {
            return Err(UserIdError::InvalidId);
        }
        Ok(Self(value))
    }

    pub fn value(&self) -> i32 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserName(String);

impl UserName {
    pub fn new(value: String) -> Result<Self, UserNameError> {
        if value.trim().is_empty() {
            return Err(UserNameError::Empty);
        }
        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Error)]
pub enum UserIdError {
    #[error("User ID must be positive")]
    InvalidId,
}

#[derive(Debug, Error)]
pub enum UserNameError {
    #[error("User name cannot be empty")]
    Empty,
}
