// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Tier C — external API (Anthropic Claude / Google Gemini / OpenAI).
//!
//! Hard rule per `conventions/llm-substrate-decision.md`: this tier is
//! reserved for narrow precision tasks (citation grounding, initial
//! graph build, entity disambiguation). It is NEVER a default fallback
//! path — calls must reference an explicit allowlist label and the
//! Doorman refuses requests whose label is not allowlisted.
//!
//! B1 establishes the client interface and the empty-allowlist default.
//! B4 populates the allowlist and the per-provider HTTP wiring.

use std::collections::HashSet;

use slm_core::{ComputeRequest, ComputeResponse, Tier};

use crate::error::{DoormanError, Result};

/// Allowlist of task labels permitted to use Tier C. The label travels
/// in the `ComputeRequest::tier_hint` chain or as an extension field
/// (TBD with B4); B1 stores the set so the router can refuse before any
/// network call.
#[derive(Clone, Debug, Default)]
pub struct ExternalAllowlist {
    labels: HashSet<String>,
}

impl ExternalAllowlist {
    pub fn empty() -> Self {
        Self {
            labels: HashSet::new(),
        }
    }

    pub fn with(mut self, label: impl Into<String>) -> Self {
        self.labels.insert(label.into());
        self
    }

    pub fn contains(&self, label: &str) -> bool {
        self.labels.contains(label)
    }

    pub fn is_empty(&self) -> bool {
        self.labels.is_empty()
    }
}

#[derive(Clone, Debug)]
pub struct ExternalTierConfig {
    pub allowlist: ExternalAllowlist,
}

impl Default for ExternalTierConfig {
    fn default() -> Self {
        Self {
            allowlist: ExternalAllowlist::empty(),
        }
    }
}

pub struct ExternalTierClient {
    config: ExternalTierConfig,
    #[allow(dead_code)] // wired up in B4
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

    /// Stub. B4 implements per-provider routing + per-call cost capture.
    /// B1 enforces the allowlist contract via the public `check_label`
    /// helper; the router calls that before invoking `complete`.
    pub async fn complete(&self, _req: &ComputeRequest) -> Result<ComputeResponse> {
        Err(DoormanError::NotImplemented {
            tier: Tier::External,
            filled_in_by: "B4",
        })
    }

    pub fn check_label(&self, label: &str) -> Result<()> {
        if self.config.allowlist.contains(label) {
            Ok(())
        } else {
            Err(DoormanError::ExternalNotAllowlisted {
                label: label.to_string(),
            })
        }
    }
}
