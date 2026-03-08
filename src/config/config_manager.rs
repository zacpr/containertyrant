use crate::config::AppConfig;
use std::fs;
use std::path::PathBuf;

pub struct ConfigManager {
    config_path: PathBuf,
}

impl ConfigManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("containertyrant");
        
        fs::create_dir_all(&config_dir)?;
        let config_path = config_dir.join("config.toml");
        
        Ok(Self { config_path })
    }
    
    pub fn load(&self) -> Result<AppConfig, Box<dyn std::error::Error>> {
        if !self.config_path.exists() {
            let default_config = AppConfig::default();
            self.save(&default_config)?;
            return Ok(default_config);
        }
        
        let content = fs::read_to_string(&self.config_path)?;
        let config: AppConfig = toml::from_str(&content)?;
        Ok(config)
    }
    
    pub fn save(&self, config: &AppConfig) -> Result<(), Box<dyn std::error::Error>> {
        let content = toml::to_string_pretty(config)?;
        fs::write(&self.config_path, content)?;
        Ok(())
    }
}
