// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Daily cost ledger — Phase 3 (P3-3.5-partial) of
//! learning-loop-master-plan-2026-05-18.md.
//!
//! Aggregates per-response `cost_usd` from the Doorman audit ledger into
//! a daily rollup at
//! `data/cost-ledger/<tier>-YYYY-MM-DD.jsonl`. Each line is one append
//! per request:
//!
//! ```json
//! {"ts":"2026-05-18T10:00:00Z","request_id":"...","tier":"yoyo",
//!  "model":"olmo-2-7b","cost_usd":0.0023,"inference_ms":420,
//!  "adapter_version":null}
//! ```
//!
//! Daily rollups are computed on demand by `daily_rollup(date)` which
//! sums the per-request lines for that date.
//!
//! **Scope:** Tier-B (Yo-Yo) cost tracking is the primary use case
//! tonight; Tier-C is excluded by operator directive 2026-05-18 (no
//! Commercial API key in production). The ledger structure is
//! tier-agnostic — when Tier C is re-enabled it transparently records
//! those rows too.
//!
//! **Kill-switch:** the per-day spend cap + automatic 503 enforcement is
//! deferred to P3-3.5-followup (alongside any future Tier C re-enable).
//! This module is the ledger half only.

use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

/// One row in the cost ledger — one HTTP response = one row.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CostRow {
    /// ISO 8601 UTC timestamp of the response.
    pub ts: String,
    /// Doorman request ID — cross-reference with `audit/<date>.jsonl`.
    pub request_id: String,
    /// `local` | `yoyo` | `external`.
    pub tier: String,
    /// Model identifier reported by the upstream tier.
    pub model: String,
    /// Inference cost in USD. For Tier A (local) this is 0.0 since the
    /// VM is already paid for; included for uniformity.
    pub cost_usd: f64,
    /// Round-trip latency in milliseconds.
    pub inference_ms: u64,
    /// Adapter version that served the request, if reported.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub adapter_version: Option<String>,
}

/// Append-only daily cost ledger writer. One process owns one
/// `CostLedger`; the internal mutex serialises concurrent writes from
/// multiple request handlers.
pub struct CostLedger {
    base_dir: PathBuf,
    inner: Mutex<()>,
}

impl CostLedger {
    /// Construct a ledger rooted at `base_dir`. Creates the directory if
    /// it does not exist.
    pub fn new(base_dir: impl Into<PathBuf>) -> std::io::Result<Self> {
        let base_dir = base_dir.into();
        std::fs::create_dir_all(&base_dir)?;
        Ok(Self {
            base_dir,
            inner: Mutex::new(()),
        })
    }

    /// Standard entrypoint — reads `FOUNDRY_ROOT/data/cost-ledger/`.
    pub fn from_env() -> std::io::Result<Self> {
        let base = std::env::var_os("FOUNDRY_ROOT")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("/srv/foundry"))
            .join("data")
            .join("cost-ledger");
        Self::new(base)
    }

    /// Append one row to the day's ledger file. The file is opened in
    /// append mode; concurrent writers from different processes are
    /// safe at the kernel level for writes under PIPE_BUF, and the
    /// in-process mutex keeps multi-line entries whole.
    pub fn append(&self, row: &CostRow) -> std::io::Result<()> {
        let path = self.path_for(&row.ts);
        let line = match serde_json::to_string(row) {
            Ok(s) => s,
            Err(e) => {
                return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, e))
            }
        };
        let _guard = self.inner.lock().expect("cost ledger mutex poisoned");
        let mut f = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)?;
        f.write_all(line.as_bytes())?;
        f.write_all(b"\n")?;
        f.flush()?;
        Ok(())
    }

    /// Compute a daily rollup for the given UTC date (`YYYY-MM-DD`).
    /// Reads the file and aggregates per-tier and per-model totals.
    ///
    /// Returns `Ok(DailyRollup{..})` even when the file is missing — an
    /// empty rollup is the correct "no requests this day" answer.
    pub fn daily_rollup(&self, date: &str) -> std::io::Result<DailyRollup> {
        let path = self.base_dir.join(format!("{date}.jsonl"));
        let mut rollup = DailyRollup {
            date: date.to_string(),
            request_count: 0,
            total_cost_usd: 0.0,
            total_inference_ms: 0,
            by_tier: std::collections::HashMap::new(),
            by_model: std::collections::HashMap::new(),
        };
        if !path.exists() {
            return Ok(rollup);
        }
        let f = std::fs::File::open(&path)?;
        let reader = BufReader::new(f);
        for line in reader.lines() {
            let line = match line {
                Ok(l) => l,
                Err(_) => continue,
            };
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            let row: CostRow = match serde_json::from_str(trimmed) {
                Ok(r) => r,
                Err(_) => continue,
            };
            rollup.request_count += 1;
            rollup.total_cost_usd += row.cost_usd;
            rollup.total_inference_ms += row.inference_ms;
            *rollup.by_tier.entry(row.tier.clone()).or_default() += row.cost_usd;
            *rollup.by_model.entry(row.model.clone()).or_default() += row.cost_usd;
        }
        Ok(rollup)
    }

    fn path_for(&self, ts: &str) -> PathBuf {
        // ts is ISO 8601 with seconds precision. Slice off the date.
        let date = ts.get(0..10).unwrap_or("1970-01-01");
        self.base_dir.join(format!("{date}.jsonl"))
    }

    /// Path to the underlying directory (diagnostic surface).
    pub fn base_dir(&self) -> &std::path::Path {
        &self.base_dir
    }
}

/// Global cost ledger handle — installed by `init()` at process startup
/// (analogous to `slm_doorman_server::metrics::HANDLE`). `append_global()`
/// is a no-op when the handle isn't set, so calling it from hot paths
/// like `router::write_audit` is safe even during tests that don't
/// install the ledger.
static GLOBAL: OnceLock<CostLedger> = OnceLock::new();

/// Install the global cost ledger. Idempotent — the first install wins;
/// subsequent calls log + ignore. Failure to install (e.g. permission
/// error on the data directory) is returned to the caller but should
/// NOT block Doorman startup; callers are expected to log + continue.
///
/// P3-3.5-followup: enables `router::write_audit` to record per-response
/// cost rows without threading the ledger through every layer of the
/// dispatch path.
pub fn init(ledger: CostLedger) -> Result<(), &'static str> {
    GLOBAL
        .set(ledger)
        .map_err(|_| "cost ledger already installed")
}

/// Append to the global cost ledger if installed; no-op otherwise.
/// Errors are swallowed with a tracing::warn — cost-ledger failure
/// must never propagate into the response path.
pub fn append_global(row: &CostRow) {
    if let Some(ledger) = GLOBAL.get() {
        if let Err(e) = ledger.append(row) {
            tracing::warn!(
                target: "slm_doorman::cost_ledger",
                error = %e,
                request_id = %row.request_id,
                "cost ledger append failed (non-fatal)"
            );
        }
    }
}

/// Diagnostic: returns true when the global cost ledger has been installed.
pub fn is_initialized() -> bool {
    GLOBAL.get().is_some()
}

/// Aggregated cost summary for one UTC day.
#[derive(Debug, Serialize)]
pub struct DailyRollup {
    pub date: String,
    pub request_count: usize,
    pub total_cost_usd: f64,
    pub total_inference_ms: u64,
    /// Per-tier $ subtotal (`local` | `yoyo` | `external`).
    pub by_tier: std::collections::HashMap<String, f64>,
    /// Per-model $ subtotal.
    pub by_model: std::collections::HashMap<String, f64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp_dir() -> PathBuf {
        let p = std::env::temp_dir().join(format!(
            "cost-ledger-{}",
            chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0)
        ));
        std::fs::create_dir_all(&p).unwrap();
        p
    }

    #[test]
    fn append_and_rollup() {
        let dir = tmp_dir();
        let ledger = CostLedger::new(&dir).unwrap();

        ledger
            .append(&CostRow {
                ts: "2026-05-18T10:00:00Z".to_string(),
                request_id: "req-1".to_string(),
                tier: "yoyo".to_string(),
                model: "olmo-2-7b".to_string(),
                cost_usd: 0.0025,
                inference_ms: 420,
                adapter_version: None,
            })
            .unwrap();
        ledger
            .append(&CostRow {
                ts: "2026-05-18T11:00:00Z".to_string(),
                request_id: "req-2".to_string(),
                tier: "yoyo".to_string(),
                model: "olmo-2-7b".to_string(),
                cost_usd: 0.0035,
                inference_ms: 510,
                adapter_version: Some("coding-lora-v1".to_string()),
            })
            .unwrap();
        ledger
            .append(&CostRow {
                ts: "2026-05-18T12:00:00Z".to_string(),
                request_id: "req-3".to_string(),
                tier: "local".to_string(),
                model: "olmo-2-1b".to_string(),
                cost_usd: 0.0,
                inference_ms: 980,
                adapter_version: None,
            })
            .unwrap();

        let rollup = ledger.daily_rollup("2026-05-18").unwrap();
        assert_eq!(rollup.request_count, 3);
        assert!((rollup.total_cost_usd - 0.006).abs() < 1e-9);
        assert_eq!(rollup.total_inference_ms, 420 + 510 + 980);
        let yoyo_cost = rollup.by_tier.get("yoyo").copied().unwrap_or(0.0);
        assert!((yoyo_cost - 0.006).abs() < 1e-9);
        let local_cost = rollup.by_tier.get("local").copied().unwrap_or(0.0);
        assert!((local_cost - 0.0).abs() < 1e-9);

        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn rollup_missing_date_returns_empty() {
        let dir = tmp_dir();
        let ledger = CostLedger::new(&dir).unwrap();
        let rollup = ledger.daily_rollup("1970-01-01").unwrap();
        assert_eq!(rollup.request_count, 0);
        assert_eq!(rollup.total_cost_usd, 0.0);
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn rollup_skips_malformed_rows() {
        let dir = tmp_dir();
        let ledger = CostLedger::new(&dir).unwrap();
        ledger
            .append(&CostRow {
                ts: "2026-05-18T10:00:00Z".to_string(),
                request_id: "ok-1".to_string(),
                tier: "yoyo".to_string(),
                model: "olmo".to_string(),
                cost_usd: 0.005,
                inference_ms: 100,
                adapter_version: None,
            })
            .unwrap();
        // Append a malformed line manually.
        let path = dir.join("2026-05-18.jsonl");
        use std::io::Write;
        let mut f = std::fs::OpenOptions::new()
            .append(true)
            .open(&path)
            .unwrap();
        writeln!(f, "{{not valid json").unwrap();
        ledger
            .append(&CostRow {
                ts: "2026-05-18T11:00:00Z".to_string(),
                request_id: "ok-2".to_string(),
                tier: "local".to_string(),
                model: "olmo".to_string(),
                cost_usd: 0.0,
                inference_ms: 200,
                adapter_version: None,
            })
            .unwrap();

        let rollup = ledger.daily_rollup("2026-05-18").unwrap();
        assert_eq!(rollup.request_count, 2);
        std::fs::remove_dir_all(&dir).ok();
    }
}
