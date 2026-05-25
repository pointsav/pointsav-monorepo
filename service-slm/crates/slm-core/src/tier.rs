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
    /// Tier A — local OLMo 3 7B Q4 (mistral.rs / llama.cpp HTTP on this VM).
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
