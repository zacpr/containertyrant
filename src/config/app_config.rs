use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub theme: String,
    pub refresh_interval_ms: u64,
    pub show_system_containers: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            theme: "dark".to_string(),
            refresh_interval_ms: 2000,
            show_system_containers: false,
        }
    }
}
