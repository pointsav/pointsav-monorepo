// SPDX-License-Identifier: LicenseRef-PointSav-Proprietary

//! `orchestration-command-server` — CommandCentre HTTP entry point.
//!
//! Binds on 127.0.0.1:8020 by default (loopback-only until Phase 4 WireGuard).
//! Uses Tokio `current_thread` flavor: ~4–8 MB RSS idle vs ~30–40 MB for
//! multi-thread. Consistent with BRIEF-OS-FAMILY Phase 2 target (≤96 MB idle).
//!
//! Environment variables:
//!
//!   COMMAND_BIND_ADDR
//!     Socket address to bind. Default: 127.0.0.1:8020
//!
//!   COMMAND_INSTANCE_ID
//!     Human-readable identifier for this deployment.
//!     Default: "gateway-orchestration-command-1"
//!
//!   COMMAND_PAIRINGS_PATH
//!     Path to pairings.yaml. Default: /srv/foundry/pairings.yaml
//!
//!   COMMAND_CLONES_ROOT
//!     Directory containing clones/<archive>/ subdirs.
//!     Default: /srv/foundry/clones
//!
//!   COMMAND_LICENSE_TOKEN
//!     Ed25519-signed license token. Absent = observation mode (no pairing).
//!
//!   COMMAND_LICENSE_PUBKEY_HEX
//!     64 hex-char Ed25519 public key for license verification.
//!     Absent or invalid = dev key (all-zero).
//!
//!   COMMAND_SLM_BINARY
//!     Path to the app-orchestration-slm binary to spawn as a child.
//!     Absent = slm child disabled.
//!
//!   COMMAND_AUDIT_LEDGER_PATH
//!     Path for the WORM audit ledger (JSONL append-only).
//!     Default: ./data/command-audit.jsonl
//!
//!   RUST_LOG
//!     Tracing filter. Default: orchestration_command=info,orchestration_command_server=info

use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Context;
use orchestration_command::{child, fleet, personnel, resolve_from_env, LicenseStatus};
use orchestration_command::invite::InviteIssuer;
use orchestration_command::pairing::PairingStore;
use orchestration_command::routing::MessageRouter;
use tracing::{info, warn};

mod http;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    init_tracing();

    let bind_addr: SocketAddr = std::env::var("COMMAND_BIND_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:8020".to_string())
        .parse()
        .context("COMMAND_BIND_ADDR must be a socket address")?;

    let instance_id = std::env::var("COMMAND_INSTANCE_ID")
        .unwrap_or_else(|_| "gateway-orchestration-command-1".to_string());

    let pairings_path = PathBuf::from(
        std::env::var("COMMAND_PAIRINGS_PATH")
            .unwrap_or_else(|_| "/srv/foundry/pairings.yaml".to_string()),
    );
    let clones_root = PathBuf::from(
        std::env::var("COMMAND_CLONES_ROOT")
            .unwrap_or_else(|_| "/srv/foundry/clones".to_string()),
    );

    // License check.
    let pubkey = load_license_pubkey();
    let license = resolve_from_env(&pubkey);
    match &license {
        LicenseStatus::Valid(p) => info!(
            licensee = %p.issued_to,
            expiry = %p.expiry.to_rfc3339(),
            "license valid — pairing endpoints enabled"
        ),
        LicenseStatus::Absent => info!("no license token — observation mode (read-only)"),
        LicenseStatus::Invalid(r) => warn!(%r, "license invalid — observation mode (read-only)"),
    }

    // Load fleet.
    let archives = fleet::load_fleet(&pairings_path, &clones_root).unwrap_or_else(|e| {
        warn!(error = %e, "failed to load fleet — serving empty archive list");
        vec![]
    });
    let archives_count = archives.len();

    // Build module_id → archive_root map for the router.
    let module_to_root: HashMap<String, PathBuf> = archives
        .iter()
        .map(|a| {
            (
                a.module_id.clone(),
                clones_root.join(&a.cluster_name),
            )
        })
        .collect();

    // Load personnel.
    let personnel_map = personnel::load_personnel(&pairings_path).unwrap_or_default();

    // Child supervisor.
    let (child_sup, should_spawn) = child::from_env();
    if should_spawn {
        let slm_path = PathBuf::from(std::env::var("COMMAND_SLM_BINARY").unwrap());
        let running = Arc::clone(&child_sup.running);
        child::ChildSupervisor::spawn(slm_path, running);
    }

    let state = Arc::new(http::AppState {
        archives: Arc::new(archives),
        personnel: Arc::new(personnel_map),
        inviter: Arc::new(InviteIssuer::new_ephemeral(&instance_id)),
        pairing_store: Arc::new(PairingStore::new(&instance_id)),
        router: Arc::new(MessageRouter::new(module_to_root, Arc::new(instance_id.clone()))),
        license: Arc::new(license),
        child_running: Arc::clone(&child_sup.running),
        child_configured: child_sup.configured,
        archives_loaded: archives_count,
    });

    info!(
        version = orchestration_command_core::COMMAND_VERSION,
        %bind_addr,
        %instance_id,
        archives = archives_count,
        "orchestration-command starting"
    );

    let app = http::router(Arc::clone(&state));
    let listener = tokio::net::TcpListener::bind(bind_addr)
        .await
        .context("failed to bind")?;

    axum::serve(listener, app).await.context("server error")?;
    Ok(())
}

fn load_license_pubkey() -> [u8; 32] {
    if let Ok(hex) = std::env::var("COMMAND_LICENSE_PUBKEY_HEX") {
        let cleaned = hex.trim().to_lowercase();
        if cleaned.len() == 64 {
            let mut bytes = [0u8; 32];
            let mut ok = true;
            for (i, chunk) in cleaned.as_bytes().chunks(2).enumerate() {
                if let Ok(s) = std::str::from_utf8(chunk) {
                    if let Ok(b) = u8::from_str_radix(s, 16) {
                        bytes[i] = b;
                    } else {
                        ok = false;
                        break;
                    }
                } else {
                    ok = false;
                    break;
                }
            }
            if ok {
                info!("license pubkey loaded from COMMAND_LICENSE_PUBKEY_HEX");
                return bytes;
            }
        }
        warn!("COMMAND_LICENSE_PUBKEY_HEX invalid — using dev key");
    }
    [0u8; 32]
}

fn init_tracing() {
    use tracing_subscriber::{fmt, prelude::*, EnvFilter};
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        EnvFilter::new("orchestration_command=info,orchestration_command_server=info")
    });
    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer())
        .init();
}
