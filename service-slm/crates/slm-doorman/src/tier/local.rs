// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Tier A — local OpenAI-compatible HTTP endpoint.
//!
//! Backed by mistral.rs (long-term Phase-2 runtime per SLM-STACK.md) or
//! llama-server (the Phase-1 prototype runtime per Master's v0.0.9
//! progress note — the runtime that A3 used). Both expose the same
//! OpenAI-compatible wire format, so the client does not branch on which
//! is running.

use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use slm_core::{CanonicalMessage, ContentBlock, ComputeRequest, ComputeResponse, GrammarConstraint, Tier};
use tracing::debug;

use crate::error::{DoormanError, Result};

#[derive(Clone, Debug)]
pub struct LocalTierConfig {
    /// Base URL of the local OpenAI-compatible server, e.g.
    /// `http://127.0.0.1:8080`.
    pub endpoint: String,
    /// Default model identifier. Local Tier A runs OLMo 3 7B Q4
    /// (Apache-2.0 + Open Data Commons; see substrate decision).
    pub default_model: String,
}

pub struct LocalTierClient {
    config: LocalTierConfig,
    http: reqwest::Client,
}

impl LocalTierClient {
    pub fn new(config: LocalTierConfig) -> Self {
        Self {
            config,
            http: reqwest::Client::new(),
        }
    }

    pub fn endpoint(&self) -> &str {
        &self.config.endpoint
    }

    /// Query `GET /health` on the llama-server and return `true` when all
    /// inference slots are busy (`slots_idle == 0`).
    ///
    /// Non-fatal by design: any network or parse error returns `false`
    /// (not busy) so a health-endpoint misconfiguration can never block
    /// inference. The 500 ms timeout prevents a slow health check from
    /// adding significant latency to the hot path.
    pub async fn is_busy(&self) -> bool {
        let url = format!("{}/health", self.config.endpoint.trim_end_matches('/'));
        let resp = match self
            .http
            .get(&url)
            .timeout(Duration::from_millis(500))
            .send()
            .await
        {
            Ok(r) => r,
            Err(_) => return false,
        };
        match resp.json::<LlamaHealthResponse>().await {
            Ok(h) => h.slots_idle == Some(0),
            Err(_) => false,
        }
    }

    pub async fn complete(&self, req: &ComputeRequest) -> Result<ComputeResponse> {
        // Grammar validation — pure check, no network calls. Runs first so
        // invalid input is rejected before the busy check round-trip.
        // Lark is rejected before any network call (invariant relied on by tests).
        let (grammar_field, json_schema_field) = match req.grammar.as_ref() {
            None => (None, None),
            Some(GrammarConstraint::Gbnf(s)) => (Some(s.clone()), None),
            Some(GrammarConstraint::JsonSchema(v)) => (None, Some(v.clone())),
            Some(GrammarConstraint::Lark(_)) => {
                return Err(DoormanError::TierAGrammarUnsupported {
                    dialect: "Lark",
                    advice: "escalate to Tier B (Yo-Yo) which supports Lark via llguidance, \
                             or provide a GBNF equivalent for Tier A",
                });
            }
        };

        let model = req
            .model
            .clone()
            .unwrap_or_else(|| self.config.default_model.clone());

        // Pre-flight busy check. Returns TierABusy when all inference slots
        // are occupied so the router can escalate to Tier B rather than
        // queuing behind a saturated local server. Runs after grammar
        // validation so invalid input is rejected cheaply before any I/O.
        if self.is_busy().await {
            return Err(crate::error::DoormanError::TierABusy);
        }

        let tools = req.tools.as_ref().map(|defs| {
            defs.iter().map(|d| OaiToolDef {
                kind: "function",
                function: OaiFunctionDef {
                    name: d.name.clone(),
                    description: d.description.clone(),
                    parameters: d.input_schema.clone(),
                },
            }).collect::<Vec<_>>()
        });
        let body = OpenAiChatRequest {
            model: model.clone(),
            messages: canonical_to_oai(&req.messages),
            stream: req.stream,
            max_tokens: req.max_tokens,
            temperature: req.temperature,
            grammar: grammar_field,
            json_schema: json_schema_field,
            tools,
        };
        let url = format!(
            "{}/v1/chat/completions",
            self.config.endpoint.trim_end_matches('/')
        );
        debug!(target: "slm_doorman::tier::local", %url, %model, "tier-A request");

        let started = Instant::now();
        let resp: OpenAiChatResponse = self
            .http
            .post(&url)
            .json(&body)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        let inference_ms = started.elapsed().as_millis() as u64;

        let msg = resp
            .choices
            .into_iter()
            .next()
            .map(|c| c.message)
            .ok_or_else(|| DoormanError::UpstreamShape("no choices in response".into()))?;

        let (content, content_blocks) = if !msg.tool_calls.is_empty() {
            let blocks = msg.tool_calls.into_iter().map(|tc| {
                let input = serde_json::from_str(&tc.function.arguments)
                    .unwrap_or(serde_json::Value::Null);
                ContentBlock::ToolUse { id: tc.id, name: tc.function.name, input }
            }).collect();
            (String::new(), blocks)
        } else {
            (msg.content.unwrap_or_default(), Vec::new())
        };

        Ok(ComputeResponse {
            request_id: req.request_id,
            tier_used: Tier::Local,
            model,
            content,
            content_blocks,
            inference_ms,
            // Tier A runs on already-paid-for VM compute; per substrate
            // decision the marginal cost is sunk in the VM cost.
            cost_usd: 0.0,
            upstream_version: None,
            // Tier A llama-server hot-swap not yet wired (Phase 1 of
            // learning-loop-master-plan-2026-05-18.md P1-1.9). Base
            // model only — report None until `--lora` path lands.
            adapter_version: None,
        })
    }

    /// Begin a streaming Tier-A request. Returns the raw llama-server HTTP
    /// response on success; the caller translates the SSE body to the target
    /// wire format using `build_stream_body`.
    ///
    /// Applies the same grammar validation and busy-check as `complete()`.
    /// Returns `Err(TierABusy)` when all inference slots are saturated.
    /// Does NOT fall back — streaming callers handle fallback themselves.
    pub async fn start_stream(&self, req: &ComputeRequest) -> Result<reqwest::Response> {
        let (grammar_field, json_schema_field) = match req.grammar.as_ref() {
            None => (None, None),
            Some(GrammarConstraint::Gbnf(s)) => (Some(s.clone()), None),
            Some(GrammarConstraint::JsonSchema(v)) => (None, Some(v.clone())),
            Some(GrammarConstraint::Lark(_)) => {
                return Err(DoormanError::TierAGrammarUnsupported {
                    dialect: "Lark",
                    advice: "escalate to Tier B (Yo-Yo) which supports Lark via llguidance, \
                             or provide a GBNF equivalent for Tier A",
                });
            }
        };

        if self.is_busy().await {
            return Err(DoormanError::TierABusy);
        }

        let model = req
            .model
            .clone()
            .unwrap_or_else(|| self.config.default_model.clone());
        let tools = req.tools.as_ref().map(|defs| {
            defs.iter().map(|d| OaiToolDef {
                kind: "function",
                function: OaiFunctionDef {
                    name: d.name.clone(),
                    description: d.description.clone(),
                    parameters: d.input_schema.clone(),
                },
            }).collect::<Vec<_>>()
        });
        let body = OpenAiChatRequest {
            model: model.clone(),
            messages: canonical_to_oai(&req.messages),
            stream: true,
            max_tokens: req.max_tokens,
            temperature: req.temperature,
            grammar: grammar_field,
            json_schema: json_schema_field,
            tools,
        };
        let url = format!(
            "{}/v1/chat/completions",
            self.config.endpoint.trim_end_matches('/')
        );
        debug!(target: "slm_doorman::tier::local", %url, %model, "tier-A stream request");

        Ok(self.http.post(&url).json(&body).send().await?.error_for_status()?)
    }
}

/// Minimal subset of the llama-server `/health` response we care about.
/// Unknown fields are ignored. `slots_idle` is `Option` because newer
/// llama-server versions omit it from `/health`; absent means not busy.
#[derive(Deserialize)]
struct LlamaHealthResponse {
    slots_idle: Option<u32>,
}

#[derive(Serialize)]
#[serde(untagged)]
enum OaiWireMessage {
    Text { role: String, content: String },
    ToolCall { role: String, content: serde_json::Value, tool_calls: Vec<OaiToolCall> },
    ToolResult { role: String, content: String, tool_call_id: String },
}

#[derive(Serialize)]
struct OaiToolCall {
    id: String,
    #[serde(rename = "type")]
    kind: &'static str,
    function: OaiFunction,
}

#[derive(Serialize)]
struct OaiFunction {
    name: String,
    arguments: String,
}

fn canonical_to_oai(msgs: &[CanonicalMessage]) -> Vec<OaiWireMessage> {
    let mut out = Vec::new();
    for msg in msgs {
        let role = msg.role.as_str().to_string();
        let texts: Vec<&str> = msg.content.iter().filter_map(|b| match b {
            ContentBlock::Text { text } => Some(text.as_str()),
            ContentBlock::Thinking { thinking } => Some(thinking.as_str()),
            _ => None,
        }).collect();
        let tool_uses: Vec<_> = msg.content.iter().filter_map(|b| match b {
            ContentBlock::ToolUse { id, name, input } => Some((id, name, input)),
            _ => None,
        }).collect();
        let tool_results: Vec<_> = msg.content.iter().filter_map(|b| match b {
            ContentBlock::ToolResult { tool_use_id, content } => Some((tool_use_id, content)),
            _ => None,
        }).collect();

        if !tool_uses.is_empty() {
            out.push(OaiWireMessage::ToolCall {
                role,
                content: serde_json::Value::Null,
                tool_calls: tool_uses.into_iter().map(|(id, name, input)| OaiToolCall {
                    id: id.clone(),
                    kind: "function",
                    function: OaiFunction {
                        name: name.clone(),
                        arguments: serde_json::to_string(input).unwrap_or_default(),
                    },
                }).collect(),
            });
        } else if !tool_results.is_empty() {
            for (tool_use_id, content) in tool_results {
                out.push(OaiWireMessage::ToolResult {
                    role: "tool".to_string(),
                    content: content.clone(),
                    tool_call_id: tool_use_id.clone(),
                });
            }
        } else {
            out.push(OaiWireMessage::Text { role, content: texts.join("\n") });
        }
    }
    out
}

#[derive(Serialize)]
struct OaiToolDef {
    #[serde(rename = "type")]
    kind: &'static str,
    function: OaiFunctionDef,
}

#[derive(Serialize)]
struct OaiFunctionDef {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    parameters: serde_json::Value,
}

#[derive(Serialize)]
struct OpenAiChatRequest {
    model: String,
    messages: Vec<OaiWireMessage>,
    #[serde(skip_serializing_if = "is_false")]
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    /// GBNF grammar string. Top-level llama-server field (NOT inside
    /// `extra_body`). Absent when `None`.
    #[serde(skip_serializing_if = "Option::is_none")]
    grammar: Option<String>,
    /// JSON Schema for structured output. Top-level llama-server field.
    /// Absent when `None`.
    #[serde(skip_serializing_if = "Option::is_none")]
    json_schema: Option<serde_json::Value>,
    /// Tool definitions (P1-1.7). Forwarded from `ComputeRequest.tools`.
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<OaiToolDef>>,
}

#[derive(Deserialize)]
struct OpenAiChatResponse {
    choices: Vec<OpenAiChatChoice>,
}

#[derive(Deserialize)]
struct OpenAiChatChoice {
    message: OaiResponseMessage,
}

#[derive(Deserialize)]
struct OaiResponseMessage {
    #[serde(default)]
    content: Option<String>,
    #[serde(default)]
    tool_calls: Vec<OaiResponseToolCall>,
}

#[derive(Deserialize)]
struct OaiResponseToolCall {
    id: String,
    function: OaiResponseFunction,
}

#[derive(Deserialize)]
struct OaiResponseFunction {
    name: String,
    arguments: String,
}

fn is_false(b: &bool) -> bool {
    !*b
}

#[cfg(test)]
mod tests {
    use super::*;
    use slm_core::{
        CanonicalMessage, Complexity, ComputeRequest, GrammarConstraint, LatencyClass, ModuleId,
        RequestId, Tier,
    };
    use std::str::FromStr;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn req() -> ComputeRequest {
        ComputeRequest {
            request_id: RequestId::new(),
            module_id: ModuleId::from_str("foundry").unwrap(),
            model: Some("OLMo-3-7B-Q4_K_M.gguf".into()),
            messages: vec![CanonicalMessage::text("user", "ping")],
            complexity: Complexity::Low,
            latency_class: LatencyClass::default(),
            tier_hint: Some(Tier::Local),
            stream: false,
            max_tokens: Some(20),
            temperature: Some(0.0),
            sanitised_outbound: true,
            tier_c_label: None,
            yoyo_label: None,
            grammar: None,
            speculation: None,
            graph_context_enabled: None,
            adapter_version: None,
            tools: None,
        }
    }

    fn ok_body() -> serde_json::Value {
        serde_json::json!({
            "choices": [
                { "message": { "role": "assistant", "content": "PONG" } }
            ]
        })
    }

    fn client(server_uri: String) -> LocalTierClient {
        LocalTierClient::new(LocalTierConfig {
            endpoint: server_uri,
            default_model: "OLMo-3-7B-Q4_K_M.gguf".into(),
        })
    }

    /// Happy path — 200 with well-formed choices array. Verify:
    /// - content extracted correctly
    /// - tier_used is Local
    /// - model is echoed from the request
    /// - cost_usd is 0.0 (VM compute is sunk cost per architecture)
    /// - request shape includes model and messages (POST to /v1/chat/completions)
    #[tokio::test]
    async fn happy_path_200_returns_content_and_local_tier() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v1/chat/completions"))
            .respond_with(ResponseTemplate::new(200).set_body_json(ok_body()))
            .expect(1)
            .mount(&server)
            .await;

        let client = client(server.uri());
        let resp = client.complete(&req()).await.expect("happy path 200");
        assert_eq!(resp.tier_used, Tier::Local);
        assert_eq!(resp.content, "PONG");
        assert_eq!(resp.model, "OLMo-3-7B-Q4_K_M.gguf");
        assert_eq!(
            resp.cost_usd, 0.0,
            "Tier A cost is always 0.0 (sunk VM compute)"
        );
        assert!(
            resp.inference_ms < 10_000,
            "sanity: inference_ms should be wall-clock ms"
        );
    }

    /// Default model is used when request carries no model field.
    /// Tier A's default is the configured `LocalTierConfig::default_model`.
    #[tokio::test]
    async fn default_model_used_when_request_omits_model() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v1/chat/completions"))
            .respond_with(ResponseTemplate::new(200).set_body_json(ok_body()))
            .expect(1)
            .mount(&server)
            .await;

        let client = client(server.uri());
        let mut r = req();
        r.model = None; // no model in request — should fall back to config default
        let resp = client.complete(&r).await.expect("default model happy path");
        assert_eq!(resp.model, "OLMo-3-7B-Q4_K_M.gguf");
    }

    /// HTTP 5xx error via `error_for_status()` must surface as
    /// `DoormanError::Upstream` (the `#[from] reqwest::Error` variant).
    /// The Doorman does NOT retry on Tier A — a 500 from the local
    /// llama-server is an operator problem; the router is responsible for
    /// any fallback to Tier B.
    #[tokio::test]
    async fn http_5xx_surfaces_as_upstream_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v1/chat/completions"))
            .respond_with(ResponseTemplate::new(500))
            .expect(1)
            .mount(&server)
            .await;

        let client = client(server.uri());
        let err = client
            .complete(&req())
            .await
            .expect_err("500 must surface as error");
        assert!(
            matches!(err, DoormanError::Upstream(_)),
            "expected DoormanError::Upstream for 5xx, got {err:?}"
        );
    }

    /// Empty `choices` array — the server returned 200 but with no
    /// candidates. Must surface `DoormanError::UpstreamShape` naming
    /// the empty-choices case, with no content extracted.
    #[tokio::test]
    async fn empty_choices_surfaces_upstream_shape() {
        let server = MockServer::start().await;
        let empty_body = serde_json::json!({ "choices": [] });
        Mock::given(method("POST"))
            .and(path("/v1/chat/completions"))
            .respond_with(ResponseTemplate::new(200).set_body_json(empty_body))
            .expect(1)
            .mount(&server)
            .await;

        let client = client(server.uri());
        let err = client
            .complete(&req())
            .await
            .expect_err("empty choices must surface as error");
        match err {
            DoormanError::UpstreamShape(msg) => {
                assert!(
                    msg.contains("no choices"),
                    "UpstreamShape message should mention empty choices, got: {msg:?}"
                );
            }
            other => panic!("expected DoormanError::UpstreamShape, got {other:?}"),
        }
    }

    /// Malformed JSON response body — server returns 200 with an
    /// invalid JSON body. The `resp.json().await?` call returns a
    /// `reqwest::Error` which maps to `DoormanError::Upstream`.
    #[tokio::test]
    async fn malformed_json_body_surfaces_upstream_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v1/chat/completions"))
            .respond_with(
                ResponseTemplate::new(200).set_body_raw(b"not json {".to_vec(), "application/json"),
            )
            .expect(1)
            .mount(&server)
            .await;

        let client = client(server.uri());
        let err = client
            .complete(&req())
            .await
            .expect_err("malformed JSON must surface as error");
        assert!(
            matches!(err, DoormanError::Upstream(_)),
            "expected DoormanError::Upstream for JSON parse failure, got {err:?}"
        );
    }

    // ── Grammar serialisation tests ────────────────────────────────────────

    /// When `grammar` is `None` the upstream body must contain neither
    /// `"grammar"` nor `"json_schema"` keys. Absence is verified both by
    /// parsing the captured body and by checking that the mock received
    /// exactly one request (sanity).
    #[tokio::test]
    async fn grammar_none_omits_all_grammar_fields() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/health"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::json!({"status": "ok", "slots_idle": 1}),
            ))
            .mount(&server)
            .await;
        Mock::given(method("POST"))
            .and(path("/v1/chat/completions"))
            .respond_with(ResponseTemplate::new(200).set_body_json(ok_body()))
            .expect(1)
            .mount(&server)
            .await;

        let client = client(server.uri());
        let mut r = req();
        r.grammar = None;
        client.complete(&r).await.expect("none grammar happy path");

        let all_requests = server.received_requests().await.unwrap();
        let post_requests: Vec<_> = all_requests
            .iter()
            .filter(|r| r.method.as_str() == "POST")
            .collect();
        assert_eq!(post_requests.len(), 1, "expected exactly one POST upstream request");
        let body: serde_json::Value =
            serde_json::from_slice(&post_requests[0].body).expect("request body must be valid JSON");
        assert!(
            body.get("grammar").is_none(),
            "body must not contain 'grammar' key when grammar is None"
        );
        assert!(
            body.get("json_schema").is_none(),
            "body must not contain 'json_schema' key when grammar is None"
        );
    }

    /// When `grammar` is `Some(Gbnf(...))` the upstream body must contain
    /// `"grammar": "<gbnf string>"` at the top level (NOT inside
    /// `extra_body`), and must NOT contain `"json_schema"`.
    #[tokio::test]
    async fn grammar_gbnf_serialises_into_top_level_grammar_field() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/health"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::json!({"status": "ok", "slots_idle": 1}),
            ))
            .mount(&server)
            .await;
        Mock::given(method("POST"))
            .and(path("/v1/chat/completions"))
            .respond_with(ResponseTemplate::new(200).set_body_json(ok_body()))
            .expect(1)
            .mount(&server)
            .await;

        let client = client(server.uri());
        let mut r = req();
        let gbnf = r#"root ::= "yes" | "no""#;
        r.grammar = Some(GrammarConstraint::Gbnf(gbnf.to_string()));
        client.complete(&r).await.expect("gbnf grammar happy path");

        let all_requests = server.received_requests().await.unwrap();
        let post_requests: Vec<_> = all_requests
            .iter()
            .filter(|r| r.method.as_str() == "POST")
            .collect();
        assert_eq!(post_requests.len(), 1);
        let body: serde_json::Value =
            serde_json::from_slice(&post_requests[0].body).expect("request body must be valid JSON");
        assert_eq!(
            body.get("grammar").and_then(|v| v.as_str()),
            Some(gbnf),
            "body must contain top-level 'grammar' with the GBNF string"
        );
        assert!(
            body.get("json_schema").is_none(),
            "body must not contain 'json_schema' when grammar is Gbnf"
        );
        // Must NOT be nested inside extra_body (llama-server native field)
        assert!(
            body.get("extra_body").is_none(),
            "Tier A must NOT use extra_body; grammar goes at top level"
        );
    }

    /// When `grammar` is `Some(JsonSchema(...))` the upstream body must
    /// contain `"json_schema": <value>` at the top level and must NOT
    /// contain a `"grammar"` key.
    #[tokio::test]
    async fn grammar_json_schema_serialises_into_top_level_json_schema_field() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/health"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::json!({"status": "ok", "slots_idle": 1}),
            ))
            .mount(&server)
            .await;
        Mock::given(method("POST"))
            .and(path("/v1/chat/completions"))
            .respond_with(ResponseTemplate::new(200).set_body_json(ok_body()))
            .expect(1)
            .mount(&server)
            .await;

        let client = client(server.uri());
        let mut r = req();
        let schema = serde_json::json!({
            "type": "object",
            "properties": {
                "answer": {"type": "string"}
            },
            "required": ["answer"]
        });
        r.grammar = Some(GrammarConstraint::JsonSchema(schema.clone()));
        client
            .complete(&r)
            .await
            .expect("json_schema grammar happy path");

        let all_requests = server.received_requests().await.unwrap();
        let post_requests: Vec<_> = all_requests
            .iter()
            .filter(|r| r.method.as_str() == "POST")
            .collect();
        assert_eq!(post_requests.len(), 1);
        let body: serde_json::Value =
            serde_json::from_slice(&post_requests[0].body).expect("request body must be valid JSON");
        assert_eq!(
            body.get("json_schema"),
            Some(&schema),
            "body must contain top-level 'json_schema' with the schema value"
        );
        assert!(
            body.get("grammar").is_none(),
            "body must not contain 'grammar' key when grammar is JsonSchema"
        );
        assert!(
            body.get("extra_body").is_none(),
            "Tier A must NOT use extra_body; json_schema goes at top level"
        );
    }

    // ── Busy-check tests ──────────────────────────────────────────────────

    /// When `/health` reports `slots_idle=0`, `complete()` returns
    /// `TierABusy` before making any inference call.
    #[tokio::test]
    async fn busy_health_check_short_circuits_inference() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/health"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "status": "no slot available",
                "slots_idle": 0,
                "slots_processing": 1
            })))
            .expect(1)
            .mount(&server)
            .await;
        // No mock for /v1/chat/completions — any inference call would be unexpected.

        let client = client(server.uri());
        let err = client.complete(&req()).await.expect_err("busy must error");
        assert!(
            matches!(err, DoormanError::TierABusy),
            "expected TierABusy when slots_idle=0, got {err:?}"
        );
    }

    /// When `/health` reports `slots_idle >= 1`, `complete()` proceeds
    /// to the inference call normally.
    #[tokio::test]
    async fn idle_health_check_proceeds_to_inference() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/health"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "status": "ok",
                "slots_idle": 1,
                "slots_processing": 0
            })))
            .expect(1)
            .mount(&server)
            .await;
        Mock::given(method("POST"))
            .and(path("/v1/chat/completions"))
            .respond_with(ResponseTemplate::new(200).set_body_json(ok_body()))
            .expect(1)
            .mount(&server)
            .await;

        let client = client(server.uri());
        let resp = client
            .complete(&req())
            .await
            .expect("idle slots must succeed");
        assert_eq!(resp.content, "PONG");
    }

    /// When `/health` omits `slots_idle` (newer llama-server), `is_busy()`
    /// returns `false` and `complete()` proceeds normally.
    #[tokio::test]
    async fn health_check_missing_slots_idle_falls_through_to_inference() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/health"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "status": "ok"
            })))
            .expect(1)
            .mount(&server)
            .await;
        Mock::given(method("POST"))
            .and(path("/v1/chat/completions"))
            .respond_with(ResponseTemplate::new(200).set_body_json(ok_body()))
            .expect(1)
            .mount(&server)
            .await;

        let client = client(server.uri());
        let resp = client
            .complete(&req())
            .await
            .expect("missing slots_idle must not block inference");
        assert_eq!(resp.content, "PONG");
    }

    /// When `/health` returns 500, `is_busy()` returns `false` (non-fatal)
    /// and `complete()` falls through to the inference call.
    #[tokio::test]
    async fn health_check_error_falls_through_to_inference() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/health"))
            .respond_with(ResponseTemplate::new(500))
            .expect(1)
            .mount(&server)
            .await;
        Mock::given(method("POST"))
            .and(path("/v1/chat/completions"))
            .respond_with(ResponseTemplate::new(200).set_body_json(ok_body()))
            .expect(1)
            .mount(&server)
            .await;

        let client = client(server.uri());
        let resp = client
            .complete(&req())
            .await
            .expect("health error must not block inference");
        assert_eq!(resp.content, "PONG");
    }

    /// When `grammar` is `Some(Lark(...))` the call must return a typed
    /// `DoormanError::TierAGrammarUnsupported` error BEFORE making any
    /// network call. The wiremock server must have received zero requests.
    #[tokio::test]
    async fn grammar_lark_rejected_before_any_network_call() {
        let server = MockServer::start().await;
        // No mock registered — any request reaching the server would be
        // an unexpected call and cause the test to fail at server drop.

        let client = client(server.uri());
        let mut r = req();
        r.grammar = Some(GrammarConstraint::Lark("start: /[a-z]+/".to_string()));
        let err = client
            .complete(&r)
            .await
            .expect_err("Lark grammar must be rejected");

        assert!(
            matches!(
                err,
                DoormanError::TierAGrammarUnsupported {
                    dialect: "Lark",
                    ..
                }
            ),
            "expected TierAGrammarUnsupported with dialect=Lark, got {err:?}"
        );

        // Critical: no upstream call must have been made.
        let received = server.received_requests().await.unwrap();
        assert!(
            received.is_empty(),
            "Lark rejection must happen before any network call; \
             wiremock received {} request(s)",
            received.len()
        );
    }
}
