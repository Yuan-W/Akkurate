//! Gemini API client for grammar checking and text enhancement

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

const GEMINI_API_URL: &str = "https://generativelanguage.googleapis.com/v1beta/models/gemini-3-flash-preview:generateContent";

/// Gemini API client
#[derive(Clone)]
pub struct GeminiClient {
    api_key: String,
    client: reqwest::Client,
}

/// A grammar issue found in the text
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrammarIssue {
    pub original: String,
    pub corrected: String,
    pub explanation: String,
    pub rule: String,
}

/// Result of grammar checking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckResult {
    pub issues: Vec<GrammarIssue>,
    pub corrected_text: String,
    #[serde(default)]
    pub summary: Option<String>,
}

/// Result of text enhancement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhanceResult {
    pub enhanced_text: String,
    pub changes_made: Vec<String>,
}

// Gemini API request/response structures
#[derive(Serialize)]
struct GenerateContentRequest {
    contents: Vec<Content>,
    generation_config: GenerationConfig,
}

#[derive(Serialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Serialize)]
struct Part {
    text: String,
}

#[derive(Serialize)]
struct GenerationConfig {
    temperature: f32,
    #[serde(rename = "responseMimeType")]
    response_mime_type: String,
}

#[derive(Deserialize)]
struct GeminiResponse {
    candidates: Vec<Candidate>,
}

#[derive(Deserialize)]
struct Candidate {
    content: CandidateContent,
}

#[derive(Deserialize)]
struct CandidateContent {
    parts: Vec<CandidatePart>,
}

#[derive(Deserialize)]
struct CandidatePart {
    text: String,
}

impl GeminiClient {
    /// Create a new Gemini client with the given API key
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: reqwest::Client::new(),
        }
    }

    /// Check grammar and return results
    pub async fn check_grammar(&self, text: &str, lang: &str) -> Result<CheckResult> {
        let prompt = format!(
            r#"Please act as a professional grammar checker. Check the following text for grammar, spelling, and punctuation errors.
The user's interface language is {}. Assessment and explanations MUST BE in {}.

For each issue found:
1.  Identify the original text.
2.  Provide the corrected text.
3.  Explain why it is an error (concise explanation in {}).
4.  Cite the grammar rule involved (in {}).

Return the result in strict JSON format matching this structure:
{{
  "issues": [
    {{
      "original": "substring with error",
      "corrected": "corrected substring",
      "explanation": "explanation in {}",
      "rule": "grammar rule in {}"
    }}
  ],
  "corrected_text": "the full text with all corrections applied"
}}

If there are no errors, return an empty "issues" list.

Text to check:
{}"#,
            lang, lang, lang, lang, lang, lang, text
        );

        let request = GenerateContentRequest {
            contents: vec![Content {
                parts: vec![Part { text: prompt }],
            }],
            generation_config: GenerationConfig {
                temperature: 0.2,
                response_mime_type: "application/json".to_string(),
            },
        };

        let response = self.send_request(request).await?;
        let text_response = response
            .candidates
            .first()
            .context("No candidates returned")?
            .content
            .parts
            .first()
            .context("No parts returned")?
            .text
            .clone();

        let json_str = self.extract_json(&text_response);
        let result: CheckResult =
            serde_json::from_str(&json_str).context("Failed to parse JSON")?;

        Ok(result)
    }

    /// Enhance text based on a preset style
    pub async fn enhance_text(
        &self,
        text: &str,
        preset: &crate::core::StylePreset,
        lang: &str,
    ) -> Result<EnhanceResult> {
        let prompt = format!(
            r#"Please act as a professional writing editor. Enhance the following text to match the style: "{}".
Description of style: {}.
The user's interface language is {}. Explanations MUST BE in {}.

Analyze the text and rewrite it to better fit the requested style.
List the specific changes you made and explain why (in {}).

Return the result in strict JSON format matching this structure:
{{
  "enhanced_text": "the rewritten text",
  "changes_made": [
    "Change 1: explanation in {}",
    "Change 2: explanation in {}"
  ]
}}

Text to enhance:
{}"#,
            preset.name, preset.instructions, lang, lang, lang, lang, lang, text
        );

        let request = GenerateContentRequest {
            contents: vec![Content {
                parts: vec![Part { text: prompt }],
            }],
            generation_config: GenerationConfig {
                temperature: 0.2,
                response_mime_type: "application/json".to_string(),
            },
        };

        let response = self.send_request(request).await?;
        let text_response = response
            .candidates
            .first()
            .context("No candidates returned")?
            .content
            .parts
            .first()
            .context("No parts returned")?
            .text
            .clone();

        let json_str = self.extract_json(&text_response);
        let result: EnhanceResult =
            serde_json::from_str(&json_str).context("Failed to parse JSON")?;

        Ok(result)
    }

    /// Send a request to the Gemini API and return the raw GeminiResponse
    async fn send_request(&self, request: GenerateContentRequest) -> Result<GeminiResponse> {
        let url = format!("{}?key={}", GEMINI_API_URL, self.api_key);

        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .context("Failed to send request to Gemini API")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("Gemini API error ({}): {}", status, error_text);
        }

        let gemini_response: GeminiResponse = response
            .json()
            .await
            .context("Failed to parse Gemini response")?;

        Ok(gemini_response)
    }

    /// Extract JSON from response text (handling potential markdown code blocks)
    fn extract_json(&self, text: &str) -> String {
        let text = text.trim();
        if text.starts_with("```json") {
            text.trim_start_matches("```json")
                .trim_end_matches("```")
                .trim()
                .to_string()
        } else if text.starts_with("```") {
            text.trim_start_matches("```")
                .trim_end_matches("```")
                .trim()
                .to_string()
        } else {
            text.to_string()
        }
    }
}
