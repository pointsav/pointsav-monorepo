// SPDX-License-Identifier: LicenseRef-PointSav-Proprietary

//! Shared wire types for the app-orchestration-slm Yo-Yo broker chassis.
//!
//! These types cross the boundary between the three crates in this workspace
//! and are also used in integration tests. Keep this crate dependency-light —
//! only serde, chrono, uuid.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// ── Fleet member ──────────────────────────────────────────────────────────────

/// A Totebox Archive registered with this chassis.
///
/// Registered at startup by each Totebox Doorman via
/// `POST /v1/discovery/register`. Persists in-memory; the chassis is
/// stateless — it rebuilds from Doorman heartbeats on restart.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FleetMember {
    /// Three-segment archive identifier: `operator::archive::scope`.
    /// Used as the per-tenant isolation key on all Yo-Yo requests.
    pub module_id: String,

    /// Human-readable archive name, e.g. `project-intelligence`.
    pub archive_id: String,

    /// Base URL of the archive's Doorman (e.g. `http://10.0.1.5:9080`).
    /// Used for callbacks and result delivery (Phase 2).
    pub doorman_endpoint: String,

    /// Whether this archive has a commercial Tier B subscription.
    /// Unauthenticated or free-tier Toteboxes set this false and receive 402
    /// on Yo-Yo proxy requests.
    pub tier_b_subscribed: bool,

    /// UTC timestamp when this member last registered or re-registered.
    pub registered_at: DateTime<Utc>,
}

// ── Registration ──────────────────────────────────────────────────────────────

/// Body for `POST /v1/discovery/register`.
///
/// A Totebox Doorman posts this on startup when `SLM_ORCHESTRATION_ENDPOINT`
/// is configured.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrationRequest {
    pub module_id: String,
    pub archive_id: String,
    pub doorman_endpoint: String,
    /// Whether the Totebox is commercially subscribed for Tier B access.
    /// The chassis trusts this field from the registration payload for MVP;
    /// Phase 3 replaces this with a signed membership token.
    pub tier_b_subscribed: bool,
}

/// Response from `POST /v1/discovery/register`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrationResponse {
    pub status: &'static str,
    pub module_id: String,
    pub chassis_version: &'static str,
}

// ── Fleet listing ─────────────────────────────────────────────────────────────

/// Response for `GET /v1/fleet`.
#[derive(Debug, Serialize, Deserialize)]
pub struct FleetResponse {
    pub members: Vec<FleetMemberSummary>,
    pub total: usize,
}

/// Abbreviated member view — doorman_endpoint omitted from public listing.
#[derive(Debug, Serialize, Deserialize)]
pub struct FleetMemberSummary {
    pub module_id: String,
    pub archive_id: String,
    pub tier_b_subscribed: bool,
    pub registered_at: DateTime<Utc>,
}

// ── Readyz ────────────────────────────────────────────────────────────────────

/// Response for `GET /readyz`.
#[derive(Debug, Serialize, Deserialize)]
pub struct ReadyzResponse {
    pub status: &'static str,
    pub yoyo_trainer_reachable: bool,
    pub yoyo_graph_reachable: bool,
    pub fleet_members: usize,
    pub chassis_version: &'static str,
    /// "valid", "absent", or "invalid"
    pub license_status: String,
    /// Per-label circuit state: "closed", "open", or "half-open"
    pub circuit_states: std::collections::HashMap<String, String>,
    /// Per-label gate state: true = closed (blocked)
    pub gate_states: std::collections::HashMap<String, bool>,
    /// True when any Yo-Yo circuit is open — "chassis up, GPU unreachable"
    /// as its own distinct, observable state rather than an opaque flat
    /// "ok". Never affects this endpoint's own HTTP status (always 200) —
    /// the chassis itself is genuinely ready; only degraded_reason and the
    /// actual /v1/yoyo/* 503s (which is where fail-fast belongs, since the
    /// chassis has no lower rung to fall back to) reflect backend health.
    pub degraded: bool,
    /// Human-readable reason when `degraded` is true (e.g. "circuit open
    /// for: trainer, graph"); `None` when not degraded.
    pub degraded_reason: Option<String>,
}

// ── Yo-Yo proxy ───────────────────────────────────────────────────────────────

/// Which Yo-Yo node this request targets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum YoyoLabel {
    /// General-purpose inference → `"default"` node.
    Proxy,
    /// LoRA training → `"trainer"` node (OLMo 3 32B-Think on L4 24GB).
    Trainer,
    /// Grammar-constrained entity extraction → `"graph"` node (Llama 3.3 70B on H100).
    Graph,
}

// ── Tier 0 inference ─────────────────────────────────────────────────────────

/// Body for `POST /v1/inference` — the Tier 0 Doorman entry point
/// (`BRIEF-os-totebox-platform.md` §6/§14 #17-18). A Totebox Doorman running
/// in Tier 0 mode (`SLM_TIER=0`, no local llama-server of its own) sends
/// this instead of talking to a local model directly.
///
/// **License-boundary note**: this type documents and tests the real wire
/// contract, but the `/v1/inference` handler itself still deserializes the
/// request body as `serde_json::Value` and proxies it through unchanged —
/// matching the established pattern of every other proxy endpoint in this
/// chassis (`/v1/yoyo/proxy` et al. do the same; `YoyoLabel` above is a
/// routing enum, not a typed chat body either). Switching the handler to
/// strict typed deserialization would be a real behavioral change — stricter
/// validation could reject payloads the current passthrough tolerates — and
/// is deliberately not done here without separate sign-off. This type's
/// value is as a formal, testable contract (see the client-side mirror,
/// `OrchestrationChatRequest` in `service-slm/crates/slm-doorman/src/tier/
/// orchestration.rs`) that catches silent client/server drift in tests,
/// not a change to the live proxy's runtime behavior.
///
/// **Not shared as a Rust dependency across the license boundary.** The
/// client-side type lives in `slm-doorman` (Apache-2.0 OR MIT, ships to
/// every Totebox Archive) and cannot depend on this crate (`orchestration-
/// slm-core`, `LicenseRef-PointSav-Proprietary`) — doing so would leak
/// proprietary chassis code into the free crate's dependency graph, or
/// break the free crate's ability to build at all for anyone without access
/// to this private repo. The two sides independently implement the same
/// JSON shape by convention; this struct is the chassis-side half of that
/// contract, not a shared library type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceRequest {
    pub model: String,
    pub messages: Vec<InferenceMessage>,
    #[serde(default)]
    pub stream: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
}

/// Mirrors `slm_core::ChatMessage`'s wire shape without depending on
/// `slm_core` — see `InferenceRequest`'s doc comment for why.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceMessage {
    pub role: String,
    pub content: String,
}

/// Response shape for `POST /v1/inference` — OpenAI-style choices array,
/// mirroring `OrchestrationChatResponse` on the client side.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceResponse {
    pub choices: Vec<InferenceChoice>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceChoice {
    pub message: InferenceResponseMessage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceResponseMessage {
    #[serde(default)]
    pub content: Option<String>,
}

// ── Audit rollup ──────────────────────────────────────────────────────────────

/// Per-tenant metering summary, returned by `GET /v1/audit/rollup`.
#[derive(Debug, Serialize, Deserialize)]
pub struct TenantRollupEntry {
    pub module_id: String,
    pub total_requests: u64,
    pub total_inference_ms: u64,
    pub total_cost_usd: f64,
}

/// Response for `GET /v1/audit/rollup`.
#[derive(Debug, Serialize, Deserialize)]
pub struct AuditRollupResponse {
    pub entries: Vec<TenantRollupEntry>,
    pub total_tenants: usize,
}

/// Chassis version string embedded in every response.
pub const CHASSIS_VERSION: &str = env!("CARGO_PKG_VERSION");

// ── Phase 2: Federated graph query ───────────────────────────────────────────

/// Body for `POST /v1/graph/federated`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederatedGraphRequest {
    /// Query string forwarded verbatim to each registered Doorman.
    pub q: String,
    #[serde(default = "default_federated_limit")]
    pub limit: usize,
}

fn default_federated_limit() -> usize {
    20
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederatedGraphEntry {
    pub module_id: String,
    pub archive_id: String,
    pub result: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FederatedGraphResponse {
    pub entries: Vec<FederatedGraphEntry>,
    pub archives_queried: usize,
    pub archives_reachable: usize,
}

// ── Phase 2: Training schedule ────────────────────────────────────────────────

/// Body for `POST /v1/training/schedule`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingScheduleRequest {
    pub module_id: String,
    pub base_model: String,
    pub dataset_uri: String,
    pub adapter_name: String,
    #[serde(default = "default_training_steps")]
    pub max_steps: u32,
}

fn default_training_steps() -> u32 {
    500
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrainingScheduleResponse {
    pub job_id: String,
    pub status: &'static str,
    pub trainer_endpoint: Option<String>,
}

// ── Phase 2: Adapter listing ──────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterEntry {
    pub name: String,
    pub base_model: String,
    pub node_label: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdaptersResponse {
    pub adapters: Vec<AdapterEntry>,
    pub total: usize,
}

// ── Per-VM discovery/allocation (§14 #20) ───────────────────────────────────

/// Body for `POST /v1/discovery/allocate` — called once by a Doorman at its
/// own first boot, before `POST /v1/discovery/register`, to obtain a
/// chassis-guaranteed-unique identity instead of a self-claimed, operator-set
/// `SLM_MODULE_ID`/`SLM_ARCHIVE_ID` pair (which has zero collision
/// detection — see `AllocationLedger` in `orchestration-slm` for the
/// concrete gap this closes).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocationRequest {
    /// Optional human-readable hint for the archive_id (e.g. an operator's
    /// intended deployment name). Purely cosmetic — never trusted for
    /// uniqueness; the chassis always verifies against its ledger and
    /// appends a disambiguating suffix on collision.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requested_archive_id: Option<String>,
}

/// Response from `POST /v1/discovery/allocate` — the assigned identity.
/// The caller should persist this locally (see `OrchestrationTierClient`'s
/// identity-cache behavior on the Doorman side) and reuse it on every
/// subsequent boot rather than re-allocating.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocationResponse {
    pub module_id: String,
    pub archive_id: String,
}

// ── Phase 2: Signed membership token ─────────────────────────────────────────

/// Registration response (Phase 2) — includes Ed25519 signed membership token.
///
/// Token format: `<base64url(claims_json)>.<base64url(ed25519_signature)>`.
/// Valid for 1 hour. The Doorman presents this as `Authorization: Bearer <token>`
/// on subsequent proxy calls; the chassis verifies signature + expiry inline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrationResponseV2 {
    pub status: &'static str,
    pub module_id: String,
    pub chassis_version: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub membership_token: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Contract check, not a handler test: confirms `InferenceRequest`/
    /// `InferenceResponse` can round-trip the exact JSON shape the real
    /// client (`OrchestrationTierClient` in `slm-doorman`) sends and expects
    /// — catches silent drift between the two independently-maintained
    /// sides of this license-boundary-separated wire contract. Does not
    /// touch the live `/v1/inference` handler, which still deserializes as
    /// `serde_json::Value` (see `InferenceRequest`'s doc comment for why).
    #[test]
    fn inference_request_matches_real_client_wire_shape() {
        // Mirrors what OrchestrationChatRequest serializes when stream=false,
        // max_tokens=Some(50), temperature=Some(0.0) — a stream: false field
        // is skipped (is_false), matching the client's actual serde config.
        let json = serde_json::json!({
            "model": "olmo-3-7b-instruct",
            "messages": [{"role": "user", "content": "ping"}],
            "max_tokens": 50,
            "temperature": 0.0
        });
        let req: InferenceRequest =
            serde_json::from_value(json).expect("must deserialize the real client shape");
        assert_eq!(req.model, "olmo-3-7b-instruct");
        assert_eq!(req.messages.len(), 1);
        assert_eq!(req.messages[0].role, "user");
        assert!(!req.stream, "stream must default to false when omitted");
        assert_eq!(req.max_tokens, Some(50));
    }

    #[test]
    fn inference_response_matches_real_client_wire_shape() {
        // Mirrors what OrchestrationChatResponse expects to parse.
        let json = serde_json::json!({
            "choices": [{ "message": { "content": "pong" } }]
        });
        let resp: InferenceResponse =
            serde_json::from_value(json).expect("must deserialize the real client shape");
        assert_eq!(resp.choices.len(), 1);
        assert_eq!(resp.choices[0].message.content.as_deref(), Some("pong"));
    }

    /// The client's OrchestrationChatResponse tolerates a choice with no
    /// content (defaults to None) — confirm this side does too.
    #[test]
    fn inference_response_message_content_defaults_to_none() {
        let json = serde_json::json!({
            "choices": [{ "message": {} }]
        });
        let resp: InferenceResponse =
            serde_json::from_value(json).expect("missing content must not fail to parse");
        assert_eq!(resp.choices[0].message.content, None);
    }
}
