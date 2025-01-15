# WhatsApp-rs

A Rust library for automating WhatsApp Web interactions. This library provides a clean and safe interface for programmatically interacting with WhatsApp Web using headless Chrome.

## Features

- 🚀 Easy WhatsApp Web automation
- 🔒 Session management and QR code login
- 💬 Send and receive messages
- 🔍 Search and interact with chats
- 📱 Support for both personal and business accounts

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
whatsapp-rs = "0.1.0"
```

## Quick Start

```rust
use whatsapp_rs::{WhatsAppClient, ClientConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new WhatsApp client
    let client = WhatsAppClient::new(ClientConfig::default())?;

    // Login (this will save QR code to QR.jpg)
    client.login()?;

    // Send a message
    client.send_message("+1234567890", "Hello from WhatsApp-rs!")?;

    Ok(())
}
```

## Usage Examples

### Initialize Client and Login

```rust
use whatsapp_rs::{WhatsAppClient, ClientConfig};

let config = ClientConfig::builder()
    .headless(false)
    .user_data_dir("/path/to/profile")
    .build()?;

let client = WhatsAppClient::new(config)?;
client.login()?;
```

### Send Messages

```rust
// Send to a single number
client.send_message("1234567890", "Hello!")?;

// Send to multiple numbers
let numbers = vec!["1234567890", "0987654321"];
for number in numbers {
    client.send_message(number, "Bulk message")?;
}
```

### Read Messages

```rust
// Get recent chat messages
let messages = client.get_recent_messages()?;
for (time, message) in messages {
    println!("Time: {}, Message: {}", time, message);
}
```

## Configuration

The library can be configured using the `ClientConfig` builder:

```rust
let config = ClientConfig::builder()
    .headless(false)                          // Run with visible browser
    .user_data_dir("/path/to/profile")        // Custom profile directory
    .window_size(1920, 1080)                  // Custom window size
    .disable_gpu(true)                        // Disable GPU acceleration
    .build()?;
```

## Error Handling

The library uses custom error types for better error handling:

```rust
use whatsapp_rs::error::WhatsAppError;

match client.send_message("1234567890", "Hello") {
    Ok(_) => println!("Message sent successfully!"),
    Err(WhatsAppError::NotLoggedIn) => println!("Please log in first"),
    Err(WhatsAppError::ConnectionError(e)) => println!("Connection error: {}", e),
    Err(e) => println!("Other error: {}", e),
}
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
