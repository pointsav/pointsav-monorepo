// SPDX-License-Identifier: Apache-2.0

use service_calendar::{EventStore, LedgerBacking};
use std::env;
use std::sync::Arc;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let module_id = env::var("CALENDAR_MODULE_ID")
        .expect("CALENDAR_MODULE_ID environment variable must be set");

    let ledger_root = env::var("CALENDAR_LEDGER_ROOT")
        .expect("CALENDAR_LEDGER_ROOT environment variable must be set (this service's own WORM ledger directory)");
    let retention_vocab_path = env::var("CALENDAR_RETENTION_VOCAB").unwrap_or_else(|_| {
        concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/retention-classes.csv").to_string()
    });
    let signing_key_path: Option<std::path::PathBuf> =
        env::var("CALENDAR_SIGNING_KEY").ok().map(Into::into);

    let bind_addr =
        env::var("CALENDAR_BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:9303".to_string());

    info!("Starting service-calendar on {}", bind_addr);
    info!("Module ID: {}", module_id);
    info!("Ledger root: {}", ledger_root);

    let ledger = worm_ledger::PosixTileLedger::open(
        &ledger_root,
        &module_id,
        signing_key_path.as_deref(),
    )
    .unwrap_or_else(|e| panic!("failed to open WORM ledger at {ledger_root}: {e}"));
    let vocab = worm_ledger::retention::load_retention_classes(&retention_vocab_path);
    if !vocab.warnings.is_empty() {
        for w in &vocab.warnings {
            tracing::warn!("retention vocabulary warning: {w}");
        }
    }

    let ledger_backing = Arc::new(LedgerBacking::new(Box::new(ledger), vocab, module_id.clone()));

    let event_store = Arc::new(EventStore::new());
    match ledger_backing.replay_into(&event_store) {
        Ok(n) => info!("replayed {n} calendar envelope(s) from the ledger into EventStore"),
        Err(e) => tracing::error!("ledger replay failed: {e} -- starting with an empty EventStore"),
    }

    let listener = tokio::net::TcpListener::bind(&bind_addr).await?;
    info!("service-calendar (CalDAV) listening on {}", bind_addr);

    let router = service_calendar::router(ledger_backing, event_store);
    axum::serve(listener, router).await?;

    Ok(())
}
