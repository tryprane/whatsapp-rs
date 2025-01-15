use whatsapp_rs::{WhatsAppClient, ClientConfig};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Create a new WhatsApp client with default configuration


    

    let client = WhatsAppClient::new(ClientConfig::default())?;
    println!("WhatsApp client initialized");

    // Check if already logged in
    if !client.is_logged_in()? {
        println!("Not logged in. Starting login process...");
        client.login()?;
        println!("Login successful!");
    } else {
        println!("Already logged in!");
    }

    // Example: Send a message
    let phone_number = "916556565656"; // Replace with actual phone number
    let message = "Hello from WhatsApp-rs!";
    println!("Sending message to {}", phone_number);
    client.send_message(phone_number, message)?;

    // Example: Get chat messages
    

    Ok(())
}