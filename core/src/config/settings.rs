//! Application settings and configuration
use serde::{Deserialize, Serialize};

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub api: ApiConfig,
    pub preferences: Preferences,
    // Add logic to skip serializing fields that shouldn't be saved if necessary,
    // but for now simple structs are fine.
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
