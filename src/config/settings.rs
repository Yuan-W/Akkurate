//! Application settings and configuration

use anyhow::{Context, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub api: ApiConfig,
    pub preferences: Preferences,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    pub gemini_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preferences {
    pub default_preset: String,
    pub theme: String,
    pub language: String,
    pub auto_copy: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            api: ApiConfig {
                gemini_key: String::new(),
            },
            preferences: Preferences {
                default_preset: "casual".to_string(),
                theme: "dark".to_string(),
                language: "chinese".to_string(),
                auto_copy: true,
            },
        }
    }
}

/// Get the configuration directory path
pub fn config_dir() -> Option<PathBuf> {
    ProjectDirs::from("com", "akkurate", "akkurate").map(|dirs| dirs.config_dir().to_path_buf())
}

/// Get the configuration file path
pub fn config_path() -> Option<PathBuf> {
    config_dir().map(|dir| dir.join("config.toml"))
}

/// Load configuration from file
pub fn load_config() -> Result<AppConfig> {
    let path = config_path().context("Could not determine config path")?;

    if !path.exists() {
        return Ok(AppConfig::default());
    }

    let content = std::fs::read_to_string(&path).context("Failed to read config file")?;

    toml::from_str(&content).context("Failed to parse config file")
}

/// Save configuration to file
pub fn save_config(config: &AppConfig) -> Result<()> {
    let path = config_path().context("Could not determine config path")?;

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).context("Failed to create config directory")?;
    }

    let content = toml::to_string_pretty(config).context("Failed to serialize config")?;

    std::fs::write(&path, content).context("Failed to write config file")
}
