use std::{error::Error, ffi::OsStr, path::PathBuf};
use headless_chrome::{Browser, LaunchOptionsBuilder, Tab};
use std::sync::Arc;
use crate::types::ClientConfig;

pub fn launch_whatsapp(config: &ClientConfig) -> Result<(Browser, Arc<Tab>), Box<dyn Error>> {
    // Convert string arguments to OsStr
    let chrome_args: Vec<&OsStr> = config.browser_args
        .iter()
        .map(|s| OsStr::new(s))
        .collect();

    // Build the launch options
    let mut builder = LaunchOptionsBuilder::default();
    let mut builder = builder
        .headless(config.headless)
        .window_size(Some(config.window_size))
        .args(chrome_args);

    // Add user data directory if specified
    if let Some(ref user_data_dir) = config.user_data_dir {
        builder = builder.user_data_dir(Some(user_data_dir.clone()));
    }

    let launch_options = builder
        .build()
        .expect("Failed to build launch options");

    println!("Launching browser with configuration:");
    println!("- Headless: {}", config.headless);
    println!("- Window size: {}x{}", config.window_size.0, config.window_size.1);
    println!("- User data dir: {:?}", config.user_data_dir);

    let browser = Browser::new(launch_options)?;
    let tab = browser.new_tab()?;

    // Navigate to WhatsApp Web
    tab.navigate_to("https://web.whatsapp.com")?;

    Ok((browser, tab))
}