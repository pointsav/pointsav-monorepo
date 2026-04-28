// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Tier A — local OpenAI-compatible HTTP endpoint.
//!
//! Backed by mistral.rs (long-term Phase-2 runtime per SLM-STACK.md) or
//! llama-server (the Phase-1 prototype runtime per Master's v0.0.9
//! progress note — the runtime that A3 used). Both expose the same
//! OpenAI-compatible wire format, so the client does not branch on which
//! is running.

use std::time::Instant;

use serde::{Deserialize, Serialize};
use slm_core::{ChatMessage, ComputeRequest, ComputeResponse, Tier};
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

    pub async fn complete(&self, req: &ComputeRequest) -> Result<ComputeResponse> {
        let model = req
            .model
            .clone()
            .unwrap_or_else(|| self.config.default_model.clone());
        let body = OpenAiChatRequest {
            model: model.clone(),
            messages: req.messages.clone(),
            stream: req.stream,
            max_tokens: req.max_tokens,
            temperature: req.temperature,
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

        let content = resp
            .choices
            .into_iter()
            .next()
            .map(|c| c.message.content)
            .ok_or_else(|| DoormanError::UpstreamShape("no choices in response".into()))?;

        Ok(ComputeResponse {
            request_id: req.request_id,
            tier_used: Tier::Local,
            model,
            content,
            inference_ms,
            // Tier A runs on already-paid-for VM compute; per substrate
            // decision the marginal cost is sunk in the VM cost.
            cost_usd: 0.0,
            upstream_version: None,
        })
    }
}

#[derive(Serialize)]
struct OpenAiChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "is_false")]
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
}

#[derive(Deserialize)]
struct OpenAiChatResponse {
    choices: Vec<OpenAiChatChoice>,
}

#[derive(Deserialize)]
struct OpenAiChatChoice {
    message: ChatMessage,
}

fn is_false(b: &bool) -> bool {
    !*b
}

#[cfg(test)]
mod tests {
    use super::*;
    use slm_core::{ChatMessage, Complexity, ComputeRequest, ModuleId, RequestId, Tier};
    use std::str::FromStr;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn req() -> ComputeRequest {
        ComputeRequest {
            request_id: RequestId::new(),
            module_id: ModuleId::from_str("foundry").unwrap(),
            model: Some("OLMo-3-7B-Q4_K_M.gguf".into()),
            messages: vec![ChatMessage {
                role: "user".into(),
                content: "ping".into(),
            }],
            complexity: Complexity::Low,
            tier_hint: Some(Tier::Local),
            stream: false,
            max_tokens: Some(20),
            temperature: Some(0.0),
            sanitised_outbound: true,
            tier_c_label: None,
            grammar: None,
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
}
