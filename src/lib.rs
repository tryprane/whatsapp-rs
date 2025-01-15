//! WhatsApp-rs is a library for automating WhatsApp Web interactions.
//! 
//! This library provides a safe and convenient interface for programmatically
//! interacting with WhatsApp Web using headless Chrome.

mod auth;
mod browser;
mod chat;
mod error;
mod types;

pub use auth::login;
pub use browser::launch_whatsapp;
pub use chat::{get_chats, send_message};
pub use error::WhatsAppError;
pub use types::*;

use headless_chrome::Browser;
use std::sync::Arc;

/// The main client for interacting with WhatsApp Web
pub struct WhatsAppClient {
    browser: Browser,
    tab: Arc<headless_chrome::Tab>,
}

impl WhatsAppClient {
    /// Creates a new WhatsApp client with the given configuration
    pub fn new(config: ClientConfig) -> Result<Self, WhatsAppError> {
        let (browser, tab) = launch_whatsapp(&config)?;
        Ok(Self { browser, tab })
    }

    /// Checks if the client is logged in
    pub fn is_logged_in(&self) -> Result<bool, WhatsAppError> {
        auth::check_login(&self.tab).map_err(WhatsAppError::from)
    }

    /// Performs login using QR code
    pub fn login(&self) -> Result<(), WhatsAppError> {
        auth::login(&self.tab).map_err(WhatsAppError::from)
    }

    /// Sends a message to the specified phone number
    pub fn send_message(&self, number: &str, message: &str) -> Result<(), WhatsAppError> {
        chat::send_message(&self.tab, number.parse()?, message).map_err(WhatsAppError::from)
    }

    /// Gets chat messages for the specified phone number
    pub fn get_chat_messages(&self, number: &str) -> Result<Vec<(String, String)>, WhatsAppError> {
        chat::get_chats(&self.tab, number.parse()?).map_err(WhatsAppError::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let config = ClientConfig::default();
        let client = WhatsAppClient::new(config);
        assert!(client.is_ok());
    }
}