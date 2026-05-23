// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Compute-tier vocabulary per `conventions/llm-substrate-decision.md`.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpeculationRequest {
    pub draft_model: String,
    pub draft_token_budget: u32,
}

/// Three compute tiers. The Doorman may pick a different tier than the
/// caller's hint based on budget caps, request shape, and warm-VM state.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Tier {
    /// Tier A — on-device OLMo 2 1B specialist (NUC-class hardware Toteboxes only).
    /// Unavailable on $7/mo e2-micro fleet nodes (DOCTRINE claim #54).
    Local,
    /// Tier B — Yo-Yo OLMo-3-1125-32B-Think on multi-cloud GPU burst.
    Yoyo,
    /// Tier C — external API (Anthropic / Google / OpenAI), narrow precision
    /// tasks only, against an explicit allowlist.
    External,
}

impl Tier {
    pub fn as_str(&self) -> &'static str {
        match self {
            Tier::Local => "local",
            Tier::Yoyo => "yoyo",
            Tier::External => "external",
        }
    }
}

/// Hint for tier selection. The router maps complexity + budget caps to a
/// concrete tier; callers do not pick the tier directly.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Complexity {
    Low,
    #[default]
    Medium,
    High,
}

impl Complexity {
    pub fn as_str(&self) -> &'static str {
        match self {
            Complexity::Low => "low",
            Complexity::Medium => "medium",
            Complexity::High => "high",
        }
    }
}

/// Caller's latency contract for this request.
/// Used alongside `Complexity` and node-class to select a tier.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LatencyClass {
    /// Respond as fast as possible; prefer on-device Tier A when available.
    #[default]
    Interactive,
    /// Can tolerate seconds of queuing; prefer Tier B for quality.
    Background,
    /// Nightly batch work; always routes via Tier B (route_yoyo_only pattern).
    Batch,
}

impl LatencyClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            LatencyClass::Interactive => "interactive",
            LatencyClass::Background => "background",
            LatencyClass::Batch => "batch",
        }
    }
}
