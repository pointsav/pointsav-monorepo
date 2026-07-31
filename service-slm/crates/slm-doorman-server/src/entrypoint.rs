// SPDX-License-Identifier: Apache-2.0 OR MIT

//! `slm-doorman-server` — HTTP entry point for the service-slm Doorman.
//!
//! B1 scope: bind axum, mount /healthz, /readyz, /v1/contract, and a
//! POST /v1/chat/completions stub that forwards through `Doorman::route`.
//! Tier B (Yo-Yo) wiring lands in B2; Tier C (External) in B4.
//!
//! Environment configuration:
//!   SLM_BIND_ADDR             default 127.0.0.1:9080
//!   SLM_LOCAL_ENDPOINT        default http://127.0.0.1:8080  (Tier A)
//!   SLM_LOCAL_MODEL           default olmo-3-7b-instruct
//!   SLM_YOYO_ENDPOINT         optional; absent = no Yo-Yo (community-tier mode)
//!   SLM_YOYO_MODEL            default Olmo-3-1125-32B-Think
//!   SLM_YOYO_BEARER           static bearer token used by Tier B (B2);
//!   SLM_YOYO_HEALTH_PATH      health probe path (default /health; use / for Ollama)
//!   SLM_YOYO_GCP_AUTH         if "true", use GCP metadata identity tokens instead of
//!                              SLM_YOYO_BEARER (required for Cloud Run endpoints)
//!                             real deployments swap StaticBearer for a
//!                             provider-specific BearerTokenProvider impl
//!   SLM_YOYO_HOURLY_USD       per-provider hourly USD rate used to
//!                             compute Tier B cost_usd in the audit
//!                             ledger; default 0.0 (unknown/dev).
//!                             Example: 0.84 for GCP L4, 0.34 for RunPod L4
//!   SLM_APPRENTICESHIP_ENABLED  AS-2..AS-4 endpoints (POST /v1/brief,
//!                             /v1/verdict, /v1/shadow). Default off.
//!                             Set to `true` or `1` to enable.
//!   FOUNDRY_ROOT              workspace root used by the apprenticeship
//!                             dispatcher to resolve scope.files paths
//!                             and read citations.yaml; default
//!                             /srv/foundry.
//!   SLM_BRIEF_TIER_B_THRESHOLD_CHARS
//!                             char-budget proxy for Tier-B routing on
//!                             /v1/brief; default 8000 (~2000 tokens).
//!   FOUNDRY_ALLOWED_SIGNERS   path to allowed_signers used by AS-3
//!                             ssh-keygen -Y verify; default
//!                             ${FOUNDRY_ROOT}/identity/allowed_signers.
//!   FOUNDRY_DOCTRINE_VERSION  doctrine version embedded in apprenticeship
//!                             corpus tuples; default 0.0.7.
//!   FOUNDRY_TENANT            tenant tag on corpus tuples; default pointsav.
//!   SLM_AUDIT_DIR             directory for the append-only JSONL audit ledger.
//!                             If unset, defaults to $HOME/.service-slm/audit/.
//!                             The directory is created on startup if absent.
//!                             A creation failure is non-fatal: the server logs
//!                             a warning and falls back to the default location.
//!   SLM_LARK_VALIDATION_ENABLED  pre-validate Lark grammars at the Doorman
//!                             boundary using llguidance (PS.3 step 5).
//!                             Default true. Set to `false` or `0` to disable.
//!   SERVICE_CONTENT_ENDPOINT  service-content graph API base URL
//!                             (e.g. http://127.0.0.1:9081). When absent
//!                             the Doorman proceeds without graph context.
//!   SLM_AUDIT_TENANT_CONCURRENCY_CAP
//!                             maximum number of concurrent in-flight audit
//!                             requests per tenant (moduleId) across BOTH
//!                             /v1/audit/proxy and /v1/audit/capture. Excess
//!                             requests → 503 SERVICE_UNAVAILABLE with
//!                             Retry-After: 5. Default 4.
//!   SLM_ORCHESTRATION_ENDPOINT  base URL of the app-orchestration-slm chassis
//!                             (e.g. http://10.0.0.1:9180). When set, the
//!                             Doorman POSTs its identity to the chassis on
//!                             startup (non-blocking). Absent = standalone mode.
//!   SLM_MODULE_ID             flat module identifier for chassis registration
//!                             (e.g. "project-jennifer"). Required when
//!                             SLM_ORCHESTRATION_ENDPOINT is set.
//!   SLM_ARCHIVE_ID            archive name for chassis registration
//!                             (e.g. "cluster-totebox-jennifer").
//!   SLM_TIER_B_SUBSCRIBED     "true" or "1" if this archive has a paid Tier B
//!                             subscription via the chassis. Default false.
//!   RUST_LOG                  default slm_doorman=info,slm_doorman_server=info
//!
//! Per `conventions/three-ring-architecture.md` the Doorman boots fine
//! with no Yo-Yo configured (Optional Intelligence). B5 verifies this
//! end-to-end.

use crate::http;
use crate::idle_monitor::IdleMonitorConfig;
use crate::queue::{
    dequeue_shadow, ensure_dirs, reap_expired_leases, release_shadow, QueueConfig, ReleaseOutcome,
};

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use anyhow::Context;
use slm_doorman::express_lane::DEFAULT_BATCH_SLOTS;
use slm_doorman::tier::{
    BearerTokenProvider, ExternalTierClient, ExternalTierConfig, LocalTierClient, LocalTierConfig,
    MetadataBearer, PricingConfig, StaticBearer, TierCPricing, TierCProvider, YoYoTierClient,
    YoYoTierConfig, FOUNDRY_DEFAULT_ALLOWLIST,
};
use slm_doorman::{
    ApprenticeshipConfig, AuditLedger, AuditProxyClient, AuditProxyConfig, BriefCache, Doorman,
    DoormanConfig, ExpressLane, GraphContextClient, LarkValidator, LocalBackend, PromotionLedger,
    SshKeygenVerifier, VerdictDispatcher, VerdictVerifier, FOUNDRY_DEFAULT_PURPOSE_ALLOWLIST,
};
use tracing::{info, warn};

/// Boots the Doorman: builds `AppState`, spawns the Brief Queue Substrate
/// background tasks (drain worker + reaper), and serves the axum router.
/// Extracted from a `#[tokio::main] async fn main()` (see `src/main.rs`,
/// now a thin wrapper) so this crate can be embedded as a library dependency
/// by a bundling binary — see `BRIEF-os-totebox-platform.md` §8/§10 (Phase 2).
pub async fn run() -> anyhow::Result<()> {
    init_tracing();

    let bind_addr: SocketAddr = std::env::var("SLM_BIND_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:9080".to_string())
        .parse()
        .context("SLM_BIND_ADDR must be a socket address")?;

    let doorman = build_doorman().await?;
    let apprenticeship = build_apprenticeship_config();
    let brief_cache = Arc::new(BriefCache::default());
    let verdict_dispatcher = match apprenticeship.as_ref() {
        Some(cfg) => Some(build_verdict_dispatcher(cfg, brief_cache.clone())?),
        None => None,
    };
    let audit_proxy_client = build_audit_proxy_client();

    // SLM_AUDIT_TENANT_CONCURRENCY_CAP — maximum in-flight audit requests per
    // tenant across both /v1/audit/proxy and /v1/audit/capture. Default 4.
    let audit_tenant_concurrency_cap: u32 = std::env::var("SLM_AUDIT_TENANT_CONCURRENCY_CAP")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(4);

    // Brief Queue Substrate (§7C) — build QueueConfig before constructing
    // AppState so both the handler and the drain worker share the same config.
    let queue_cfg = QueueConfig::from_env();

    // Graph proxy — reuse the SERVICE_CONTENT_ENDPOINT already consumed by
    // GraphContextClient above. Default to 127.0.0.1:9081 if unset so the
    // proxy is available in community-tier deployments without extra config.
    let service_content_endpoint = std::env::var("SERVICE_CONTENT_ENDPOINT")
        .unwrap_or_else(|_| http::DEFAULT_SERVICE_CONTENT_ENDPOINT.to_string());

    // SLM_BATCH_SLOTS — concurrency limit for /v1/chat/completions and /v1/messages.
    // Defaults to DEFAULT_BATCH_SLOTS (2). Returns 429 when all slots are in use.
    // Set to a higher value on nodes with more VRAM headroom (e.g. 4 on L4/24GB).
    let batch_slots: usize = std::env::var("SLM_BATCH_SLOTS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_BATCH_SLOTS);
    let mut express_caps = HashMap::new();
    express_caps.insert("batch".to_string(), batch_slots);
    let express_lane = Arc::new(ExpressLane::new(express_caps));
    info!(batch_slots, "express lane initialised");

    // Node class: env-var override or default "hardware".
    // "micro" = $7/mo e2-micro fleet; "hardware" = workspace VM; "cloud" = GCE GPU node.
    let node_class: &'static str = match std::env::var("SLM_NODE_CLASS").as_deref() {
        Ok("micro") => "micro",
        Ok("cloud") => "cloud",
        _ => "hardware",
    };

    // Derive Tier A availability reason for /readyz diagnostics.
    let force_broker = std::env::var("SLM_FORCE_BROKER_MODE")
        .map(|v| v == "true" || v == "1")
        .unwrap_or(false);
    let tier_a_reason: &'static str = if force_broker {
        "force-broker-mode"
    } else if node_class == "micro" {
        "micro-node-class"
    } else if doorman.has_local() {
        "available"
    } else {
        "no-local-tier"
    };

    let state = Arc::new(http::AppState {
        doorman,
        apprenticeship,
        brief_cache,
        verdict_dispatcher,
        audit_proxy_client,
        // PS.4 step 3 — purpose allowlist. Default: four documented purposes.
        // Operator overrides by replacing with a custom const via deployment
        // env config (compile-time extension per doctrine).
        audit_proxy_purpose_allowlist: FOUNDRY_DEFAULT_PURPOSE_ALLOWLIST,
        // Per-tenant concurrency semaphore map — lazily populated on first
        // request from each tenant.
        audit_tenant_concurrency: Arc::new(Mutex::new(HashMap::new())),
        audit_tenant_concurrency_cap,
        // Brief Queue Substrate (§7C) — shadow_handler enqueues here;
        // drain worker reads from the same config.
        queue_config: Arc::new(queue_cfg.clone()),
        // Graph proxy — base URL for service-content (datagraph-access-discipline).
        service_content_endpoint,
        node_class,
        tier_a_reason,
        express_lane,
    });

    info!(
        version = slm_doorman::DOORMAN_VERSION,
        %bind_addr,
        has_local = state.doorman.has_local(),
        has_yoyo = state.doorman.has_yoyo(),
        has_external = state.doorman.has_external(),
        apprenticeship_enabled = state.apprenticeship.is_some(),
        audit_proxy_enabled = state.audit_proxy_client.is_some(),
        "service-slm Doorman starting"
    );

    // ── Brief Queue Substrate (apprenticeship-substrate.md §7C) ─────────
    //
    // Spawn two background tokio tasks:
    //   1. `queue_drain_worker` — polls queue/ at configurable interval and
    //      dispatches briefs to the apprentice via dispatch_shadow.
    //   2. `queue_reaper`       — reclaims expired leases from queue-in-flight/
    //      so crashed workers' briefs are retried.
    //
    // Both tasks run regardless of SLM_APPRENTICESHIP_ENABLED.  If
    // apprenticeship is disabled the drain worker finds no briefs in the queue
    // (capture-edit.py also checks the flag before writing) and exits each
    // poll cycle immediately.  This keeps the queue infrastructure live and
    // ready for the flag to be enabled without a restart.
    //
    // Env vars:
    //   SLM_QUEUE_DRAIN_INTERVAL_SEC   drain poll interval; default 30s
    //   SLM_QUEUE_LEASE_EXPIRY_SEC     lease age before reaper reclaims; default 2100s
    //   SLM_DRAIN_MAX_RETRIES          retries before a brief is poisoned; default 5
    {
        // Ensure queue directories exist at startup so the background tasks
        // can scan them immediately.  A creation failure is non-fatal (we log
        // and continue); the tasks will retry on each cycle.
        if let Err(e) = ensure_dirs(&queue_cfg) {
            tracing::warn!(error = %e, "brief queue directory bootstrap failed; retrying lazily");
        }

        // ── Drain worker ─────────────────────────────────────────────────
        let drain_interval_secs: u64 = std::env::var("SLM_QUEUE_DRAIN_INTERVAL_SEC")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(30);
        let drain_interval = Duration::from_secs(drain_interval_secs);

        // Maximum times a brief is retried before being moved to queue-poison/.
        // A brief that always fails (scope-resolution error, unreachable files,
        // etc.) would otherwise retry indefinitely and block the serial drain.
        let max_retries: u32 = std::env::var("SLM_DRAIN_MAX_RETRIES")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(5);

        // Tier A slot-contention (LocalSaturated) retries are exempt from the
        // attempts budget above (see escalate_retry_outcome's doc comment) —
        // but that exemption assumed the generic drain_interval sleep between
        // retries. Observed live 2026-07-14/15: with SLM_QUEUE_DRAIN_INTERVAL_SEC
        // tuned down to 1s for backlog throughput, a brief hitting sustained
        // LocalSaturated retried ~3-4x/second per worker (180 attempts/60s
        // observed for one brief) instead of backing off, hammering the local
        // background slot and flooding logs. Give LocalSaturated its own,
        // longer sleep — independent of drain_interval — so tuning the poll
        // interval for normal throughput can't reintroduce this. Default 30s
        // matches the drain_interval default and is short relative to real
        // Tier A inference (17-60 min) while avoiding sub-second hammering.
        let local_saturated_backoff_secs: u64 = std::env::var("SLM_LOCAL_SATURATED_BACKOFF_SEC")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(30);
        let local_saturated_backoff = Duration::from_secs(local_saturated_backoff_secs);

        // Sprint 3C: hold queue when all Tier B nodes have been circuit-open
        // for longer than this threshold. Briefs stay in queue/ until circuit
        // closes. Env var: SLM_HOLD_THRESHOLD_SECS (default 3600 = 1 h).
        // When all Tier B nodes are circuit-open or health-probe-down for longer
        // than this threshold, the drain worker holds the queue (no dispatch).
        // Not bypassed by SLM_TIER_A_FIRST — see Sprint 3C hold below.
        let hold_threshold_secs: u64 = std::env::var("SLM_HOLD_THRESHOLD_SECS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(3600);
        // Read for consistency; the drain worker no longer uses this value
        // directly (Sprint 3C hold and yoyo_node_ready guard replaced the
        // old tier_a_first bypass). Doorman config reads it again at startup.
        let _tier_a_first: bool = std::env::var("SLM_TIER_A_FIRST")
            .ok()
            .map(|v| v.eq_ignore_ascii_case("true") || v == "1")
            .unwrap_or(false);

        // SLM_DRAIN_PAUSED: hard, unconditional pause of shadow-brief dispatch.
        // When set, the drain worker skips every cycle WITHOUT dequeuing —
        // briefs stay untouched in queue/ and the inference tier is never hit.
        // Decoupled from SLM_TIER_A_FIRST (which bypasses the Sprint 3C hold)
        // and from SLM_APPRENTICESHIP_ENABLED (which 404s the /v1/shadow capture
        // endpoint). This lets the operator stop wasteful CPU drain while keeping
        // capture writing new briefs to queue/ for later GPU processing.
        // Read once at startup; restart Doorman to change. See
        // BRIEF-slm-learning-loop.md §10.2.
        let drain_paused: bool = std::env::var("SLM_DRAIN_PAUSED")
            .ok()
            .map(|v| v.eq_ignore_ascii_case("true") || v == "1")
            .unwrap_or(false);

        // Payload size gate: briefs exceeding this byte limit are poisoned without
        // dispatch. Prevents oversized payloads (large diffs, injected data) from
        // wedging OLMo or consuming unbounded tokens. Default 16 KiB.
        // Env var: SLM_QUEUE_MAX_PAYLOAD_BYTES
        let max_payload_bytes: usize = std::env::var("SLM_QUEUE_MAX_PAYLOAD_BYTES")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(16 * 1024);

        // Number of concurrent drain workers. The queue uses file-level locking
        // (QueueLockFailed → 2s back-off) so N workers race safely — each grabs
        // a different lease file. With Tier B GPU, 4 workers × 227s/item ≈ 18h
        // to drain 1,128 items; 1 worker ≈ 71h. Default 1 for safe rollout;
        // set SLM_DRAIN_CONCURRENCY=4 in systemd override once Tier B is stable.
        let drain_concurrency: usize = std::env::var("SLM_DRAIN_CONCURRENCY")
            .ok()
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(1)
            .max(1);

        for drain_worker_num in 0..drain_concurrency {
            let drain_cfg = queue_cfg.clone();
            let drain_doorman_arc = Arc::clone(&state);

            tokio::spawn(async move {
                // Worker identifier — PID + worker number makes lease filenames
                // unique across Doorman restarts and concurrent workers.
                let worker_id = format!("drain-{}-{}", std::process::id(), drain_worker_num);
                info!(
                    %worker_id,
                    drain_interval_secs,
                    "brief queue drain worker started"
                );

                if drain_paused {
                    info!(
                        %worker_id,
                        "drain worker: SLM_DRAIN_PAUSED=true — dispatch suspended; \
                         capture continues writing to queue/ for later GPU processing"
                    );
                }

                // Rate-limits the Tier-B-offline hold log (see `should_log_hold`) and
                // remembers whether the worker was holding last iteration, so a
                // "resumed draining" transition log can fire exactly once on recovery.
                let mut last_hold_log: Option<Instant> = None;
                const HOLD_LOG_REPEAT_INTERVAL: Duration = Duration::from_secs(300);

                loop {
                    // SLM_DRAIN_PAUSED: unconditional pause — never dequeue, never
                    // dispatch. Briefs accumulate untouched in queue/. The reaper
                    // still runs (separate task) to reclaim any stale in-flight
                    // leases. Highest-priority skip — checked before the Sprint 3C
                    // hold and before any dequeue.
                    if drain_paused {
                        tokio::time::sleep(drain_interval).await;
                        continue;
                    }

                    // Sprint 3C (+ health_up extension): when all configured Tier B
                    // nodes have been circuit-open longer than the hold threshold,
                    // or when all Tier B health probes are currently failing, skip
                    // this drain cycle. Briefs accumulate in queue/ until Tier B
                    // recovers. Health probe failures do not trip the circuit
                    // breaker directly (only dispatch failures do), so the
                    // original circuit-only predicate would miss the health-down
                    // case. SLM_TIER_A_FIRST does NOT bypass this hold: draining
                    // shadow briefs through Tier A when Tier B is offline starves
                    // entity extraction (same OLMo 7B slot). Hold regardless of
                    // tier routing preference.
                    let tier_b = drain_doorman_arc.doorman.tier_b_status();
                    if !tier_b.is_empty()
                        && tier_b.values().all(|info| {
                            // Either the circuit has been open long enough, or
                            // health probes are failing (circuit may still be
                            // "closed" if no dispatch failures have occurred yet).
                            let effectively_down = info.circuit == "open" || !info.health_up;
                            let long_enough = info
                                .opened_for_secs
                                .map(|s| s >= hold_threshold_secs)
                                // No circuit timer means circuit is closed but health
                                // is down — treat as long enough (3 consecutive probe
                                // failures already confirm the node is offline).
                                .unwrap_or(!info.health_up);
                            effectively_down && long_enough
                        })
                    {
                        if should_log_hold(last_hold_log, Instant::now(), HOLD_LOG_REPEAT_INTERVAL)
                        {
                            info!(
                            hold_threshold_secs,
                            "drain worker: all Tier B nodes offline (circuit or health) — holding queue"
                        );
                            last_hold_log = Some(Instant::now());
                        }
                        tokio::time::sleep(drain_interval).await;
                        continue;
                    } else if last_hold_log.is_some() {
                        info!(
                            %worker_id,
                            "drain worker: Tier B available again — resuming queue drain"
                        );
                        last_hold_log = None;
                    }

                    // Drain-target guard: hold if the specific "trainer" node is
                    // circuit-open or health-probe-down. The Sprint 3C hold above
                    // handles the all-nodes-down case; this guard handles the
                    // targeted-node case independently of the global tier_a_first
                    // setting. Checked before dequeue so no lease is acquired during
                    // the hold — the brief stays untouched in queue/.
                    //
                    // Does not close the 5-failure startup window (allow_request()
                    // is optimistic until the circuit opens), but closes the
                    // steady-state gap where select_tier() would fall to Tier A
                    // after the circuit has opened.
                    if !drain_doorman_arc.doorman.yoyo_node_ready("trainer") {
                        tracing::debug!(
                            target: "slm_doorman_server",
                            %worker_id,
                            "drain worker: trainer node not ready (circuit-open or health-down) \
                             — holding queue to protect Tier A inference slots"
                        );
                        tokio::time::sleep(drain_interval).await;
                        continue;
                    }

                    match dequeue_shadow(&drain_cfg, &worker_id) {
                        Ok(None) => {
                            // Queue empty; sleep and poll again.
                            tokio::time::sleep(drain_interval).await;
                        }
                        Ok(Some(leased)) => {
                            let brief_id = leased.entry.brief.brief_id.clone();
                            // Set inside the LocalSaturated match arm below; read by the
                            // retry-counter block to exempt pure slot-contention retries
                            // from the shared attempts budget (see comment there).
                            let mut is_local_saturated_retry = false;

                            // Payload size gate: poison oversized briefs immediately so
                            // they never reach OLMo or the dispatch path.
                            let payload_size = serde_json::to_vec(&leased.entry)
                                .map(|v| v.len())
                                .unwrap_or(usize::MAX);
                            if payload_size > max_payload_bytes {
                                tracing::warn!(
                                    brief_id = %brief_id,
                                    size_bytes = payload_size,
                                    max_bytes = max_payload_bytes,
                                    "drain worker: oversized payload — poisoning without dispatch"
                                );
                                if let Err(e) =
                                    release_shadow(&drain_cfg, &leased, ReleaseOutcome::Poison)
                                {
                                    tracing::warn!(
                                        brief_id = %brief_id,
                                        error = %e,
                                        "drain worker: release_shadow failed for oversized entry"
                                    );
                                }
                                continue;
                            }

                            // P0 guard: skip briefs with an empty actual_diff. These
                            // carry no ground-truth reference, so dispatching them to
                            // OLMo yields a hallucinated diff with nothing to compare
                            // against — pure wasted CPU. Worse, OLMo can run away on
                            // such out-of-distribution prompts and block the whole
                            // drain queue for the full max_tokens budget. Move straight
                            // to done without ever touching the inference tier. The
                            // decision lives in `drain::classify_shadow_brief` so it is
                            // unit-testable (drain.rs + drain_worker_integration test).
                            if matches!(
                                crate::drain::classify_shadow_brief(&leased.entry),
                                crate::drain::DrainDecision::Skip
                            ) {
                                tracing::warn!(
                                    brief_id = %brief_id,
                                    task_type = %leased.entry.brief.task_type,
                                    "drain worker: skipping empty-diff brief (no actual_diff captured); \
                                     marking done without OLMo dispatch"
                                );
                                if let Err(e) =
                                    release_shadow(&drain_cfg, &leased, ReleaseOutcome::Done)
                                {
                                    tracing::warn!(
                                        brief_id = %brief_id,
                                        error = %e,
                                        "drain worker: release_shadow failed for empty-diff brief"
                                    );
                                }
                                continue;
                            }

                            info!(
                                brief_id = %brief_id,
                                task_type = %leased.entry.brief.task_type,
                                "drain worker: dispatching queued shadow brief"
                            );

                            // Only dispatch if apprenticeship is enabled.
                            let outcome = if let Some(cfg) =
                                drain_doorman_arc.apprenticeship.as_ref()
                            {
                                use slm_doorman::ApprenticeshipDispatcher;
                                // The drain queue exists specifically to use Tier B for
                                // enrichment when it is available. Override tier_a_first
                                // (which is normally true to protect real-time paths from
                                // accruing GPU charges) and pin the yoyo node to "trainer"
                                // so briefs reach the L4 GPU rather than any offline default
                                // node. The hold-check above already ensures Tier B is healthy
                                // before this dispatcher is created.
                                let mut drain_cfg = cfg.clone();
                                drain_cfg.tier_a_first = false;
                                // All drain items go to Tier B regardless of body size:
                                // the queue exists for Tier B enrichment, and brief bodies
                                // are 100–400 chars while the default threshold is 8,000 —
                                // without this override nothing ever routes to Yoyo.
                                drain_cfg.brief_tier_b_threshold_chars = 0;
                                drain_cfg.yoyo_dispatch_label = Some("trainer".to_string());
                                let dispatcher = ApprenticeshipDispatcher::with_cache(
                                    &drain_doorman_arc.doorman,
                                    drain_cfg,
                                    Arc::clone(&drain_doorman_arc.brief_cache),
                                );
                                // Pass the actual_diff from the queue entry so the
                                // corpus tuple carries the senior's real committed diff
                                // (per §7B capture-on-completion semantics).
                                // 1860 s safety-net: the Tier A HTTP client timeout is
                                // 1800 s; this wrapper fires 60 s later to catch any
                                // path that bypasses the client timeout. With the
                                // empty-diff guard above and the Tier A max_tokens=512
                                // cap, worst-case Tier A dispatch is ~4 min, so this
                                // timeout should never fire in practice.
                                let dispatch_result = tokio::time::timeout(
                                    std::time::Duration::from_secs(1860),
                                    dispatcher.dispatch_shadow(
                                        &leased.entry.brief,
                                        &leased.entry.actual_diff,
                                    ),
                                )
                                .await;
                                match dispatch_result {
                                    Err(_elapsed) => {
                                        tracing::warn!(
                                            brief_id = %brief_id,
                                            "drain worker: dispatch timed out after 1860s — \
                                             brief will be re-queued by reaper"
                                        );
                                        ReleaseOutcome::Retry
                                    }
                                    Ok(Ok(_)) => {
                                        info!(brief_id = %brief_id, "drain worker: shadow dispatch ok");
                                        ReleaseOutcome::Done
                                    }
                                    Ok(Err(e)) => {
                                        tracing::warn!(
                                            brief_id = %brief_id,
                                            error = %e,
                                            "drain worker: shadow dispatch failed; retry"
                                        );
                                        // Check for malformed-brief class errors that should
                                        // not be retried — move to poison instead.
                                        if matches!(
                                            e,
                                            slm_doorman::DoormanError::QueueMalformedBrief { .. }
                                        ) {
                                            ReleaseOutcome::Poison
                                        } else {
                                            // Tier A slot contention (LocalSaturated) is a
                                            // known, self-imposed, transient admission-control
                                            // rejection (router.rs classifies it PolicyDenied),
                                            // not a real failure — real Tier A inference runs
                                            // 17-60 min (see local.rs's 1800s timeout) while the
                                            // shared attempts budget below is only 30s x 5 = 150s.
                                            // Without this exemption, any period of Tier A
                                            // activity poisons the brief regardless of whether
                                            // the underlying task is actually broken.
                                            is_local_saturated_retry = matches!(
                                                e,
                                                slm_doorman::DoormanError::LocalSaturated
                                            );
                                            ReleaseOutcome::Retry
                                        }
                                    }
                                }
                            } else {
                                // Apprenticeship disabled — re-queue the brief for when
                                // the operator enables the flag without restarting.
                                tracing::debug!(
                                    brief_id = %brief_id,
                                    "drain worker: apprenticeship disabled; re-queuing brief"
                                );
                                ReleaseOutcome::Retry
                            };

                            // Retry counter: escalate Retry → Poison once a brief has been
                            // retried too many times (except pure Tier A slot-contention,
                            // which is exempt — see escalate_retry_outcome's doc comment).
                            let outcome = if outcome == ReleaseOutcome::Retry {
                                escalate_retry_outcome(
                                    &drain_cfg,
                                    &brief_id,
                                    is_local_saturated_retry,
                                    max_retries,
                                )
                            } else {
                                outcome
                            };

                            // Clear the attempts sidecar on terminal outcomes so
                            // stale counters do not accumulate in queue-attempts/.
                            if matches!(outcome, ReleaseOutcome::Done | ReleaseOutcome::Poison) {
                                crate::queue::clear_attempts(&drain_cfg, &brief_id);
                            }

                            if let Err(e) = release_shadow(&drain_cfg, &leased, outcome) {
                                tracing::warn!(
                                    brief_id = %brief_id,
                                    error = %e,
                                    "drain worker: release_shadow failed"
                                );
                            }

                            // Back off after a transient failure so we don't spin
                            // tight-looping on briefs when the inference tier is
                            // unavailable (circuit open, Yo-Yo offline, etc.).
                            // LocalSaturated gets its own, longer backoff — see
                            // local_saturated_backoff's comment above; using
                            // drain_interval here let it retry at 1x/sec/worker
                            // whenever the interval was tuned down for throughput.
                            if outcome == ReleaseOutcome::Retry {
                                tokio::time::sleep(retry_sleep_duration(
                                    is_local_saturated_retry,
                                    drain_interval,
                                    local_saturated_backoff,
                                ))
                                .await;
                            }
                            // Do NOT sleep on Done/Poison — drain the queue as fast
                            // as the apprentice tier allows when it IS available.
                        }
                        Err(slm_doorman::DoormanError::QueueLockFailed { .. }) => {
                            // Another worker (or the reaper) holds the lock.  Back off
                            // and retry after a short interval.
                            tokio::time::sleep(Duration::from_secs(2)).await;
                        }
                        Err(e) => {
                            tracing::warn!(error = %e, "drain worker: dequeue error; sleeping");
                            tokio::time::sleep(drain_interval).await;
                        }
                    }
                }
            }); // end tokio::spawn
        } // end for drain_worker_num in 0..drain_concurrency

        // ── Reaper task ───────────────────────────────────────────────────
        let reap_interval = Duration::from_secs(60);
        // 2100 s = dispatch timeout (1860 s) + 240 s buffer. Must be > the
        // dispatch timeout or the reaper reclaims in-flight leases mid-dispatch,
        // producing spurious retries. Was 300 s (too short).
        let lease_expiry_secs: u64 = std::env::var("SLM_QUEUE_LEASE_EXPIRY_SEC")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(2_100);
        let lease_expiry = Duration::from_secs(lease_expiry_secs);

        let reap_cfg = queue_cfg.clone();

        tokio::spawn(async move {
            info!(
                lease_expiry_secs,
                reap_interval_secs = reap_interval.as_secs(),
                "brief queue reaper started"
            );
            loop {
                tokio::time::sleep(reap_interval).await;
                match reap_expired_leases(&reap_cfg, lease_expiry) {
                    Ok(0) => {} // nothing to do
                    Ok(n) => info!(reclaimed = n, "reaper: reclaimed expired leases"),
                    Err(e) => tracing::warn!(error = %e, "reaper: reap_expired_leases failed"),
                }
            }
        });
    }
    // ────────────────────────────────────────────────────────────────────

    // ── Yo-Yo idle monitor (B5) ─────────────────────────────────────────
    //
    // Polls llama-server /metrics every 5 min. After SLM_YOYO_IDLE_MINUTES
    // (default 30) of zero active slots, sends a GCP instances.stop request
    // via the workspace SA ADC token from the GCE metadata server.
    // Requires all four GCP env vars — absent any, the monitor does not start.
    if let Some(idle_cfg) = IdleMonitorConfig::from_env() {
        info!(
            idle_threshold_secs = idle_cfg.idle_threshold.as_secs(),
            gcp_instance = %idle_cfg.gcp_instance,
            "Yo-Yo idle monitor enabled"
        );
        tokio::spawn(crate::idle_monitor::run_idle_monitor(idle_cfg));
    }
    // ────────────────────────────────────────────────────────────────────

    // ── Chassis self-registration (app-orchestration-slm) ────────────────
    //
    // When SLM_ORCHESTRATION_ENDPOINT is set, POST our identity to the
    // chassis on startup so it can include us in GET /v1/fleet.
    // Non-blocking — a registration failure never prevents the Doorman
    // from serving local requests.
    //
    // Env vars:
    //   SLM_ORCHESTRATION_ENDPOINT  chassis base URL (e.g. http://10.0.0.1:9180)
    //   SLM_MODULE_ID               flat module identifier (e.g. "project-jennifer")
    //   SLM_ARCHIVE_ID              archive name (e.g. "cluster-totebox-jennifer")
    //   SLM_TIER_B_SUBSCRIBED       "true" if this archive has a paid Tier B
    //                               subscription; default false
    // Skip when SLM_TIER=0 already registered, fail-fast, inside build_doorman()
    // above (decision #9) — this best-effort block exists for archives that keep
    // a local llama-server AND want the chassis to know about them for GET
    // /v1/fleet; Tier 0 mode's registration already covers that and duplicating
    // it here would just be a second, redundant /v1/discovery/register call.
    let already_registered_via_tier_0 = slm_tier_is_zero();
    if let Ok(chassis_endpoint) = std::env::var("SLM_ORCHESTRATION_ENDPOINT") {
        if already_registered_via_tier_0 {
            info!(
                "SLM_TIER=0: skipping best-effort discovery ping — already registered at startup"
            );
        } else {
            let module_id = std::env::var("SLM_MODULE_ID").unwrap_or_default();
            let archive_id = std::env::var("SLM_ARCHIVE_ID").unwrap_or_default();
            let tier_b_subscribed = std::env::var("SLM_TIER_B_SUBSCRIBED")
                .map(|v| v.eq_ignore_ascii_case("true") || v == "1")
                .unwrap_or(false);
            info!(
                %chassis_endpoint,
                %module_id,
                %archive_id,
                tier_b_subscribed,
                "registering with orchestration chassis"
            );
            tokio::spawn(async move {
                let body = serde_json::json!({
                    "module_id": module_id,
                    "archive_id": archive_id,
                    "doorman_endpoint": "",
                    "tier_b_subscribed": tier_b_subscribed
                });
                let url = format!("{chassis_endpoint}/v1/discovery/register");
                match reqwest::Client::new().post(&url).json(&body).send().await {
                    Ok(resp) if resp.status().is_success() => {
                        tracing::info!(%url, "chassis registration succeeded");
                    }
                    Ok(resp) => {
                        tracing::warn!(%url, status = %resp.status(), "chassis registration rejected");
                    }
                    Err(e) => {
                        tracing::warn!(%url, error = %e, "chassis registration failed; continuing");
                    }
                }
            });
        }
    }
    // ────────────────────────────────────────────────────────────────────

    let app = http::router(state);
    let listener = tokio::net::TcpListener::bind(bind_addr)
        .await
        .with_context(|| format!("failed to bind {bind_addr}"))?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("axum serve loop exited")?;
    info!("Doorman shut down cleanly.");
    Ok(())
}

/// Awaits SIGTERM, then returns — drives the Doorman's own axum graceful
/// shutdown (drain in-flight requests, stop accepting new ones). Independent
/// of `service-content`'s own SIGTERM listener for its watcher loop; both
/// run in the same os-totebox process on separate threads, and multiple
/// independent listeners for the same signal is a safe, supported pattern
/// (see that crate's `lib.rs::run()` SIGTERM comment for the underlying
/// mechanism). tokio's "signal" feature is already required here (build
/// fails without it — this crate has depended on it since the Tier 0
/// registration code was added), so this adds no new dependency.
async fn shutdown_signal() {
    match tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()) {
        Ok(mut sigterm) => {
            sigterm.recv().await;
            info!("SIGTERM received — draining in-flight requests, no new connections accepted...");
        }
        Err(e) => {
            tracing::error!(error = %e, "failed to install SIGTERM handler — graceful shutdown unavailable, server will only stop on abrupt termination");
            std::future::pending::<()>().await;
        }
    }
}

/// Rate-limits the drain worker's "holding queue" log line while all Tier B
/// nodes remain offline. `SLM_DRAIN_CONCURRENCY=4` + `SLM_QUEUE_DRAIN_INTERVAL_SEC=1`
/// (intentional config for fast backlog draining once Tier B is healthy) means
/// 4 workers would otherwise each re-log this line every ~1s — appropriate
/// during active draining, pure log spam during a sustained multi-hour hold.
/// Returns `true` on first entering hold (`last_logged` is `None`) or once
/// `interval` has elapsed since the last log; `false` otherwise.
fn should_log_hold(last_logged: Option<Instant>, now: Instant, interval: Duration) -> bool {
    match last_logged {
        None => true,
        Some(t) => now.duration_since(t) >= interval,
    }
}

/// Decides the final `ReleaseOutcome` for a brief already classified as
/// `Retry`, applying the shared attempts-budget escalation (Retry -> Poison
/// once `attempts >= max_retries`) — with one exception: pure Tier A
/// slot-contention (`DoormanError::LocalSaturated`, `is_local_saturated_retry
/// = true`) never counts against this budget. `LocalSaturated` is a known,
/// self-imposed, transient admission-control rejection (`router.rs`
/// classifies it `PolicyDenied`), not a real failure — real Tier A inference
/// runs 17-60 min (see `local.rs`'s 1800s timeout) while the shared attempts
/// budget is only `drain_interval * max_retries` (30s x 5 = 150s by default).
/// Without this exemption, any period of Tier A activity poisons the brief
/// regardless of whether the underlying task is actually broken. The brief
/// simply re-queues and is retried again next drain cycle, indefinitely,
/// until it either succeeds or hits a genuine (non-contention) failure.
fn escalate_retry_outcome(
    cfg: &QueueConfig,
    brief_id: &str,
    is_local_saturated_retry: bool,
    max_retries: u32,
) -> ReleaseOutcome {
    if is_local_saturated_retry {
        tracing::debug!(
            brief_id = %brief_id,
            "drain worker: Tier A slot contention — retrying without \
             counting against attempts budget"
        );
        return ReleaseOutcome::Retry;
    }
    let attempts = crate::queue::bump_attempts(cfg, brief_id).unwrap_or_else(|e| {
        tracing::warn!(
            brief_id = %brief_id,
            error = %e,
            "drain worker: attempts counter I/O error; treating as 1"
        );
        1
    });
    if attempts >= max_retries {
        tracing::warn!(
            brief_id = %brief_id,
            attempts,
            max_retries,
            "drain worker: max retries reached — poisoning brief"
        );
        ReleaseOutcome::Poison
    } else {
        tracing::info!(
            brief_id = %brief_id,
            attempts,
            max_retries,
            "drain worker: brief retry {attempts}/{max_retries}"
        );
        ReleaseOutcome::Retry
    }
}

/// True when `SLM_TIER=0` (Tier 0 Doorman mode) is set — this archive has no
/// local llama-server; the "Local" compute slot routes through
/// app-orchestration-slm instead. Shared by `build_doorman` (which registers
/// and constructs the `LocalBackend::Orchestrated` client) and the later
/// best-effort chassis discovery ping (which skips itself to avoid a
/// redundant second registration).
fn slm_tier_is_zero() -> bool {
    std::env::var("SLM_TIER")
        .map(|v| v.trim() == "0")
        .unwrap_or(false)
}

/// What to do to resolve this Doorman's chassis identity at Tier 0 startup
/// (§14 #20 — per-VM discovery/allocation). Pure decision logic, no I/O —
/// see `read_identity_cache`/`write_identity_cache` for the file side and
/// `slm_doorman::tier::allocate_identity` for the chassis call.
#[derive(Debug, PartialEq, Eq)]
enum IdentityResolution {
    /// Both SLM_MODULE_ID and SLM_ARCHIVE_ID were explicitly set (non-empty)
    /// — operator override always wins, no allocation call, no cache write.
    UseProvided {
        module_id: String,
        archive_id: String,
    },
    /// No env override; a cached identity from a prior boot exists on this
    /// VM — reuse it so the identity stays stable across restarts.
    UseCached {
        module_id: String,
        archive_id: String,
    },
    /// No env override, no cache — a genuine first boot. Caller must call
    /// `allocate_identity()` and persist the result via
    /// `write_identity_cache()` before this Doorman starts serving.
    MustAllocate,
}

fn resolve_identity(
    env_module_id: Option<&str>,
    env_archive_id: Option<&str>,
    cached: Option<(String, String)>,
) -> IdentityResolution {
    match (env_module_id, env_archive_id) {
        (Some(m), Some(a)) if !m.is_empty() && !a.is_empty() => IdentityResolution::UseProvided {
            module_id: m.to_string(),
            archive_id: a.to_string(),
        },
        _ => match cached {
            Some((module_id, archive_id)) => IdentityResolution::UseCached {
                module_id,
                archive_id,
            },
            None => IdentityResolution::MustAllocate,
        },
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct IdentityCache {
    module_id: String,
    archive_id: String,
}

/// Reads a cached identity from a prior boot. Any failure (file absent,
/// unreadable, malformed) is treated as "no cache" rather than fatal — the
/// caller falls through to allocation, which is always a safe recovery path.
fn read_identity_cache(path: &std::path::Path) -> Option<(String, String)> {
    let bytes = std::fs::read(path).ok()?;
    let cache: IdentityCache = serde_json::from_slice(&bytes).ok()?;
    Some((cache.module_id, cache.archive_id))
}

/// Persists a newly-allocated identity so the next boot reuses it via
/// `read_identity_cache` instead of allocating again. Errors here are
/// propagated (not swallowed) — a silent write failure would mean every
/// subsequent boot re-allocates a fresh identity, defeating the point of
/// caching and slowly growing the chassis-side allocation ledger.
fn write_identity_cache(
    path: &std::path::Path,
    module_id: &str,
    archive_id: &str,
) -> anyhow::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("create_dir_all {}", parent.display()))?;
    }
    let cache = IdentityCache {
        module_id: module_id.to_string(),
        archive_id: archive_id.to_string(),
    };
    let json = serde_json::to_vec_pretty(&cache).context("serialize identity cache")?;
    std::fs::write(path, json).with_context(|| format!("write {}", path.display()))?;
    Ok(())
}

/// Resolves this Doorman's identity (operator-provided, cached, or freshly
/// allocated) and registers with the orchestration chassis — the whole
/// SLM_TIER=0 startup sequence as one fallible unit. Callers decide what to
/// do with an `Err` (as of 2026-07-29, the standalone-first revision: log a
/// warning and continue with no Tier 0 backend, never fail the boot — see
/// this fn's call site in `build_doorman()`).
async fn register_tier_0(
    orchestration_endpoint: &str,
    registration_token: Option<&str>,
) -> anyhow::Result<slm_doorman::tier::OrchestrationTierClient> {
    // Per-VM discovery/allocation (§14 #20): resolve this Doorman's identity —
    // operator-provided env vars win if both are set; otherwise reuse a
    // cached identity from a prior boot; otherwise this is a genuine first
    // boot, so allocate a fresh chassis-guaranteed-unique identity and
    // persist it so subsequent boots reuse it instead of re-allocating.
    let identity_cache_path = std::env::var("SLM_IDENTITY_CACHE_PATH")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| std::path::PathBuf::from("/var/lib/local-doorman/identity.json"));
    let env_module_id = std::env::var("SLM_MODULE_ID").ok();
    let env_archive_id = std::env::var("SLM_ARCHIVE_ID").ok();
    let cached_identity = read_identity_cache(&identity_cache_path);

    let (module_id, archive_id) = match resolve_identity(
        env_module_id.as_deref(),
        env_archive_id.as_deref(),
        cached_identity,
    ) {
        IdentityResolution::UseProvided {
            module_id,
            archive_id,
        } => {
            info!(%module_id, %archive_id, "SLM_TIER=0: using operator-provided identity");
            (module_id, archive_id)
        }
        IdentityResolution::UseCached {
            module_id,
            archive_id,
        } => {
            info!(
                %module_id, %archive_id, path = ?identity_cache_path,
                "SLM_TIER=0: reusing cached identity from a prior boot"
            );
            (module_id, archive_id)
        }
        IdentityResolution::MustAllocate => {
            info!(
                endpoint = %orchestration_endpoint,
                "SLM_TIER=0: no identity provided or cached — allocating a fresh one"
            );
            let (module_id, archive_id) = slm_doorman::tier::allocate_identity(
                orchestration_endpoint,
                None,
                registration_token,
            )
            .await
            .context("SLM_TIER=0: identity allocation failed")?;
            write_identity_cache(&identity_cache_path, &module_id, &archive_id).context(
                "SLM_TIER=0: failed to persist allocated identity to SLM_IDENTITY_CACHE_PATH",
            )?;
            info!(
                %module_id, %archive_id, path = ?identity_cache_path,
                "SLM_TIER=0: allocated and cached a new identity"
            );
            (module_id, archive_id)
        }
    };

    let doorman_endpoint = std::env::var("SLM_DOORMAN_ADVERTISE_ENDPOINT").unwrap_or_default();
    info!(
        endpoint = %orchestration_endpoint,
        %module_id,
        %archive_id,
        "SLM_TIER=0: registering with orchestration chassis as this Doorman's compute source"
    );
    let client = slm_doorman::tier::OrchestrationTierClient::new(
        slm_doorman::tier::OrchestrationTierConfig {
            endpoint: orchestration_endpoint.to_string(),
            module_id,
            archive_id,
            doorman_endpoint,
            registration_token: registration_token.map(str::to_string),
        },
    );
    client
        .register()
        .await
        .context("SLM_TIER=0: chassis registration failed")?;
    Ok(client)
}

/// Picks the sleep duration between drain-worker retries. `LocalSaturated`
/// retries use `local_saturated_backoff` instead of `drain_interval` — see
/// the comment where `local_saturated_backoff` is parsed from
/// `SLM_LOCAL_SATURATED_BACKOFF_SEC` for why: `drain_interval` is an
/// operator-tunable throughput knob (observed live tuned to 1s), and a
/// sustained slot-contention retry must not inherit that tuning or it
/// hammers the local background slot at whatever rate throughput was tuned
/// to, rather than backing off.
fn retry_sleep_duration(
    is_local_saturated_retry: bool,
    drain_interval: Duration,
    local_saturated_backoff: Duration,
) -> Duration {
    if is_local_saturated_retry {
        local_saturated_backoff
    } else {
        drain_interval
    }
}

async fn build_doorman() -> anyhow::Result<Doorman> {
    let force_broker = std::env::var("SLM_FORCE_BROKER_MODE")
        .map(|v| matches!(v.trim(), "true" | "1"))
        .unwrap_or(false);
    let tier_a_first = std::env::var("SLM_TIER_A_FIRST")
        .map(|v| matches!(v.trim(), "true" | "1"))
        .unwrap_or(false);

    if force_broker && tier_a_first {
        anyhow::bail!(
            "SLM_FORCE_BROKER_MODE=true and SLM_TIER_A_FIRST=true are mutually exclusive. \
             FORCE_BROKER_MODE disables Tier A entirely; TIER_A_FIRST makes it the primary. \
             Set at most one of these flags."
        );
    }

    if tier_a_first {
        info!("SLM_TIER_A_FIRST=true: Tier A is the confident primary; Tier B used only when explicitly hinted and circuit closed");
    }

    // ── Admission control semaphores ─────────────────────────────────────────
    // SLM_LOCAL_CONCURRENT (default 2): total OLMo slots across all callers.
    // SLM_BACKGROUND_CONCURRENT (default 1): cap for extraction + drain only;
    //   ensures at least one slot is always free for interactive callers.
    // SLM_TIER_B_CONCURRENT (default 4): GPU handles more concurrency than CPU.
    let local_concurrent = std::env::var("SLM_LOCAL_CONCURRENT")
        .ok()
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(2);
    let background_concurrent = std::env::var("SLM_BACKGROUND_CONCURRENT")
        .ok()
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(1);
    let tier_b_concurrent = std::env::var("SLM_TIER_B_CONCURRENT")
        .ok()
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(4);
    info!(
        local_concurrent,
        background_concurrent, tier_b_concurrent, "admission control semaphores initialised"
    );
    let total_sem = Arc::new(tokio::sync::Semaphore::new(local_concurrent));
    let background_sem = Arc::new(tokio::sync::Semaphore::new(background_concurrent));
    let tier_b_sem = Arc::new(tokio::sync::Semaphore::new(tier_b_concurrent));

    // Tier 0 Doorman mode (SLM_TIER=0): this archive has no local llama-server
    // of its own — the "Local" compute slot is served remotely through
    // app-orchestration-slm's POST /v1/inference instead. Per decision #9,
    // **superseded 2026-07-29** (BRIEF-os-totebox-platform.md "Boot dependency
    // resolved — standalone-first"): registration is attempted once here at
    // startup, but a failure is no longer fatal — VM-totebox must always boot
    // and serve DataGraph/deterministic operations regardless of
    // os-orchestration reachability (claim #54, "AI is value-add, not
    // load-bearing"). The real target market includes individuals running
    // os-totebox as a personal server who never pair with os-orchestration at
    // all. `SLM_ORCHESTRATION_ENDPOINT` being set (non-empty) is treated as
    // the operator's opt-in signal: unset/empty skips registration entirely
    // (no attempt, no warning — clean standalone boot); set-but-failing warns
    // and continues with no Tier 0 backend instead of aborting. This takes
    // precedence over SLM_FORCE_BROKER_MODE/SLM_LOCAL_ENDPOINT below when it
    // actually activates; if both are set but Tier 0 doesn't activate
    // (standalone), SLM_FORCE_BROKER_MODE/SLM_LOCAL_ENDPOINT fall through
    // normally to the `else if`/`else` arms below.
    //
    // Reuses the same SLM_ORCHESTRATION_ENDPOINT/SLM_MODULE_ID/SLM_ARCHIVE_ID
    // env vars as the existing best-effort chassis discovery ping further
    // down in run() — see that block's own comment for why it skips itself
    // when Tier 0 mode already registered here.
    //
    // Env vars:
    //   SLM_TIER                      "0" activates Tier 0 mode; unset/other = normal
    //   SLM_ORCHESTRATION_ENDPOINT    chassis base URL; unset/empty = standalone mode,
    //                                    no registration attempt at all
    //   SLM_ORCHESTRATION_REGISTRATION_TOKEN  shared admission-control secret sent as
    //                                    `Authorization: Bearer <token>` to the
    //                                    chassis's /v1/discovery/* endpoints. Absent =
    //                                    no token sent (matches the chassis's own
    //                                    backward-compatible default when its
    //                                    ORCHESTRATION_REGISTRATION_TOKEN is unset).
    //   SLM_MODULE_ID                 flat module identifier — operator override; if
    //                                    both this and SLM_ARCHIVE_ID are set, no
    //                                    allocation call happens at all (§14 #20)
    //   SLM_ARCHIVE_ID                archive name — see SLM_MODULE_ID above
    //   SLM_IDENTITY_CACHE_PATH       where an allocated identity is persisted across
    //                                    reboots, so a VM keeps the same identity
    //                                    instead of re-allocating on every boot.
    //                                    Default: /var/lib/local-doorman/identity.json
    //   SLM_DOORMAN_ADVERTISE_ENDPOINT  this Doorman's own reachable endpoint,
    //                                    advertised to the chassis at registration.
    //                                    Empty string if unset — same documented gap
    //                                    as the existing discovery-ping block below
    //                                    (SLM_BIND_ADDR is typically loopback-only and
    //                                    not usefully advertisable as-is).
    let tier_0_mode = slm_tier_is_zero();
    let orchestration_endpoint_raw = std::env::var("SLM_ORCHESTRATION_ENDPOINT").unwrap_or_default();
    let orchestration_endpoint_configured = !orchestration_endpoint_raw.trim().is_empty();

    let local = if tier_0_mode && orchestration_endpoint_configured {
        if force_broker {
            tracing::warn!(
                "SLM_TIER=0 and SLM_FORCE_BROKER_MODE=true are both set — Tier 0 mode takes \
                 precedence; SLM_FORCE_BROKER_MODE has no effect here."
            );
        }
        let orchestration_endpoint = orchestration_endpoint_raw;
        let registration_token = std::env::var("SLM_ORCHESTRATION_REGISTRATION_TOKEN").ok();

        match register_tier_0(&orchestration_endpoint, registration_token.as_deref()).await {
            Ok(client) => {
                info!(
                    "SLM_TIER=0: chassis registration succeeded; Doorman ready to serve via \
                     orchestration"
                );
                Some(LocalBackend::from(client))
            }
            Err(e) => {
                tracing::warn!(
                    error = %e,
                    "SLM_TIER=0: chassis registration did not succeed — starting with no Tier 0 \
                     compute backend; inference reports unavailable until orchestration is \
                     reachable and the Doorman is restarted (standalone-first, supersedes the \
                     old decision #9 fail-fast)"
                );
                None
            }
        }
    } else if tier_0_mode {
        info!(
            "SLM_TIER=0 set but SLM_ORCHESTRATION_ENDPOINT is unset/empty — starting in \
             standalone mode (no Tier 0 compute backend); inference is unavailable until an \
             orchestration endpoint is configured and the Doorman is restarted. \
             DataGraph/deterministic operations are unaffected."
        );
        None
    } else if force_broker {
        info!("SLM_FORCE_BROKER_MODE=true: Tier A disabled; all inference routes to Yo-Yo");
        None
    } else {
        let local_endpoint = std::env::var("SLM_LOCAL_ENDPOINT")
            .unwrap_or_else(|_| "http://127.0.0.1:8080".to_string());
        let local_default_model =
            std::env::var("SLM_LOCAL_MODEL").unwrap_or_else(|_| "olmo-3-7b-instruct".to_string());
        // Surfaces SLM_LOCAL_MODEL resolution at boot — this is a response-metadata/
        // audit-ledger LABEL only (llama-server ignores the request's "model" field
        // for routing), but it has drifted silently before: 3 separate env sources
        // (main unit Environment=, EnvironmentFile=, and a drop-in override) can each
        // set this var, and a `daemon-reload` without a following `restart` leaves an
        // already-running process on a stale value that `systemctl show` no longer
        // reflects (confirmed live 2026-07-03: process env still said
        // OLMo-2-1124-7B-Instruct-Q4_K_M.gguf while the drop-in and `systemctl show`
        // both correctly resolved to OLMo-3-7B-Instruct). Logging it here makes that
        // class of drift visible in journalctl without manual /proc/<pid>/environ
        // inspection.
        info!(
            default_model = %local_default_model,
            endpoint = %local_endpoint,
            "Local tier (Tier A) model label resolved at startup"
        );
        Some(LocalBackend::from(
            LocalTierClient::new(LocalTierConfig {
                endpoint: local_endpoint,
                default_model: local_default_model,
            })
            .with_semaphores(Arc::clone(&total_sem), Arc::clone(&background_sem)),
        ))
    };

    let mut yoyo = std::collections::HashMap::new();

    // 1. Check for legacy SLM_YOYO_ENDPOINT (mapped to "default")
    if let Some(client) = build_yoyo_client(
        "SLM_YOYO_ENDPOINT",
        "SLM_YOYO_MODEL",
        "SLM_YOYO_BEARER",
        "SLM_YOYO_HOURLY_USD",
    )
    .map(|c| c.with_concurrency_sem(Arc::clone(&tier_b_sem)))
    {
        yoyo.insert("default".to_string(), client);
    }

    // 2. Check for specialized Multi-Yo-Yo endpoints (Leapfrog 2030)
    if let Some(client) = build_yoyo_client(
        "SLM_YOYO_TRAINER_ENDPOINT",
        "SLM_YOYO_TRAINER_MODEL",
        "SLM_YOYO_TRAINER_BEARER",
        "SLM_YOYO_TRAINER_HOURLY_USD",
    )
    .map(|c| c.with_concurrency_sem(Arc::clone(&tier_b_sem)))
    {
        info!("Yo-Yo 'trainer' node configured");
        yoyo.insert("trainer".to_string(), client);
    }

    if let Some(client) = build_yoyo_client(
        "SLM_YOYO_GRAPH_ENDPOINT",
        "SLM_YOYO_GRAPH_MODEL",
        "SLM_YOYO_GRAPH_BEARER",
        "SLM_YOYO_GRAPH_HOURLY_USD",
    )
    .map(|c| c.with_concurrency_sem(Arc::clone(&tier_b_sem)))
    {
        info!("Yo-Yo 'graph' node configured");
        yoyo.insert("graph".to_string(), client);
    }

    let external = build_external_tier_client();

    // PS.3 step 5 — Lark grammar pre-validation.
    // Enabled by default; set SLM_LARK_VALIDATION_ENABLED=false to disable
    // (e.g., if the llguidance init overhead is undesirable in a test
    // environment that never submits Lark grammars).
    let lark_validator = {
        let enabled = std::env::var("SLM_LARK_VALIDATION_ENABLED")
            .map(|v| !matches!(v.trim(), "false" | "0"))
            .unwrap_or(true);
        if enabled {
            match LarkValidator::new() {
                Ok(v) => {
                    info!("Lark grammar pre-validation enabled (PS.3 step 5)");
                    Some(v)
                }
                Err(e) => {
                    // Validation init failure is non-fatal — the Doorman
                    // starts without it and logs a warning.
                    tracing::warn!("LarkValidator init failed (Lark pre-validation disabled): {e}");
                    None
                }
            }
        } else {
            info!("Lark grammar pre-validation disabled (SLM_LARK_VALIDATION_ENABLED=false)");
            None
        }
    };

    // Resolve the audit ledger directory.  SLM_AUDIT_DIR takes precedence;
    // fall back to the $HOME/.service-slm/audit/ default on any error.
    let ledger = match std::env::var_os("SLM_AUDIT_DIR") {
        Some(path) if !path.is_empty() => {
            let dir = std::path::PathBuf::from(&path);
            match std::fs::create_dir_all(&dir) {
                Ok(()) => match AuditLedger::new(&dir) {
                    Ok(l) => {
                        info!(audit_dir = %dir.display(), "audit ledger directory (SLM_AUDIT_DIR)");
                        l
                    }
                    Err(e) => {
                        tracing::warn!(
                            audit_dir = %dir.display(),
                            error = %e,
                            "SLM_AUDIT_DIR unusable; falling back to default"
                        );
                        AuditLedger::default_for_user()
                            .context("failed to open fallback audit ledger; ensure HOME is set")?
                    }
                },
                Err(e) => {
                    tracing::warn!(
                        audit_dir = %dir.display(),
                        error = %e,
                        "SLM_AUDIT_DIR create_dir_all failed; falling back to default"
                    );
                    AuditLedger::default_for_user()
                        .context("failed to open fallback audit ledger; ensure HOME is set")?
                }
            }
        }
        _ => {
            let l = AuditLedger::default_for_user()
                .context("failed to open audit ledger; ensure HOME is set")?;
            info!(audit_dir = %l.base_dir().display(), "audit ledger directory (default)");
            l
        }
    };

    // Graph context (service-content Ring 2 — Brief E).
    // When SERVICE_CONTENT_ENDPOINT is set, the Doorman queries the
    // service-content graph before each inference call and injects matching
    // entity rows as a system message. Non-fatal if absent.
    let graph_context_client = std::env::var("SERVICE_CONTENT_ENDPOINT").ok().map(|ep| {
        info!("Graph context enabled; service-content endpoint: {}", ep);
        GraphContextClient::new(ep)
    });

    // Daily Tier B spend cap (P3-3.5-followup). Non-fatal if unavailable.
    let foundry_root = std::env::var_os("FOUNDRY_ROOT")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| std::path::PathBuf::from("/srv/foundry"));
    let cost_ledger_dir = foundry_root.join("data").join("cost-ledger");
    let cost_ledger = match std::fs::create_dir_all(&cost_ledger_dir)
        .and_then(|_| slm_doorman::cost_ledger::CostLedger::new(&cost_ledger_dir))
    {
        Ok(cl) => {
            info!(dir = %cost_ledger_dir.display(), "cost ledger initialised");
            Some(std::sync::Arc::new(cl))
        }
        Err(e) => {
            tracing::warn!(error = %e, "cost ledger unavailable — no spend tracking or cap enforcement");
            None
        }
    };
    let daily_yoyo_cap_usd = std::env::var("SLM_YOYO_DAILY_CAP_USD")
        .ok()
        .and_then(|v| v.parse::<f64>().ok())
        .filter(|&v| v > 0.0);
    if let Some(cap) = daily_yoyo_cap_usd {
        info!(
            cap_usd = cap,
            "daily Tier B spend cap configured (SLM_YOYO_DAILY_CAP_USD)"
        );
    }

    Ok(Doorman::new(
        DoormanConfig {
            local,
            yoyo,
            external,
            lark_validator,
            graph_context_client,
            tier_a_first,
            daily_yoyo_cap_usd,
            cost_ledger,
        },
        ledger,
    ))
}

fn build_yoyo_client(
    env_endpoint: &str,
    env_model: &str,
    env_bearer: &str,
    env_hourly: &str,
) -> Option<YoYoTierClient> {
    match std::env::var(env_endpoint) {
        Ok(endpoint) if !endpoint.is_empty() => {
            let use_gcp_auth = std::env::var("SLM_YOYO_GCP_AUTH")
                .map(|v| v.eq_ignore_ascii_case("true"))
                .unwrap_or(false);
            if use_gcp_auth && std::env::var("SLM_YOYO_GCP_ZONE").is_err() {
                warn!("SLM_YOYO_GCP_AUTH=true but SLM_YOYO_GCP_ZONE is unset; /readyz zone field will be empty");
            }
            let bearer: Arc<dyn BearerTokenProvider> = if use_gcp_auth {
                Arc::new(MetadataBearer::new(&endpoint))
            } else {
                let bearer_token = std::env::var(env_bearer).unwrap_or_default();
                Arc::new(StaticBearer::new(bearer_token))
            };
            let yoyo_hourly_usd = std::env::var(env_hourly)
                .ok()
                .and_then(|s| s.parse::<f64>().ok())
                .unwrap_or(0.0);
            let health_path =
                std::env::var("SLM_YOYO_HEALTH_PATH").unwrap_or_else(|_| "/health".to_string());
            if !health_path.starts_with('/') {
                eprintln!(
                    "[FATAL] SLM_YOYO_HEALTH_PATH must start with '/' (got {:?})",
                    health_path
                );
                std::process::exit(1);
            }
            // Persist circuit-breaker state across restarts (fixes the bug
            // where every `systemctl restart local-doorman` reset a
            // genuinely-open breaker back to "healthy" regardless of Tier
            // B's real state, letting fresh batches through to a target
            // that was still down). `SLM_CIRCUIT_STATE_PATH` overrides the
            // default; relative paths resolve against the service's
            // WorkingDirectory (/var/lib/local-doorman per the systemd unit).
            let circuit_state_path = std::env::var("SLM_CIRCUIT_STATE_PATH")
                .unwrap_or_else(|_| "circuit_breaker_state.json".to_string());
            Some(
                YoYoTierClient::new(
                    YoYoTierConfig {
                        endpoint,
                        default_model: std::env::var(env_model)
                            .unwrap_or_else(|_| "Olmo-3-1125-32B-Think".to_string()),
                        contract_version: slm_doorman::YOYO_CONTRACT_VERSION.to_string(),
                        pricing: PricingConfig { yoyo_hourly_usd },
                        zone: std::env::var("SLM_YOYO_GCP_ZONE").ok(),
                        health_path,
                    },
                    bearer,
                )
                .with_persistent_circuit(std::path::PathBuf::from(circuit_state_path)),
            )
        }
        _ => None,
    }
}

/// Build the Tier C (external API) client from env vars. Returns `None`
/// if no provider endpoints are configured — operator cost guardrail
/// ensures no Tier C dispatch happens unless explicitly enabled.
///
/// Env var format per provider:
///   SLM_TIER_C_ANTHROPIC_ENDPOINT      base URL (e.g., https://api.anthropic.com)
///   SLM_TIER_C_ANTHROPIC_API_KEY       API key (can be empty in dev/mock mode)
///   SLM_TIER_C_ANTHROPIC_INPUT_PER_MTOK_USD    pricing (default 0.0)
///   SLM_TIER_C_ANTHROPIC_OUTPUT_PER_MTOK_USD   pricing (default 0.0)
/// Same pattern for GEMINI and OPENAI.
fn build_external_tier_client() -> Option<ExternalTierClient> {
    let mut endpoints = std::collections::HashMap::new();
    let mut api_keys = std::collections::HashMap::new();
    let mut pricing = TierCPricing::default();

    // Anthropic
    if let Ok(endpoint) = std::env::var("SLM_TIER_C_ANTHROPIC_ENDPOINT") {
        if !endpoint.is_empty() {
            endpoints.insert(TierCProvider::Anthropic, endpoint);
            api_keys.insert(
                TierCProvider::Anthropic,
                std::env::var("SLM_TIER_C_ANTHROPIC_API_KEY").unwrap_or_default(),
            );
            pricing.anthropic_input_per_mtok_usd =
                std::env::var("SLM_TIER_C_ANTHROPIC_INPUT_PER_MTOK_USD")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0.0);
            pricing.anthropic_output_per_mtok_usd =
                std::env::var("SLM_TIER_C_ANTHROPIC_OUTPUT_PER_MTOK_USD")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0.0);
        }
    }

    // Gemini
    if let Ok(endpoint) = std::env::var("SLM_TIER_C_GEMINI_ENDPOINT") {
        if !endpoint.is_empty() {
            endpoints.insert(TierCProvider::Gemini, endpoint);
            api_keys.insert(
                TierCProvider::Gemini,
                std::env::var("SLM_TIER_C_GEMINI_API_KEY").unwrap_or_default(),
            );
            pricing.gemini_input_per_mtok_usd =
                std::env::var("SLM_TIER_C_GEMINI_INPUT_PER_MTOK_USD")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0.0);
            pricing.gemini_output_per_mtok_usd =
                std::env::var("SLM_TIER_C_GEMINI_OUTPUT_PER_MTOK_USD")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0.0);
        }
    }

    // OpenAI
    if let Ok(endpoint) = std::env::var("SLM_TIER_C_OPENAI_ENDPOINT") {
        if !endpoint.is_empty() {
            endpoints.insert(TierCProvider::Openai, endpoint);
            api_keys.insert(
                TierCProvider::Openai,
                std::env::var("SLM_TIER_C_OPENAI_API_KEY").unwrap_or_default(),
            );
            pricing.openai_input_per_mtok_usd =
                std::env::var("SLM_TIER_C_OPENAI_INPUT_PER_MTOK_USD")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0.0);
            pricing.openai_output_per_mtok_usd =
                std::env::var("SLM_TIER_C_OPENAI_OUTPUT_PER_MTOK_USD")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0.0);
        }
    }

    // Only build the client if at least one provider is configured.
    if endpoints.is_empty() {
        return None;
    }

    let config = ExternalTierConfig {
        allowlist: FOUNDRY_DEFAULT_ALLOWLIST,
        provider_endpoints: endpoints,
        provider_api_keys: api_keys,
        pricing,
    };

    Some(ExternalTierClient::new(config))
}

/// Build the audit proxy client from env vars. Reuses the same
/// `SLM_TIER_C_*` namespace as `build_external_tier_client()` — the
/// audit_proxy relay and the Tier C compute routing share provider
/// config so operators only need one set of env vars.
///
/// Returns `None` if no providers are configured. An absent client causes
/// `POST /v1/audit/proxy` to return 503 with a clear "unconfigured" message.
fn build_audit_proxy_client() -> Option<AuditProxyClient> {
    let mut endpoints = std::collections::HashMap::new();
    let mut api_keys = std::collections::HashMap::new();
    let mut pricing = TierCPricing::default();

    // Anthropic
    if let Ok(endpoint) = std::env::var("SLM_TIER_C_ANTHROPIC_ENDPOINT") {
        if !endpoint.is_empty() {
            endpoints.insert(TierCProvider::Anthropic, endpoint);
            api_keys.insert(
                TierCProvider::Anthropic,
                std::env::var("SLM_TIER_C_ANTHROPIC_API_KEY").unwrap_or_default(),
            );
            pricing.anthropic_input_per_mtok_usd =
                std::env::var("SLM_TIER_C_ANTHROPIC_INPUT_PER_MTOK_USD")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0.0);
            pricing.anthropic_output_per_mtok_usd =
                std::env::var("SLM_TIER_C_ANTHROPIC_OUTPUT_PER_MTOK_USD")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0.0);
        }
    }

    // Gemini
    if let Ok(endpoint) = std::env::var("SLM_TIER_C_GEMINI_ENDPOINT") {
        if !endpoint.is_empty() {
            endpoints.insert(TierCProvider::Gemini, endpoint);
            api_keys.insert(
                TierCProvider::Gemini,
                std::env::var("SLM_TIER_C_GEMINI_API_KEY").unwrap_or_default(),
            );
            pricing.gemini_input_per_mtok_usd =
                std::env::var("SLM_TIER_C_GEMINI_INPUT_PER_MTOK_USD")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0.0);
            pricing.gemini_output_per_mtok_usd =
                std::env::var("SLM_TIER_C_GEMINI_OUTPUT_PER_MTOK_USD")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0.0);
        }
    }

    // OpenAI
    if let Ok(endpoint) = std::env::var("SLM_TIER_C_OPENAI_ENDPOINT") {
        if !endpoint.is_empty() {
            endpoints.insert(TierCProvider::Openai, endpoint);
            api_keys.insert(
                TierCProvider::Openai,
                std::env::var("SLM_TIER_C_OPENAI_API_KEY").unwrap_or_default(),
            );
            pricing.openai_input_per_mtok_usd =
                std::env::var("SLM_TIER_C_OPENAI_INPUT_PER_MTOK_USD")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0.0);
            pricing.openai_output_per_mtok_usd =
                std::env::var("SLM_TIER_C_OPENAI_OUTPUT_PER_MTOK_USD")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0.0);
        }
    }

    if endpoints.is_empty() {
        return None;
    }

    Some(AuditProxyClient::new(AuditProxyConfig {
        provider_endpoints: endpoints,
        provider_api_keys: api_keys,
        pricing,
        // PS.4 step 3 — default to the four documented purposes.
        purpose_allowlist: FOUNDRY_DEFAULT_PURPOSE_ALLOWLIST,
    }))
}

/// Build the apprenticeship config when `SLM_APPRENTICESHIP_ENABLED=true`.
/// Default off — existing deployments keep their existing behaviour
/// (the three apprenticeship endpoints return 404). Per design-pass Q9
/// + Master's brief.
fn build_apprenticeship_config() -> Option<ApprenticeshipConfig> {
    let enabled = std::env::var("SLM_APPRENTICESHIP_ENABLED")
        .ok()
        .map(|s| s.eq_ignore_ascii_case("true") || s == "1")
        .unwrap_or(false);
    if !enabled {
        return None;
    }
    Some(ApprenticeshipConfig::from_env())
}

/// Build the AS-3 verdict dispatcher: shells out to `ssh-keygen -Y
/// verify` against `${FOUNDRY_ROOT}/identity/allowed_signers` (or
/// `FOUNDRY_ALLOWED_SIGNERS` override per design-pass Q1) and writes
/// corpus tuples + ledger events under `${FOUNDRY_ROOT}/data/`.
fn build_verdict_dispatcher(
    cfg: &ApprenticeshipConfig,
    cache: Arc<BriefCache>,
) -> anyhow::Result<VerdictDispatcher> {
    let allowed_signers = std::env::var_os("FOUNDRY_ALLOWED_SIGNERS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| cfg.foundry_root.join("identity").join("allowed_signers"));
    let verifier: Arc<dyn VerdictVerifier> = Arc::new(SshKeygenVerifier::new(allowed_signers));
    let ledger_dir = cfg.foundry_root.join("data").join("apprenticeship");
    let ledger = PromotionLedger::new(ledger_dir).context("create promotion ledger dir")?;
    let doctrine_version =
        std::env::var("FOUNDRY_DOCTRINE_VERSION").unwrap_or_else(|_| "0.0.7".to_string());
    let tenant = std::env::var("FOUNDRY_TENANT").unwrap_or_else(|_| "pointsav".to_string());
    Ok(VerdictDispatcher {
        verifier,
        cache,
        ledger,
        corpus_root: cfg.foundry_root.clone(),
        doctrine_version,
        tenant,
    })
}

fn init_tracing() {
    use tracing_subscriber::{fmt, prelude::*, EnvFilter};

    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("slm_doorman=info,slm_doorman_server=info,axum=warn"));
    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer())
        .init();
}

#[cfg(test)]
mod escalate_retry_outcome_tests {
    use super::*;

    fn tmp_cfg(label: &str) -> QueueConfig {
        let dir = std::env::temp_dir().join(format!(
            "slm-doorman-escalate-test-{label}-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("queue-attempts")).unwrap();
        QueueConfig::with_base_dir(dir)
    }

    #[test]
    fn local_saturated_retry_never_bumps_attempts_or_poisons() {
        let cfg = tmp_cfg("saturated");
        // Call it more times than max_retries would normally allow — a pure
        // slot-contention retry must never escalate to Poison, no matter how
        // many times it happens.
        for _ in 0..10 {
            let outcome = escalate_retry_outcome(&cfg, "brief-a", true, 5);
            assert_eq!(outcome, ReleaseOutcome::Retry);
        }
        // Confirm the attempts sidecar was genuinely never touched — this is
        // the actual behavioral guarantee, not just the return value.
        let attempts_file = cfg.base_dir.join("queue-attempts").join("brief-a.attempts");
        assert!(
            !attempts_file.exists(),
            "LocalSaturated retries must not write to the attempts sidecar"
        );
    }

    #[test]
    fn genuine_retry_still_escalates_to_poison_after_max_retries() {
        let cfg = tmp_cfg("genuine");
        let mut last = ReleaseOutcome::Retry;
        for _ in 0..5 {
            last = escalate_retry_outcome(&cfg, "brief-b", false, 5);
        }
        assert_eq!(
            last,
            ReleaseOutcome::Poison,
            "a genuinely failing brief must still poison after max_retries — \
             the LocalSaturated exemption must not weaken this"
        );
    }

    #[test]
    fn genuine_retry_does_not_poison_before_max_retries() {
        let cfg = tmp_cfg("genuine-early");
        let outcome = escalate_retry_outcome(&cfg, "brief-c", false, 5);
        assert_eq!(outcome, ReleaseOutcome::Retry);
    }
}

#[cfg(test)]
mod retry_sleep_duration_tests {
    use super::*;

    /// The bug fixed 2026-07-15: a LocalSaturated-exempt brief was sleeping
    /// for `drain_interval` (an operator-tunable throughput knob observed
    /// live at 1s) between retries, producing ~180 dispatch attempts/60s
    /// against the local background slot instead of a bounded backoff.
    #[test]
    fn local_saturated_retry_uses_dedicated_backoff_not_drain_interval() {
        let drain_interval = Duration::from_secs(1);
        let local_saturated_backoff = Duration::from_secs(30);
        let sleep_for = retry_sleep_duration(true, drain_interval, local_saturated_backoff);
        assert_eq!(sleep_for, local_saturated_backoff);
        assert_ne!(
            sleep_for, drain_interval,
            "LocalSaturated retries must not inherit drain_interval's tuning"
        );
    }

    #[test]
    fn genuine_retry_still_uses_drain_interval() {
        let drain_interval = Duration::from_secs(1);
        let local_saturated_backoff = Duration::from_secs(30);
        let sleep_for = retry_sleep_duration(false, drain_interval, local_saturated_backoff);
        assert_eq!(sleep_for, drain_interval);
    }
}

#[cfg(test)]
mod identity_resolution_tests {
    use super::*;

    #[test]
    fn both_env_vars_set_wins_even_with_a_cache_present() {
        let cached = Some(("op::cached::slm".to_string(), "cached".to_string()));
        let res = resolve_identity(Some("op::env::slm"), Some("env-archive"), cached);
        assert_eq!(
            res,
            IdentityResolution::UseProvided {
                module_id: "op::env::slm".to_string(),
                archive_id: "env-archive".to_string(),
            },
            "operator-provided env vars must win over any cached identity"
        );
    }

    #[test]
    fn only_one_env_var_set_is_not_provided_falls_through() {
        // Half-configured (e.g. SLM_MODULE_ID set, SLM_ARCHIVE_ID forgotten)
        // must not be treated as a valid operator override.
        let res = resolve_identity(Some("op::env::slm"), None, None);
        assert_eq!(res, IdentityResolution::MustAllocate);
    }

    #[test]
    fn empty_string_env_vars_are_not_provided() {
        // std::env::var returns Some("") for a var set to an empty string,
        // distinct from None (unset) — must not be treated as valid.
        let res = resolve_identity(Some(""), Some(""), None);
        assert_eq!(res, IdentityResolution::MustAllocate);
    }

    #[test]
    fn no_env_vars_but_cache_present_reuses_cache() {
        let cached = Some(("op::cached::slm".to_string(), "cached".to_string()));
        let res = resolve_identity(None, None, cached);
        assert_eq!(
            res,
            IdentityResolution::UseCached {
                module_id: "op::cached::slm".to_string(),
                archive_id: "cached".to_string(),
            }
        );
    }

    #[test]
    fn no_env_vars_no_cache_must_allocate() {
        let res = resolve_identity(None, None, None);
        assert_eq!(res, IdentityResolution::MustAllocate);
    }

    fn tmp_cache_path(label: &str) -> std::path::PathBuf {
        std::env::temp_dir().join(format!(
            "slm-doorman-identity-cache-test-{label}-{}",
            std::process::id()
        ))
    }

    #[test]
    fn read_identity_cache_returns_none_when_file_absent() {
        let path = tmp_cache_path("absent");
        let _ = std::fs::remove_file(&path);
        assert_eq!(read_identity_cache(&path), None);
    }

    #[test]
    fn read_identity_cache_returns_none_for_malformed_json() {
        let path = tmp_cache_path("malformed");
        std::fs::write(&path, b"not json").unwrap();
        assert_eq!(
            read_identity_cache(&path),
            None,
            "a malformed cache file must fall through to allocation, not panic or error"
        );
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn write_then_read_identity_cache_round_trips() {
        let path = tmp_cache_path("roundtrip");
        let _ = std::fs::remove_file(&path);
        write_identity_cache(&path, "op::written::slm", "written-archive")
            .expect("write must succeed");
        let read_back = read_identity_cache(&path);
        assert_eq!(
            read_back,
            Some((
                "op::written::slm".to_string(),
                "written-archive".to_string()
            ))
        );
        let _ = std::fs::remove_file(&path);
    }
}

#[cfg(test)]
mod should_log_hold_tests {
    use super::*;

    #[test]
    fn first_entry_into_hold_always_logs() {
        assert!(should_log_hold(
            None,
            Instant::now(),
            Duration::from_secs(300)
        ));
    }

    #[test]
    fn does_not_relog_before_interval_elapses() {
        let now = Instant::now();
        assert!(!should_log_hold(
            Some(now),
            now + Duration::from_secs(1),
            Duration::from_secs(300)
        ));
    }

    #[test]
    fn relogs_once_interval_has_elapsed() {
        let now = Instant::now();
        assert!(should_log_hold(
            Some(now),
            now + Duration::from_secs(301),
            Duration::from_secs(300)
        ));
    }

    #[test]
    fn relogs_exactly_at_interval_boundary() {
        let now = Instant::now();
        assert!(should_log_hold(
            Some(now),
            now + Duration::from_secs(300),
            Duration::from_secs(300)
        ));
    }
}
