//! Grammar checking logic

use crate::api::GeminiClient;

/// Grammar checker using Gemini API
pub struct GrammarChecker {
    client: GeminiClient,
}

impl GrammarChecker {
    /// Create a new grammar checker
    pub fn new(client: GeminiClient) -> Self {
        Self { client }
    }

    /// Check grammar for the given text
    pub async fn check_grammar(&self, text: &str, lang: &str) -> anyhow::Result<crate::api::CheckResult> {
        self.client.check_grammar(text, lang).await
    }
}
