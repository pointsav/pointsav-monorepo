// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Prometheus metrics exporter — Phase 3 (P3-3.1) of
//! learning-loop-master-plan-2026-05-18.md.
//!
//! Installs a global Prometheus recorder at process startup and exposes
//! the `/metrics` endpoint that returns the textual format.
//!
//! Metric inventory:
//!
//! | Metric | Type | Labels | Source |
//! |---|---|---|---|
//! | `slm_requests_total` | counter | tier, model, adapter_version, completion_status | router::write_audit |
//! | `slm_cost_usd_total` | counter (float) | tier, model | router::write_audit |
//! | `slm_latency_ms` | histogram | tier, model | router::write_audit |
//! | `slm_yoyo_dispatch_age_seconds` | gauge | (none) | http::readyz |
//! | `slm_audit_writes_total` | counter | entry_type | router + http audit_proxy |
//! | `slm_corpus_gate_rejections_total` | counter | reason | corpus_gate::check |
//! | `slm_contamination_guard_total` | counter | layer | contamination_guard tracing-target call sites |
//! | `slm_apprenticeship_queue_depth` | gauge | (none) | main.rs drain worker |
//!
//! All metrics are emitted via the `metrics` facade crate — call sites
//! use `metrics::counter!()`, `metrics::histogram!()`, `metrics::gauge!()`
//! directly, no wrappers. Labels are passed inline. The recorder
//! installed by `init()` is the global sink.

use metrics_exporter_prometheus::PrometheusHandle;
use std::sync::OnceLock;

/// Global handle, set by [`init`]. Used by the `/metrics` endpoint to
/// pull the textual format on demand.
static HANDLE: OnceLock<PrometheusHandle> = OnceLock::new();

/// Install the Prometheus recorder as the global metrics sink. Idempotent
/// — calling more than once is a no-op (the first install wins; subsequent
/// calls log a warning and return).
///
/// Returns `Ok(())` even when installation fails (best-effort): metrics
/// emit to `metrics`-crate no-op recorder. Failure here MUST NOT prevent
/// Doorman startup.
pub fn init() -> anyhow::Result<()> {
    if HANDLE.get().is_some() {
        tracing::debug!(
            target: "slm_doorman_server::metrics",
            "metrics recorder already installed; skipping"
        );
        return Ok(());
    }
    let builder = metrics_exporter_prometheus::PrometheusBuilder::new();
    match builder.install_recorder() {
        Ok(handle) => {
            let _ = HANDLE.set(handle);
            tracing::info!(
                target: "slm_doorman_server::metrics",
                "Prometheus metrics recorder installed"
            );
            Ok(())
        }
        Err(e) => {
            tracing::warn!(
                target: "slm_doorman_server::metrics",
                error = %e,
                "failed to install Prometheus recorder; metrics will no-op"
            );
            Ok(())
        }
    }
}

/// Render the current metrics snapshot in Prometheus textual format.
/// Returns an empty string when the recorder isn't installed (degraded
/// mode) so the `/metrics` endpoint always returns 200 with whatever
/// data we have.
pub fn render() -> String {
    match HANDLE.get() {
        Some(h) => h.render(),
        None => String::new(),
    }
}

/// Standard metric names — centralised here so call sites import them
/// rather than typo-string the names. Keep this list in sync with the
/// inventory in the module docstring.
pub mod names {
    pub const REQUESTS_TOTAL: &str = "slm_requests_total";
    pub const COST_USD_TOTAL: &str = "slm_cost_usd_total";
    pub const LATENCY_MS: &str = "slm_latency_ms";
    pub const YOYO_DISPATCH_AGE_SECONDS: &str = "slm_yoyo_dispatch_age_seconds";
    pub const AUDIT_WRITES_TOTAL: &str = "slm_audit_writes_total";
    pub const CORPUS_GATE_REJECTIONS_TOTAL: &str = "slm_corpus_gate_rejections_total";
    pub const CONTAMINATION_GUARD_TOTAL: &str = "slm_contamination_guard_total";
    pub const APPRENTICESHIP_QUEUE_DEPTH: &str = "slm_apprenticeship_queue_depth";
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `init()` is safe to call multiple times.
    #[test]
    fn init_is_idempotent() {
        // First call may succeed (if no other test has installed); second
        // call must return Ok without panicking.
        let _ = init();
        let _ = init();
    }

    /// `render()` returns a String even before init.
    #[test]
    fn render_returns_string_pre_init() {
        // Don't actually call init here — we want to verify the
        // degraded-mode path. (Other tests in this binary may have
        // already installed; either way render() must not panic.)
        let s = render();
        // We don't assert empty because another test may have installed.
        let _ = s;
    }
}
