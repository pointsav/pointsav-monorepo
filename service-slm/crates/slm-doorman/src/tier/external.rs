// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Tier C — external API (Anthropic Claude / Google Gemini / OpenAI).
//!
//! Hard rule per `~/Foundry/conventions/llm-substrate-decision.md`:
//! this tier is reserved for narrow precision tasks (citation
//! grounding, initial graph build, entity disambiguation). It is
//! NEVER a default fallback path — calls must reference an explicit
//! allowlist label and the Doorman refuses requests whose label is
//! not allowlisted.
//!
//! Per Master's 2026-04-26 brief Answer 3, the allowlist is hardcoded
//! in v0.1.x as a `&'static [&'static str]` slice. Master operator
//! extends the allowlist via PR; runtime cannot extend it. The
//! mismatched-label error path returns `ExternalNotAllowlisted` which
//! the inbound HTTP layer surfaces as 403.
//!
//! Operator cost guardrail (Master 2026-04-26): no live API calls in
//! v0.1.x. The client is wired against `wiremock` for unit tests; live
//! Tier C activation is a separate operator decision with billing-
//! capped keys provisioned via Secret Manager and a configured
//! killswitch.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

use serde::{Deserialize, Serialize};
use slm_core::{ChatMessage, ComputeRequest, ComputeResponse, Tier};
use tracing::debug;

use crate::error::{DoormanError, Result};

/// Compile-time allowlist of Tier C task labels permitted to fire a
/// request. Values are `&'static str` — operator extends by editing
/// this constant (or the per-deployment override) and recompiling.
/// Per Master's 2026-04-26 brief Answer 3.
#[derive(Clone, Copy, Debug)]
pub struct ExternalAllowlist {
    labels: &'static [&'static str],
}

impl ExternalAllowlist {
    /// Empty allowlist — no Tier C call permitted. Default for any
    /// `ExternalTierConfig::default()`.
    pub const EMPTY: Self = Self { labels: &[] };

    /// Build from a `&'static [&'static str]` slice. Used by both the
    /// hardcoded `FOUNDRY_DEFAULT_ALLOWLIST` below and any per-
    /// deployment override (still compile-time per the doctrine).
    pub const fn from_static(labels: &'static [&'static str]) -> Self {
        Self { labels }
    }

    pub fn contains(&self, label: &str) -> bool {
        self.labels.contains(&label)
    }

    pub fn is_empty(&self) -> bool {
        self.labels.is_empty()
    }

    pub fn as_slice(&self) -> &'static [&'static str] {
        self.labels
    }
}

impl Default for ExternalAllowlist {
    fn default() -> Self {
        Self::EMPTY
    }
}

/// Foundry's default Tier C allowlist. The three labels match the
/// "narrow precision tasks" enumeration in
/// `~/Foundry/conventions/llm-substrate-decision.md` §"Three compute
/// tiers". Master operator extends here via PR.
pub const FOUNDRY_DEFAULT_ALLOWLIST: ExternalAllowlist = ExternalAllowlist::from_static(&[
    "citation-grounding",
    "initial-graph-build",
    "entity-disambiguation",
]);

/// Which external API to call. Derived from a `provider:model`
/// prefix on `ComputeRequest::model`, e.g. `anthropic:claude-haiku-4-5`,
/// `gemini:gemini-2-flash`, `openai:gpt-4o-mini`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TierCProvider {
    Anthropic,
    Gemini,
    Openai,
}

impl TierCProvider {
    pub fn as_str(&self) -> &'static str {
        match self {
            TierCProvider::Anthropic => "anthropic",
            TierCProvider::Gemini => "gemini",
            TierCProvider::Openai => "openai",
        }
    }

    /// Parse `"anthropic:claude-haiku-4-5"` → `(Anthropic, "claude-haiku-4-5")`.
    /// Returns `None` if the string has no recognised prefix.
    pub fn parse_model_id(model: &str) -> Option<(Self, &str)> {
        let (prefix, rest) = model.split_once(':')?;
        let p = match prefix {
            "anthropic" => Self::Anthropic,
            "gemini" => Self::Gemini,
            "openai" => Self::Openai,
            _ => return None,
        };
        Some((p, rest))
    }
}

/// Per-provider per-token rates for Tier C cost computation. Defaults
/// are zero (operator-config required for production).
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TierCPricing {
    pub anthropic_input_per_mtok_usd: f64,
    pub anthropic_output_per_mtok_usd: f64,
    pub gemini_input_per_mtok_usd: f64,
    pub gemini_output_per_mtok_usd: f64,
    pub openai_input_per_mtok_usd: f64,
    pub openai_output_per_mtok_usd: f64,
}

impl TierCPricing {
    /// Compute cost in USD from prompt/completion token counts.
    /// Returns 0.0 if rates are unconfigured.
    pub fn cost_usd(&self, provider: TierCProvider, prompt_toks: u32, completion_toks: u32) -> f64 {
        const M: f64 = 1_000_000.0;
        let (in_rate, out_rate) = match provider {
            TierCProvider::Anthropic => (
                self.anthropic_input_per_mtok_usd,
                self.anthropic_output_per_mtok_usd,
            ),
            TierCProvider::Gemini => (
                self.gemini_input_per_mtok_usd,
                self.gemini_output_per_mtok_usd,
            ),
            TierCProvider::Openai => (
                self.openai_input_per_mtok_usd,
                self.openai_output_per_mtok_usd,
            ),
        };
        (prompt_toks as f64 / M) * in_rate + (completion_toks as f64 / M) * out_rate
    }
}

#[derive(Clone, Debug)]
pub struct ExternalTierConfig {
    pub allowlist: ExternalAllowlist,
    /// Per-provider base URL. Defaults to the providers' production
    /// endpoints; tests inject mock-server URIs. Map key is the
    /// provider; value is the base URL (no trailing slash).
    pub provider_endpoints: HashMap<TierCProvider, String>,
    /// Per-provider API key. Empty string when not configured (the
    /// dev / community-tier default — Doorman refuses any actual
    /// dispatch when the key is empty).
    pub provider_api_keys: HashMap<TierCProvider, String>,
    pub pricing: TierCPricing,
}

impl Default for ExternalTierConfig {
    fn default() -> Self {
        Self {
            allowlist: ExternalAllowlist::EMPTY,
            provider_endpoints: HashMap::new(),
            provider_api_keys: HashMap::new(),
            pricing: TierCPricing::default(),
        }
    }
}

pub struct ExternalTierClient {
    config: ExternalTierConfig,
    http: reqwest::Client,
}

impl ExternalTierClient {
    pub fn new(config: ExternalTierConfig) -> Self {
        Self {
            config,
            http: reqwest::Client::new(),
        }
    }

    pub fn allowlist(&self) -> &ExternalAllowlist {
        &self.config.allowlist
    }

    /// Verify a label is on the allowlist. The router calls this
    /// before dispatch so an unallowlisted Tier C call never even
    /// gets a token from the bearer provider.
    pub fn check_label(&self, label: &str) -> Result<()> {
        if self.config.allowlist.contains(label) {
            Ok(())
        } else {
            Err(DoormanError::ExternalNotAllowlisted {
                label: label.to_string(),
            })
        }
    }

    pub async fn complete(&self, req: &ComputeRequest) -> Result<ComputeResponse> {
        // 1. Allowlist check before any network attempt.
        let label =
            req.tier_c_label
                .as_deref()
                .ok_or_else(|| DoormanError::ExternalNotAllowlisted {
                    label: "<unset>".to_string(),
                })?;
        self.check_label(label)?;

        // 2. Provider routing — model identifier carries `provider:`.
        let model_id = req.model.as_deref().ok_or_else(|| {
            DoormanError::UpstreamShape(
                "Tier C request missing `model` field; expected `<provider>:<model>` form".into(),
            )
        })?;
        let (provider, provider_model) =
            TierCProvider::parse_model_id(model_id).ok_or_else(|| {
                DoormanError::UpstreamShape(format!(
                    "Tier C model {model_id:?} missing recognised provider prefix \
                     (anthropic / gemini / openai)"
                ))
            })?;

        // 3. Provider config — endpoint + key.
        let endpoint = self
            .config
            .provider_endpoints
            .get(&provider)
            .cloned()
            .ok_or_else(|| {
                DoormanError::UpstreamShape(format!(
                    "Tier C provider {} has no configured endpoint",
                    provider.as_str()
                ))
            })?;
        let api_key = self
            .config
            .provider_api_keys
            .get(&provider)
            .cloned()
            .unwrap_or_default();

        // 4. Wire — OpenAI-compatible POST. (All three providers expose
        //    OpenAI-compatible shims in 2026; native per-provider
        //    request shapes can land in a follow-up if needed.)
        let body = OpenAiChatRequest {
            model: provider_model.to_string(),
            messages: req.messages.clone(),
            stream: req.stream,
            max_tokens: req.max_tokens,
            temperature: req.temperature,
        };
        let url = format!("{}/v1/chat/completions", endpoint.trim_end_matches('/'));
        debug!(
            target: "slm_doorman::tier::external",
            provider = provider.as_str(),
            model = provider_model,
            label,
            "tier-C request"
        );

        let started = Instant::now();
        let resp = self
            .http
            .post(&url)
            .bearer_auth(&api_key)
            .header("X-Foundry-Request-ID", req.request_id.to_string())
            .header("X-Foundry-Module-ID", req.module_id.as_str())
            .header("X-Foundry-Tier-C-Label", label)
            .json(&body)
            .send()
            .await?
            .error_for_status()?;
        let inference_ms = started.elapsed().as_millis() as u64;

        let body: OpenAiChatResponse = resp.json().await?;
        let content = body
            .choices
            .into_iter()
            .next()
            .map(|c| c.message.content)
            .ok_or_else(|| DoormanError::UpstreamShape("no choices in response".into()))?;
        let usage = body.usage.unwrap_or_default();
        let cost_usd =
            self.config
                .pricing
                .cost_usd(provider, usage.prompt_tokens, usage.completion_tokens);

        Ok(ComputeResponse {
            request_id: req.request_id,
            tier_used: Tier::External,
            model: model_id.to_string(),
            content,
            inference_ms,
            cost_usd,
            upstream_version: Some(provider.as_str().to_string()),
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
    #[serde(default)]
    usage: Option<OpenAiUsage>,
}

#[derive(Deserialize)]
struct OpenAiChatChoice {
    message: ChatMessage,
}

#[derive(Deserialize, Default)]
struct OpenAiUsage {
    #[serde(default)]
    prompt_tokens: u32,
    #[serde(default)]
    completion_tokens: u32,
}

fn is_false(b: &bool) -> bool {
    !*b
}

// `Arc` is currently unused at module level but exported — silence any
// clippy nag. The `Arc<dyn BearerTokenProvider>` shape lives in
// `tier::yoyo`; Tier C uses static keys instead.
#[allow(dead_code)]
fn _arc_kept_for_future_dyn_credential_providers() -> Option<Arc<()>> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use slm_core::{ChatMessage, ModuleId, RequestId};
    use std::str::FromStr;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    const TEST_ALLOWLIST: ExternalAllowlist =
        ExternalAllowlist::from_static(&["citation-grounding", "initial-graph-build"]);

    fn req_for(label: Option<&str>, model: Option<&str>) -> ComputeRequest {
        ComputeRequest {
            request_id: RequestId::new(),
            module_id: ModuleId::from_str("foundry").unwrap(),
            model: model.map(|s| s.to_string()),
            messages: vec![ChatMessage {
                role: "user".into(),
                content: "ping".into(),
            }],
            complexity: slm_core::Complexity::Low,
            tier_hint: Some(Tier::External),
            stream: false,
            max_tokens: Some(50),
            temperature: Some(0.0),
            sanitised_outbound: true,
            tier_c_label: label.map(|s| s.to_string()),
            grammar: None,
        }
    }

    fn ok_body() -> serde_json::Value {
        serde_json::json!({
            "choices": [
                { "message": { "role": "assistant", "content": "GROUNDED" } }
            ],
            "usage": { "prompt_tokens": 50, "completion_tokens": 20 }
        })
    }

    fn anthropic_client(server_uri: String, pricing: TierCPricing) -> ExternalTierClient {
        let mut endpoints = HashMap::new();
        endpoints.insert(TierCProvider::Anthropic, server_uri);
        let mut keys = HashMap::new();
        keys.insert(
            TierCProvider::Anthropic,
            "sk-ant-test-key-DO-NOT-USE-LIVE".to_string(),
        );
        ExternalTierClient::new(ExternalTierConfig {
            allowlist: TEST_ALLOWLIST,
            provider_endpoints: endpoints,
            provider_api_keys: keys,
            pricing,
        })
    }

    /// Happy path — allowlisted label + recognised provider + 200
    /// returns content and computes per-token cost.
    #[tokio::test]
    async fn happy_path_allowlist_match_returns_content_and_cost() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v1/chat/completions"))
            .and(header(
                "authorization",
                "Bearer sk-ant-test-key-DO-NOT-USE-LIVE",
            ))
            .and(header("x-foundry-module-id", "foundry"))
            .and(header("x-foundry-tier-c-label", "citation-grounding"))
            .respond_with(ResponseTemplate::new(200).set_body_json(ok_body()))
            .expect(1)
            .mount(&server)
            .await;

        // Anthropic Haiku ~ $0.25 / mtok in, $1.25 / mtok out (mock rates).
        let pricing = TierCPricing {
            anthropic_input_per_mtok_usd: 0.25,
            anthropic_output_per_mtok_usd: 1.25,
            ..Default::default()
        };
        let client = anthropic_client(server.uri(), pricing);
        let resp = client
            .complete(&req_for(
                Some("citation-grounding"),
                Some("anthropic:claude-haiku-4-5"),
            ))
            .await
            .expect("happy path 200");
        assert_eq!(resp.tier_used, Tier::External);
        assert_eq!(resp.model, "anthropic:claude-haiku-4-5");
        assert_eq!(resp.content, "GROUNDED");
        assert_eq!(resp.upstream_version.as_deref(), Some("anthropic"));
        // 50 in × $0.25/M + 20 out × $1.25/M = 0.0000125 + 0.000025 = 0.0000375
        assert!(
            (resp.cost_usd - 0.0000375).abs() < 1e-12,
            "expected $0.0000375, got ${}",
            resp.cost_usd
        );
    }

    /// Unallowlisted label — no network attempt; immediate
    /// `ExternalNotAllowlisted`.
    #[tokio::test]
    async fn unallowlisted_label_refuses_before_any_network_call() {
        // Mount nothing — wiremock would 404 if any call landed.
        let server = MockServer::start().await;
        let client = anthropic_client(server.uri(), TierCPricing::default());
        let err = client
            .complete(&req_for(
                Some("forbidden-label"),
                Some("anthropic:claude-haiku-4-5"),
            ))
            .await
            .expect_err("unallowlisted label must fail");
        match err {
            DoormanError::ExternalNotAllowlisted { label } => {
                assert_eq!(label, "forbidden-label");
            }
            other => panic!("expected ExternalNotAllowlisted, got {other:?}"),
        }
        // Verify no HTTP call landed by asserting the server saw zero
        // requests (no mounted Mock = received_requests is the audit).
        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(
            received.len(),
            0,
            "Doorman MUST NOT issue a network call before the allowlist check"
        );
    }

    /// Missing `tier_c_label` — same denial path; the absence is its
    /// own policy violation.
    #[tokio::test]
    async fn missing_label_refuses_before_any_network_call() {
        let server = MockServer::start().await;
        let client = anthropic_client(server.uri(), TierCPricing::default());
        let err = client
            .complete(&req_for(None, Some("anthropic:claude-haiku-4-5")))
            .await
            .expect_err("missing label must fail");
        assert!(matches!(err, DoormanError::ExternalNotAllowlisted { .. }));
        assert_eq!(
            server.received_requests().await.unwrap_or_default().len(),
            0
        );
    }

    /// Unknown provider prefix — surfaced as `UpstreamShape`; no
    /// network attempt.
    #[tokio::test]
    async fn unknown_provider_prefix_surfaces_upstream_shape() {
        let server = MockServer::start().await;
        let client = anthropic_client(server.uri(), TierCPricing::default());
        let err = client
            .complete(&req_for(
                Some("citation-grounding"),
                Some("not-a-real-provider:some-model"),
            ))
            .await
            .expect_err("unknown provider prefix must fail");
        assert!(matches!(err, DoormanError::UpstreamShape(_)));
        assert_eq!(
            server.received_requests().await.unwrap_or_default().len(),
            0
        );
    }

    /// Provider parsing — the unit method itself.
    #[test]
    fn provider_parses_known_prefixes() {
        assert_eq!(
            TierCProvider::parse_model_id("anthropic:claude-haiku-4-5"),
            Some((TierCProvider::Anthropic, "claude-haiku-4-5"))
        );
        assert_eq!(
            TierCProvider::parse_model_id("gemini:gemini-2-flash"),
            Some((TierCProvider::Gemini, "gemini-2-flash"))
        );
        assert_eq!(
            TierCProvider::parse_model_id("openai:gpt-4o-mini"),
            Some((TierCProvider::Openai, "gpt-4o-mini"))
        );
        assert_eq!(TierCProvider::parse_model_id("plain-model-no-prefix"), None);
        assert_eq!(TierCProvider::parse_model_id("unknown:provider"), None);
    }

    /// FOUNDRY_DEFAULT_ALLOWLIST contains the three labels the
    /// substrate-decision doc names.
    #[test]
    fn foundry_default_allowlist_contains_documented_labels() {
        assert!(FOUNDRY_DEFAULT_ALLOWLIST.contains("citation-grounding"));
        assert!(FOUNDRY_DEFAULT_ALLOWLIST.contains("initial-graph-build"));
        assert!(FOUNDRY_DEFAULT_ALLOWLIST.contains("entity-disambiguation"));
        assert!(!FOUNDRY_DEFAULT_ALLOWLIST.contains("not-allowlisted"));
    }

    /// TierCPricing arithmetic — independently testable.
    #[test]
    fn tier_c_pricing_arithmetic() {
        let p = TierCPricing {
            anthropic_input_per_mtok_usd: 1.0,
            anthropic_output_per_mtok_usd: 4.0,
            ..Default::default()
        };
        // 1M input + 0.5M output = $1 + $2 = $3
        assert!((p.cost_usd(TierCProvider::Anthropic, 1_000_000, 500_000) - 3.0).abs() < 1e-9);
        // Defaults stay zero
        assert_eq!(p.cost_usd(TierCProvider::Gemini, 999, 999), 0.0);
    }
}
