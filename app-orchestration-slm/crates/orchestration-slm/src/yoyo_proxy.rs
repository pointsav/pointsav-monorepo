// SPDX-License-Identifier: LicenseRef-PointSav-Proprietary

//! Yo-Yo proxy — forwards chat-completions requests to the shared Yo-Yo fleet.
//!
//! The chassis front-ends the existing Yo-Yo fleet (Yo-Yo #1 "trainer" L4 +
//! Yo-Yo #2 "graph" H100). No new Yo-Yo VMs are provisioned here.
//!
//! Request flow:
//!   1. Doorman authenticates via fleet.authenticate_proxy()
//!   2. Chassis validates X-Foundry-Module-ID header (anti-spoofing)
//!   3. Chassis injects X-Foundry-Module-ID into the upstream request
//!   4. Proxies to the target Yo-Yo via reqwest (streaming pass-through)
//!   5. Records inference_ms from X-Foundry-Inference-Ms response header
//!   6. Returns upstream response body + headers to caller

use std::time::Duration;

use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde_json::Value;
use tracing::{debug, warn};

use crate::error::{ChassisError, Result};

/// Yo-Yo endpoint configuration. Built from env vars at startup.
#[derive(Debug, Clone)]
pub struct YoyoEndpoints {
    /// Default general-purpose inference node.
    pub default_endpoint: Option<String>,
    /// Training node (OLMo 3 32B-Think on L4 24GB).
    pub trainer_endpoint: Option<String>,
    /// Grammar-constrained extraction node (Llama 3.3 70B on H100).
    pub graph_endpoint: Option<String>,
    /// Bearer token for the actual Yo-Yo VMs (one shared secret for MVP).
    pub yoyo_bearer: Option<String>,
    /// Hourly USD rate used for cost metering (default 0.0 in dev).
    pub hourly_usd_rate: f64,
}

impl YoyoEndpoints {
    /// Build from environment variables.
    ///
    /// ```text
    /// ORCHESTRATION_YOYO_DEFAULT_ENDPOINT   optional; absent = proxy label unavailable
    /// ORCHESTRATION_YOYO_TRAINER_ENDPOINT   optional; absent = trainer label unavailable
    /// ORCHESTRATION_YOYO_GRAPH_ENDPOINT     optional; absent = graph label unavailable
    /// ORCHESTRATION_YOYO_BEARER             optional; auth header for Yo-Yo VMs
    /// ORCHESTRATION_YOYO_HOURLY_USD         hourly USD rate; default 0.0
    /// ```
    pub fn from_env() -> Self {
        Self {
            default_endpoint: std::env::var("ORCHESTRATION_YOYO_DEFAULT_ENDPOINT").ok(),
            trainer_endpoint: std::env::var("ORCHESTRATION_YOYO_TRAINER_ENDPOINT").ok(),
            graph_endpoint: std::env::var("ORCHESTRATION_YOYO_GRAPH_ENDPOINT").ok(),
            yoyo_bearer: std::env::var("ORCHESTRATION_YOYO_BEARER").ok(),
            hourly_usd_rate: std::env::var("ORCHESTRATION_YOYO_HOURLY_USD")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0.0),
        }
    }

    pub fn endpoint_for(&self, label: &str) -> Option<&str> {
        match label {
            "proxy" => self.default_endpoint.as_deref(),
            "trainer" => self.trainer_endpoint.as_deref(),
            "graph" => self.graph_endpoint.as_deref(),
            _ => None,
        }
    }

    pub fn any_configured(&self) -> (bool, bool) {
        let trainer = self.trainer_endpoint.is_some();
        let graph = self.graph_endpoint.is_some();
        (trainer, graph)
    }
}

/// HTTP client for proxying to Yo-Yo VMs.
///
/// Uses a shared reqwest client with connection pooling and a 90-second
/// timeout matching the outer deadline in service-slm's yoyo.rs.
pub struct YoyoProxyClient {
    client: reqwest::Client,
    pub endpoints: YoyoEndpoints,
}

impl YoyoProxyClient {
    pub fn new(endpoints: YoyoEndpoints) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(90))
            .build()
            .expect("failed to build reqwest client");
        Self { client, endpoints }
    }

    /// Proxy a chat-completions request to the named Yo-Yo node.
    ///
    /// `label`     — "proxy", "trainer", or "graph"
    /// `module_id` — validated module-id; injected as X-Foundry-Module-ID
    /// `body`      — raw JSON body from the incoming request
    ///
    /// Returns `(response_body_bytes, inference_ms)`.
    pub async fn proxy(
        &self,
        label: &str,
        module_id: &str,
        body: Value,
    ) -> Result<(Vec<u8>, Option<u64>)> {
        let base = self
            .endpoints
            .endpoint_for(label)
            .ok_or_else(|| ChassisError::YoyoNotConfigured(label.to_string()))?;

        let url = format!("{}/v1/chat/completions", base.trim_end_matches('/'));

        debug!(label, module_id, %url, "yoyo_proxy: forwarding request");

        let mut req = self
            .client
            .post(&url)
            .json(&body)
            .header("X-Foundry-Module-ID", module_id)
            .header("X-Foundry-Contract-Version", "2");

        if let Some(bearer) = &self.endpoints.yoyo_bearer {
            req = req.header("Authorization", format!("Bearer {}", bearer));
        }

        let resp = req
            .send()
            .await
            .map_err(|e| ChassisError::YoyoUpstream(e.to_string()))?;

        let inference_ms = resp
            .headers()
            .get("x-foundry-inference-ms")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse::<u64>().ok());

        let status = resp.status();
        if !status.is_success() {
            warn!(label, module_id, status = %status, "yoyo_proxy: upstream returned non-2xx");
        }

        let bytes = resp
            .bytes()
            .await
            .map_err(|e| ChassisError::YoyoUpstream(e.to_string()))?;

        Ok((bytes.to_vec(), inference_ms))
    }

    /// Probe the Yo-Yo VM's /health endpoint to determine reachability.
    /// Returns true if the probe succeeds within 2 seconds.
    pub async fn probe(&self, label: &str) -> bool {
        let base = match self.endpoints.endpoint_for(label) {
            Some(e) => e,
            None => return false,
        };
        let url = format!("{}/health", base.trim_end_matches('/'));
        let probe_client = reqwest::Client::builder()
            .timeout(Duration::from_secs(2))
            .build()
            .unwrap_or_else(|_| self.client.clone());
        probe_client.get(&url).send().await.is_ok()
    }

    /// Build downstream response headers — pass through content-type.
    pub fn passthrough_headers(upstream_headers: &HeaderMap) -> HeaderMap {
        let mut out = HeaderMap::new();
        for name in &[
            "content-type",
            "x-foundry-tier-used",
            "x-foundry-yoyo-version",
        ] {
            if let Some(v) = upstream_headers.get(*name) {
                if let Ok(k) = HeaderName::from_bytes(name.as_bytes()) {
                    out.insert(k, v.clone());
                }
            }
        }
        // Always tag the response as Tier B
        out.insert(
            HeaderName::from_static("x-foundry-tier-used"),
            HeaderValue::from_static("yoyo"),
        );
        out
    }
}
