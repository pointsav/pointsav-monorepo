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
pub use yoyo::{BearerTokenProvider, PricingConfig, StaticBearer, YoYoTierClient, YoYoTierConfig};

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
