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
//!   RUST_LOG                  default slm_doorman=info,slm_doorman_server=info
//!
//! Per `conventions/three-ring-architecture.md` the Doorman boots fine
//! with no Yo-Yo configured (Optional Intelligence). B5 verifies this
//! end-to-end.

mod http;

use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::Context;
use slm_doorman::tier::{
    BearerTokenProvider, LocalTierClient, LocalTierConfig, PricingConfig, StaticBearer,
    YoYoTierClient, YoYoTierConfig,
};
use slm_doorman::{
    ApprenticeshipConfig, AuditLedger, BriefCache, Doorman, DoormanConfig, PromotionLedger,
    SshKeygenVerifier, VerdictDispatcher, VerdictVerifier,
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
    let state = Arc::new(http::AppState {
        doorman,
        apprenticeship,
        brief_cache,
        verdict_dispatcher,
    });

    info!(
        version = slm_doorman::DOORMAN_VERSION,
        %bind_addr,
        has_local = state.doorman.has_local(),
        has_yoyo = state.doorman.has_yoyo(),
        has_external = state.doorman.has_external(),
        apprenticeship_enabled = state.apprenticeship.is_some(),
        "service-slm Doorman starting"
    );

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

    let yoyo = match std::env::var("SLM_YOYO_ENDPOINT") {
        Ok(endpoint) if !endpoint.is_empty() => {
            let bearer_token = std::env::var("SLM_YOYO_BEARER").unwrap_or_default();
            let bearer: Arc<dyn BearerTokenProvider> = Arc::new(StaticBearer::new(bearer_token));
            let yoyo_hourly_usd = std::env::var("SLM_YOYO_HOURLY_USD")
                .ok()
                .and_then(|s| s.parse::<f64>().ok())
                .unwrap_or(0.0);
            Some(YoYoTierClient::new(
                YoYoTierConfig {
                    endpoint,
                    default_model: std::env::var("SLM_YOYO_MODEL")
                        .unwrap_or_else(|_| "Olmo-3-1125-32B-Think".to_string()),
                    contract_version: slm_doorman::YOYO_CONTRACT_VERSION.to_string(),
                    pricing: PricingConfig { yoyo_hourly_usd },
                },
                bearer,
            ))
        }
        _ => None,
    };

    let ledger = AuditLedger::default_for_user()
        .context("failed to open audit ledger; ensure HOME is set")?;

    Ok(Doorman::new(
        DoormanConfig {
            local,
            yoyo,
            external: None, // wired by B4
        },
        ledger,
    ))
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
