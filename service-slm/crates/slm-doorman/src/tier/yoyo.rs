// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Tier B — Yo-Yo cloud burst over the contract at
//! `infrastructure/slm-yoyo/CONTRACT.md`.
//!
//! Implements the **client** side of the Yo-Yo HTTP API:
//!
//! - `POST /v1/chat/completions` with `Authorization: Bearer <token>`
//!   plus the four `X-Foundry-*` headers
//!   (`Request-ID`, `Module-ID`, `Contract-Version`, `Complexity`)
//! - Retry-on-503 honouring `Retry-After` (one retry, capped at 60 s)
//! - Auth refresh on 401 / 403 (one retry against a fresh token)
//! - MAJOR contract mismatch on 410 (no retry; refuse loudly)
//! - Capture of `X-Foundry-Inference-Ms` and `X-Foundry-Yoyo-Version`
//!   response headers for the audit ledger
//!
//! Per operator direction relayed via Master 2026-04-26: this code is
//! mock-tested only — no `tofu apply`, no live HTTP, no real
//! bearer-token consumption against any provider. Live Yo-Yo
//! deployments are a separate Master-scope decision with explicit
//! cost-cap configuration.

use std::sync::Arc;
use std::time::{Duration, Instant};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use slm_core::{ChatMessage, ComputeRequest, ComputeResponse, Tier};
use tracing::{debug, warn};

use crate::error::{DoormanError, Result};

/// Bearer-token source for Tier B requests. Real implementations
/// (GCP Workload Identity, RunPod / Modal API key from Secret
/// Manager, customer mTLS / shared secret) implement this trait;
/// `StaticBearer` covers tests and dev-loop usage.
#[async_trait]
pub trait BearerTokenProvider: Send + Sync + std::fmt::Debug {
    /// Returns the current bearer token. Implementations may cache
    /// and refresh proactively.
    async fn token(&self) -> Result<String>;

    /// Forces a token refresh after a 401 / 403 response. Returns
    /// the freshly obtained token.
    async fn refresh(&self) -> Result<String>;
}

/// Static bearer token. Used by the Doorman binary when
/// `SLM_YOYO_BEARER` env var is set, and by the unit tests below.
#[derive(Clone, Debug)]
pub struct StaticBearer {
    token: String,
}

impl StaticBearer {
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
        }
    }
}

#[async_trait]
impl BearerTokenProvider for StaticBearer {
    async fn token(&self) -> Result<String> {
        Ok(self.token.clone())
    }

    async fn refresh(&self) -> Result<String> {
        Ok(self.token.clone())
    }
}

#[derive(Clone, Debug)]
pub struct YoYoTierConfig {
    /// Base URL of the Yo-Yo node (e.g. `https://yoyo-foundry.run.app`).
    pub endpoint: String,
    /// Default model identifier. Yo-Yo runs `Olmo-3-1125-32B-Think`
    /// (canonical Allen AI name; see substrate decision).
    pub default_model: String,
    /// Contract version this client speaks. Sent in
    /// `X-Foundry-Contract-Version` per CONTRACT.md.
    pub contract_version: String,
}

impl Default for YoYoTierConfig {
    fn default() -> Self {
        Self {
            endpoint: String::new(),
            default_model: "Olmo-3-1125-32B-Think".to_string(),
            contract_version: crate::YOYO_CONTRACT_VERSION.to_string(),
        }
    }
}

pub struct YoYoTierClient {
    config: YoYoTierConfig,
    http: reqwest::Client,
    bearer: Arc<dyn BearerTokenProvider>,
}

impl YoYoTierClient {
    pub fn new(config: YoYoTierConfig, bearer: Arc<dyn BearerTokenProvider>) -> Self {
        Self {
            config,
            http: reqwest::Client::new(),
            bearer,
        }
    }

    pub fn endpoint(&self) -> &str {
        &self.config.endpoint
    }

    pub fn contract_version(&self) -> &str {
        &self.config.contract_version
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
        debug!(target: "slm_doorman::tier::yoyo", %url, %model, "tier-B request");

        let started = Instant::now();
        let resp = self.send_with_retries(&url, &body, req).await?;

        // Capture Foundry response metadata before consuming the body.
        let inference_ms = resp
            .headers()
            .get("x-foundry-inference-ms")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or_else(|| started.elapsed().as_millis() as u64);
        let upstream_version = resp
            .headers()
            .get("x-foundry-yoyo-version")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        let body: OpenAiChatResponse = resp.json().await?;
        let content = body
            .choices
            .into_iter()
            .next()
            .map(|c| c.message.content)
            .ok_or_else(|| DoormanError::UpstreamShape("no choices in response".into()))?;

        Ok(ComputeResponse {
            request_id: req.request_id,
            tier_used: Tier::Yoyo,
            model,
            content,
            inference_ms,
            // CONTRACT.md does not carry a cost field on the wire; the
            // Doorman computes Tier B cost from inference_ms × per-
            // provider hourly rate. That `PricingConfig` lands in a
            // follow-up; for B1+B2 we leave 0 so the audit-ledger
            // entry is correct (cost is unknown rather than mis-
            // attributed).
            cost_usd: 0.0,
            upstream_version,
        })
    }

    /// Send one request with up to one retry. Retry policy:
    /// - 200..=299: success
    /// - 503 + Retry-After: sleep up to min(Retry-After, 60s) then retry once
    /// - 401 / 403: refresh token, retry once with fresh token
    /// - 410: contract MAJOR mismatch — refuse, no retry
    /// - other: surface as `UpstreamShape`
    async fn send_with_retries(
        &self,
        url: &str,
        body: &OpenAiChatRequest,
        req: &ComputeRequest,
    ) -> Result<reqwest::Response> {
        let token = self.bearer.token().await?;
        let resp = self.send_once(url, body, req, &token).await?;
        let status = resp.status().as_u16();
        match status {
            200..=299 => Ok(resp),
            503 => {
                let retry_after = resp
                    .headers()
                    .get("retry-after")
                    .and_then(|v| v.to_str().ok())
                    .and_then(|s| s.parse::<u64>().ok())
                    .unwrap_or(1)
                    .min(60);
                warn!(
                    target: "slm_doorman::tier::yoyo",
                    retry_after_s = retry_after,
                    "503 cold start; sleeping then retrying once"
                );
                tokio::time::sleep(Duration::from_secs(retry_after)).await;
                let resp2 = self.send_once(url, body, req, &token).await?;
                if resp2.status().is_success() {
                    Ok(resp2)
                } else {
                    Err(DoormanError::UpstreamShape(format!(
                        "retry after 503 returned {}",
                        resp2.status()
                    )))
                }
            }
            401 | 403 => {
                warn!(
                    target: "slm_doorman::tier::yoyo",
                    status,
                    "auth failure; refreshing token and retrying once"
                );
                let new_token = self.bearer.refresh().await?;
                let resp2 = self.send_once(url, body, req, &new_token).await?;
                if resp2.status().is_success() {
                    Ok(resp2)
                } else {
                    Err(DoormanError::UpstreamShape(format!(
                        "retry after auth-refresh returned {}",
                        resp2.status()
                    )))
                }
            }
            410 => Err(DoormanError::ContractMajorMismatch {
                remote_status: 410,
                doorman_version: crate::YOYO_CONTRACT_VERSION,
            }),
            _ => {
                let body_preview = resp
                    .text()
                    .await
                    .unwrap_or_else(|e| format!("<body read failed: {e}>"));
                Err(DoormanError::UpstreamShape(format!(
                    "unexpected status {status}: {}",
                    body_preview.chars().take(200).collect::<String>()
                )))
            }
        }
    }

    async fn send_once(
        &self,
        url: &str,
        body: &OpenAiChatRequest,
        req: &ComputeRequest,
        token: &str,
    ) -> Result<reqwest::Response> {
        let resp = self
            .http
            .post(url)
            .bearer_auth(token)
            .header("X-Foundry-Request-ID", req.request_id.to_string())
            .header("X-Foundry-Module-ID", req.module_id.as_str())
            .header("X-Foundry-Contract-Version", &self.config.contract_version)
            .header("X-Foundry-Complexity", req.complexity.as_str())
            .json(body)
            .send()
            .await?;
        Ok(resp)
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
    use slm_core::{ChatMessage, ComputeRequest, ModuleId, RequestId};
    use std::str::FromStr;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, Request, ResponseTemplate};

    fn req() -> ComputeRequest {
        ComputeRequest {
            request_id: RequestId::new(),
            module_id: ModuleId::from_str("foundry").unwrap(),
            model: Some("Olmo-3-1125-32B-Think".into()),
            messages: vec![ChatMessage {
                role: "user".into(),
                content: "ping".into(),
            }],
            complexity: slm_core::Complexity::High,
            tier_hint: Some(Tier::Yoyo),
            stream: false,
            max_tokens: Some(20),
            temperature: Some(0.0),
            sanitised_outbound: true,
        }
    }

    fn ok_body() -> serde_json::Value {
        serde_json::json!({
            "choices": [
                { "message": { "role": "assistant", "content": "PONG" } }
            ]
        })
    }

    fn client(server_uri: String) -> YoYoTierClient {
        YoYoTierClient::new(
            YoYoTierConfig {
                endpoint: server_uri,
                default_model: "Olmo-3-1125-32B-Think".into(),
                contract_version: crate::YOYO_CONTRACT_VERSION.into(),
            },
            Arc::new(StaticBearer::new("test-token-v1")),
        )
    }

    /// Happy path — 200 with the four required X-Foundry-* request
    /// headers and a content string in the response.
    #[tokio::test]
    async fn happy_path_200_round_trips_with_headers() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v1/chat/completions"))
            .and(header("authorization", "Bearer test-token-v1"))
            .and(header("x-foundry-module-id", "foundry"))
            .and(header(
                "x-foundry-contract-version",
                crate::YOYO_CONTRACT_VERSION,
            ))
            .and(header("x-foundry-complexity", "high"))
            .respond_with(
                ResponseTemplate::new(200)
                    .insert_header("x-foundry-yoyo-version", "mistralrs:0.8")
                    .insert_header("x-foundry-inference-ms", "412")
                    .set_body_json(ok_body()),
            )
            .expect(1)
            .mount(&server)
            .await;

        let client = client(server.uri());
        let resp = client.complete(&req()).await.expect("happy path 200");
        assert_eq!(resp.tier_used, Tier::Yoyo);
        assert_eq!(resp.content, "PONG");
        assert_eq!(resp.inference_ms, 412);
        assert_eq!(resp.upstream_version.as_deref(), Some("mistralrs:0.8"));
    }

    /// 503 + Retry-After cold-start: client sleeps then retries once;
    /// the second response succeeds.
    #[tokio::test]
    async fn cold_start_503_retries_once_then_succeeds() {
        let server = MockServer::start().await;
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();

        Mock::given(method("POST"))
            .and(path("/v1/chat/completions"))
            .respond_with(move |_req: &Request| {
                let n = counter_clone.fetch_add(1, Ordering::SeqCst);
                if n == 0 {
                    ResponseTemplate::new(503).insert_header("retry-after", "1")
                } else {
                    ResponseTemplate::new(200)
                        .insert_header("x-foundry-inference-ms", "200")
                        .set_body_json(ok_body())
                }
            })
            .expect(2)
            .mount(&server)
            .await;

        let client = client(server.uri());
        let resp = client
            .complete(&req())
            .await
            .expect("retry after 503 should succeed");
        assert_eq!(resp.content, "PONG");
        assert_eq!(counter.load(Ordering::SeqCst), 2);
    }

    /// 401 unauthorised: client refreshes the bearer token and retries
    /// once with the fresh token.
    #[tokio::test]
    async fn auth_failure_401_refreshes_and_retries() {
        // Bearer provider that flips its token on refresh, so we can
        // verify the second request uses the refreshed value.
        #[derive(Debug)]
        struct FlippingBearer {
            v1: String,
            v2: String,
            refreshed: AtomicUsize,
        }
        #[async_trait]
        impl BearerTokenProvider for FlippingBearer {
            async fn token(&self) -> Result<String> {
                Ok(self.v1.clone())
            }
            async fn refresh(&self) -> Result<String> {
                self.refreshed.fetch_add(1, Ordering::SeqCst);
                Ok(self.v2.clone())
            }
        }

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v1/chat/completions"))
            .and(header("authorization", "Bearer stale-token"))
            .respond_with(ResponseTemplate::new(401))
            .expect(1)
            .mount(&server)
            .await;
        Mock::given(method("POST"))
            .and(path("/v1/chat/completions"))
            .and(header("authorization", "Bearer fresh-token"))
            .respond_with(
                ResponseTemplate::new(200)
                    .insert_header("x-foundry-inference-ms", "300")
                    .set_body_json(ok_body()),
            )
            .expect(1)
            .mount(&server)
            .await;

        let bearer = Arc::new(FlippingBearer {
            v1: "stale-token".into(),
            v2: "fresh-token".into(),
            refreshed: AtomicUsize::new(0),
        });
        let client = YoYoTierClient::new(
            YoYoTierConfig {
                endpoint: server.uri(),
                default_model: "Olmo-3-1125-32B-Think".into(),
                contract_version: crate::YOYO_CONTRACT_VERSION.into(),
            },
            bearer.clone(),
        );

        let resp = client
            .complete(&req())
            .await
            .expect("retry after 401 should succeed");
        assert_eq!(resp.content, "PONG");
        assert_eq!(bearer.refreshed.load(Ordering::SeqCst), 1);
    }

    /// 410 MAJOR contract mismatch: client must NOT retry and must
    /// surface `ContractMajorMismatch`.
    #[tokio::test]
    async fn contract_410_returns_major_mismatch_no_retry() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v1/chat/completions"))
            .respond_with(ResponseTemplate::new(410))
            .expect(1)
            .mount(&server)
            .await;

        let client = client(server.uri());
        let err = client
            .complete(&req())
            .await
            .expect_err("410 must surface as error");
        match err {
            DoormanError::ContractMajorMismatch {
                remote_status,
                doorman_version,
            } => {
                assert_eq!(remote_status, 410);
                assert_eq!(doorman_version, crate::YOYO_CONTRACT_VERSION);
            }
            other => panic!("expected ContractMajorMismatch, got {other:?}"),
        }
    }
}
