// SPDX-License-Identifier: Apache-2.0 OR MIT

use slm_core::Tier;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, DoormanError>;

#[derive(Debug, Error)]
pub enum DoormanError {
    #[error("requested tier {0:?} is not configured")]
    TierUnavailable(Tier),

    #[error("tier {tier:?} is not yet implemented (filled in {filled_in_by})")]
    NotImplemented {
        tier: Tier,
        filled_in_by: &'static str,
    },

    #[error(
        "tier-C task label {label:?} is not on the allowlist; \
         see DoormanConfig::external_allowlist"
    )]
    ExternalNotAllowlisted { label: String },

    #[error("upstream HTTP error: {0}")]
    Upstream(#[from] reqwest::Error),

    #[error("audit ledger I/O error: {0}")]
    LedgerIo(#[from] std::io::Error),

    #[error("audit ledger serialisation error: {0}")]
    LedgerSerde(#[from] serde_json::Error),

    #[error("home directory not resolvable; HOME env var is unset")]
    HomeUnset,

    #[error("upstream response could not be parsed as OpenAI-compatible: {0}")]
    UpstreamShape(String),

    #[error(
        "Yo-Yo contract MAJOR-version mismatch: remote returned {remote_status} \
         (Doorman speaks contract {doorman_version}); refusing to retry"
    )]
    ContractMajorMismatch {
        remote_status: u16,
        doorman_version: &'static str,
    },

    #[error("Yo-Yo bearer-token provider returned no token: {0}")]
    BearerToken(String),

    #[error("verdict signature verification failed: {0}")]
    VerifySignature(String),

    #[error("apprenticeship ledger lock or write failed: {0}")]
    LedgerLock(String),

    #[error("apprenticeship corpus write failed at {path}: {reason}")]
    CorpusWrite { path: String, reason: String },

    #[error("verdict body could not be parsed: {0}")]
    VerdictParse(String),

    #[error(
        "verdict POST referenced an unknown (brief_id, attempt_id) — \
             brief cache miss; senior must reissue the brief"
    )]
    BriefCacheMiss,

    #[error(
        "Tier A (llama-server) does not accept {dialect} grammars; \
         {advice}"
    )]
    TierAGrammarUnsupported {
        /// The grammar dialect that was rejected, e.g. `"Lark"`.
        dialect: &'static str,
        /// Human-readable advice for the caller.
        advice: &'static str,
    },

    #[error(
        "Tier C (external API) does not accept {dialect} grammars; \
         {advice}"
    )]
    TierCGrammarUnsupported {
        /// The grammar dialect that was rejected, e.g. `"Lark"`, `"GBNF"`,
        /// or `"JsonSchema"`.
        dialect: &'static str,
        /// Human-readable advice for the caller.
        advice: &'static str,
    },

    /// The caller supplied a Lark grammar that failed to compile at the
    /// Doorman boundary (PS.3 step 5). The error message from llguidance
    /// includes line/column context so the caller can fix the grammar
    /// without needing to route to a backend at all.
    #[error("Lark grammar failed pre-validation: {reason}")]
    MalformedLarkGrammar {
        /// Parse-error message from llguidance's Lark compiler, including
        /// line/column context and a snippet of the offending input.
        reason: String,
    },

    /// `POST /v1/audit/proxy` caller supplied an unrecognised provider string
    /// (PS.4). Accepted values: "anthropic", "gemini", "openai". 400 BAD_REQUEST.
    #[error(
        "audit_proxy provider {provider:?} is not recognised; \
         accepted values: anthropic, gemini, openai"
    )]
    AuditProxyInvalidProvider {
        /// The provider string the caller submitted.
        provider: String,
    },

    /// `POST /v1/audit/proxy` targeted a provider that has no configured
    /// endpoint or API key at Doorman startup (PS.4 step 2). This is a
    /// server-side configuration gap, not a caller policy violation —
    /// hence 503 SERVICE_UNAVAILABLE rather than 403.
    #[error(
        "audit_proxy provider {provider:?} is not configured; \
         set SLM_TIER_C_{PROVIDER}_ENDPOINT and SLM_TIER_C_{PROVIDER}_API_KEY \
         to enable this provider",
        PROVIDER = provider.to_ascii_uppercase()
    )]
    AuditProxyProviderUnavailable {
        /// The provider string (e.g. "anthropic", "gemini", "openai").
        provider: String,
    },

    /// `POST /v1/audit/proxy` caller supplied a purpose that is not on the
    /// server's `AuditProxyPurposeAllowlist` (PS.4 step 3). This is a
    /// caller-side policy violation — the purpose is not documented as
    /// permissible on this deployment. 403 FORBIDDEN (same classification
    /// as `ExternalNotAllowlisted`, which mirrors this pattern for Tier C
    /// task labels).
    #[error(
        "audit_proxy purpose {purpose:?} is not on the purpose allowlist; \
         see AuditProxyConfig::purpose_allowlist for documented purposes"
    )]
    AuditProxyPurposeNotAllowlisted {
        /// The purpose string the caller submitted.
        purpose: String,
    },

    /// `POST /v1/audit/capture` caller supplied an event_type that is not one
    /// of the five accepted values (PS.4 step 4). Accepted values:
    /// "prose-edit", "design-edit", "graph-mutation", "anchor-event",
    /// "verdict-issued". 400 BAD_REQUEST.
    #[error(
        "audit_capture event_type {event_type:?} is not recognised; \
         accepted values: prose-edit, design-edit, graph-mutation, anchor-event, verdict-issued"
    )]
    AuditCaptureUnknownEventType {
        /// The event_type string the caller submitted.
        event_type: String,
    },

    /// `POST /v1/audit/capture` caller submitted a payload larger than the
    /// maximum permitted size (`AUDIT_CAPTURE_MAX_PAYLOAD_BYTES`). 413
    /// PAYLOAD_TOO_LARGE. Classified as PolicyDenied — the caller must shrink
    /// the payload before retrying.
    #[error(
        "audit_capture payload is {size_bytes} bytes, exceeding the {max_bytes}-byte limit; \
         reduce the payload size before retrying"
    )]
    AuditCapturePayloadTooLarge {
        /// Serialised payload size in bytes.
        size_bytes: usize,
        /// Maximum permitted size in bytes.
        max_bytes: usize,
    },

    /// `POST /v1/audit/capture` caller supplied an `event_at` timestamp that
    /// does not parse as RFC 3339 / ISO 8601. 400 BAD_REQUEST.
    #[error(
        "audit_capture event_at {value:?} is not a valid RFC 3339 timestamp; \
         use format YYYY-MM-DDTHH:MM:SSZ or YYYY-MM-DDTHH:MM:SS+HH:MM"
    )]
    AuditCaptureInvalidTimestamp {
        /// The raw string the caller submitted.
        value: String,
    },

    /// `POST /v1/audit/proxy` caller submitted a raw request body that exceeds
    /// `AUDIT_PROXY_MAX_REQUEST_BYTES` (64 KiB). Checked BEFORE JSON
    /// deserialisation so an oversized request is rejected early without
    /// allocating heap memory for the payload. 413 PAYLOAD_TOO_LARGE.
    /// Classified as PolicyDenied — the caller must reduce the request size.
    #[error(
        "audit_proxy request is {size_bytes} bytes, exceeding the {max_bytes}-byte limit; \
         reduce the request size before retrying"
    )]
    AuditProxyPayloadTooLarge {
        /// Raw body size in bytes.
        size_bytes: usize,
        /// Maximum permitted size in bytes (`AUDIT_PROXY_MAX_REQUEST_BYTES`).
        max_bytes: usize,
    },

    /// A tenant's (`moduleId`) in-flight request count for the audit endpoints
    /// (`/v1/audit/proxy` and `/v1/audit/capture`) has reached
    /// `SLM_AUDIT_TENANT_CONCURRENCY_CAP` (default 4). The per-tenant
    /// `Semaphore` has no permits available. 503 SERVICE_UNAVAILABLE with
    /// `Retry-After: 5`. Classified as PolicyDenied — the caller may retry
    /// once an in-flight request from the same tenant completes.
    #[error(
        "audit tenant {module_id:?} has reached the concurrency cap of {cap} \
         in-flight requests; retry after in-flight requests complete"
    )]
    AuditTenantConcurrencyExhausted {
        /// The `moduleId` that hit the cap.
        module_id: String,
        /// The configured cap value (`SLM_AUDIT_TENANT_CONCURRENCY_CAP`).
        cap: u32,
    },
}
