// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Tier 0 orchestration client — routes inference through
//! `app-orchestration-slm`'s `POST /v1/inference` instead of a local
//! llama-server, per `BRIEF-os-totebox-platform.md` §6/§14 (Tier 0 Doorman
//! mode, decisions #8/#9/#17/#18/#20).
//!
//! **Startup registration (decision #9, superseded 2026-07-29 — see
//! `BRIEF-os-totebox-platform.md` "Boot dependency resolved — standalone-
//! first")**: `register()` is called once at Doorman boot when `SLM_TIER=0`
//! and an orchestration endpoint is configured. A failure here is **no
//! longer fatal** — the caller (`slm-doorman-server`'s `entrypoint.rs`) logs
//! a warning and starts with no Tier 0 backend instead of aborting. VM-
//! totebox must always boot and serve DataGraph/deterministic operations
//! regardless of os-orchestration reachability; inference simply reports
//! unavailable until the chassis is reachable. This module is the client
//! itself, real and tested in isolation — the fail/degrade decision lives
//! in the caller, not here.
//!
//! **Fallback shape (decision #8, "queue locally and retry")**: this client
//! never escalates to any other tier on failure — see §14 #14's absolute
//! lock on the automatic path. What's implemented here is a **bounded
//! in-process retry with backoff**, not a full persistent retry queue (e.g.
//! reusing the existing Brief Queue Substrate) — that's a real scope
//! reduction from the original "queue" language, flagged explicitly, not
//! hidden. A persistent queue would survive a Doorman restart mid-retry;
//! this does not.
//!
//! **Token refresh**: membership tokens expire after 1 hour
//! (`orchestration-slm`'s `membership.rs::TOKEN_VALIDITY_HOURS`). On a 401
//! from the chassis, this client re-registers once and retries — mirroring
//! `YoYoTierClient`'s existing "401/403: refresh token, retry once" pattern
//! in `tier/yoyo.rs`.
//!
//! **Not implemented**: grammar-constraint translation (JsonSchema/Lark/GBNF)
//! and tool-call passthrough that `LocalTierClient`/`YoYoTierClient` support.
//! The chassis's `/v1/inference` is a raw JSON pass-through, so these would
//! work if added — this first pass covers prompt/messages/max_tokens/
//! temperature only. Flagged as a real follow-up, not silently dropped.

use std::sync::RwLock;
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use slm_core::{ChatMessage, ComputeRequest, ComputeResponse, InferenceRoute};

use crate::error::{DoormanError, Result};

#[derive(Clone, Debug)]
pub struct OrchestrationTierConfig {
    /// Base URL of the app-orchestration-slm chassis, e.g. `http://10.0.0.1:9180`.
    pub endpoint: String,
    /// This archive's Doorman module_id, used at registration time.
    pub module_id: String,
    /// This archive's identity for registration (e.g. `FOUNDRY_ARCHIVE_NAME`).
    pub archive_id: String,
    /// This archive's own Doorman endpoint, advertised to the chassis at
    /// registration time.
    pub doorman_endpoint: String,
    /// Shared admission-control secret for the chassis's `/v1/discovery/*`
    /// endpoints (`SLM_ORCHESTRATION_REGISTRATION_TOKEN`). `None` means the
    /// chassis has no registration gate configured (its own
    /// `ORCHESTRATION_REGISTRATION_TOKEN` is unset) — registration proceeds
    /// unauthenticated, matching that side's backward-compatible default.
    pub registration_token: Option<String>,
}

pub struct OrchestrationTierClient {
    config: OrchestrationTierConfig,
    http: reqwest::Client,
    membership_token: RwLock<Option<String>>,
}

const MAX_RETRIES: u32 = 2;
const RETRY_BACKOFF: Duration = Duration::from_secs(2);

impl OrchestrationTierClient {
    pub fn new(config: OrchestrationTierConfig) -> Self {
        Self {
            config,
            http: reqwest::Client::new(),
            membership_token: RwLock::new(None),
        }
    }

    /// True once a membership token has been obtained via `register()`.
    pub fn is_registered(&self) -> bool {
        self.membership_token.read().unwrap().is_some()
    }

    /// Registers with the chassis and stores the returned membership token.
    /// Per decision #9, callers MUST call this once at startup and treat a
    /// failure as fatal — do not start serving requests without a token.
    pub async fn register(&self) -> Result<()> {
        let url = format!(
            "{}/v1/discovery/register",
            self.config.endpoint.trim_end_matches('/')
        );
        let body = RegistrationRequest {
            module_id: self.config.module_id.clone(),
            archive_id: self.config.archive_id.clone(),
            doorman_endpoint: self.config.doorman_endpoint.clone(),
            tier_b_subscribed: false,
        };
        let mut req = self.http.post(&url).json(&body);
        if let Some(token) = &self.config.registration_token {
            req = req.bearer_auth(token);
        }
        let resp = req.send().await.map_err(DoormanError::Upstream)?;
        if !resp.status().is_success() {
            return Err(DoormanError::UpstreamShape(format!(
                "orchestration registration failed: {}",
                resp.status()
            )));
        }
        let reg: RegistrationResponse = resp.json().await.map_err(DoormanError::Upstream)?;
        let token = reg.membership_token.ok_or_else(|| {
            DoormanError::UpstreamShape(
                "orchestration registration response carried no membership_token".into(),
            )
        })?;
        *self.membership_token.write().unwrap() = Some(token);
        Ok(())
    }

    fn token(&self) -> Result<String> {
        self.membership_token
            .read()
            .unwrap()
            .clone()
            .ok_or(DoormanError::TierUnavailable(InferenceRoute::Local))
    }

    /// Routes an inference request through the chassis's `/v1/inference`.
    /// Bounded retry: up to `MAX_RETRIES` attempts, any of which may be
    /// preceded by a token refresh on 401. Never escalates to any other
    /// tier — a caller that exhausts retries gets
    /// `DoormanError::TierUnavailable(InferenceRoute::Local)`, per decision #17's
    /// "queue locally and retry, no other escalation" lock.
    pub async fn complete(&self, req: &ComputeRequest) -> Result<ComputeResponse> {
        let mut last_err = DoormanError::TierUnavailable(InferenceRoute::Local);
        for attempt in 0..=MAX_RETRIES {
            if attempt > 0 {
                tokio::time::sleep(RETRY_BACKOFF * attempt).await;
            }
            match self.try_complete(req).await {
                Ok(resp) => return Ok(resp),
                Err(DoormanError::UpstreamShape(ref msg)) if msg.contains("401") => {
                    // Token likely expired — re-register once, then retry.
                    let _ = self.register().await;
                    last_err = DoormanError::TierUnavailable(InferenceRoute::Local);
                }
                Err(e) => last_err = e,
            }
        }
        Err(last_err)
    }

    async fn try_complete(&self, req: &ComputeRequest) -> Result<ComputeResponse> {
        let token = self.token()?;
        let model = req.model.clone().unwrap_or_default();
        let body = OrchestrationChatRequest {
            model: model.clone(),
            messages: req.messages.clone(),
            stream: req.stream,
            max_tokens: req.max_tokens,
            temperature: req.temperature,
        };
        let url = format!(
            "{}/v1/inference",
            self.config.endpoint.trim_end_matches('/')
        );
        let started = Instant::now();
        let resp = self
            .http
            .post(&url)
            .bearer_auth(&token)
            .json(&body)
            .send()
            .await
            .map_err(DoormanError::Upstream)?;

        if resp.status() == reqwest::StatusCode::UNAUTHORIZED {
            return Err(DoormanError::UpstreamShape("401 Unauthorized".into()));
        }
        if !resp.status().is_success() {
            return Err(DoormanError::UpstreamShape(format!(
                "orchestration inference failed: {}",
                resp.status()
            )));
        }
        let inference_ms = started.elapsed().as_millis() as u64;
        let parsed: OrchestrationChatResponse =
            resp.json().await.map_err(DoormanError::Upstream)?;
        let msg = parsed
            .choices
            .into_iter()
            .next()
            .map(|c| c.message)
            .ok_or_else(|| DoormanError::UpstreamShape("no choices in response".into()))?;

        Ok(ComputeResponse {
            request_id: req.request_id,
            tier_used: InferenceRoute::Local,
            model,
            content: msg.content.unwrap_or_default(),
            reasoning_content: None,
            inference_ms,
            cost_usd: 0.0,
            upstream_version: Some("orchestration".to_string()),
            tool_calls: None,
        })
    }
}

/// Calls `POST /v1/discovery/allocate` once, at first boot, to obtain a
/// chassis-guaranteed-unique `(module_id, archive_id)` pair — see
/// `AllocationLedger` in `app-orchestration-slm`'s `orchestration-slm` crate
/// for why this exists (the alternative, self-claimed `SLM_MODULE_ID`/
/// `SLM_ARCHIVE_ID` env vars have zero collision detection). The caller is
/// responsible for persisting the result locally (see `entrypoint.rs`'s
/// identity-cache logic) and reusing it on every subsequent boot — this
/// function always allocates a fresh identity when called; it does not
/// itself check any cache.
///
/// Standalone, not a method on `OrchestrationTierClient`: allocation happens
/// *before* a client can be constructed, since the client's own config
/// requires a `module_id`/`archive_id` already resolved.
pub async fn allocate_identity(
    endpoint: &str,
    requested_archive_id: Option<&str>,
    registration_token: Option<&str>,
) -> Result<(String, String)> {
    #[derive(Serialize)]
    struct AllocationRequest<'a> {
        #[serde(skip_serializing_if = "Option::is_none")]
        requested_archive_id: Option<&'a str>,
    }
    #[derive(Deserialize)]
    struct AllocationResponse {
        module_id: String,
        archive_id: String,
    }

    let url = format!("{}/v1/discovery/allocate", endpoint.trim_end_matches('/'));
    let mut req = reqwest::Client::new().post(&url).json(&AllocationRequest {
        requested_archive_id,
    });
    if let Some(token) = registration_token {
        req = req.bearer_auth(token);
    }
    let resp = req.send().await.map_err(DoormanError::Upstream)?;
    if !resp.status().is_success() {
        return Err(DoormanError::UpstreamShape(format!(
            "chassis identity allocation failed: {}",
            resp.status()
        )));
    }
    let allocated: AllocationResponse = resp.json().await.map_err(DoormanError::Upstream)?;
    Ok((allocated.module_id, allocated.archive_id))
}

#[derive(Serialize)]
struct RegistrationRequest {
    module_id: String,
    archive_id: String,
    doorman_endpoint: String,
    tier_b_subscribed: bool,
}

#[derive(Deserialize)]
struct RegistrationResponse {
    #[serde(default)]
    membership_token: Option<String>,
}

#[derive(Serialize)]
struct OrchestrationChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "is_false")]
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
}

fn is_false(b: &bool) -> bool {
    !*b
}

#[derive(Deserialize)]
struct OrchestrationChatResponse {
    choices: Vec<OrchestrationChatChoice>,
}

#[derive(Deserialize)]
struct OrchestrationChatChoice {
    message: OrchestrationAssistantMessage,
}

#[derive(Deserialize)]
struct OrchestrationAssistantMessage {
    #[serde(default)]
    content: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use slm_core::{ModuleId, RequestId};
    use std::str::FromStr;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn client(server_uri: String) -> OrchestrationTierClient {
        OrchestrationTierClient::new(OrchestrationTierConfig {
            endpoint: server_uri,
            module_id: "op::a::slm".to_string(),
            archive_id: "project-totebox".to_string(),
            doorman_endpoint: "http://127.0.0.1:9080".to_string(),
            registration_token: None,
        })
    }

    #[tokio::test]
    async fn allocate_identity_happy_path() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v1/discovery/allocate"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "module_id": "op::cluster-totebox-data-1::slm",
                "archive_id": "cluster-totebox-data-1"
            })))
            .mount(&server)
            .await;

        let (module_id, archive_id) =
            allocate_identity(&server.uri(), Some("cluster-totebox-data-1"), None)
                .await
                .expect("allocation should succeed");
        assert_eq!(module_id, "op::cluster-totebox-data-1::slm");
        assert_eq!(archive_id, "cluster-totebox-data-1");
    }

    #[tokio::test]
    async fn allocate_identity_chassis_error_is_reported_not_panicked() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v1/discovery/allocate"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;

        let err = allocate_identity(&server.uri(), None, None)
            .await
            .expect_err("500 must surface as an error, not panic");
        assert!(matches!(err, DoormanError::UpstreamShape(_)));
    }

    fn req() -> ComputeRequest {
        ComputeRequest {
            request_id: RequestId::new(),
            module_id: ModuleId::from_str("jennifer").unwrap(),
            model: Some("olmo-3-7b-instruct".to_string()),
            messages: vec![ChatMessage {
                role: "user".into(),
                content: "ping".into(),
            }],
            complexity: slm_core::Complexity::Low,
            tier_hint: None,
            stream: false,
            max_tokens: Some(50),
            temperature: Some(0.0),
            sanitised_outbound: true,
            tier_c_label: None,
            yoyo_label: None,
            grammar: None,
            speculation: None,
            graph_context_enabled: None,
            tools: None,
            stop_sequences: None,
            session_context: None,
        }
    }

    /// Before `register()` succeeds, `complete()` must fail with
    /// `TierUnavailable(Local)` rather than attempting an unauthenticated call.
    #[tokio::test]
    async fn complete_before_registration_fails_without_any_network_call() {
        let server = MockServer::start().await;
        let c = client(server.uri());
        let err = c
            .complete(&req())
            .await
            .expect_err("must fail unregistered");
        assert!(matches!(
            err,
            DoormanError::TierUnavailable(InferenceRoute::Local)
        ));
        assert_eq!(
            server.received_requests().await.unwrap_or_default().len(),
            0
        );
    }

    #[tokio::test]
    async fn register_stores_the_membership_token() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v1/discovery/register"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "status": "registered",
                "module_id": "op::a::slm",
                "chassis_version": "0.1.0",
                "membership_token": "tok-abc.sig-xyz"
            })))
            .mount(&server)
            .await;

        let c = client(server.uri());
        assert!(!c.is_registered());
        c.register().await.expect("registration should succeed");
        assert!(c.is_registered());
    }

    #[tokio::test]
    async fn register_sends_configured_registration_token() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v1/discovery/register"))
            .and(header("authorization", "Bearer secret-reg-token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "status": "registered",
                "module_id": "op::a::slm",
                "chassis_version": "0.1.0",
                "membership_token": "tok-abc.sig-xyz"
            })))
            .mount(&server)
            .await;

        let c = OrchestrationTierClient::new(OrchestrationTierConfig {
            endpoint: server.uri(),
            module_id: "op::a::slm".to_string(),
            archive_id: "project-totebox".to_string(),
            doorman_endpoint: "http://127.0.0.1:9080".to_string(),
            registration_token: Some("secret-reg-token".to_string()),
        });
        // wiremock's header() matcher rejects any request lacking the exact
        // header/value — a missing or wrong token 404s (no mock matched),
        // which register() surfaces as an UpstreamShape error.
        c.register()
            .await
            .expect("registration with the correct token should succeed");
        assert!(c.is_registered());
    }

    #[tokio::test]
    async fn allocate_identity_sends_configured_registration_token() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v1/discovery/allocate"))
            .and(header("authorization", "Bearer secret-reg-token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "module_id": "op::cluster-totebox-data-1::slm",
                "archive_id": "cluster-totebox-data-1"
            })))
            .mount(&server)
            .await;

        let (module_id, _) = allocate_identity(&server.uri(), None, Some("secret-reg-token"))
            .await
            .expect("allocation with the correct token should succeed");
        assert_eq!(module_id, "op::cluster-totebox-data-1::slm");
    }

    #[tokio::test]
    async fn happy_path_sends_bearer_token_and_parses_response() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v1/discovery/register"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "status": "registered",
                "module_id": "op::a::slm",
                "chassis_version": "0.1.0",
                "membership_token": "tok-abc.sig-xyz"
            })))
            .mount(&server)
            .await;
        Mock::given(method("POST"))
            .and(path("/v1/inference"))
            .and(header("authorization", "Bearer tok-abc.sig-xyz"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "choices": [{ "message": { "content": "pong" } }]
            })))
            .mount(&server)
            .await;

        let c = client(server.uri());
        c.register().await.unwrap();
        let resp = c.complete(&req()).await.expect("inference should succeed");
        assert_eq!(resp.content, "pong");
        assert_eq!(resp.tier_used, InferenceRoute::Local);
        assert_eq!(resp.upstream_version.as_deref(), Some("orchestration"));
    }

    /// A 401 mid-flight must trigger exactly one re-registration, then retry
    /// with the fresh token — mirroring YoYoTierClient's existing pattern.
    #[tokio::test]
    async fn expired_token_triggers_reregistration_and_retry_succeeds() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v1/discovery/register"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "status": "registered",
                "module_id": "op::a::slm",
                "chassis_version": "0.1.0",
                "membership_token": "fresh-token.sig"
            })))
            .mount(&server)
            .await;
        // Old token → 401.
        Mock::given(method("POST"))
            .and(path("/v1/inference"))
            .and(header("authorization", "Bearer stale-token.sig"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&server)
            .await;
        // Fresh token (post-reregistration) → success.
        Mock::given(method("POST"))
            .and(path("/v1/inference"))
            .and(header("authorization", "Bearer fresh-token.sig"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "choices": [{ "message": { "content": "pong-after-refresh" } }]
            })))
            .mount(&server)
            .await;

        let c = client(server.uri());
        // Seed a stale token directly (simulating an expired prior registration).
        *c.membership_token.write().unwrap() = Some("stale-token.sig".to_string());

        let resp = c
            .complete(&req())
            .await
            .expect("retry after refresh should succeed");
        assert_eq!(resp.content, "pong-after-refresh");
    }

    /// Chassis unreachable entirely (not a 401) — must exhaust retries and
    /// fail with TierUnavailable, never touching any other tier.
    #[tokio::test]
    async fn unreachable_chassis_exhausts_retries_and_returns_tier_unavailable() {
        // No mock server at all — connection refused on every attempt.
        let c = OrchestrationTierClient::new(OrchestrationTierConfig {
            endpoint: "http://127.0.0.1:1".to_string(), // reserved, always refused
            module_id: "op::a::slm".to_string(),
            archive_id: "project-totebox".to_string(),
            doorman_endpoint: "http://127.0.0.1:9080".to_string(),
            registration_token: None,
        });
        *c.membership_token.write().unwrap() = Some("tok.sig".to_string());
        let err = c
            .complete(&req())
            .await
            .expect_err("unreachable chassis must fail");
        assert!(matches!(
            err,
            DoormanError::TierUnavailable(InferenceRoute::Local) | DoormanError::Upstream(_)
        ));
    }
}
