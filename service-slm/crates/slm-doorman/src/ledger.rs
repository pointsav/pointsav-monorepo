// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Append-only JSONL audit ledger.
//!
//! v0.1 stores entries at `~/.service-slm/audit/<YYYY-MM-DD>.jsonl`. A
//! later phase proxies writes through Ring 1 service-fs (WORM Immutable
//! Ledger); the wire format here is the v0.1 substrate for that path.
//!
//! One entry per Doorman call. Per `ARCHITECTURE.md` §8 the long-form
//! schema includes more fields (cache hit ratio, operator id, etc.); B1
//! captures only the fields the inbox brief named (request-id, moduleId,
//! tier, inference-ms, cost-usd, sanitised-outbound). The remaining
//! fields fold in as later phases add the data sources for them.
//!
//! ## entry_type discriminator (contract v0.2.0)
//!
//! All four entry types carry `entry_type: String` with a canonical
//! kebab-case value set by each `AuditLedger::append_*` method at write
//! time:
//!
//! | Struct | `entry_type` value |
//! |---|---|
//! | `AuditEntry` | `"chat-completion"` |
//! | `AuditProxyStubEntry` | `"audit-proxy-stub"` |
//! | `AuditProxyEntry` | `"audit-proxy"` |
//! | `AuditCaptureEntry` | `"audit-capture"` |
//!
//! Backwards compatibility: the field uses `#[serde(default = "...")]`
//! so old JSONL entries that pre-date contract v0.2.0 (i.e. that lack
//! the field) still deserialise correctly — each struct's default
//! function returns its own canonical string.

use std::fs::{self, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use slm_core::{ModuleId, RequestId, Tier};

use crate::error::{DoormanError, Result};

// ---------------------------------------------------------------------------
// Canonical entry_type strings and serde default functions
// ---------------------------------------------------------------------------

/// Canonical `entry_type` string for `AuditEntry` (chat-completion routing).
pub const ENTRY_TYPE_CHAT_COMPLETION: &str = "chat-completion";
/// Canonical `entry_type` string for `AuditProxyStubEntry`.
pub const ENTRY_TYPE_AUDIT_PROXY_STUB: &str = "audit-proxy-stub";
/// Canonical `entry_type` string for `AuditProxyEntry` (final outcome).
pub const ENTRY_TYPE_AUDIT_PROXY: &str = "audit-proxy";
/// Canonical `entry_type` string for `AuditCaptureEntry`.
pub const ENTRY_TYPE_AUDIT_CAPTURE: &str = "audit-capture";
/// Canonical `entry_type` string for `ExtractionAuditEntry` (`POST /v1/extract`).
pub const ENTRY_TYPE_EXTRACT: &str = "extract";

// Serde default fns — used in `#[serde(default = "...")]` on each struct's
// `entry_type` field so that old JSONL entries that pre-date contract v0.2.0
// and therefore lack the field still deserialise to the correct value.
fn default_entry_type_chat_completion() -> String {
    ENTRY_TYPE_CHAT_COMPLETION.to_string()
}
fn default_entry_type_audit_proxy_stub() -> String {
    ENTRY_TYPE_AUDIT_PROXY_STUB.to_string()
}
fn default_entry_type_audit_proxy() -> String {
    ENTRY_TYPE_AUDIT_PROXY.to_string()
}
fn default_entry_type_audit_capture() -> String {
    ENTRY_TYPE_AUDIT_CAPTURE.to_string()
}
fn default_entry_type_extract() -> String {
    ENTRY_TYPE_EXTRACT.to_string()
}

// ---------------------------------------------------------------------------
// Entry structs
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuditEntry {
    /// Explicit discriminator field (contract v0.2.0). Always `"chat-completion"`.
    /// `#[serde(default)]` ensures old JSONL entries lacking the field still
    /// deserialise correctly.
    #[serde(default = "default_entry_type_chat_completion")]
    pub entry_type: String,
    pub timestamp_utc: DateTime<Utc>,
    pub request_id: RequestId,
    pub module_id: ModuleId,
    pub tier: Tier,
    pub model: String,
    pub inference_ms: u64,
    pub cost_usd: f64,
    pub sanitised_outbound: bool,
    pub completion_status: CompletionStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CompletionStatus {
    Ok,
    UpstreamError,
    PolicyDenied,
    TierUnavailable,
}

/// Append-only ledger writer. One process owns one `AuditLedger`; the
/// internal mutex serialises concurrent writes from multiple request
/// handlers.
pub struct AuditLedger {
    base_dir: PathBuf,
    inner: Mutex<()>,
}

impl AuditLedger {
    /// Default location: `$HOME/.service-slm/audit/`.
    pub fn default_for_user() -> Result<Self> {
        let home = std::env::var_os("HOME").ok_or(DoormanError::HomeUnset)?;
        let base = Path::new(&home).join(".service-slm").join("audit");
        Self::new(base)
    }

    pub fn new(base_dir: impl Into<PathBuf>) -> Result<Self> {
        let base_dir = base_dir.into();
        fs::create_dir_all(&base_dir)?;
        Ok(Self {
            base_dir,
            inner: Mutex::new(()),
        })
    }

    /// Write one entry. Each line is one JSON object; the file is opened
    /// in append mode so concurrent writers from different processes
    /// would be safe at the kernel level for writes under PIPE_BUF, but
    /// in-process serialisation via the mutex keeps lines whole even for
    /// large entries.
    ///
    /// The `entry_type` field is forced to the canonical value
    /// `"chat-completion"` at write time regardless of what the caller
    /// placed in the struct, so cross-cluster consumers always see the
    /// correct discriminator.
    pub fn append(&self, entry: &AuditEntry) -> Result<()> {
        // Force the canonical entry_type at write time (contract v0.2.0).
        let mut entry = entry.clone();
        entry.entry_type = ENTRY_TYPE_CHAT_COMPLETION.to_string();
        let path = self.path_for(&entry.timestamp_utc);
        let line = serde_json::to_vec(&entry)?;
        let _guard = self.inner.lock().expect("audit ledger mutex poisoned");
        let file = OpenOptions::new().create(true).append(true).open(&path)?;
        let mut writer = BufWriter::new(file);
        writer.write_all(&line)?;
        writer.write_all(b"\n")?;
        writer.flush()?;
        Ok(())
    }

    pub fn base_dir(&self) -> &Path {
        &self.base_dir
    }

    fn path_for(&self, ts: &DateTime<Utc>) -> PathBuf {
        self.base_dir
            .join(format!("{}.jsonl", ts.format("%Y-%m-%d")))
    }
}

/// Audit ledger entry for a `POST /v1/audit/proxy` call captured in the
/// scaffold phase (PS.4 step 1). Written before the upstream provider is
/// called so we have a paper trail even during the scaffold phase when the
/// upstream relay is not yet wired.
///
/// The `status` field holds `"scaffold-stub-no-relay-yet"` in step 1;
/// PS.4 step 2 writes a second entry (see `AuditProxyEntry`) with the final
/// outcome: `"ok"` or `"upstream-error"`.
///
/// Two-entry design: every inbound request writes a stub first, then a
/// final entry after the upstream call completes. This preserves the paper
/// trail even when the upstream call fails partway through.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuditProxyStubEntry {
    /// Explicit discriminator field (contract v0.2.0). Always `"audit-proxy-stub"`.
    #[serde(default = "default_entry_type_audit_proxy_stub")]
    pub entry_type: String,
    pub audit_id: String,
    pub inbound_at: DateTime<Utc>,
    pub module_id: ModuleId,
    pub purpose: String,
    pub provider: String,
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caller_request_id: Option<String>,
    pub request_messages_count: usize,
    pub status: String,
}

/// Full audit ledger entry written AFTER the upstream provider call completes
/// (PS.4 step 2). Written as the second JSONL line for the same `audit_id`
/// — the stub entry (status: "scaffold-stub-no-relay-yet" / "inbound") is
/// the first line; this is the second.
///
/// The two-entry design is deliberate: if the relay client panics after
/// the stub is written but before this entry is written, the audit trail
/// still captures that the inbound request was received. The final entry
/// records the outcome — success or failure — plus token counts and cost.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuditProxyEntry {
    /// Explicit discriminator field (contract v0.2.0). Always `"audit-proxy"`.
    #[serde(default = "default_entry_type_audit_proxy")]
    pub entry_type: String,
    pub audit_id: String,
    pub completed_at: DateTime<Utc>,
    pub module_id: ModuleId,
    pub purpose: String,
    pub provider: String,
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caller_request_id: Option<String>,
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub cost_usd: f64,
    pub latency_ms: u64,
    /// `"ok"` on success; `"upstream-error"` when the relay call failed.
    pub status: String,
    /// Present when `status == "upstream-error"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

/// Single audit ledger entry for a `POST /v1/audit/capture` call (PS.4 step 4).
///
/// Written when a cross-cluster caller pushes a local-work event to the Doorman
/// for central audit-trail recording. Unlike the two-entry proxy design (stub +
/// final), capture is single-entry: the work already happened locally; there is
/// no upstream call to instrument.
///
/// The `captured_at` field is the Doorman's receipt timestamp; `event_at` is
/// the caller's timestamp from the request. Both are preserved so downstream
/// analysis can detect clock skew between clusters.
///
/// Note: this struct has two distinct string fields whose names are similar but
/// carry different semantics:
/// - `entry_type`: discriminator for the JSONL entry kind; always `"audit-capture"`.
/// - `event_type`: the kind of local work that was captured (e.g. `"prose-edit"`,
///   `"anchor-event"`). This is the caller-supplied vocabulary from the request.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuditCaptureEntry {
    /// Explicit discriminator field (contract v0.2.0). Always `"audit-capture"`.
    /// Distinct from `event_type` (which describes the captured local-work kind).
    #[serde(default = "default_entry_type_audit_capture")]
    pub entry_type: String,
    pub audit_id: String,
    pub module_id: ModuleId,
    /// The kind of local work captured (e.g. `"prose-edit"`, `"anchor-event"`).
    /// Not the same as `entry_type`; this comes from the caller's request.
    pub event_type: String,
    pub source: String,
    pub status: String,
    /// Caller's clock at the time the local work occurred (RFC 3339, parsed
    /// from the request's `event_at` field).
    pub event_at: DateTime<Utc>,
    /// Doorman's clock at the time the capture request was received.
    pub captured_at: DateTime<Utc>,
    /// Event-specific payload (untyped JSON object; future steps may validate
    /// per-event-type schemas).
    pub payload: serde_json::Value,
    /// Optional caller request correlation ID for cross-system tracing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caller_request_id: Option<String>,
}

/// Audit ledger entry for a `POST /v1/extract` call.
///
/// Written once per extraction attempt — after the Yo-Yo call completes (or
/// is deferred). A single-entry design: unlike audit_proxy there is no
/// provider-relay; the Doorman itself makes the inference call, so one entry
/// captures the full outcome.
///
/// `entry_type` is always `"extract"` (PS.4 discriminator for extraction
/// pipeline, distinct from `"chat-completion"` to preserve audit-filter
/// correctness — SYS-ADR-10).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExtractionAuditEntry {
    /// Explicit discriminator field. Always `"extract"`.
    #[serde(default = "default_entry_type_extract")]
    pub entry_type: String,
    pub timestamp_utc: DateTime<Utc>,
    pub request_id: RequestId,
    pub module_id: ModuleId,
    /// `true` when entities were successfully extracted and parsed.
    pub extraction_ok: bool,
    /// `true` when the request was deferred (Yo-Yo unavailable).
    pub deferred: bool,
    /// Number of entities returned (0 when deferred).
    pub entities_count: usize,
    /// `"yoyo_trainer"` on success, `"deferred"` otherwise.
    pub tier_used: String,
    /// Elapsed time for the Yo-Yo call, or circuit-check time when deferred.
    pub latency_ms: u64,
    /// Model identifier returned by the upstream tier (empty string when deferred).
    pub model: String,
    /// Upstream inference cost in USD (0.0 when deferred or Tier A).
    pub cost_usd: f64,
    /// `true` when the caller attested the payload was sanitised before dispatch.
    pub sanitised_outbound: bool,
    /// Kebab-case defer reason; present when `deferred: true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defer_reason: Option<String>,
    /// Upstream error message; present on Yo-Yo call failure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

impl AuditLedger {
    /// Write a stub entry for a `POST /v1/audit/proxy` call. This is the
    /// PS.4 step 1 paper trail: we capture the inbound request shape before
    /// attempting (or declining) the upstream call.
    ///
    /// The `entry_type` field is forced to `"audit-proxy-stub"` at write time.
    pub fn append_proxy_stub(&self, entry: &AuditProxyStubEntry) -> Result<()> {
        // Force the canonical entry_type at write time (contract v0.2.0).
        let mut entry = entry.clone();
        entry.entry_type = ENTRY_TYPE_AUDIT_PROXY_STUB.to_string();
        let path = self.path_for(&entry.inbound_at);
        let line = serde_json::to_vec(&entry)?;
        let _guard = self.inner.lock().expect("audit ledger mutex poisoned");
        let file = OpenOptions::new().create(true).append(true).open(&path)?;
        let mut writer = BufWriter::new(file);
        writer.write_all(&line)?;
        writer.write_all(b"\n")?;
        writer.flush()?;
        Ok(())
    }

    /// Write the final outcome entry for a `POST /v1/audit/proxy` call (PS.4
    /// step 2). This is the second JSONL line for the same `audit_id`; the
    /// stub entry is always written first.
    ///
    /// `completed_at` is supplied by the caller (the handler) so the timestamp
    /// records when the relay call returned, not when the ledger write happened.
    ///
    /// The `entry_type` field is forced to `"audit-proxy"` at write time.
    pub fn append_proxy_entry(&self, entry: &AuditProxyEntry) -> Result<()> {
        // Force the canonical entry_type at write time (contract v0.2.0).
        let mut entry = entry.clone();
        entry.entry_type = ENTRY_TYPE_AUDIT_PROXY.to_string();
        let path = self.path_for(&entry.completed_at);
        let line = serde_json::to_vec(&entry)?;
        let _guard = self.inner.lock().expect("audit ledger mutex poisoned");
        let file = OpenOptions::new().create(true).append(true).open(&path)?;
        let mut writer = BufWriter::new(file);
        writer.write_all(&line)?;
        writer.write_all(b"\n")?;
        writer.flush()?;
        Ok(())
    }

    /// Write a single audit capture entry for a `POST /v1/audit/capture` call
    /// (PS.4 step 4). Unlike the two-entry proxy design, capture writes exactly
    /// one entry — the work already happened locally; there is no upstream call
    /// to instrument. `captured_at` is the Doorman's clock at receipt time.
    ///
    /// The `entry_type` field is forced to `"audit-capture"` at write time.
    pub fn append_capture_entry(&self, entry: &AuditCaptureEntry) -> Result<()> {
        // Force the canonical entry_type at write time (contract v0.2.0).
        let mut entry = entry.clone();
        entry.entry_type = ENTRY_TYPE_AUDIT_CAPTURE.to_string();
        let path = self.path_for(&entry.captured_at);
        let line = serde_json::to_vec(&entry)?;
        let _guard = self.inner.lock().expect("audit ledger mutex poisoned");
        let file = OpenOptions::new().create(true).append(true).open(&path)?;
        let mut writer = BufWriter::new(file);
        writer.write_all(&line)?;
        writer.write_all(b"\n")?;
        writer.flush()?;
        Ok(())
    }

    /// Write one extraction audit entry for a `POST /v1/extract` call.
    ///
    /// Single-entry design: written after the Yo-Yo call completes or the
    /// request is deferred. `entry_type` is forced to `"extract"` at write
    /// time regardless of the caller's field value.
    pub fn append_extract_entry(&self, entry: &ExtractionAuditEntry) -> Result<()> {
        let mut entry = entry.clone();
        entry.entry_type = ENTRY_TYPE_EXTRACT.to_string();
        let path = self.path_for(&entry.timestamp_utc);
        let line = serde_json::to_vec(&entry)?;
        let _guard = self.inner.lock().expect("audit ledger mutex poisoned");
        let file = OpenOptions::new().create(true).append(true).open(&path)?;
        let mut writer = BufWriter::new(file);
        writer.write_all(&line)?;
        writer.write_all(b"\n")?;
        writer.flush()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn appends_round_trips_via_jsonl() {
        let tmp = tempdir();
        let ledger = AuditLedger::new(tmp.clone()).unwrap();
        let entry = AuditEntry {
            entry_type: ENTRY_TYPE_CHAT_COMPLETION.to_string(),
            timestamp_utc: Utc::now(),
            request_id: RequestId::new(),
            module_id: ModuleId::from_str("foundry").unwrap(),
            tier: Tier::Local,
            model: "olmo-3-7b-q4".into(),
            inference_ms: 412,
            cost_usd: 0.0,
            sanitised_outbound: true,
            completion_status: CompletionStatus::Ok,
            error_message: None,
        };
        ledger.append(&entry).unwrap();
        ledger.append(&entry).unwrap();

        let path = ledger.path_for(&entry.timestamp_utc);
        let contents = std::fs::read_to_string(&path).unwrap();
        assert_eq!(contents.lines().count(), 2);
        for line in contents.lines() {
            let parsed: AuditEntry = serde_json::from_str(line).unwrap();
            assert_eq!(
                parsed.entry_type, ENTRY_TYPE_CHAT_COMPLETION,
                "serialised AuditEntry must carry entry_type = 'chat-completion'"
            );
        }
    }

    /// Verify that an `AuditEntry` serialised WITHOUT the `entry_type` field
    /// (as old JSONL written before contract v0.2.0 would look) still
    /// deserialises correctly, defaulting to the canonical value.
    #[test]
    fn audit_entry_missing_entry_type_field_deserialises_with_correct_default() {
        // Minimal AuditEntry JSON without the entry_type field — simulates an
        // entry written by code predating contract v0.2.0.
        let old_json = r#"{
            "timestamp_utc": "2026-04-28T00:00:00Z",
            "request_id": "01930000-0000-7000-0000-000000000001",
            "module_id": "foundry",
            "tier": "local",
            "model": "olmo-3-7b-q4",
            "inference_ms": 412,
            "cost_usd": 0.0,
            "sanitised_outbound": true,
            "completion_status": "ok"
        }"#;
        let parsed: AuditEntry =
            serde_json::from_str(old_json).expect("old AuditEntry JSON must deserialise");
        assert_eq!(
            parsed.entry_type, ENTRY_TYPE_CHAT_COMPLETION,
            "old AuditEntry lacking entry_type must default to 'chat-completion'"
        );
    }

    /// Verify that old JSONL entries for the other three entry types also
    /// deserialise correctly with correct defaults when entry_type is absent.
    #[test]
    fn all_entry_types_default_correctly_when_entry_type_field_absent() {
        // AuditProxyStubEntry without entry_type.
        let stub_json = r#"{
            "audit_id": "stub-001",
            "inbound_at": "2026-04-28T00:00:00Z",
            "module_id": "foundry",
            "purpose": "editorial-refinement",
            "provider": "anthropic",
            "model": "claude-opus-4-7",
            "request_messages_count": 1,
            "status": "inbound"
        }"#;
        let stub: AuditProxyStubEntry =
            serde_json::from_str(stub_json).expect("old stub JSON must deserialise");
        assert_eq!(stub.entry_type, ENTRY_TYPE_AUDIT_PROXY_STUB);

        // AuditProxyEntry without entry_type.
        let proxy_json = r#"{
            "audit_id": "stub-001",
            "completed_at": "2026-04-28T00:00:01Z",
            "module_id": "foundry",
            "purpose": "editorial-refinement",
            "provider": "anthropic",
            "model": "claude-opus-4-7",
            "prompt_tokens": 80,
            "completion_tokens": 32,
            "cost_usd": 0.0001,
            "latency_ms": 500,
            "status": "ok"
        }"#;
        let proxy: AuditProxyEntry =
            serde_json::from_str(proxy_json).expect("old proxy JSON must deserialise");
        assert_eq!(proxy.entry_type, ENTRY_TYPE_AUDIT_PROXY);

        // AuditCaptureEntry without entry_type.
        let capture_json = r#"{
            "audit_id": "cap-001",
            "module_id": "foundry",
            "event_type": "anchor-event",
            "source": "project-data",
            "status": "ok",
            "event_at": "2026-04-28T00:00:00Z",
            "captured_at": "2026-04-28T00:00:01Z",
            "payload": {}
        }"#;
        let capture: AuditCaptureEntry =
            serde_json::from_str(capture_json).expect("old capture JSON must deserialise");
        assert_eq!(capture.entry_type, ENTRY_TYPE_AUDIT_CAPTURE);
    }

    // ---- Audit-ledger error-path tests (PS.6 chunk #6 tail) ----

    /// When `HOME` is unset, `AuditLedger::default_for_user()` must return
    /// `DoormanError::HomeUnset` — not panic.
    ///
    /// The test saves and restores the `HOME` env var around the assertion so
    /// it does not leak mutations to other tests running in the same process.
    /// Note: Rust test threads share the process env; this test is deliberately
    /// single-threaded (no async, no `tokio::test`) so there is no race window
    /// with other tests that read `HOME`. The save-and-restore is a best-effort
    /// courtesy for tests that run in this same binary.
    #[test]
    fn default_for_user_with_home_unset_returns_home_unset_error() {
        let saved = std::env::var_os("HOME");
        std::env::remove_var("HOME");
        let result = AuditLedger::default_for_user();
        // Restore HOME before any assertions (so a panic doesn't leak the unset state).
        if let Some(v) = saved {
            std::env::set_var("HOME", v);
        }
        // If HOME was already unset before this test, nothing to restore.
        match result {
            Err(crate::error::DoormanError::HomeUnset) => {}
            Err(other) => panic!("expected HomeUnset, got {other:?}"),
            Ok(_) => panic!("expected HomeUnset error but got Ok"),
        }
    }

    /// When the ledger directory path cannot be created (parent is
    /// read-only), `AuditLedger::new()` must return a `DoormanError::LedgerIo`
    /// — not panic.
    #[test]
    fn new_with_readonly_parent_returns_ledger_io_error() {
        let parent = tempdir();
        // Make the parent read-only so subdirectory creation fails.
        let metadata = std::fs::metadata(&parent).unwrap();
        let mut perms = metadata.permissions();
        use std::os::unix::fs::PermissionsExt;
        perms.set_mode(0o555);
        std::fs::set_permissions(&parent, perms.clone()).unwrap();

        let unreachable_child = parent.join("audit-subdir");
        let result = AuditLedger::new(&unreachable_child);

        // Restore write permissions so the TempDir cleanup can delete it.
        perms.set_mode(0o755);
        std::fs::set_permissions(&parent, perms).unwrap();

        match result {
            Err(crate::error::DoormanError::LedgerIo(_)) => {}
            Err(other) => panic!("expected LedgerIo, got {other:?}"),
            Ok(_) => panic!("expected LedgerIo error but got Ok (maybe running as root?)"),
        }
    }

    /// When the ledger directory exists but the JSONL file inside it is not
    /// writable, `AuditLedger::append()` must return a `DoormanError::LedgerIo`
    /// — not panic.
    #[test]
    fn append_to_readonly_directory_returns_ledger_io_error() {
        use std::os::unix::fs::PermissionsExt;

        // Create the ledger with a valid (writable) directory first.
        let dir = tempdir();
        let ledger = AuditLedger::new(dir.clone()).unwrap();

        let entry = AuditEntry {
            entry_type: ENTRY_TYPE_CHAT_COMPLETION.to_string(),
            timestamp_utc: Utc::now(),
            request_id: RequestId::new(),
            module_id: ModuleId::from_str("foundry").unwrap(),
            tier: Tier::Local,
            model: "olmo-3-7b-q4".into(),
            inference_ms: 1,
            cost_usd: 0.0,
            sanitised_outbound: true,
            completion_status: CompletionStatus::Ok,
            error_message: None,
        };

        // Now make the directory read-only so subsequent file opens fail.
        let mut perms = std::fs::metadata(&dir).unwrap().permissions();
        perms.set_mode(0o555);
        std::fs::set_permissions(&dir, perms.clone()).unwrap();

        let result = ledger.append(&entry);

        // Restore write permissions so TempDir cleanup succeeds.
        perms.set_mode(0o755);
        std::fs::set_permissions(&dir, perms).unwrap();

        match result {
            Err(crate::error::DoormanError::LedgerIo(_)) => {}
            Err(other) => panic!("expected LedgerIo, got {other:?}"),
            Ok(()) => panic!("expected LedgerIo error but got Ok (maybe running as root?)"),
        }
    }

    fn tempdir() -> PathBuf {
        let dir =
            std::env::temp_dir().join(format!("slm-doorman-ledger-test-{}", uuid_like_suffix()));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn uuid_like_suffix() -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        format!("{nanos:x}")
    }
}
