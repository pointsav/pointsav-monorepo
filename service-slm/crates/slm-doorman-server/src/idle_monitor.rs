// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Yo-Yo idle monitor (B5) — replaces yoyo-manual/yoyo-idle-check.sh.
//!
//! Polls the Yo-Yo VM `/metrics` endpoint every 5 minutes for an active-request
//! counter. When the VM has been idle (zero active requests) longer than
//! `SLM_YOYO_IDLE_MINUTES` (default 30), sends a GCP `instances.stop` request
//! via the Compute Engine API using the workspace Service Account ADC token from
//! the GCE metadata server.
//!
//! **Preemption auto-restart (B5-ext):** when `/metrics` is unreachable and we
//! did NOT issue the stop ourselves (`stop_sent=false`), the VM was likely
//! preempted by GCP. The monitor calls `instances.start` automatically, subject
//! to a rolling restart budget (`SLM_YOYO_MAX_RESTARTS_PER_HOUR`, default 3)
//! and a boot-grace window (`SLM_YOYO_RESTART_BOOT_GRACE_SEC`, default 90 s)
//! that suppresses the next poll so we don't count a booting VM as unreachable.
//!
//! The monitor is spawned as a background tokio task in `main.rs` only when all
//! four GCP env vars are set (`SLM_YOYO_GCP_PROJECT`, `SLM_YOYO_GCP_ZONE`,
//! `SLM_YOYO_GCP_INSTANCE`, and `SLM_YOYO_ENDPOINT`). Absent any of these,
//! `IdleMonitorConfig::from_env()` returns `None` and no task is spawned.
//!
//! Env vars:
//!   SLM_YOYO_ENDPOINT              Yo-Yo base URL (also consumed by Tier B client)
//!   SLM_YOYO_IDLE_MINUTES          idle threshold in minutes; default 30
//!   SLM_YOYO_METRICS_KEY           Prometheus metric name for active-request count;
//!                                  default: llama_active_slots_total (llama-server);
//!                                  set to vllm:num_requests_running for vLLM
//!   SLM_YOYO_GCP_PROJECT           GCP project id (e.g. pointsav-public)
//!   SLM_YOYO_GCP_ZONE              GCP zone (e.g. us-west1-b)
//!   SLM_YOYO_GCP_INSTANCE          GCP instance name (e.g. yoyo-tier-b-1)
//!   SLM_YOYO_AUTO_RESTART          auto-restart on preemption; default true;
//!                                  set false or 0 to disable (e.g. incident response)
//!   SLM_YOYO_MAX_RESTARTS_PER_HOUR rolling auto-restart cap; default 3
//!   SLM_YOYO_RESTART_BOOT_GRACE_SEC seconds to wait after a start call before
//!                                  resuming /metrics polling; default 90

use std::collections::VecDeque;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use tracing::{info, warn};

const POLL_INTERVAL: Duration = Duration::from_secs(300); // 5 minutes
const HTTP_TIMEOUT: Duration = Duration::from_secs(10);
const GCP_METADATA_TOKEN_URL: &str =
    "http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/token";

// Minimum unreachable duration before triggering a preemption-restart attempt.
// One poll interval ensures we don't restart on a transient /health blip.
const PREEMPTION_PROBE_THRESHOLD: Duration = Duration::from_secs(60);

// ── Restart budget ────────────────────────────────────────────────────────────

/// Rolling-window restart tracker. Enforces a maximum number of auto-restart
/// attempts within a configurable time window to prevent thrashing when GCP is
/// under sustained preemption pressure.
#[derive(Debug, Default)]
pub struct RestartBudget {
    window: Duration,
    attempts: VecDeque<Instant>,
}

impl RestartBudget {
    pub fn new(window: Duration) -> Self {
        Self { window, attempts: VecDeque::new() }
    }

    /// Evict stale entries, then return true and record an attempt if the cap
    /// has not been reached. Returns false (and records nothing) when the cap
    /// is already full within the rolling window.
    pub fn try_consume(&mut self, cap: u32, now: Instant) -> bool {
        // Evict attempts that have aged out of the window.
        while let Some(&front) = self.attempts.front() {
            if now.duration_since(front) > self.window {
                self.attempts.pop_front();
            } else {
                break;
            }
        }
        if cap == 0 || self.attempts.len() as u32 >= cap {
            return false;
        }
        self.attempts.push_back(now);
        true
    }

    pub fn count(&self) -> usize {
        self.attempts.len()
    }
}

// ── Config ────────────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct IdleMonitorConfig {
    pub yoyo_endpoint: String,
    pub yoyo_bearer: Option<String>,
    pub idle_threshold: Duration,
    pub metrics_key: String,
    pub gcp_project: String,
    pub gcp_zone: String,
    pub gcp_instance: String,
    /// Shared atomic updated by the HTTP router on each successful Tier B
    /// dispatch. Stores Unix epoch seconds; zero means no dispatch yet.
    /// The idle monitor reads this on every poll cycle to prevent the 5-min
    /// poll granularity from misfiring when the model is actively serving
    /// but the poll catches an inter-request gap (slots=0).
    pub last_yoyo_dispatch: Arc<AtomicU64>,
    /// Auto-restart on preemption detection. Default true.
    /// Set SLM_YOYO_AUTO_RESTART=false to disable.
    pub auto_restart_enabled: bool,
    /// Maximum auto-restart attempts within a rolling 60-minute window.
    /// Default 3. Prevents thrash on sustained GCP preemption pressure.
    pub max_restarts_per_hour: u32,
    /// Seconds to suppress polling after a successful instances.start call,
    /// allowing the VM to boot before being counted as unreachable again.
    /// Default 90.
    pub restart_boot_grace: Duration,
}

impl IdleMonitorConfig {
    /// Constructs config from env vars. Returns `None` if any required var is absent.
    pub fn from_env() -> Option<Self> {
        let yoyo_endpoint = std::env::var("SLM_YOYO_ENDPOINT").ok()?;
        if yoyo_endpoint.is_empty() {
            return None;
        }
        let yoyo_bearer = std::env::var("SLM_YOYO_BEARER").ok().filter(|s| !s.is_empty());
        let idle_minutes: u64 = std::env::var("SLM_YOYO_IDLE_MINUTES")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(30);
        let metrics_key = std::env::var("SLM_YOYO_METRICS_KEY")
            .unwrap_or_else(|_| "llama_active_slots_total".to_string());
        let gcp_project = std::env::var("SLM_YOYO_GCP_PROJECT").ok()?;
        let gcp_zone = std::env::var("SLM_YOYO_GCP_ZONE").ok()?;
        let gcp_instance = std::env::var("SLM_YOYO_GCP_INSTANCE").ok()?;
        let auto_restart_enabled = std::env::var("SLM_YOYO_AUTO_RESTART")
            .map(|v| !matches!(v.trim(), "false" | "0"))
            .unwrap_or(true);
        let max_restarts_per_hour: u32 = std::env::var("SLM_YOYO_MAX_RESTARTS_PER_HOUR")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(3);
        let restart_boot_grace_secs: u64 = std::env::var("SLM_YOYO_RESTART_BOOT_GRACE_SEC")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(90);

        Some(Self {
            yoyo_endpoint,
            yoyo_bearer,
            idle_threshold: Duration::from_secs(idle_minutes * 60),
            metrics_key,
            gcp_project,
            gcp_zone,
            gcp_instance,
            last_yoyo_dispatch: Arc::new(AtomicU64::new(0)),
            auto_restart_enabled,
            max_restarts_per_hour,
            restart_boot_grace: Duration::from_secs(restart_boot_grace_secs),
        })
    }
}

// ── GCP URL helpers (extracted so unit tests can verify without network) ──────

pub fn gcp_stop_url(project: &str, zone: &str, instance: &str) -> String {
    format!(
        "https://compute.googleapis.com/compute/v1/projects/{project}/zones/{zone}/instances/{instance}/stop"
    )
}

pub fn gcp_start_url(project: &str, zone: &str, instance: &str) -> String {
    format!(
        "https://compute.googleapis.com/compute/v1/projects/{project}/zones/{zone}/instances/{instance}/start"
    )
}

// ── Main loop ─────────────────────────────────────────────────────────────────

/// Run the idle monitor loop. Call via `tokio::spawn(run_idle_monitor(config))`.
pub async fn run_idle_monitor(config: IdleMonitorConfig) {
    let client = reqwest::Client::builder()
        .timeout(HTTP_TIMEOUT)
        .build()
        .unwrap_or_default();

    // Start the idle clock at task-spawn time so a cold VM doesn't get stopped
    // before its first request within the threshold window.
    let mut last_active = Instant::now();
    // stop_sent doubles as the "we_stopped_it" flag: when true, an unreachable
    // /metrics endpoint is the expected post-idle-shutdown state — do not restart.
    let mut stop_sent = false;
    // Track continuous unreachable duration for the crash-guard path.
    let mut unreachable_since: Option<Instant> = None;
    // Rolling restart budget — enforces max_restarts_per_hour.
    let mut restart_budget = RestartBudget::new(Duration::from_secs(3600));
    // When Some, suppress poll cycles until the deadline passes (boot grace).
    let mut in_boot_grace_until: Option<Instant> = None;

    info!(
        target: "slm_doorman::idle_monitor",
        endpoint = %config.yoyo_endpoint,
        idle_threshold_secs = config.idle_threshold.as_secs(),
        gcp_instance = %config.gcp_instance,
        auto_restart = config.auto_restart_enabled,
        max_restarts_per_hour = config.max_restarts_per_hour,
        "Yo-Yo idle monitor started"
    );

    loop {
        tokio::time::sleep(POLL_INTERVAL).await;

        // Boot-grace suppression: skip this poll if a recent start call means
        // the VM is still booting. Avoids counting a booting VM as unreachable
        // and burning restart budget immediately after a successful start.
        if let Some(deadline) = in_boot_grace_until {
            if Instant::now() < deadline {
                info!(
                    target: "slm_doorman::idle_monitor",
                    grace_remaining_secs = deadline.saturating_duration_since(Instant::now()).as_secs(),
                    "boot-grace window active; skipping poll"
                );
                continue;
            }
            in_boot_grace_until = None;
        }

        // Incorporate dispatch-based last_active: if the HTTP router signalled
        // a Tier B dispatch more recently than the poll-based last_active,
        // rewind last_active so the 5-min poll granularity cannot fire a
        // premature stop when the model is between requests (slots=0).
        {
            let dispatch_secs = config.last_yoyo_dispatch.load(Ordering::Relaxed);
            if dispatch_secs > 0 {
                let now_secs = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();
                let dispatch_elapsed =
                    Duration::from_secs(now_secs.saturating_sub(dispatch_secs));
                if dispatch_elapsed < last_active.elapsed() {
                    last_active = Instant::now() - dispatch_elapsed;
                    stop_sent = false; // VM dispatched recently; allow future stop after new idle window
                }
            }
        }

        match poll_active_slots(&client, &config.yoyo_endpoint, config.yoyo_bearer.as_deref(), &config.metrics_key).await {
            Some(0) => {
                // Metrics reachable, zero active slots — VM is up but idle.
                unreachable_since = None;
                let idle_secs = last_active.elapsed().as_secs();
                if !stop_sent && last_active.elapsed() >= config.idle_threshold {
                    warn!(
                        target: "slm_doorman::idle_monitor",
                        idle_secs,
                        project = %config.gcp_project,
                        zone = %config.gcp_zone,
                        instance = %config.gcp_instance,
                        "Yo-Yo idle threshold reached; sending GCP stop request"
                    );
                    match stop_gcp_instance(&client, &config).await {
                        Ok(()) => {
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
                unreachable_since = None;
                info!(
                    target: "slm_doorman::idle_monitor",
                    active_slots = n,
                    "Yo-Yo busy; idle clock reset"
                );
            }
            None => {
                // Metrics unreachable — VM is booting, stopped, or crashed.
                //
                // Two cases:
                //   (a) stop_sent=true  — we issued the stop ourselves (idle shutdown).
                //                         This is expected. Do nothing except the
                //                         crash-guard when auto_restart is off.
                //   (b) stop_sent=false — we did NOT stop it. Most likely GCP preempted
                //                         the VM. Trigger auto-restart if enabled.

                if unreachable_since.is_none() {
                    unreachable_since = Some(Instant::now());
                }
                let unreachable_for = unreachable_since
                    .map(|t| t.elapsed())
                    .unwrap_or_default();

                if stop_sent {
                    // Idle-shutdown case: VM is intentionally down. Only apply
                    // the crash-guard when auto_restart is off (otherwise the
                    // start call IS the response to extended unreachability and
                    // the crash-guard would race against it).
                    if !config.auto_restart_enabled {
                        let safety_threshold = config.idle_threshold * 2;
                        if unreachable_for >= safety_threshold {
                            let unreachable_secs = unreachable_for.as_secs();
                            warn!(
                                target: "slm_doorman::idle_monitor",
                                unreachable_secs,
                                project = %config.gcp_project,
                                zone = %config.gcp_zone,
                                instance = %config.gcp_instance,
                                "Yo-Yo metrics unreachable past safety threshold (crash-guard); sending stop"
                            );
                            match stop_gcp_instance(&client, &config).await {
                                Ok(()) => {
                                    info!(
                                        target: "slm_doorman::idle_monitor",
                                        instance = %config.gcp_instance,
                                        "GCP stop request accepted (crash-guard)"
                                    );
                                    stop_sent = true;
                                }
                                Err(reason) => {
                                    warn!(
                                        target: "slm_doorman::idle_monitor",
                                        %reason,
                                        "GCP stop request (crash-guard) failed; will retry next cycle"
                                    );
                                }
                            }
                        }
                    }
                } else if config.auto_restart_enabled
                    && unreachable_for >= PREEMPTION_PROBE_THRESHOLD
                {
                    // Preemption detected: VM is unreachable and we didn't stop it.
                    // Attempt instances.start subject to the rolling budget.
                    let now = Instant::now();
                    if !restart_budget.try_consume(config.max_restarts_per_hour, now) {
                        warn!(
                            target: "slm_doorman::idle_monitor",
                            attempts_in_window = restart_budget.count(),
                            cap = config.max_restarts_per_hour,
                            "Yo-Yo restart budget exhausted; sustained preemption pressure — operator intervention required"
                        );
                    } else {
                        let attempt = restart_budget.count();
                        warn!(
                            target: "slm_doorman::idle_monitor",
                            attempt,
                            cap = config.max_restarts_per_hour,
                            unreachable_secs = unreachable_for.as_secs(),
                            project = %config.gcp_project,
                            zone = %config.gcp_zone,
                            instance = %config.gcp_instance,
                            "Yo-Yo VM preempted; auto-restarting (attempt {}/{})",
                            attempt, config.max_restarts_per_hour
                        );
                        match start_gcp_instance(&client, &config).await {
                            Ok(()) => {
                                info!(
                                    target: "slm_doorman::idle_monitor",
                                    instance = %config.gcp_instance,
                                    boot_grace_secs = config.restart_boot_grace.as_secs(),
                                    "GCP start request accepted; entering boot-grace window"
                                );
                                // Reset idle clock and unreachable window for the
                                // freshly started VM.
                                last_active = Instant::now();
                                unreachable_since = None;
                                in_boot_grace_until = Some(now + config.restart_boot_grace);
                            }
                            Err(reason) => {
                                warn!(
                                    target: "slm_doorman::idle_monitor",
                                    %reason,
                                    "GCP instances.start failed; will retry next cycle (budget slot consumed)"
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}

// ── Metrics parsing ───────────────────────────────────────────────────────────

/// Poll the Yo-Yo `/metrics` endpoint and extract the active-request counter
/// named by `metrics_key`. Returns `None` on network error, non-200 response,
/// or missing metric.
///
/// Uses exact token matching (`"<key> "` prefix) to avoid false positives from
/// metrics whose names share the key as a prefix
/// (e.g. `llama_active_slots_total_avg` must not match key `llama_active_slots_total`).
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
    parse_metric(&text, metrics_key)
}

/// Extract a Prometheus gauge value from a metrics text body.
/// Skips `#`-prefixed comment/help lines. Matches only lines where the metric
/// name is followed immediately by a space (exact token boundary), preventing
/// prefix collisions (e.g. `llama_active_slots_total_avg` vs `llama_active_slots_total`).
pub fn parse_metric(text: &str, metrics_key: &str) -> Option<u64> {
    let prefix = format!("{} ", metrics_key);
    for line in text.lines() {
        if line.starts_with('#') {
            continue;
        }
        if line.starts_with(&prefix) {
            let val = line[prefix.len()..].trim();
            return val.parse::<f64>().ok().map(|f| f as u64);
        }
    }
    None
}

// ── GCP API helpers ───────────────────────────────────────────────────────────

/// Fetch an ADC bearer token from the GCE metadata server.
async fn fetch_gcp_adc_token(client: &reqwest::Client) -> Result<String, String> {
    #[derive(serde::Deserialize)]
    struct TokenResp {
        access_token: String,
    }
    let resp = client
        .get(GCP_METADATA_TOKEN_URL)
        .header("Metadata-Flavor", "Google")
        .send()
        .await
        .map_err(|e| format!("metadata server unreachable: {e}"))?;
    if !resp.status().is_success() {
        return Err(format!("metadata server HTTP {}", resp.status()));
    }
    let t: TokenResp = resp
        .json()
        .await
        .map_err(|e| format!("token JSON parse failed: {e}"))?;
    Ok(t.access_token)
}

/// POST `instances.stop` to the GCP Compute Engine API.
async fn stop_gcp_instance(
    client: &reqwest::Client,
    config: &IdleMonitorConfig,
) -> Result<(), String> {
    let token = fetch_gcp_adc_token(client).await?;
    let url = gcp_stop_url(&config.gcp_project, &config.gcp_zone, &config.gcp_instance);
    // GCP Compute Engine API requires Content-Length: 0 on empty-body POSTs.
    let resp = client
        .post(&url)
        .bearer_auth(&token)
        .header(reqwest::header::CONTENT_LENGTH, "0")
        .body(reqwest::Body::from(""))
        .send()
        .await
        .map_err(|e| format!("GCP API request failed: {e}"))?;
    if resp.status().is_success() {
        Ok(())
    } else {
        Err(format!("GCP instances.stop returned HTTP {}", resp.status()))
    }
}

/// POST `instances.start` to the GCP Compute Engine API.
/// Idempotent: GCP returns success when called on an already-running VM.
async fn start_gcp_instance(
    client: &reqwest::Client,
    config: &IdleMonitorConfig,
) -> Result<(), String> {
    let token = fetch_gcp_adc_token(client).await?;
    let url = gcp_start_url(&config.gcp_project, &config.gcp_zone, &config.gcp_instance);
    let resp = client
        .post(&url)
        .bearer_auth(&token)
        .header(reqwest::header::CONTENT_LENGTH, "0")
        .body(reqwest::Body::from(""))
        .send()
        .await
        .map_err(|e| format!("GCP API request failed: {e}"))?;
    if resp.status().is_success() {
        Ok(())
    } else {
        Err(format!("GCP instances.start returned HTTP {}", resp.status()))
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Mutex, OnceLock};

    // Serialize tests that mutate process-global env vars.
    static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    fn env_lock() -> &'static Mutex<()> {
        ENV_LOCK.get_or_init(|| Mutex::new(()))
    }

    // ── IdleMonitorConfig::from_env ───────────────────────────────────────────

    #[test]
    fn from_env_returns_none_without_gcp_vars() {
        let _g = env_lock().lock().unwrap();
        std::env::remove_var("SLM_YOYO_GCP_PROJECT");
        std::env::remove_var("SLM_YOYO_GCP_ZONE");
        std::env::remove_var("SLM_YOYO_GCP_INSTANCE");
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
    fn from_env_auto_restart_defaults_to_true() {
        let _g = env_lock().lock().unwrap();
        std::env::set_var("SLM_YOYO_ENDPOINT", "http://1.2.3.4:9443");
        std::env::set_var("SLM_YOYO_GCP_PROJECT", "proj");
        std::env::set_var("SLM_YOYO_GCP_ZONE", "us-west1-a");
        std::env::set_var("SLM_YOYO_GCP_INSTANCE", "inst");
        std::env::remove_var("SLM_YOYO_AUTO_RESTART");
        let cfg = IdleMonitorConfig::from_env().unwrap();
        assert!(cfg.auto_restart_enabled);
        std::env::remove_var("SLM_YOYO_ENDPOINT");
        std::env::remove_var("SLM_YOYO_GCP_PROJECT");
        std::env::remove_var("SLM_YOYO_GCP_ZONE");
        std::env::remove_var("SLM_YOYO_GCP_INSTANCE");
    }

    #[test]
    fn from_env_auto_restart_disabled_by_zero() {
        let _g = env_lock().lock().unwrap();
        std::env::set_var("SLM_YOYO_ENDPOINT", "http://1.2.3.4:9443");
        std::env::set_var("SLM_YOYO_GCP_PROJECT", "proj");
        std::env::set_var("SLM_YOYO_GCP_ZONE", "us-west1-a");
        std::env::set_var("SLM_YOYO_GCP_INSTANCE", "inst");
        std::env::set_var("SLM_YOYO_AUTO_RESTART", "0");
        let cfg = IdleMonitorConfig::from_env().unwrap();
        assert!(!cfg.auto_restart_enabled);
        std::env::remove_var("SLM_YOYO_ENDPOINT");
        std::env::remove_var("SLM_YOYO_GCP_PROJECT");
        std::env::remove_var("SLM_YOYO_GCP_ZONE");
        std::env::remove_var("SLM_YOYO_GCP_INSTANCE");
        std::env::remove_var("SLM_YOYO_AUTO_RESTART");
    }

    #[test]
    fn from_env_auto_restart_disabled_by_false() {
        let _g = env_lock().lock().unwrap();
        std::env::set_var("SLM_YOYO_ENDPOINT", "http://1.2.3.4:9443");
        std::env::set_var("SLM_YOYO_GCP_PROJECT", "proj");
        std::env::set_var("SLM_YOYO_GCP_ZONE", "us-west1-a");
        std::env::set_var("SLM_YOYO_GCP_INSTANCE", "inst");
        std::env::set_var("SLM_YOYO_AUTO_RESTART", "false");
        let cfg = IdleMonitorConfig::from_env().unwrap();
        assert!(!cfg.auto_restart_enabled);
        std::env::remove_var("SLM_YOYO_ENDPOINT");
        std::env::remove_var("SLM_YOYO_GCP_PROJECT");
        std::env::remove_var("SLM_YOYO_GCP_ZONE");
        std::env::remove_var("SLM_YOYO_GCP_INSTANCE");
        std::env::remove_var("SLM_YOYO_AUTO_RESTART");
    }

    #[test]
    fn from_env_max_restarts_per_hour_defaults_to_three() {
        let _g = env_lock().lock().unwrap();
        std::env::set_var("SLM_YOYO_ENDPOINT", "http://1.2.3.4:9443");
        std::env::set_var("SLM_YOYO_GCP_PROJECT", "proj");
        std::env::set_var("SLM_YOYO_GCP_ZONE", "us-west1-a");
        std::env::set_var("SLM_YOYO_GCP_INSTANCE", "inst");
        std::env::remove_var("SLM_YOYO_MAX_RESTARTS_PER_HOUR");
        let cfg = IdleMonitorConfig::from_env().unwrap();
        assert_eq!(cfg.max_restarts_per_hour, 3);
        std::env::remove_var("SLM_YOYO_ENDPOINT");
        std::env::remove_var("SLM_YOYO_GCP_PROJECT");
        std::env::remove_var("SLM_YOYO_GCP_ZONE");
        std::env::remove_var("SLM_YOYO_GCP_INSTANCE");
    }

    #[test]
    fn from_env_max_restarts_per_hour_parses_override() {
        let _g = env_lock().lock().unwrap();
        std::env::set_var("SLM_YOYO_ENDPOINT", "http://1.2.3.4:9443");
        std::env::set_var("SLM_YOYO_GCP_PROJECT", "proj");
        std::env::set_var("SLM_YOYO_GCP_ZONE", "us-west1-a");
        std::env::set_var("SLM_YOYO_GCP_INSTANCE", "inst");
        std::env::set_var("SLM_YOYO_MAX_RESTARTS_PER_HOUR", "5");
        let cfg = IdleMonitorConfig::from_env().unwrap();
        assert_eq!(cfg.max_restarts_per_hour, 5);
        std::env::remove_var("SLM_YOYO_ENDPOINT");
        std::env::remove_var("SLM_YOYO_GCP_PROJECT");
        std::env::remove_var("SLM_YOYO_GCP_ZONE");
        std::env::remove_var("SLM_YOYO_GCP_INSTANCE");
        std::env::remove_var("SLM_YOYO_MAX_RESTARTS_PER_HOUR");
    }

    #[test]
    fn from_env_restart_boot_grace_defaults_to_90() {
        let _g = env_lock().lock().unwrap();
        std::env::set_var("SLM_YOYO_ENDPOINT", "http://1.2.3.4:9443");
        std::env::set_var("SLM_YOYO_GCP_PROJECT", "proj");
        std::env::set_var("SLM_YOYO_GCP_ZONE", "us-west1-a");
        std::env::set_var("SLM_YOYO_GCP_INSTANCE", "inst");
        std::env::remove_var("SLM_YOYO_RESTART_BOOT_GRACE_SEC");
        let cfg = IdleMonitorConfig::from_env().unwrap();
        assert_eq!(cfg.restart_boot_grace, Duration::from_secs(90));
        std::env::remove_var("SLM_YOYO_ENDPOINT");
        std::env::remove_var("SLM_YOYO_GCP_PROJECT");
        std::env::remove_var("SLM_YOYO_GCP_ZONE");
        std::env::remove_var("SLM_YOYO_GCP_INSTANCE");
    }

    // ── GCP URL construction ──────────────────────────────────────────────────

    #[test]
    fn gcp_stop_url_is_well_formed() {
        let url = gcp_stop_url("my-project", "europe-west4-a", "yoyo-tier-b-1");
        assert_eq!(
            url,
            "https://compute.googleapis.com/compute/v1/projects/my-project/zones/europe-west4-a/instances/yoyo-tier-b-1/stop"
        );
    }

    #[test]
    fn gcp_start_url_is_well_formed() {
        let url = gcp_start_url("my-project", "europe-west4-a", "yoyo-tier-b-1");
        assert_eq!(
            url,
            "https://compute.googleapis.com/compute/v1/projects/my-project/zones/europe-west4-a/instances/yoyo-tier-b-1/start"
        );
    }

    // ── parse_metric ─────────────────────────────────────────────────────────

    #[test]
    fn parse_metric_extracts_integer_value() {
        let text = "# HELP llama_active_slots_total Active slots\n\
                    # TYPE llama_active_slots_total gauge\n\
                    llama_active_slots_total 3\n";
        assert_eq!(parse_metric(text, "llama_active_slots_total"), Some(3));
    }

    #[test]
    fn parse_metric_extracts_float_value() {
        let text = "llama_active_slots_total 2.0\n";
        assert_eq!(parse_metric(text, "llama_active_slots_total"), Some(2));
    }

    #[test]
    fn parse_metric_extracts_zero() {
        let text = "llama_active_slots_total 0.0\n";
        assert_eq!(parse_metric(text, "llama_active_slots_total"), Some(0));
    }

    #[test]
    fn parse_metric_skips_help_and_type_comments() {
        let text = "# HELP llama_active_slots_total Active slots\n\
                    # TYPE llama_active_slots_total gauge\n\
                    llama_active_slots_total 5\n";
        assert_eq!(parse_metric(text, "llama_active_slots_total"), Some(5));
    }

    #[test]
    fn parse_metric_returns_none_for_missing_key() {
        let text = "other_metric 7\n";
        assert_eq!(parse_metric(text, "llama_active_slots_total"), None);
    }

    #[test]
    fn parse_metric_avoids_prefix_collision() {
        // "llama_active_slots_total_avg" must NOT match key "llama_active_slots_total"
        // because exact-token matching requires the key to be followed by a space.
        let text = "llama_active_slots_total_avg 99\n\
                    llama_active_slots_total 4\n";
        assert_eq!(parse_metric(text, "llama_active_slots_total"), Some(4));
    }

    #[test]
    fn parse_metric_returns_none_for_empty_text() {
        assert_eq!(parse_metric("", "llama_active_slots_total"), None);
    }

    // ── RestartBudget ─────────────────────────────────────────────────────────

    #[test]
    fn restart_budget_admits_up_to_cap() {
        let mut budget = RestartBudget::new(Duration::from_secs(3600));
        let now = Instant::now();
        assert!(budget.try_consume(3, now));
        assert!(budget.try_consume(3, now));
        assert!(budget.try_consume(3, now));
        assert_eq!(budget.count(), 3);
    }

    #[test]
    fn restart_budget_rejects_over_cap() {
        let mut budget = RestartBudget::new(Duration::from_secs(3600));
        let now = Instant::now();
        budget.try_consume(3, now);
        budget.try_consume(3, now);
        budget.try_consume(3, now);
        // 4th attempt within the window must be rejected
        assert!(!budget.try_consume(3, now));
        assert_eq!(budget.count(), 3); // count unchanged
    }

    #[test]
    fn restart_budget_evicts_stale_entries() {
        let mut budget = RestartBudget::new(Duration::from_secs(10));
        // Record two attempts with a timestamp 20s in the past (outside window).
        let old = Instant::now() - Duration::from_secs(20);
        budget.try_consume(3, old);
        budget.try_consume(3, old);
        assert_eq!(budget.count(), 2);
        // Now attempt at "now" — the two old entries should be evicted, so the cap is not hit.
        let now = Instant::now();
        assert!(budget.try_consume(3, now));
        assert_eq!(budget.count(), 1); // only the fresh attempt remains
    }

    #[test]
    fn restart_budget_zero_cap_always_rejects() {
        let mut budget = RestartBudget::new(Duration::from_secs(3600));
        assert!(!budget.try_consume(0, Instant::now()));
        assert_eq!(budget.count(), 0);
    }
}
