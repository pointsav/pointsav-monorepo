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
    BearerTokenProvider, LocalTierClient, LocalTierConfig, StaticBearer, YoYoTierClient,
    YoYoTierConfig,
};
use slm_doorman::{AuditLedger, Doorman, DoormanConfig};
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing();

    let bind_addr: SocketAddr = std::env::var("SLM_BIND_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:9080".to_string())
        .parse()
        .context("SLM_BIND_ADDR must be a socket address")?;

    let doorman = build_doorman()?;
    let state = Arc::new(http::AppState { doorman });

    info!(
        version = slm_doorman::DOORMAN_VERSION,
        %bind_addr,
        has_local = state.doorman.has_local(),
        has_yoyo = state.doorman.has_yoyo(),
        has_external = state.doorman.has_external(),
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
            Some(YoYoTierClient::new(
                YoYoTierConfig {
                    endpoint,
                    default_model: std::env::var("SLM_YOYO_MODEL")
                        .unwrap_or_else(|_| "Olmo-3-1125-32B-Think".to_string()),
                    contract_version: slm_doorman::YOYO_CONTRACT_VERSION.to_string(),
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

fn init_tracing() {
    use tracing_subscriber::{fmt, prelude::*, EnvFilter};

    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("slm_doorman=info,slm_doorman_server=info,axum=warn"));
    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer())
        .init();
}
