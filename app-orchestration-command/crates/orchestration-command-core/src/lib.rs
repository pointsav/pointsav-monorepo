// SPDX-License-Identifier: LicenseRef-PointSav-Proprietary

//! Wire types for `app-orchestration-command` — the CommandCentre hub for a
//! Totebox Archive fleet.
//!
//! All types are `serde`-serialisable. The server and library crates share
//! these definitions; downstream tools (os-console, tests) can depend on
//! this crate without pulling in the full business logic.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Semver string embedded in every `/readyz` response.
pub const COMMAND_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Product identifier required in a valid CommandCentre license token.
pub const REQUIRED_PRODUCT: &str = "soft-orchestration-command";

// ---------------------------------------------------------------------------
// Fleet
// ---------------------------------------------------------------------------

/// One Totebox Archive entry returned by `GET /v1/archives`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchiveEntry {
    /// Unique tenant identifier (from the archive's manifest `module_id`).
    pub module_id: String,
    /// Human-readable name matching the directory under `clones/`.
    pub cluster_name: String,
    /// SLM Doorman endpoint for this archive.
    pub slm_endpoint: String,
    /// Tetrad leg completion status.
    pub tetrad: TetradStatus,
    /// Count of pending messages in the archive's inbox.
    pub inbox_pending: u32,
}

/// Summary of which Tetrad legs are complete for an archive.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TetradStatus {
    pub vendor: LegStatus,
    pub customer: LegStatus,
    pub deployment: LegStatus,
    pub wiki: LegStatus,
}

/// Whether a Tetrad leg is complete, pending, or unknown.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum LegStatus {
    Active,
    LegPending,
    Unknown,
}

/// Response for `GET /v1/archives`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchivesResponse {
    pub archives: Vec<ArchiveEntry>,
    pub total: usize,
}

// ---------------------------------------------------------------------------
// Personnel
// ---------------------------------------------------------------------------

/// Permission tier for a contributor.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PermissionTier {
    /// System Administrator — full workspace access.
    P1,
    /// Package Manager — specific archives + Stage 6 promotion.
    P2,
    /// User — specific archives only; no COMMAND pairing.
    P3,
    /// Interface — read-only API surface only.
    P4,
}

/// Response for `GET /v1/personnel/:user`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonnelEntry {
    pub unix_user: String,
    pub tier: PermissionTier,
    /// module_ids this user is paired with.
    pub pairing_set: Vec<String>,
}

// ---------------------------------------------------------------------------
// Invite tokens
// ---------------------------------------------------------------------------

/// Pairing role granted by an invite token.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PairingRole {
    /// Read/write access — daily operator.
    User,
    /// Full access — system administrator.
    Admin,
    /// Metadata-only — orchestration aggregator.
    Interface,
}

/// The JSON payload inside a signed invite token.
///
/// Token wire format: `<base64url(payload_json)>.<base64url(ed25519_signature)>`
/// The signature is over the raw bytes of the first segment (encoded payload),
/// matching the license token convention in `app-orchestration-slm`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InviteTokenPayload {
    /// Issuing instance identifier (e.g. "gateway-orchestration-command-1").
    pub issuer: String,
    /// Role granted on successful pairing.
    pub role: PairingRole,
    /// UUID v7 — single-use nonce; replayed tokens return 409.
    pub nonce: Uuid,
    /// Token expiry (typically 24 hours from issuance).
    pub expiry: DateTime<Utc>,
    /// module_ids this token grants access to (empty = issuer's full fleet).
    #[serde(default)]
    pub archive_scope: Vec<String>,
}

/// Request body for `POST /v1/invite` (P1 only).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InviteRequest {
    pub role: PairingRole,
    /// Optional restriction to a subset of archives.
    #[serde(default)]
    pub archive_scope: Vec<String>,
}

/// Response for `POST /v1/invite`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InviteResponse {
    /// The signed token string — hand this to the invited user.
    pub token: String,
    pub expires_at: DateTime<Utc>,
    pub role: PairingRole,
}

// ---------------------------------------------------------------------------
// Pairing exchange
// ---------------------------------------------------------------------------

/// Request body for `POST /v1/pair` — exchange an invite token for a pairing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PairRequest {
    /// The invite token string received from `POST /v1/invite`.
    pub token: String,
    /// Ed25519 public key of the connecting node, base64url-encoded.
    pub public_key: String,
    /// Human-readable label for this node (e.g. "jennifer-macbook").
    #[serde(default)]
    pub node_label: String,
}

/// Response for `POST /v1/pair`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PairResponse {
    pub status: PairStatus,
    pub paired_on: DateTime<Utc>,
    pub role: PairingRole,
    pub archive_scope: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PairStatus {
    Paired,
    AlreadyPaired,
}

// ---------------------------------------------------------------------------
// Message routing
// ---------------------------------------------------------------------------

/// Request body for `POST /v1/message`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageEnvelope {
    /// module_id of the requesting archive (validated against caller's pairing).
    pub from_module_id: String,
    /// module_id of the target archive.
    pub to_module_id: String,
    /// Subject line (matches mailbox `re:` field).
    pub re: String,
    /// Message body (plain text or markdown).
    pub body: String,
    /// Optional client-supplied idempotency key.
    #[serde(default)]
    pub client_msg_id: Option<String>,
}

/// Response for `POST /v1/message`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageResponse {
    pub msg_id: String,
    pub routed_at: DateTime<Utc>,
    pub status: MessageStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MessageStatus {
    Delivered,
    /// Target archive is not paired with this instance.
    Rejected,
}

// ---------------------------------------------------------------------------
// Audit
// ---------------------------------------------------------------------------

/// One archive's routing summary, returned by `GET /v1/audit/rollup`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub module_id: String,
    pub messages_routed: u64,
    pub invites_issued: u64,
    pub pairings_created: u64,
}

/// Response for `GET /v1/audit/rollup`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditRollupResponse {
    pub entries: Vec<AuditEntry>,
    pub as_of: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// Health
// ---------------------------------------------------------------------------

/// Response for `GET /readyz`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadyzResponse {
    pub version: &'static str,
    pub license: LicenseLabel,
    pub slm_child: ChildStatus,
    pub archives_loaded: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LicenseLabel {
    Valid,
    Absent,
    Invalid,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ChildStatus {
    Running,
    Stopped,
    NotConfigured,
}
