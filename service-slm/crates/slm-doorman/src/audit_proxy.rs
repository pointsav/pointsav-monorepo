// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Audit proxy client — Doorman-mediated relay to external providers.
//!
//! `AuditProxyClient` is the implementation surface for
//! `POST /v1/audit/proxy`. The caller (e.g., project-language editorial
//! gateway) holds no provider API keys; it submits a structured request
//! to the Doorman, which authenticates with the provider, captures the
//! full request + response + cost into the audit ledger, and returns the
//! provider's response.
//!
//! This module is intentionally parallel to `tier::external` rather than
//! shared with it, to avoid coupling the audit_proxy relay surface to the
//! Tier C compute routing surface. Step 3 (purpose allowlist) and step 4
//! (audit_capture endpoints) can evolve independently.
//!
//! Operator cost guardrail: no live API calls in tests. All testing via
//! `wiremock`. No provider-SDK crates (`anthropic`, `google-generative-ai`,
//! `openai`). Per Master 2026-04-26 brief Answer 3 and operator guardrail
//! carried forward through B4.

use std::collections::HashMap;
use std::time::Instant;

use serde::{Deserialize, Serialize};
use slm_core::{AuditProxyRequest, AuditProxyResponse, AuditUsage, ChatMessage};
use tracing::debug;

use crate::error::{DoormanError, Result};
use crate::tier::{TierCPricing, TierCProvider};

/// Compile-time allowlist of purposes permitted to use `POST /v1/audit/proxy`.
///
/// Mirrors `ExternalAllowlist` from `tier::external` exactly — same shape,
/// same `&'static [&'static str]` backing, same const-fn constructor, same
/// `is_allowed` method name. Different list (purposes vs Tier C task labels)
/// but the same family of types.
///
/// **Empty-list semantic: fail-closed.** An empty `AuditProxyPurposeAllowlist`
/// rejects ALL purpose values. This is intentional: an allowlist without any
/// entries means "no audit_proxy calls are authorised on this deployment",
/// which is a stricter, safer default than "allow everything". Use
/// `FOUNDRY_DEFAULT_PURPOSE_ALLOWLIST` for the standard four documented
/// purposes, or `from_static` to provide a deployment-specific override.
///
/// Operator extends by editing `FOUNDRY_DEFAULT_PURPOSE_ALLOWLIST` (or a
/// per-deployment override constant) and recompiling. Runtime extension is
/// not supported — per `conventions/llm-substrate-decision.md` the allowlist
/// is compile-time to make extensions visible in code review.
#[derive(Clone, Copy, Debug)]
pub struct AuditProxyPurposeAllowlist {
    purposes: &'static [&'static str],
}

impl AuditProxyPurposeAllowlist {
    /// Empty allowlist — no `audit_proxy` purpose permitted. Default when
    /// no explicit allowlist is provided.
    ///
    /// An empty allowlist is **fail-closed**: every purpose is denied.
    /// This is stricter than "allow all" and is the correct posture for
    /// a deployment that has not opted into audit_proxy calls.
    pub const EMPTY: Self = Self { purposes: &[] };

    /// Build from a `&'static [&'static str]` slice. Used by both the
    /// hardcoded `FOUNDRY_DEFAULT_PURPOSE_ALLOWLIST` below and any per-
    /// deployment override (still compile-time per the doctrine).
    pub const fn from_static(purposes: &'static [&'static str]) -> Self {
        Self { purposes }
    }

    /// Return `true` if `purpose` is present in the allowlist.
    pub fn is_allowed(&self, purpose: &str) -> bool {
        self.purposes.contains(&purpose)
    }

    pub fn is_empty(&self) -> bool {
        self.purposes.is_empty()
    }

    pub fn as_slice(&self) -> &'static [&'static str] {
        self.purposes
    }
}

impl Default for AuditProxyPurposeAllowlist {
    fn default() -> Self {
        Self::EMPTY
    }
}

/// Foundry's default purpose allowlist for `POST /v1/audit/proxy`.
///
/// The four entries are the documented purposes from
/// `~/Foundry/conventions/llm-substrate-decision.md` §"Three compute tiers":
///   - `editorial-refinement`   — project-language gateway refining drafts
///   - `citation-grounding`     — verifying citations against external sources
///   - `entity-disambiguation`  — resolving named entities
///   - `initial-graph-build`    — bootstrapping a fresh service-content graph
///
/// An unenumerated purpose is rejected (403 FORBIDDEN). This prevents
/// ad-hoc external calls that would slip past the audit-trail intent, and
/// prevents auto-promotion to live Tier C without explicit operator awareness
/// (per v0.0.10 hard rule #4).
pub const FOUNDRY_DEFAULT_PURPOSE_ALLOWLIST: AuditProxyPurposeAllowlist =
    AuditProxyPurposeAllowlist::from_static(&[
        "editorial-refinement",
        "citation-grounding",
        "entity-disambiguation",
        "initial-graph-build",
    ]);

/// Configuration for the audit proxy client.
///
/// Mirrors `ExternalTierConfig` shape intentionally: the env-var contract
/// (SLM_TIER_C_*) is shared between `ExternalTierClient` and
/// `AuditProxyClient`. Both read the same per-provider endpoint + key env
/// vars at startup; having one set of env vars avoids per-surface
/// configuration divergence.
#[derive(Clone, Debug)]
pub struct AuditProxyConfig {
    /// Per-provider base URL (no trailing slash). Map key is the provider.
    pub provider_endpoints: HashMap<TierCProvider, String>,
    /// Per-provider API key. Empty string when not configured (the
    /// dev / community-tier default — Doorman refuses any actual dispatch
    /// when the key is empty).
    pub provider_api_keys: HashMap<TierCProvider, String>,
    /// Per-provider per-token pricing for cost computation.
    pub pricing: TierCPricing,
    /// Purpose allowlist. Requests with a purpose not in this list are
    /// rejected BEFORE any upstream provider call.
    ///
    /// **Empty list = fail-closed**: all purposes are denied.
    /// Default: `FOUNDRY_DEFAULT_PURPOSE_ALLOWLIST` (four documented purposes).
    pub purpose_allowlist: AuditProxyPurposeAllowlist,
}

impl Default for AuditProxyConfig {
    fn default() -> Self {
        Self {
            provider_endpoints: HashMap::new(),
            provider_api_keys: HashMap::new(),
            pricing: TierCPricing::default(),
            purpose_allowlist: FOUNDRY_DEFAULT_PURPOSE_ALLOWLIST,
        }
    }
}

/// Doorman-mediated relay client for `POST /v1/audit/proxy`.
///
/// `AuditProxyClient` is constructed at startup from env vars and held
/// in `AppState`. When present (`Some`), the audit_proxy handler delegates
/// the upstream call to this client. When absent (`None`), the handler
/// returns 503 with an "unconfigured" message.
pub struct AuditProxyClient {
    http: reqwest::Client,
    config: AuditProxyConfig,
}

impl AuditProxyClient {
    pub fn new(config: AuditProxyConfig) -> Self {
        Self {
            http: reqwest::Client::new(),
            config,
        }
    }

    /// Relay a validated `AuditProxyRequest` to the upstream provider.
    ///
    /// The `audit_id` is supplied by the handler (generated before the
    /// ledger stub write); this method does not generate it. The returned
    /// `AuditProxyResponse` carries the `audit_id` echoed back.
    ///
    /// Two-entry ledger design (enforced by the handler, not here):
    ///   1. Stub entry written BEFORE this call (already done by the handler
    ///      in PS.4 step 1 logic — preserved in step 2).
    ///   2. Full entry written AFTER this call returns (Ok or Err).
    ///
    /// The relay method is pure: it does not write to the ledger itself.
    /// Keeping ledger writes in the handler preserves the paper trail even
    /// if the relay client panics or returns a network error.
    pub async fn relay(
        &self,
        req: &AuditProxyRequest,
        audit_id: &str,
    ) -> Result<AuditProxyResponse> {
        // 1. Parse provider string. The handler already validated the
        //    provider string, but we do a defensive parse here so the
        //    relay method is self-contained.
        let provider_lc = req.provider.to_ascii_lowercase();
        let provider = match provider_lc.as_str() {
            "anthropic" => TierCProvider::Anthropic,
            "gemini" => TierCProvider::Gemini,
            "openai" => TierCProvider::Openai,
            _ => {
                return Err(DoormanError::AuditProxyInvalidProvider {
                    provider: req.provider.clone(),
                });
            }
        };

        // 2. Look up endpoint + key. Missing either → provider unconfigured.
        let endpoint = self
            .config
            .provider_endpoints
            .get(&provider)
            .cloned()
            .ok_or_else(|| DoormanError::AuditProxyProviderUnavailable {
                provider: provider.as_str().to_string(),
            })?;
        let api_key = self
            .config
            .provider_api_keys
            .get(&provider)
            .cloned()
            .ok_or_else(|| DoormanError::AuditProxyProviderUnavailable {
                provider: provider.as_str().to_string(),
            })?;

        // 3. Build OpenAI-compatible request body. All three providers expose
        //    OpenAI-compatible shims in 2026 (matching the precedent set by
        //    ExternalTierClient::complete() which uses the same wire format).
        let body = OpenAiChatRequest {
            model: req.model.clone(),
            messages: req.messages.clone(),
            stream: false,
            max_tokens: req.max_tokens,
            temperature: req.temperature,
        };
        let url = format!("{}/v1/chat/completions", endpoint.trim_end_matches('/'));

        debug!(
            target: "slm_doorman::audit_proxy",
            provider = provider.as_str(),
            model = %req.model,
            audit_id,
            purpose = %req.purpose,
            "audit_proxy relay"
        );

        // 4. Build provider-specific headers and POST.
        let started = Instant::now();
        let raw_resp = self
            .http
            .post(&url)
            .bearer_auth(&api_key)
            .json(&body)
            .send()
            .await?;

        // Non-2xx upstream: surface as UpstreamShape with status + body preview.
        let status = raw_resp.status();
        if !status.is_success() {
            let preview = raw_resp
                .text()
                .await
                .unwrap_or_else(|_| "<unreadable>".into());
            return Err(DoormanError::UpstreamShape(format!(
                "audit_proxy upstream returned {status}: {preview}"
            )));
        }

        let _inference_ms = started.elapsed().as_millis() as u64;

        // 5. Parse the response. Same OpenAI-compatible shape.
        let resp_body: OpenAiChatResponse = raw_resp.json().await?;
        let content = resp_body
            .choices
            .into_iter()
            .next()
            .map(|c| c.message.content)
            .ok_or_else(|| DoormanError::UpstreamShape("no choices in response".into()))?;
        let usage = resp_body.usage.unwrap_or_default();
        let cost_usd =
            self.config
                .pricing
                .cost_usd(provider, usage.prompt_tokens, usage.completion_tokens);

        Ok(AuditProxyResponse {
            audit_id: audit_id.to_string(),
            caller_request_id: req.caller_request_id.clone(),
            content,
            usage: AuditUsage {
                prompt_tokens: usage.prompt_tokens,
                completion_tokens: usage.completion_tokens,
                cost_usd,
            },
        })
    }

    /// Return the inference_ms for timing capture. This is extracted from
    /// relay() for test purposes — in production relay() returns the
    /// AuditProxyResponse which carries usage.
    pub fn config(&self) -> &AuditProxyConfig {
        &self.config
    }
}

// ---------------------------------------------------------------------------
// Private wire shapes — OpenAI-compatible JSON
// ---------------------------------------------------------------------------

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn ok_body(prompt_tokens: u32, completion_tokens: u32) -> serde_json::Value {
        serde_json::json!({
            "choices": [
                { "message": { "role": "assistant", "content": "audit-relay-response" } }
            ],
            "usage": {
                "prompt_tokens": prompt_tokens,
                "completion_tokens": completion_tokens
            }
        })
    }

    fn anthropic_client(server_uri: String, pricing: TierCPricing) -> AuditProxyClient {
        let mut endpoints = HashMap::new();
        endpoints.insert(TierCProvider::Anthropic, server_uri);
        let mut keys = HashMap::new();
        keys.insert(
            TierCProvider::Anthropic,
            "sk-ant-test-DO-NOT-USE-LIVE".to_string(),
        );
        AuditProxyClient::new(AuditProxyConfig {
            provider_endpoints: endpoints,
            provider_api_keys: keys,
            pricing,
            purpose_allowlist: FOUNDRY_DEFAULT_PURPOSE_ALLOWLIST,
        })
    }

    fn sample_request() -> AuditProxyRequest {
        use slm_core::ChatMessage;
        AuditProxyRequest {
            module_id: "woodfine".to_string(),
            purpose: "editorial-grammar-check".to_string(),
            provider: "anthropic".to_string(),
            model: "claude-opus-4-7".to_string(),
            messages: vec![ChatMessage {
                role: "user".into(),
                content: "Please review this paragraph.".into(),
            }],
            max_tokens: Some(100),
            temperature: None,
            caller_request_id: Some("caller-abc".to_string()),
        }
    }

    #[tokio::test]
    async fn relay_happy_path_returns_content_and_cost() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v1/chat/completions"))
            .respond_with(ResponseTemplate::new(200).set_body_json(ok_body(50, 20)))
            .expect(1)
            .mount(&server)
            .await;

        let pricing = TierCPricing {
            anthropic_input_per_mtok_usd: 0.25,
            anthropic_output_per_mtok_usd: 1.25,
            ..Default::default()
        };
        let client = anthropic_client(server.uri(), pricing);
        let resp = client
            .relay(&sample_request(), "audit-id-test-001")
            .await
            .expect("relay happy path");

        assert_eq!(resp.audit_id, "audit-id-test-001");
        assert_eq!(resp.caller_request_id.as_deref(), Some("caller-abc"));
        assert_eq!(resp.content, "audit-relay-response");
        assert_eq!(resp.usage.prompt_tokens, 50);
        assert_eq!(resp.usage.completion_tokens, 20);
        // 50 in × $0.25/M + 20 out × $1.25/M = 0.0000125 + 0.000025 = 0.0000375
        assert!(
            (resp.usage.cost_usd - 0.0000375).abs() < 1e-12,
            "expected $0.0000375, got ${}",
            resp.usage.cost_usd
        );
    }

    #[tokio::test]
    async fn relay_unconfigured_provider_returns_provider_unavailable() {
        // Empty config — no providers configured.
        let client = AuditProxyClient::new(AuditProxyConfig::default());
        let err = client
            .relay(&sample_request(), "audit-id-test-002")
            .await
            .expect_err("unconfigured provider must fail");
        match err {
            DoormanError::AuditProxyProviderUnavailable { provider } => {
                assert_eq!(provider, "anthropic");
            }
            other => panic!("expected AuditProxyProviderUnavailable, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn relay_upstream_500_returns_upstream_shape_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v1/chat/completions"))
            .respond_with(ResponseTemplate::new(500).set_body_string("internal server error"))
            .expect(1)
            .mount(&server)
            .await;

        let client = anthropic_client(server.uri(), TierCPricing::default());
        let err = client
            .relay(&sample_request(), "audit-id-test-003")
            .await
            .expect_err("upstream 500 must fail");
        assert!(
            matches!(err, DoormanError::UpstreamShape(_)),
            "upstream 500 must produce UpstreamShape; got {err:?}"
        );
    }
}
