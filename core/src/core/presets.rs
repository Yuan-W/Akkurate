//! Style presets for text enhancement

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// A style preset for text enhancement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StylePreset {
    pub name: String,
    pub tone: String,
    pub formality: String,
    pub instructions: String,
}

/// Manages style presets
pub struct PresetManager {
    presets: HashMap<String, StylePreset>,
}

impl PresetManager {
    /// Create a new preset manager with default presets
    pub fn new() -> Self {
        let mut presets = HashMap::new();

        presets.insert(
            "casual".to_string(),
            StylePreset {
                name: "Casual".to_string(),
                tone: "friendly, conversational".to_string(),
                formality: "informal".to_string(),
                instructions:
                    "Use simple words, contractions are okay, keep it natural and relaxed"
                        .to_string(),
            },
        );

        presets.insert(
            "business".to_string(),
            StylePreset {
                name: "Business".to_string(),
                tone: "professional, polite".to_string(),
                formality: "formal".to_string(),
                instructions: "Clear and concise, avoid slang, maintain professional courtesy"
                    .to_string(),
            },
        );

        presets.insert(
            "academic".to_string(),
            StylePreset {
                name: "Academic".to_string(),
                tone: "objective, analytical".to_string(),
                formality: "highly formal".to_string(),
                instructions:
                    "Use precise terminology, passive voice acceptable, maintain scholarly tone"
                        .to_string(),
            },
        );

        presets.insert(
            "creative".to_string(),
            StylePreset {
                name: "Creative".to_string(),
                tone: "expressive, vivid".to_string(),
                formality: "flexible".to_string(),
                instructions: "Encourage creativity, use varied sentence structures, be engaging"
                    .to_string(),
            },
        );

        Self { presets }
    }

    /// Load custom presets from a TOML file
    pub fn load_custom_presets(&mut self, path: &Path) -> Result<()> {
        if !path.exists() {
            return Ok(());
        }

        let content = std::fs::read_to_string(path).context("Failed to read presets file")?;

        #[derive(Deserialize)]
        struct PresetsFile {
            presets: HashMap<String, StylePreset>,
        }

        let file: PresetsFile = toml::from_str(&content).context("Failed to parse presets file")?;

        self.presets.extend(file.presets);
        Ok(())
    }

    /// Get a preset by key
    pub fn get(&self, key: &str) -> Option<&StylePreset> {
        self.presets.get(key)
    }

    /// Get all preset keys
    pub fn keys(&self) -> Vec<&String> {
        self.presets.keys().collect()
    }

    /// Get all presets as a vector of (key, preset) pairs
    pub fn all(&self) -> Vec<(&String, &StylePreset)> {
        self.presets.iter().collect()
    }
}

impl Default for PresetManager {
    fn default() -> Self {
        Self::new()
    }
}
