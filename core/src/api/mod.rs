//! API module for external service integrations

pub mod gemini;

pub use gemini::{CheckResult, EnhanceResult, GeminiClient};
