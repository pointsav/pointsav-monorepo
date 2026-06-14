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
//!   RUST_LOG
//!     Tracing filter. Default: orchestration_slm=info,orchestration_slm_server=info

use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::Context;
use orchestration_slm::yoyo_proxy::YoyoEndpoints;
use orchestration_slm::{
    resolve_from_env, ChassisFlowGate, CircuitRegistry, FleetRegistry, LicenseStatus,
    MeteringLedger, YoyoProxyClient,
};
use tracing::{info, warn};

mod http;

// Placeholder embedded public key. Override with ORCHESTRATION_LICENSE_PUBKEY_HEX at deploy time.
// The all-zero key is intentionally invalid — any real license token will fail unless the operator
// sets the correct public key, preventing accidental grant of Tier B on a dev chassis.
const EMBEDDED_LICENSE_PUBKEY: &[u8; 32] = &[0u8; 32];

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

    let license = resolve_from_env(EMBEDDED_LICENSE_PUBKEY);
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

    let state = Arc::new(http::AppState {
        fleet: FleetRegistry::new(),
        proxy: Arc::new(YoyoProxyClient::new(endpoints)),
        metering: MeteringLedger::new(),
        circuits: Arc::new(CircuitRegistry::new(YOYO_LABELS.iter().copied())),
        gates: Arc::new(ChassisFlowGate::new(YOYO_LABELS.iter().copied())),
        license: Arc::new(license),
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

    axum::serve(listener, app).await.context("server error")?;
    Ok(())
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
