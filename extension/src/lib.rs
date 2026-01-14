use akkurate_core::api::GeminiClient;
use akkurate_core::core::PresetManager;
use serde_json::json;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    log("Akkurate Verify Extension Loaded (WASM)");
}

#[wasm_bindgen]
pub async fn check_grammar_wasm(text: String, api_key: String) -> JsValue {
    // console_error_panic_hook::set_once();
    log(&format!("Checking grammar for text: {}", text));

    if api_key.is_empty() {
        return JsValue::from_str(
            &json!({
                "error": "API Key is missing"
            })
            .to_string(),
        );
    }

    let client = GeminiClient::new(api_key);
    // Assuming browser language is English or we pass it in. Defaulting to English for now.
    // TODO: Pass language from JS side.
    let lang = "English";

    match client.check_grammar(&text, lang).await {
        Ok(result) => match serde_json::to_string(&result) {
            Ok(json) => JsValue::from_str(&json),
            Err(e) => JsValue::from_str(
                &json!({"error": format!("Serialization error: {}", e)}).to_string(),
            ),
        },
        Err(e) => JsValue::from_str(
            &json!({
                "error": format!("API Error: {}", e)
            })
            .to_string(),
        ),
    }
}

#[wasm_bindgen]
pub async fn enhance_text_wasm(text: String, api_key: String, preset_key: String) -> JsValue {
    log(&format!(
        "Enhancing text: {} with preset {}",
        text, preset_key
    ));

    if api_key.is_empty() {
        return JsValue::from_str(
            &json!({
                "error": "API Key is missing"
            })
            .to_string(),
        );
    }

    let client = GeminiClient::new(api_key);
    let lang = "English";

    // We need presets. Since we can't load from file, we need a way to get them.
    // Core's PresetManager has hardcoded defaults if new() is called?
    // Let's check PresetManager implementation in core.
    let manager = PresetManager::new();
    // Defaults are loaded in new() usually?
    // Need to verify core/src/core/presets.rs

    let preset = if let Some(p) = manager.get(&preset_key) {
        p.clone()
    } else {
        return JsValue::from_str(
            &json!({
                "error": "Invalid preset"
            })
            .to_string(),
        );
    };

    match client.enhance_text(&text, &preset, lang).await {
        Ok(result) => match serde_json::to_string(&result) {
            Ok(json) => JsValue::from_str(&json),
            Err(e) => JsValue::from_str(
                &json!({"error": format!("Serialization error: {}", e)}).to_string(),
            ),
        },
        Err(e) => JsValue::from_str(
            &json!({
                "error": format!("API Error: {}", e)
            })
            .to_string(),
        ),
    }
}
