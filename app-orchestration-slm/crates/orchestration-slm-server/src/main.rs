// SPDX-License-Identifier: LicenseRef-PointSav-Proprietary

//! `orchestration-slm-server` — HTTP entry point for the Yo-Yo broker chassis.
//!
//! Binds on port :9180 by default. Multiple Totebox Archives connect their
//! service-slm Doorman to this chassis by setting:
//!   SLM_YOYO_DEFAULT_ENDPOINT=http://<chassis-host>:9180/v1/yoyo/proxy
//!   SLM_YOYO_TRAINER_ENDPOINT=http://<chassis-host>:9180/v1/yoyo/trainer
//!   SLM_YOYO_GRAPH_ENDPOINT=http://<chassis-host>:9180/v1/yoyo/graph
//!
//! Environment configuration:
//!
//!   ORCHESTRATION_BIND_ADDR
//!     Socket address to bind. Default: 0.0.0.0:9180
//!
//!   ORCHESTRATION_YOYO_DEFAULT_ENDPOINT
//!     Base URL of the default Yo-Yo node (e.g. http://10.10.0.5:8080).
//!     Absent = /v1/yoyo/proxy returns 503.
//!
//!   ORCHESTRATION_YOYO_TRAINER_ENDPOINT
//!     Base URL of the trainer Yo-Yo node (L4 24GB, OLMo 3 32B-Think).
//!     Absent = /v1/yoyo/trainer returns 503.
//!
//!   ORCHESTRATION_YOYO_GRAPH_ENDPOINT
//!     Base URL of the graph Yo-Yo node (H100 80GB, Llama 3.3 70B grammar).
//!     Absent = /v1/yoyo/graph returns 503.
//!
//!   ORCHESTRATION_YOYO_BEARER
//!     Bearer token sent to the actual Yo-Yo VMs. Absent = no auth header.
//!
//!   ORCHESTRATION_YOYO_HOURLY_USD
//!     Hourly USD rate for cost metering. Default: 0.0 (dev/unknown).
//!     Example: 2.21 for GCP H100 on-demand.
//!
//!   ORCHESTRATION_LICENSE_PUBKEY_HEX
//!     Ed25519 public key for license token verification, as 64 hex characters.
//!     Absent or invalid = dev key (all-zero); any real license token will fail.
//!     Set to the key produced by `tool-wallet keygen` when a license is issued.
//!
//!   ORCHESTRATION_ALLOCATION_LEDGER_PATH
//!     Path to the per-VM discovery/allocation ledger (§14 #20) — an append-only
//!     JSONL file of every module_id ever handed out by POST /v1/discovery/allocate.
//!     Default: /var/lib/orchestration-slm/allocated-ids.jsonl
//!
//!   ORCHESTRATION_REGISTRATION_TOKEN
//!     Shared admission-control secret for POST /v1/discovery/allocate and
//!     POST /v1/discovery/register. Callers must send it as
//!     `Authorization: Bearer <token>`. Absent = both endpoints stay open to
//!     any caller that can reach the chassis (the prior, unauthenticated
//!     behavior) — a loud startup warning is logged in that case. Set this
//!     in any real deployment; the corresponding Doorman-side setting is
//!     `SLM_ORCHESTRATION_REGISTRATION_TOKEN`.
//!
//!   RUST_LOG
//!     Tracing filter. Default: orchestration_slm=info,orchestration_slm_server=info

use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::Context;
use orchestration_slm::yoyo_proxy::YoyoEndpoints;
use orchestration_slm::{
    resolve_from_env, AllocationLedger, ChassisFlowGate, CircuitRegistry, FleetRegistry,
    LicenseStatus, MembershipKey, MeteringLedger, YoyoProxyClient,
};
use tracing::{info, warn};

mod http;

fn load_license_pubkey() -> [u8; 32] {
    if let Ok(hex) = std::env::var("ORCHESTRATION_LICENSE_PUBKEY_HEX") {
        let cleaned = hex.trim().to_lowercase();
        if cleaned.len() == 64 {
            let mut bytes = [0u8; 32];
            let mut ok = true;
            for (i, chunk) in cleaned.as_bytes().chunks(2).enumerate() {
                if let Ok(s) = std::str::from_utf8(chunk) {
                    if let Ok(b) = u8::from_str_radix(s, 16) {
                        bytes[i] = b;
                    } else { ok = false; break; }
                } else { ok = false; break; }
            }
            if ok {
                tracing::info!("license pubkey loaded from ORCHESTRATION_LICENSE_PUBKEY_HEX");
                return bytes;
            }
        }
        tracing::warn!("ORCHESTRATION_LICENSE_PUBKEY_HEX is set but invalid (must be 64 hex chars) — using dev key");
    }
    [0u8; 32]
}

const YOYO_LABELS: &[&str] = &["proxy", "trainer", "graph"];

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing();

    let bind_addr: SocketAddr = std::env::var("ORCHESTRATION_BIND_ADDR")
        .unwrap_or_else(|_| "0.0.0.0:9180".to_string())
        .parse()
        .context("ORCHESTRATION_BIND_ADDR must be a socket address")?;

    let endpoints = YoyoEndpoints::from_env();
    let (trainer_cfg, graph_cfg) = endpoints.any_configured();

    let pubkey = load_license_pubkey();
    let license = resolve_from_env(&pubkey);
    match &license {
        LicenseStatus::Valid(p) => info!(
            licensee = %p.issued_to,
            expiry = %p.expiry.to_rfc3339(),
            "chassis license valid — Tier B brokering enabled"
        ),
        LicenseStatus::Absent => info!("no license token — Tier B brokering disabled"),
        LicenseStatus::Invalid(reason) => {
            warn!(%reason, "chassis license invalid — Tier B brokering disabled")
        }
    }

    let membership = MembershipKey::generate().context("failed to generate membership keypair")?;

    let allocation_ledger_path = std::env::var("ORCHESTRATION_ALLOCATION_LEDGER_PATH")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| {
            std::path::PathBuf::from("/var/lib/orchestration-slm/allocated-ids.jsonl")
        });
    let allocation = AllocationLedger::load(allocation_ledger_path.clone())
        .with_context(|| format!("failed to load allocation ledger at {allocation_ledger_path:?}"))?;
    info!(path = ?allocation_ledger_path, "allocation ledger loaded");

    let registration_token = std::env::var("ORCHESTRATION_REGISTRATION_TOKEN").ok();
    match &registration_token {
        Some(_) => info!("ORCHESTRATION_REGISTRATION_TOKEN configured — discovery endpoints require it"),
        None => warn!(
            "ORCHESTRATION_REGISTRATION_TOKEN is unset — /v1/discovery/allocate and \
             /v1/discovery/register are UNAUTHENTICATED; any caller that can reach this \
             chassis can self-register and be issued a membership token. Set \
             ORCHESTRATION_REGISTRATION_TOKEN before production use."
        ),
    }

    let state = Arc::new(http::AppState {
        fleet: FleetRegistry::new(),
        proxy: Arc::new(YoyoProxyClient::new(endpoints)),
        metering: MeteringLedger::new(),
        circuits: Arc::new(CircuitRegistry::new(YOYO_LABELS.iter().copied())),
        gates: Arc::new(ChassisFlowGate::new(YOYO_LABELS.iter().copied())),
        license: Arc::new(license),
        membership: Arc::new(membership),
        allocation: Arc::new(allocation),
        registration_token,
    });

    info!(
        version = orchestration_slm_core::CHASSIS_VERSION,
        %bind_addr,
        yoyo_trainer_configured = trainer_cfg,
        yoyo_graph_configured = graph_cfg,
        "orchestration-slm chassis starting"
    );

    let app = http::router(Arc::clone(&state));
    let listener = tokio::net::TcpListener::bind(bind_addr)
        .await
        .context("failed to bind")?;

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("server error")?;
    info!("chassis shut down cleanly.");
    Ok(())
}

/// Awaits SIGTERM, then returns — drives the chassis's own axum graceful
/// shutdown (drain in-flight requests, stop accepting new ones). The
/// chassis is stateless (rebuilds its fleet registry from heartbeats on
/// restart per this crate's own CLAUDE.md), so there is no checkpoint/WAL
/// concern here like os-totebox's LadybugDB — draining in-flight HTTP is
/// the whole story. Mirrors the same pattern already proven in
/// slm-doorman-server and service-content (BRIEF-os-totebox-platform.md,
/// T11/T12).
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

fn init_tracing() {
    use tracing_subscriber::{fmt, prelude::*, EnvFilter};
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("orchestration_slm=info,orchestration_slm_server=info"));
    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer())
        .init();
}
