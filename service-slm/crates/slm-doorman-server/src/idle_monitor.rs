// SPDX-License-Identifier: Apache-2.0

//! Yo-Yo idle monitor (B5) — replaces yoyo-manual/yoyo-idle-check.sh.
//!
//! Polls the Yo-Yo VM `/metrics` endpoint every 5 minutes for an active-request
//! counter. When the VM has been idle (zero active requests) longer than
//! `SLM_YOYO_IDLE_MINUTES` (default 30), sends a GCP `instances.stop` request
//! via [`slm_doorman::GcpComputeClient`] (workspace Service Account ADC token
//! from the GCE metadata server, same auth path `scripts/start-yoyo.sh` and
//! `nightly-run.sh` already rely on for the *other* real recovery path —
//! GCP-side preemption).
//!
//! Production-readiness roadmap Tier 1.1 (2026-09-14): this previously sent
//! `instances.delete` via a hand-rolled request, reasoning that a nightly
//! `yoyo-daily-cycle.sh` `instances.create` call would recreate the VM (GCE's
//! documented capacity-queue behavior only applies to `create`, not `start`).
//! Verified live: `yoyo-daily-cycle.sh` does not exist anywhere in this repo
//! despite extensive prior development history referencing it (real drift,
//! not this file's mistake) — the actual nightly automation
//! (`scripts/nightly-run.sh` → `scripts/start-yoyo.sh`) can only `start` an
//! already-existing instance; `start-yoyo.sh`'s own header explicitly treats
//! the VM as "TERMINATED (preempted)" and recovers it in-zone with its own
//! stockout-retry loop (`--retry-cycles`/`--retry-wait-seconds`) — i.e. this
//! codebase already has a proven, working capacity-queue substitute for
//! `instances.start`, it just was never connected to *this* idle-shutdown
//! path. `delete` left nothing able to bring Tier B back — this is very
//! plausibly the root cause of prior multi-hour, human-intervention-required
//! Tier B outages. `stop` (not `delete`) is the correct action here: the
//! boot disk is untouched either way (a stopped instance keeps its disks
//! regardless of `autoDelete`), and `start-yoyo.sh` already knows exactly
//! how to recover a stopped instance.
//!
//! The monitor is spawned as a background tokio task in `main.rs` only when all
//! four GCP env vars are set (`SLM_YOYO_GCP_PROJECT`, `SLM_YOYO_GCP_ZONE`,
//! `SLM_YOYO_GCP_INSTANCE`, and `SLM_YOYO_ENDPOINT`). Absent any of these,
//! `IdleMonitorConfig::from_env()` returns `None` and no task is spawned.
//!
//! Env vars:
//!   SLM_YOYO_ENDPOINT        Yo-Yo base URL (also consumed by Tier B client)
//!   SLM_YOYO_IDLE_MINUTES    idle threshold in minutes; default 30
//!   SLM_YOYO_METRICS_KEY     Prometheus metric name for active-request count;
//!                             default: llama_active_slots_total (llama-server);
//!                             set to vllm:num_requests_running for vLLM
//!   SLM_YOYO_GCP_PROJECT     GCP project id (e.g. pointsav-public)
//!   SLM_YOYO_GCP_ZONE        GCP zone (e.g. us-west1-b)
//!   SLM_YOYO_GCP_INSTANCE    GCP instance name (e.g. yoyo-tier-b-1)

use std::time::{Duration, Instant};

use slm_doorman::GcpComputeClient;
use tracing::{info, warn};

use crate::http::parse_prometheus_line;

const POLL_INTERVAL: Duration = Duration::from_secs(300); // 5 minutes
const HTTP_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Clone, Debug)]
pub struct IdleMonitorConfig {
    pub yoyo_endpoint: String,
    pub yoyo_bearer: Option<String>,
    pub idle_threshold: Duration,
    pub metrics_key: String,
    pub gcp_project: String,
    pub gcp_zone: String,
    pub gcp_instance: String,
}

impl IdleMonitorConfig {
    /// Constructs config from env vars. Returns `None` if any required var is absent.
    pub fn from_env() -> Option<Self> {
        let yoyo_endpoint = std::env::var("SLM_YOYO_ENDPOINT").ok()?;
        if yoyo_endpoint.is_empty() {
            return None;
        }
        let yoyo_bearer = std::env::var("SLM_YOYO_BEARER")
            .ok()
            .filter(|s| !s.is_empty());
        let idle_minutes: u64 = std::env::var("SLM_YOYO_IDLE_MINUTES")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(30);
        let metrics_key = std::env::var("SLM_YOYO_METRICS_KEY")
            .unwrap_or_else(|_| "llama_active_slots_total".to_string());
        let gcp_project = std::env::var("SLM_YOYO_GCP_PROJECT").ok()?;
        let gcp_zone = std::env::var("SLM_YOYO_GCP_ZONE").ok()?;
        let gcp_instance = std::env::var("SLM_YOYO_GCP_INSTANCE").ok()?;

        Some(Self {
            yoyo_endpoint,
            yoyo_bearer,
            idle_threshold: Duration::from_secs(idle_minutes * 60),
            metrics_key,
            gcp_project,
            gcp_zone,
            gcp_instance,
        })
    }
}

/// Run the idle monitor loop. Call via `tokio::spawn(run_idle_monitor(config))`.
pub async fn run_idle_monitor(config: IdleMonitorConfig) {
    let client = reqwest::Client::builder()
        .timeout(HTTP_TIMEOUT)
        .build()
        .unwrap_or_default();

    // Start the idle clock at task-spawn time so a cold VM doesn't get stopped
    // before its first request within the threshold window.
    let mut last_active = Instant::now();
    // Guard against sending repeated stop requests for the same idle period.
    let mut stop_sent = false;
    let gcp = GcpComputeClient::new(
        config.gcp_project.clone(),
        config.gcp_zone.clone(),
        config.gcp_instance.clone(),
    );

    info!(
        target: "slm_doorman::idle_monitor",
        endpoint = %config.yoyo_endpoint,
        idle_threshold_secs = config.idle_threshold.as_secs(),
        gcp_instance = %config.gcp_instance,
        "Yo-Yo idle monitor started"
    );

    loop {
        tokio::time::sleep(POLL_INTERVAL).await;

        match poll_active_slots(
            &client,
            &config.yoyo_endpoint,
            config.yoyo_bearer.as_deref(),
            &config.metrics_key,
        )
        .await
        {
            Some(0) => {
                // Metrics reachable, zero active slots — VM is up but idle.
                // Let the idle clock run; fire stop if threshold exceeded.
                let idle_secs = last_active.elapsed().as_secs();
                if !stop_sent && last_active.elapsed() >= config.idle_threshold {
                    warn!(
                        target: "slm_doorman::idle_monitor",
                        idle_secs,
                        project = %config.gcp_project,
                        zone = %config.gcp_zone,
                        instance = %config.gcp_instance,
                        "Yo-Yo idle threshold reached; sending GCP stop request (boot disk preserved; start-yoyo.sh recovers a stopped instance)"
                    );
                    match gcp.stop_instance().await {
                        Ok(_op_name) => {
                            info!(
                                target: "slm_doorman::idle_monitor",
                                instance = %config.gcp_instance,
                                "GCP stop request accepted"
                            );
                            stop_sent = true;
                        }
                        Err(reason) => {
                            warn!(
                                target: "slm_doorman::idle_monitor",
                                %reason,
                                "GCP stop request failed; will retry next cycle"
                            );
                        }
                    }
                }
            }
            Some(n) => {
                // VM is serving (n > 0) — reset idle clock.
                last_active = Instant::now();
                stop_sent = false;
                info!(
                    target: "slm_doorman::idle_monitor",
                    active_slots = n,
                    "Yo-Yo busy; idle clock reset"
                );
            }
            None => {
                // Metrics unreachable — VM is booting, stopped, or llama-server not
                // yet loaded. Reset the idle clock so we don't stop a VM that was
                // never serving. The nightly cycle's own hard-stop (Phase 8) is the
                // fallback when the VM is unreachable.
                last_active = Instant::now();
                stop_sent = false;
            }
        }
    }
}

/// Poll the Yo-Yo `/metrics` endpoint and extract the active-request counter
/// named by `metrics_key`. Returns `None` on network error, non-200 response,
/// or missing metric.
async fn poll_active_slots(
    client: &reqwest::Client,
    endpoint: &str,
    bearer: Option<&str>,
    metrics_key: &str,
) -> Option<u64> {
    let url = format!("{}/metrics", endpoint.trim_end_matches('/'));
    let mut req = client.get(&url);
    if let Some(token) = bearer {
        req = req.bearer_auth(token);
    }
    let resp = req.send().await.ok()?;
    if !resp.status().is_success() {
        return None;
    }
    let text = resp.text().await.ok()?;
    find_metric_value(&text, metrics_key)
}

/// Scan Prometheus text-exposition output for `metrics_key`'s value.
///
/// D5 fix (Fable bug-hunt audit, Session 24): this had the same naive-
/// Prometheus-parsing bug already fixed in `http.rs::status_tier_a` (see
/// `parse_prometheus_line`'s own doc comment for the full failure-mode
/// rationale — labeled metrics and a trailing timestamp field both broke
/// it), plus a second, distinct issue: the old code matched via
/// `line.starts_with(metrics_key)`, a *prefix* match rather than an exact
/// metric-name match — a metric sharing `metrics_key` as a literal prefix
/// (e.g. a hypothetical `requests_processing_total` when `metrics_key` is
/// `requests_processing`) would have matched wrongly, and it returned
/// immediately on the first prefix-matching line regardless of whether it
/// actually parsed, so it never fell through to check a later line for the
/// same key. Reusing the shared parser fixes both: exact name match, and
/// the loop keeps scanning past a line it can't parse instead of giving up
/// on the first (possibly wrong) match. Extracted as its own sync,
/// non-`reqwest`-dependent function so it's directly unit-testable — the
/// pre-fix test for this reimplemented the (buggy) matching logic inline
/// rather than exercising the real code path at all.
fn find_metric_value(text: &str, metrics_key: &str) -> Option<u64> {
    for line in text.lines() {
        if let Some((name, val)) = parse_prometheus_line(line) {
            if name == metrics_key {
                return Some(val as u64);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Mutex, OnceLock};

    // Serialize tests that mutate process-global env vars.
    static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    fn env_lock() -> &'static Mutex<()> {
        ENV_LOCK.get_or_init(|| Mutex::new(()))
    }

    #[test]
    fn from_env_returns_none_without_gcp_vars() {
        let _g = env_lock().lock().unwrap();
        // All GCP env vars unset in test environment — should return None.
        // (SLM_YOYO_ENDPOINT may or may not be set; we rely on GCP vars being absent.)
        std::env::remove_var("SLM_YOYO_GCP_PROJECT");
        std::env::remove_var("SLM_YOYO_GCP_ZONE");
        std::env::remove_var("SLM_YOYO_GCP_INSTANCE");
        // Even if endpoint is set, missing GCP vars → None
        std::env::set_var("SLM_YOYO_ENDPOINT", "http://127.0.0.1:8080");
        let result = IdleMonitorConfig::from_env();
        assert!(result.is_none());
        std::env::remove_var("SLM_YOYO_ENDPOINT");
    }

    #[test]
    fn from_env_builds_config_with_all_vars() {
        let _g = env_lock().lock().unwrap();
        std::env::set_var("SLM_YOYO_ENDPOINT", "http://1.2.3.4:8080");
        std::env::set_var("SLM_YOYO_IDLE_MINUTES", "45");
        std::env::set_var("SLM_YOYO_GCP_PROJECT", "my-project");
        std::env::set_var("SLM_YOYO_GCP_ZONE", "us-west1-a");
        std::env::set_var("SLM_YOYO_GCP_INSTANCE", "yoyo-tier-b-1");
        std::env::remove_var("SLM_YOYO_METRICS_KEY");
        let cfg = IdleMonitorConfig::from_env().expect("should build config");
        assert_eq!(cfg.idle_threshold, Duration::from_secs(45 * 60));
        assert_eq!(cfg.gcp_project, "my-project");
        assert_eq!(cfg.metrics_key, "llama_active_slots_total");
        std::env::remove_var("SLM_YOYO_ENDPOINT");
        std::env::remove_var("SLM_YOYO_IDLE_MINUTES");
        std::env::remove_var("SLM_YOYO_GCP_PROJECT");
        std::env::remove_var("SLM_YOYO_GCP_ZONE");
        std::env::remove_var("SLM_YOYO_GCP_INSTANCE");
    }

    #[test]
    fn from_env_builds_config_with_custom_metrics_key() {
        let _g = env_lock().lock().unwrap();
        std::env::set_var("SLM_YOYO_ENDPOINT", "http://1.2.3.4:9443");
        std::env::set_var("SLM_YOYO_GCP_PROJECT", "pointsav-public");
        std::env::set_var("SLM_YOYO_GCP_ZONE", "us-west1-b");
        std::env::set_var("SLM_YOYO_GCP_INSTANCE", "yoyo-tier-b-1");
        std::env::set_var("SLM_YOYO_METRICS_KEY", "vllm:num_requests_running");
        let cfg = IdleMonitorConfig::from_env().expect("should build config");
        assert_eq!(cfg.metrics_key, "vllm:num_requests_running");
        std::env::remove_var("SLM_YOYO_ENDPOINT");
        std::env::remove_var("SLM_YOYO_GCP_PROJECT");
        std::env::remove_var("SLM_YOYO_GCP_ZONE");
        std::env::remove_var("SLM_YOYO_GCP_INSTANCE");
        std::env::remove_var("SLM_YOYO_METRICS_KEY");
    }

    #[test]
    fn poll_active_slots_parses_prometheus_line() {
        // Exercises the real matching/parsing function directly (no network
        // call needed) rather than reimplementing the logic inline — the
        // pre-D5-fix version of this test did the latter, which meant it
        // kept "passing" even when the real function's matching logic had
        // the prefix-match and trailing-timestamp bugs described on
        // `find_metric_value`'s doc comment.
        let metrics_text = "# HELP llama_active_slots_total Active slots\n\
                            # TYPE llama_active_slots_total gauge\n\
                            llama_active_slots_total 3\n";
        assert_eq!(
            find_metric_value(metrics_text, "llama_active_slots_total"),
            Some(3)
        );
    }

    #[test]
    fn find_metric_value_does_not_prefix_match_a_different_metric() {
        // Regression for the D5 fix: the old `line.starts_with(metrics_key)`
        // would have wrongly matched this line when asked for
        // "requests_processing" alone.
        let metrics_text = "requests_processing_total 42\n";
        assert_eq!(find_metric_value(metrics_text, "requests_processing"), None);
    }

    #[test]
    fn find_metric_value_ignores_trailing_timestamp() {
        // Regression for the D5 fix: `line.split_once(' ')` left the value
        // token as "3 1395066363000" (one string with an embedded space),
        // which failed to parse and returned None pre-fix.
        let metrics_text = "llama_active_slots_total 3 1395066363000\n";
        assert_eq!(
            find_metric_value(metrics_text, "llama_active_slots_total"),
            Some(3)
        );
    }

    #[test]
    fn find_metric_value_keeps_scanning_past_an_unparseable_line() {
        // Regression for the D5 fix: the old code returned immediately on
        // the first prefix-matching line even if that line failed to parse,
        // never checking a later line for the same key.
        let metrics_text = "llama_active_slots_total not_a_number\n\
                            llama_active_slots_total 5\n";
        assert_eq!(
            find_metric_value(metrics_text, "llama_active_slots_total"),
            Some(5)
        );
    }
}
