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

/// Chassis version string embedded in every response.
pub const CHASSIS_VERSION: &str = env!("CARGO_PKG_VERSION");
