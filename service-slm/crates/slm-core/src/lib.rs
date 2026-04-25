// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Shared types for service-slm.
//!
//! `slm-core` holds only types and small value-objects. It has no async
//! runtime, no HTTP client, no I/O. Crates that route, log, or serve HTTP
//! depend on this crate; nothing in this crate depends on them.

pub mod error;
pub mod module_id;
pub mod request_id;
pub mod tier;

pub use error::{CoreError, Result};
pub use module_id::ModuleId;
pub use request_id::RequestId;
pub use tier::{Complexity, Tier};

use serde::{Deserialize, Serialize};

/// One inbound message in OpenAI chat-completions shape.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

/// Request crossing the Doorman boundary.
///
/// `request_id` and `module_id` are mandatory — they tag every audit-ledger
/// entry and every multi-tenant routing decision. `tier_hint` is advisory;
/// the router may override based on budget caps and request shape.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ComputeRequest {
    pub request_id: RequestId,
    pub module_id: ModuleId,
    pub model: Option<String>,
    pub messages: Vec<ChatMessage>,
    #[serde(default)]
    pub complexity: Complexity,
    #[serde(default)]
    pub tier_hint: Option<Tier>,
    #[serde(default)]
    pub stream: bool,
    #[serde(default)]
    pub max_tokens: Option<u32>,
    #[serde(default)]
    pub temperature: Option<f32>,
    /// True if the caller has already sanitised identifiers out of the
    /// payload per the Doorman Protocol (`ARCHITECTURE.md` §1).
    #[serde(default)]
    pub sanitised_outbound: bool,
}

/// Response returned through the Doorman.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ComputeResponse {
    pub request_id: RequestId,
    pub tier_used: Tier,
    pub model: String,
    pub content: String,
    pub inference_ms: u64,
    pub cost_usd: f64,
    /// Yo-Yo or external-API implementation version, opaque string.
    #[serde(default)]
    pub upstream_version: Option<String>,
}
