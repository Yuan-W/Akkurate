//! Text enhancement logic

use crate::api::GeminiClient;

/// Text enhancer using Gemini API
pub struct TextEnhancer {
    client: GeminiClient,
}

impl TextEnhancer {
    /// Create a new text enhancer
    pub fn new(client: GeminiClient) -> Self {
        Self { client }
    }

    /// Enhance text with the given style preset
    pub async fn enhance_text(
        &self,
        text: &str,
        preset: &crate::core::StylePreset,
        lang: &str,
    ) -> anyhow::Result<crate::api::EnhanceResult> {
        self.client.enhance_text(text, preset, lang).await
    }
}
