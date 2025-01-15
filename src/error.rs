use thiserror::Error;

#[derive(Error, Debug)]
pub enum WhatsAppError {
    #[error("Browser error: {0}")]
    BrowserError(String),

    #[error("Navigation error: {0}")]
    NavigationError(String),

    #[error("Element not found: {0}")]
    ElementNotFound(String),

    #[error("Not logged in")]
    NotLoggedIn,

    #[error("Invalid phone number: {0}")]
    InvalidPhoneNumber(#[from] std::num::ParseIntError),

    #[error("JavaScript execution error: {0}")]
    JavaScriptError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Other error: {0}")]
    Other(String),
}

impl From<Box<dyn std::error::Error>> for WhatsAppError {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        WhatsAppError::Other(error.to_string())
    }
}