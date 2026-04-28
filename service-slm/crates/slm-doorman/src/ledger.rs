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

use std::fs::{self, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use slm_core::{ModuleId, RequestId, Tier};

use crate::error::{DoormanError, Result};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuditEntry {
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
    pub fn append(&self, entry: &AuditEntry) -> Result<()> {
        let path = self.path_for(&entry.timestamp_utc);
        let line = serde_json::to_vec(entry)?;
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
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuditCaptureEntry {
    pub audit_id: String,
    pub module_id: ModuleId,
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

impl AuditLedger {
    /// Write a stub entry for a `POST /v1/audit/proxy` call. This is the
    /// PS.4 step 1 paper trail: we capture the inbound request shape before
    /// attempting (or declining) the upstream call.
    pub fn append_proxy_stub(&self, entry: &AuditProxyStubEntry) -> Result<()> {
        let path = self.path_for(&entry.inbound_at);
        let line = serde_json::to_vec(entry)?;
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
    pub fn append_proxy_entry(&self, entry: &AuditProxyEntry) -> Result<()> {
        let path = self.path_for(&entry.completed_at);
        let line = serde_json::to_vec(entry)?;
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
    pub fn append_capture_entry(&self, entry: &AuditCaptureEntry) -> Result<()> {
        let path = self.path_for(&entry.captured_at);
        let line = serde_json::to_vec(entry)?;
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
            let _: AuditEntry = serde_json::from_str(line).unwrap();
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
