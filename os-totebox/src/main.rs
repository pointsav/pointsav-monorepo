// SPDX-License-Identifier: FSL-1.1-ALv2
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! os-totebox product bundle — runs service-content and service-slm's Doorman
//! (`slm-doorman-server`) as one deployable process.
//!
//! Per `BRIEF-os-totebox-platform.md` §8 (the "single binary" clarification)
//! and §10 (dogfooding, resolved 2026-07-14): this is NOT a merge of the two
//! services' internals. Each service's own router/business logic is
//! completely unmodified — this file only spawns each service's existing,
//! unmodified `run()` entry point on its own OS thread. This mirrors the
//! HashiCorp Consul/Nomad/Vault (single-binary-multiple-role) and SurrealDB
//! (single-self-contained-binary) precedent researched for §8, mechanically
//! automating what the individual `local-content.service` /
//! `local-doorman.service` systemd units already do today.
//!
//! `service_content::run()` is a synchronous entry point that manages its
//! own execution model internally (a blocking file-watcher loop plus its own
//! axum server) — it is run on its own dedicated OS thread, not via
//! `tokio::spawn`, since it is not an async-compatible future.
//! `slm_doorman_server::run()` is a proper async service; it gets its own
//! dedicated multi-threaded tokio runtime on its own OS thread too, so a
//! panic or long block in one service's thread can never starve the other's
//! runtime. Neither service's logging/tracing setup is touched here —
//! `slm_doorman_server::run()` initializes the process-global tracing
//! subscriber itself, exactly as it does when run as its own standalone
//! binary; `service_content::run()` uses `println!` and is unaffected.
//!
//! Configuration is entirely via each service's own existing environment
//! variables (`SERVICE_CONTENT_*` / `SLM_*`) — this wrapper introduces none
//! of its own.

use std::thread;

fn main() -> anyhow::Result<()> {
    tracing::info!(target: "os_totebox", "os-totebox bundle starting: service-content + slm-doorman-server");

    let content_handle = thread::Builder::new()
        .name("service-content".to_string())
        .spawn(|| {
            if let Err(e) = service_content::run() {
                eprintln!("[os-totebox] service-content exited with error: {e}");
            }
        })
        .expect("failed to spawn service-content thread");

    let doorman_handle = thread::Builder::new()
        .name("slm-doorman-server".to_string())
        .spawn(|| {
            let rt = tokio::runtime::Runtime::new()
                .expect("failed to build tokio runtime for slm-doorman-server");
            if let Err(e) = rt.block_on(slm_doorman_server::run()) {
                eprintln!("[os-totebox] slm-doorman-server exited with error: {e}");
            }
        })
        .expect("failed to spawn slm-doorman-server thread");

    // Both services run until their own internal loop exits (normally never,
    // for a live server) or panics. Join both so the process's exit code
    // reflects whichever thread stops first, rather than exiting silently
    // the instant main() returns while a sibling thread is still serving.
    content_handle.join().ok();
    doorman_handle.join().ok();

    Ok(())
}
