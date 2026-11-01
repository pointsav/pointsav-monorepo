// SPDX-License-Identifier: Apache-2.0

//! `service-email` daemon — Ring 1 Communications Ledger ingest.
//!
//! Two concurrent workloads in one process:
//!
//! 1. **MCP HTTP server** (POST /mcp) — always started. Accepts
//!    `email.ingest` tool calls from Ring 2 clients, writes raw EML
//!    into this service's own `worm-ledger` instance. Requires only
//!    EMAIL_MODULE_ID + EMAIL_LEDGER_ROOT.
//!
//! 2. **EWS polling daemon** — started only when Exchange credentials
//!    are present in the environment. Polls an Exchange mailbox via
//!    the `exchange::EwsClient` `IngestAdapter` impl, fetches unread
//!    messages as raw MIME, writes each to the ledger, and marks the
//!    message as read.
//!
//! Startup order: MCP server binds first. EWS credentials are checked
//! afterwards. Missing credentials produce a warning; the MCP server
//! continues running so project-console (os-console F3) can connect
//! and inject messages directly while Exchange access is configured.
//!
//! Required env vars (MCP server):
//!   EMAIL_MODULE_ID       — per-tenant moduleId
//!   EMAIL_LEDGER_ROOT     — this service's own WORM ledger directory
//!                           (BRIEF-os-totebox-communication-complex-
//!                           2026-09.md decision 6 — no more remote
//!                           service-fs HTTP hop)
//!
//! Optional env vars (enables EWS daemon):
//!   AZURE_ACCESS_TOKEN    — pre-acquired EWS bearer token; acquire via
//!                           `az account get-access-token \
//!                            --resource https://outlook.office365.com`
//!   EXCHANGE_TARGET_USER  — target mailbox SMTP address
//!   EWS_ENDPOINT          — EWS URL (default: Exchange Online)
//!
//! Optional env vars (MCP server):
//!   EMAIL_BIND_ADDR       — bind address (default: 127.0.0.1:9200)
//!   EMAIL_RETENTION_VOCAB — retention-classes.csv path (default:
//!                           tests/fixtures/retention-classes.csv)
//!
//! Optional env vars (native SMTP receive server — always starts;
//! plaintext only, see `smtp_receive`'s module doc for the tracked
//! STARTTLS gap):
//!   EMAIL_SMTP_BIND_ADDR  — bind address (default: 127.0.0.1:2525 —
//!                           an unprivileged dev/relay port, not 25)
//!
//! Optional env vars (enables the `email.send` MCP tool):
//!   EMAIL_SMTP_RELAY_HOST — outbound relay hostname
//!   EMAIL_SMTP_RELAY_PORT — outbound relay port
//!
//! Optional env vars (native IMAP serve — always starts; LOGIN
//! refuses all credentials until both of these are set):
//!   EMAIL_IMAP_BIND_ADDR  — bind address (default: 127.0.0.1:1143 —
//!                           an unprivileged dev port, not 143)
//!   IMAP_USERNAME         — accepted LOGIN username
//!   IMAP_PASSWORD         — accepted LOGIN password

mod auth;
mod http;
mod imap_serve;
mod ingest;
mod ledger_backing;
mod mcp;
mod smtp_receive;
mod smtp_send;

use auth::EwsCredentials;
use http::AppState;
use ingest::exchange::EwsClient;
use ingest::IngestAdapter;
use ledger_backing::{Direction, LedgerBacking};
use std::sync::Arc;
use std::time::Duration;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "service_email=info".parse().unwrap()),
        )
        .init();

    let module_id = std::env::var("EMAIL_MODULE_ID").expect("EMAIL_MODULE_ID is required");
    let ledger_root =
        std::env::var("EMAIL_LEDGER_ROOT").expect("EMAIL_LEDGER_ROOT is required (this service's own WORM ledger directory)");
    let retention_vocab_path = std::env::var("EMAIL_RETENTION_VOCAB").unwrap_or_else(|_| {
        concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/retention-classes.csv").to_string()
    });
    let bind_addr =
        std::env::var("EMAIL_BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:9200".to_string());
    let smtp_bind_addr =
        std::env::var("EMAIL_SMTP_BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:2525".to_string());
    let imap_bind_addr =
        std::env::var("EMAIL_IMAP_BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:1143".to_string());
    let imap_credentials: imap_serve::Credentials = match (
        std::env::var("IMAP_USERNAME"),
        std::env::var("IMAP_PASSWORD"),
    ) {
        (Ok(user), Ok(pass)) => Some((user, pass)),
        _ => {
            tracing::info!("IMAP_USERNAME/IMAP_PASSWORD not set; IMAP LOGIN will refuse all credentials");
            None
        }
    };
    let smtp_relay = match (
        std::env::var("EMAIL_SMTP_RELAY_HOST"),
        std::env::var("EMAIL_SMTP_RELAY_PORT"),
    ) {
        (Ok(host), Ok(port_str)) => match port_str.parse::<u16>() {
            Ok(port) => Some((host, port)),
            Err(e) => {
                tracing::warn!("EMAIL_SMTP_RELAY_PORT invalid ({e}); email.send tool disabled");
                None
            }
        },
        _ => {
            tracing::info!("EMAIL_SMTP_RELAY_HOST/PORT not set; email.send tool disabled");
            None
        }
    };

    let ledger = worm_ledger::PosixTileLedger::open(&ledger_root, &module_id, None::<&std::path::Path>)
        .unwrap_or_else(|e| panic!("failed to open WORM ledger at {ledger_root}: {e}"));
    let vocab = worm_ledger::retention::load_retention_classes(&retention_vocab_path);
    if !vocab.warnings.is_empty() {
        for w in &vocab.warnings {
            tracing::warn!("retention vocabulary warning: {w}");
        }
    }
    let ledger_backing_daemon = Arc::new(LedgerBacking::new(Box::new(ledger), vocab, module_id.clone()));

    // MCP server and EWS daemon share one ledger instance via this
    // single Arc<LedgerBacking> -- both append to the same log.
    let state = Arc::new(AppState {
        module_id: module_id.clone(),
        ledger_backing: ledger_backing_daemon.clone(),
        smtp_relay,
    });

    // ── MCP server — always starts ────────────────────────────────────
    let app = http::router(state);
    let listener = tokio::net::TcpListener::bind(&bind_addr)
        .await
        .unwrap_or_else(|e| panic!("failed to bind {bind_addr}: {e}"));

    tracing::info!(module_id, bind_addr, "service-email MCP server ready");

    tokio::spawn(async move {
        axum::serve(listener, app).await.expect("axum serve error");
    });

    // ── native SMTP receive server — always starts ─────────────────────
    let smtp_ledger_backing = ledger_backing_daemon.clone();
    let smtp_hostname: Arc<str> = Arc::from(module_id.as_str());
    let smtp_bind_addr_log = smtp_bind_addr.clone();
    tokio::spawn(async move {
        smtp_receive::serve(&smtp_bind_addr, smtp_ledger_backing, smtp_hostname)
            .await
            .expect("smtp receive server error");
    });
    tracing::info!(smtp_bind_addr = %smtp_bind_addr_log, "service-email SMTP receive server ready");

    // ── native IMAP serve — always starts ───────────────────────────────
    let imap_ledger_backing = ledger_backing_daemon.clone();
    let imap_credentials = Arc::new(imap_credentials);
    let imap_bind_addr_log = imap_bind_addr.clone();
    tokio::spawn(async move {
        imap_serve::serve(&imap_bind_addr, imap_ledger_backing, imap_credentials)
            .await
            .expect("imap serve error");
    });
    tracing::info!(imap_bind_addr = %imap_bind_addr_log, "service-email IMAP serve ready");

    // ── EWS daemon — starts only when credentials are present ─────────
    match EwsCredentials::from_env() {
        Err(e) => {
            tracing::warn!(
                "EWS polling daemon disabled: {e}. \
                 Set AZURE_ACCESS_TOKEN + EXCHANGE_TARGET_USER to enable \
                 Exchange inbox polling. MCP server is running."
            );
            // Keep the process alive so the MCP server keeps serving.
            loop {
                tokio::time::sleep(Duration::from_secs(3600)).await;
            }
        }
        Ok(creds) => {
            tracing::info!(
                target_user = %creds.target_user,
                endpoint = %creds.ews_endpoint,
                "EWS polling daemon starting (60s cadence)"
            );
            ews_poll_loop(creds, ledger_backing_daemon).await;
        }
    }
}

async fn ews_poll_loop(creds: EwsCredentials, ledger_backing: Arc<LedgerBacking>) {
    loop {
        let adapter = EwsClient::new(
            creds.access_token.clone(),
            creds.ews_endpoint.clone(),
            creds.target_user.clone(),
        );

        match adapter.find_new().await {
            Err(e) => tracing::error!("find_new: {e}"),
            Ok(ids) if ids.is_empty() => {
                tracing::debug!("no unread messages; sleeping");
            }
            Ok(ids) => {
                tracing::info!("{} unread message(s) found", ids.len());
                let mut ingested = 0usize;
                for id in &ids {
                    match adapter.fetch_raw(id).await {
                        Err(e) => tracing::error!("fetch_raw({id}): {e}"),
                        Ok(mime_bytes) => match ledger_backing.append_message(id, &mime_bytes, Direction::Inbound) {
                            Err(e) => tracing::error!("ledger append({id}): {e}"),
                            Ok(cursor) => {
                                tracing::debug!("appended {id} at cursor {cursor}");
                                if let Err(e) = adapter.mark_ingested(id).await {
                                    tracing::error!("mark_ingested({id}): {e}");
                                }
                                ingested += 1;
                            }
                        },
                    }
                    tokio::time::sleep(Duration::from_millis(50)).await;
                }
                tracing::info!("ingested {ingested}/{} messages", ids.len());
            }
        }

        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}
