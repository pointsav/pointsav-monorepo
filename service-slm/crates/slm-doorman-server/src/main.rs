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
//!   RUST_LOG                  default slm_doorman=info,slm_doorman_server=info
//!
//! Per `conventions/three-ring-architecture.md` the Doorman boots fine
//! with no Yo-Yo configured (Optional Intelligence). B5 verifies this
//! end-to-end.

use slm_doorman_server::http;
use slm_doorman_server::idle_monitor::IdleMonitorConfig;
use std::sync::atomic::AtomicU64;
use slm_doorman_server::queue::{
    dequeue_shadow, ensure_dirs, reap_expired_leases, release_shadow, QueueConfig, ReleaseOutcome,
};

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use anyhow::Context;
use slm_doorman::tier::{
    BearerTokenProvider, ExternalTierClient, ExternalTierConfig, LocalTierClient, LocalTierConfig,
    PricingConfig, StaticBearer, TierCPricing, TierCProvider, YoYoTierClient, YoYoTierConfig,
    FOUNDRY_DEFAULT_ALLOWLIST,
};
use slm_doorman::{
    ApprenticeshipConfig, AuditLedger, AuditProxyClient, AuditProxyConfig, BriefCache, Doorman,
    DoormanConfig, GraphContextClient, LarkValidator, PromotionLedger, SshKeygenVerifier,
    VerdictDispatcher, VerdictVerifier, FOUNDRY_DEFAULT_PURPOSE_ALLOWLIST,
};
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing();

    let bind_addr: SocketAddr = std::env::var("SLM_BIND_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:9080".to_string())
        .parse()
        .context("SLM_BIND_ADDR must be a socket address")?;

    let doorman = build_doorman()?;
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

    // Shared dispatch clock — updated by the HTTP router on every successful
    // Tier B dispatch; read by the idle monitor to prevent premature VM stops
    // when the 5-min poll catches an inter-request gap (slots=0).
    let last_yoyo_dispatch = Arc::new(AtomicU64::new(0));

    // Graph proxy — reuse the SERVICE_CONTENT_ENDPOINT already consumed by
    // GraphContextClient above. Default to 127.0.0.1:9081 if unset so the
    // proxy is available in community-tier deployments without extra config.
    let service_content_endpoint = std::env::var("SERVICE_CONTENT_ENDPOINT")
        .unwrap_or_else(|_| http::DEFAULT_SERVICE_CONTENT_ENDPOINT.to_string());

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
        // Dispatch clock shared with the idle monitor.
        last_yoyo_dispatch: Arc::clone(&last_yoyo_dispatch),
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
    //   SLM_QUEUE_LEASE_EXPIRY_SEC     lease age before reaper reclaims; default 300s
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

        // Clone only what the drain worker needs.
        let drain_cfg = queue_cfg.clone();
        let drain_doorman_arc = Arc::clone(&state);

        tokio::spawn(async move {
            // Worker identifier — use the process PID so lease filenames are
            // unique across Doorman restarts without any coordination.
            let worker_id = format!("drain-{}", std::process::id());
            info!(
                %worker_id,
                drain_interval_secs,
                "brief queue drain worker started"
            );

            loop {
                match dequeue_shadow(&drain_cfg, &worker_id) {
                    Ok(None) => {
                        // Queue empty; sleep and poll again.
                        tokio::time::sleep(drain_interval).await;
                    }
                    Ok(Some(leased)) => {
                        let brief_id = leased.entry.brief.brief_id.clone();
                        info!(
                            brief_id = %brief_id,
                            task_type = %leased.entry.brief.task_type,
                            "drain worker: dispatching queued shadow brief"
                        );

                        // Only dispatch if apprenticeship is enabled.
                        let outcome = if let Some(cfg) = drain_doorman_arc.apprenticeship.as_ref() {
                            use slm_doorman::ApprenticeshipDispatcher;
                            let dispatcher = ApprenticeshipDispatcher::with_cache(
                                &drain_doorman_arc.doorman,
                                cfg.clone(),
                                Arc::clone(&drain_doorman_arc.brief_cache),
                            );
                            // Pass the actual_diff from the queue entry so the
                            // corpus tuple carries the senior's real committed diff
                            // (per §7B capture-on-completion semantics).
                            match dispatcher
                                .dispatch_shadow(&leased.entry.brief, &leased.entry.actual_diff)
                                .await
                            {
                                Ok(_) => {
                                    info!(brief_id = %brief_id, "drain worker: shadow dispatch ok");
                                    ReleaseOutcome::Done
                                }
                                Err(e) => {
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

                        if let Err(e) = release_shadow(&drain_cfg, &leased, outcome) {
                            tracing::warn!(
                                brief_id = %brief_id,
                                error = %e,
                                "drain worker: release_shadow failed"
                            );
                        }

                        // Do NOT sleep between briefs — drain the queue as fast as
                        // the apprentice tier allows.  The poll sleep only fires when
                        // the queue is empty.
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
        });

        // ── Reaper task ───────────────────────────────────────────────────
        let reap_interval = Duration::from_secs(60);
        let lease_expiry_secs: u64 = std::env::var("SLM_QUEUE_LEASE_EXPIRY_SEC")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(300);
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
    if let Some(mut idle_cfg) = IdleMonitorConfig::from_env() {
        // Wire in the shared dispatch clock so the idle monitor can account for
        // Tier B dispatches that occurred between 5-min poll intervals.
        idle_cfg.last_yoyo_dispatch = Arc::clone(&last_yoyo_dispatch);
        info!(
            idle_threshold_secs = idle_cfg.idle_threshold.as_secs(),
            gcp_instance = %idle_cfg.gcp_instance,
            "Yo-Yo idle monitor enabled"
        );
        tokio::spawn(slm_doorman_server::idle_monitor::run_idle_monitor(idle_cfg));
    }
    // ────────────────────────────────────────────────────────────────────

    let app = http::router(state);
    let listener = tokio::net::TcpListener::bind(bind_addr)
        .await
        .with_context(|| format!("failed to bind {bind_addr}"))?;
    axum::serve(listener, app)
        .await
        .context("axum serve loop exited")?;
    Ok(())
}

fn build_doorman() -> anyhow::Result<Doorman> {
    let local = Some(LocalTierClient::new(LocalTierConfig {
        endpoint: std::env::var("SLM_LOCAL_ENDPOINT")
            .unwrap_or_else(|_| "http://127.0.0.1:8080".to_string()),
        default_model: std::env::var("SLM_LOCAL_MODEL")
            .unwrap_or_else(|_| "olmo-3-7b-instruct".to_string()),
    }));

    let mut yoyo = std::collections::HashMap::new();

    // 1. Check for legacy SLM_YOYO_ENDPOINT (mapped to "default")
    if let Some(client) = build_yoyo_client(
        "SLM_YOYO_ENDPOINT",
        "SLM_YOYO_MODEL",
        "SLM_YOYO_BEARER",
        "SLM_YOYO_HOURLY_USD",
    ) {
        yoyo.insert("default".to_string(), client);
    }

    // 2. Check for specialized Multi-Yo-Yo endpoints (Leapfrog 2030)
    if let Some(client) = build_yoyo_client(
        "SLM_YOYO_TRAINER_ENDPOINT",
        "SLM_YOYO_TRAINER_MODEL",
        "SLM_YOYO_TRAINER_BEARER",
        "SLM_YOYO_TRAINER_HOURLY_USD",
    ) {
        info!("Yo-Yo 'trainer' node configured");
        yoyo.insert("trainer".to_string(), client);
    }

    if let Some(client) = build_yoyo_client(
        "SLM_YOYO_GRAPH_ENDPOINT",
        "SLM_YOYO_GRAPH_MODEL",
        "SLM_YOYO_GRAPH_BEARER",
        "SLM_YOYO_GRAPH_HOURLY_USD",
    ) {
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
    let graph_context_client =
        std::env::var("SERVICE_CONTENT_ENDPOINT")
            .ok()
            .map(|ep| {
                info!("Graph context enabled; service-content endpoint: {}", ep);
                GraphContextClient::new(ep)
            });

    Ok(Doorman::new(
        DoormanConfig {
            local,
            yoyo,
            external,
            lark_validator,
            graph_context_client,
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
            let bearer_token = std::env::var(env_bearer).unwrap_or_default();
            let bearer: Arc<dyn BearerTokenProvider> = Arc::new(StaticBearer::new(bearer_token));
            let yoyo_hourly_usd = std::env::var(env_hourly)
                .ok()
                .and_then(|s| s.parse::<f64>().ok())
                .unwrap_or(0.0);
            Some(YoYoTierClient::new(
                YoYoTierConfig {
                    endpoint,
                    default_model: std::env::var(env_model)
                        .unwrap_or_else(|_| "Olmo-3-1125-32B-Think".to_string()),
                    contract_version: slm_doorman::YOYO_CONTRACT_VERSION.to_string(),
                    pricing: PricingConfig { yoyo_hourly_usd },
                },
                bearer,
            ))
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
