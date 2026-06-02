// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Three compute tiers per `conventions/llm-substrate-decision.md`.
//!
//! The Doorman holds optional instances of each tier client. Missing
//! tiers are not errors at boot; per the Optional Intelligence principle
//! (`conventions/three-ring-architecture.md`), the system functions
//! without them and the router routes around them.

mod circuit_breaker;
mod external;
mod local;
mod yoyo;

pub use external::{
    ExternalAllowlist, ExternalTierClient, ExternalTierConfig, TierCPricing, TierCProvider,
    FOUNDRY_DEFAULT_ALLOWLIST,
};
pub use local::{LocalTierClient, LocalTierConfig};
pub use yoyo::{
    BearerTokenProvider, MetadataBearer, PricingConfig, StaticBearer, YoYoTierClient,
    YoYoTierConfig,
};

/// Convert Anthropic-format tools to OpenAI-format tools for llama-server.
///
/// Anthropic: `[{"name": "X", "description": "...", "input_schema": {...}}]`
/// OpenAI:    `[{"type": "function", "function": {"name": "X", "description": "...", "parameters": {...}}}]`
pub(crate) fn anthropic_tools_to_openai(tools: &serde_json::Value) -> serde_json::Value {
    let arr = match tools.as_array() {
        Some(a) => a,
        None => return tools.clone(),
    };
    let converted: Vec<serde_json::Value> = arr
        .iter()
        .map(|tool| {
            let mut func = serde_json::Map::new();
            if let Some(name) = tool.get("name") {
                func.insert("name".into(), name.clone());
            }
            if let Some(desc) = tool.get("description") {
                func.insert("description".into(), desc.clone());
            }
            // Anthropic uses "input_schema"; OpenAI uses "parameters" — same JSON Schema content.
            if let Some(schema) = tool.get("input_schema") {
                func.insert("parameters".into(), schema.clone());
            }
            serde_json::json!({"type": "function", "function": func})
        })
        .collect();
    serde_json::Value::Array(converted)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn anthropic_tools_to_openai_converts_correctly() {
        let anthropic = serde_json::json!([{
            "name": "Read",
            "description": "Read a file",
            "input_schema": {"type": "object", "properties": {"file_path": {"type": "string"}}, "required": ["file_path"]}
        }]);
        let openai = anthropic_tools_to_openai(&anthropic);
        let arr = openai.as_array().unwrap();
        assert_eq!(arr.len(), 1);
        assert_eq!(arr[0]["type"], "function");
        assert_eq!(arr[0]["function"]["name"], "Read");
        assert_eq!(arr[0]["function"]["description"], "Read a file");
        assert!(arr[0]["function"].get("parameters").is_some());
        assert!(arr[0]["function"].get("input_schema").is_none());
    }

    #[test]
    fn anthropic_tools_to_openai_handles_non_array() {
        let val = serde_json::json!(null);
        let out = anthropic_tools_to_openai(&val);
        assert!(out.is_null());
    }
}
