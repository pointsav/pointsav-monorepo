// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Shared types for service-slm.
//!
//! `slm-core` holds only types and small value-objects. It has no async
//! runtime, no HTTP client, no I/O. Crates that route, log, or serve HTTP
//! depend on this crate; nothing in this crate depends on them.

pub mod apprenticeship;
pub mod error;
pub mod module_id;
pub mod request_id;
pub mod tier;

pub use apprenticeship::{
    ApprenticeshipAttempt, ApprenticeshipBrief, ApprenticeshipVerdict, BriefScope, SeniorRole,
    VerdictOutcome, APPRENTICE_ESCALATE_THRESHOLD, DEFAULT_BRIEF_TIER_B_THRESHOLD_CHARS,
    VERDICT_BATCH_NAMESPACE, VERDICT_NAMESPACE,
};
pub use error::{CoreError, Result};
pub use module_id::ModuleId;
pub use request_id::RequestId;
pub use tier::{Complexity, Tier};

use serde::{Deserialize, Serialize};

/// One inbound message in OpenAI chat-completions shape.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

/// Decode-time grammar constraint that the caller wants the Doorman to
/// enforce on the backend's output.
///
/// Wire shape (adjacent-tagged, matching OpenAI `function_call` / `tool_choice`
/// conventions):
///
/// ```json
/// {"type": "lark",        "value": "start: ..."}
/// {"type": "gbnf",        "value": "root ::= ..."}
/// {"type": "json-schema", "value": {"type": "object", ...}}
/// ```
///
/// Tier translation policy:
/// - **Tier A** (llama-server): `gbnf` and `json-schema` accepted natively;
///   `lark` rejected (llama-server HTTP API does not accept Lark grammars).
/// - **Tier B** (vLLM ≥0.12): all three variants forwarded via
///   `extra_body.structured_outputs.{grammar,json_schema}`.
/// - **Tier C** (external API): all variants rejected — no arbitrary grammar
///   support across vendors. Steps 2-5 (PS.3) wire the per-tier logic.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "value", rename_all = "kebab-case")]
pub enum GrammarConstraint {
    /// Lark EBNF grammar string.
    Lark(String),
    /// GBNF grammar string (llama.cpp native format).
    Gbnf(String),
    /// JSON Schema object. Version not pinned at the type level; schema
    /// validity is the backend's responsibility.
    JsonSchema(serde_json::Value),
}

/// Request crossing the Doorman boundary.
///
/// `request_id` and `module_id` are mandatory — they tag every audit-ledger
/// entry and every multi-tenant routing decision. `tier_hint` is advisory;
/// the router may override based on budget caps and request shape.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ComputeRequest {
    pub request_id: RequestId,
    pub module_id: ModuleId,
    pub model: Option<String>,
    pub messages: Vec<ChatMessage>,
    #[serde(default)]
    pub complexity: Complexity,
    #[serde(default)]
    pub tier_hint: Option<Tier>,
    #[serde(default)]
    pub stream: bool,
    #[serde(default)]
    pub max_tokens: Option<u32>,
    #[serde(default)]
    pub temperature: Option<f32>,
    /// True if the caller has already sanitised identifiers out of the
    /// payload per the Doorman Protocol (`ARCHITECTURE.md` §1).
    #[serde(default)]
    pub sanitised_outbound: bool,
    /// Required to dispatch to Tier C (External API). The Doorman
    /// refuses any request hinted at Tier C without an allowlisted
    /// label per `~/Foundry/conventions/llm-substrate-decision.md`.
    /// Optional for Tier A and Tier B.
    #[serde(default)]
    pub tier_c_label: Option<String>,
    /// Optional decode-time grammar constraint. When present the Doorman
    /// translates the constraint into the backend-specific wire format for
    /// the selected tier. Absent from most requests; omitted from the
    /// serialised form when `None` so existing callers are unaffected.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grammar: Option<GrammarConstraint>,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a minimal `ComputeRequest` with only mandatory fields.
    fn minimal_request() -> ComputeRequest {
        use std::str::FromStr;
        ComputeRequest {
            request_id: RequestId::new(),
            module_id: ModuleId::from_str("test-module").unwrap(),
            model: None,
            messages: vec![ChatMessage {
                role: "user".to_string(),
                content: "hello".to_string(),
            }],
            complexity: Complexity::default(),
            tier_hint: None,
            stream: false,
            max_tokens: None,
            temperature: None,
            sanitised_outbound: false,
            tier_c_label: None,
            grammar: None,
        }
    }

    #[test]
    fn compute_request_serde_round_trip_no_grammar() {
        let req = minimal_request();
        let json = serde_json::to_string(&req).unwrap();
        // grammar field must be absent when None (skip_serializing_if)
        assert!(
            !json.contains("grammar"),
            "serialised form must not contain 'grammar' key when grammar is None; got: {json}"
        );
        let req2: ComputeRequest = serde_json::from_str(&json).unwrap();
        assert!(req2.grammar.is_none());
    }

    #[test]
    fn compute_request_serde_round_trip_lark() {
        let grammar_str = r#"start: /[a-z]+/ NEWLINE?"#;
        let mut req = minimal_request();
        req.grammar = Some(GrammarConstraint::Lark(grammar_str.to_string()));

        let json = serde_json::to_string(&req).unwrap();
        // Wire shape must contain the lark type discriminant.
        assert!(
            json.contains(r#""type":"lark""#),
            "expected lark type discriminant; got: {json}"
        );
        // The grammar string value must survive in the serialised form.
        assert!(
            json.contains(grammar_str),
            "grammar string must survive serialisation; got: {json}"
        );

        // Round-trip: deserialise back and check equality.
        let req2: ComputeRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(
            req2.grammar,
            Some(GrammarConstraint::Lark(grammar_str.to_string()))
        );
    }

    #[test]
    fn compute_request_serde_round_trip_gbnf() {
        let grammar_str = r#"root ::= "yes" | "no""#;
        let mut req = minimal_request();
        req.grammar = Some(GrammarConstraint::Gbnf(grammar_str.to_string()));

        let json = serde_json::to_string(&req).unwrap();
        assert!(
            json.contains(r#""type":"gbnf""#),
            "expected gbnf type discriminant; got: {json}"
        );

        let req2: ComputeRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(
            req2.grammar,
            Some(GrammarConstraint::Gbnf(grammar_str.to_string()))
        );
    }

    #[test]
    fn compute_request_serde_round_trip_json_schema() {
        let schema: serde_json::Value = serde_json::json!({
            "type": "object",
            "properties": {
                "name": {"type": "string"},
                "age":  {"type": "integer"}
            },
            "required": ["name"]
        });
        let mut req = minimal_request();
        req.grammar = Some(GrammarConstraint::JsonSchema(schema.clone()));

        let json = serde_json::to_string(&req).unwrap();
        assert!(
            json.contains(r#""type":"json-schema""#),
            "expected json-schema type discriminant; got: {json}"
        );

        let req2: ComputeRequest = serde_json::from_str(&json).unwrap();
        match req2.grammar {
            Some(GrammarConstraint::JsonSchema(v)) => {
                assert_eq!(v, schema, "JSON Schema value must be preserved exactly");
            }
            other => panic!("expected JsonSchema variant, got: {other:?}"),
        }
    }

    #[test]
    fn compute_request_default_grammar_is_none() {
        // Construct a request without setting grammar; verify the field is None.
        let req = minimal_request();
        assert!(
            req.grammar.is_none(),
            "grammar must default to None when not set"
        );

        // Also verify that deserialising a JSON object that lacks the grammar
        // key produces grammar: None (serde default attribute).
        let json_without_grammar = serde_json::json!({
            "request_id": req.request_id,
            "module_id": req.module_id,
            "messages": [{"role": "user", "content": "test"}],
        })
        .to_string();
        let req2: ComputeRequest = serde_json::from_str(&json_without_grammar).unwrap();
        assert!(
            req2.grammar.is_none(),
            "grammar must default to None when absent from JSON"
        );
    }
}

/// Response returned through the Doorman.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ComputeResponse {
    pub request_id: RequestId,
    pub tier_used: Tier,
    pub model: String,
    pub content: String,
    pub inference_ms: u64,
    pub cost_usd: f64,
    /// Yo-Yo or external-API implementation version, opaque string.
    #[serde(default)]
    pub upstream_version: Option<String>,
}
