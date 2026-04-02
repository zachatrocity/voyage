use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct AppConfig {
    pub server_url: String,
    pub api_key: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server_url: "http://localhost:8181".into(),
            api_key: String::new(),
        }
    }
}

pub static APP_CONFIG: GlobalSignal<AppConfig> = Signal::global(|| AppConfig::default());
