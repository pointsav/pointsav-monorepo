// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Tier B — Yo-Yo cloud burst over the contract at
//! `infrastructure/slm-yoyo/CONTRACT.md`.
//!
//! B1 establishes the client interface and the X-Foundry-* header shape.
//! B2 fills in the bearer-token providers (GCP ID token from Workload
//! Identity, RunPod / Modal API key from Secret Manager, customer mTLS)
//! and the actual POST. Until B2 lands, `complete()` returns
//! `DoormanError::NotImplemented { filled_in_by: "B2" }` so the router
//! exercises the Optional Intelligence fallback path without confusion.

use slm_core::{ComputeRequest, ComputeResponse, Tier};

use crate::error::{DoormanError, Result};

#[derive(Clone, Debug)]
pub struct YoYoTierConfig {
    /// Base URL of the Yo-Yo node (e.g. `https://yoyo-foundry.run.app`).
    pub endpoint: String,
    /// Default model identifier. Yo-Yo runs Olmo-3-1125-32B-Think Q4
    /// (canonical Allen AI name; see Master's v0.0.9 nomenclature note).
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
    #[allow(dead_code)] // wired up in B2
    http: reqwest::Client,
}

impl YoYoTierClient {
    pub fn new(config: YoYoTierConfig) -> Self {
        Self {
            config,
            http: reqwest::Client::new(),
        }
    }

    pub fn endpoint(&self) -> &str {
        &self.config.endpoint
    }

    pub fn contract_version(&self) -> &str {
        &self.config.contract_version
    }

    /// Stub. B2 implements: token acquisition, POST `/v1/chat/completions`
    /// with required headers (`Authorization: Bearer ...`,
    /// `X-Foundry-Request-ID`, `X-Foundry-Module-ID`,
    /// `X-Foundry-Contract-Version`), retry on 503 + Retry-After,
    /// auth-refresh on 401/403, MAJOR mismatch on 410.
    pub async fn complete(&self, _req: &ComputeRequest) -> Result<ComputeResponse> {
        Err(DoormanError::NotImplemented {
            tier: Tier::Yoyo,
            filled_in_by: "B2",
        })
    }
}
