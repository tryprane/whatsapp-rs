use std::path::PathBuf;
use std::env;

#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub headless: bool,
    pub user_data_dir: Option<PathBuf>,
    pub window_size: (u32, u32),
    pub disable_gpu: bool,
    pub browser_args: Vec<String>,
    pub user_agent: String,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            headless: true,
            user_data_dir: Some(get_default_user_data_dir()),
            window_size: (1920, 1080),
            disable_gpu: true,
            browser_args: get_default_browser_args(),
            user_agent: get_default_user_agent(),
        }
    }
}

impl ClientConfig {
    pub fn builder() -> ClientConfigBuilder {
        ClientConfigBuilder::default()
    }
}

#[derive(Default)]
pub struct ClientConfigBuilder {
    config: ClientConfig,
}

impl ClientConfigBuilder {
    pub fn headless(mut self, headless: bool) -> Self {
        self.config.headless = headless;
        self
    }

    pub fn user_data_dir<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.config.user_data_dir = Some(path.into());
        self
    }

    pub fn window_size(mut self, width: u32, height: u32) -> Self {
        self.config.window_size = (width, height);
        self
    }

    pub fn disable_gpu(mut self, disable: bool) -> Self {
        self.config.disable_gpu = disable;
        self
    }

    pub fn add_browser_arg<S: Into<String>>(mut self, arg: S) -> Self {
        self.config.browser_args.push(arg.into());
        self
    }

    pub fn user_agent<S: Into<String>>(mut self, agent: S) -> Self {
        self.config.user_agent = agent.into();
        self
    }

    pub fn build(self) -> ClientConfig {
        self.config
    }
}

fn get_default_user_data_dir() -> PathBuf {
    // Try to use the current directory
    if let Ok(current_dir) = env::current_dir() {
        return current_dir.join("whatsapp-rs");
    }
    
    // Fallback to the current directory
    PathBuf::from("WhatsApp")
}

fn get_default_browser_args() -> Vec<String> {
    vec![
        "--no-sandbox".to_string(),
        "--disable-setuid-sandbox".to_string(),
        "--disable-infobars".to_string(),
        "--window-position=0,0".to_string(),
        "--ignore-certificate-errors".to_string(),
        "--ignore-certificate-errors-spki-list".to_string(),
        "--disable-blink-features=AutomationControlled".to_string(),
        "--disable-extensions".to_string(),
        "--disable-gpu".to_string(),
        "--hide-scrollbars".to_string(),
        "--mute-audio".to_string(),
        "--disable-background-networking".to_string(),
    ]
}

fn get_default_user_agent() -> String {
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string()
}