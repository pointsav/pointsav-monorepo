// SPDX-License-Identifier: Apache-2.0 OR MIT

//! `slm-doorman-server` — thin binary entry point for the service-slm Doorman.
//!
//! Full boot sequence (env parsing, `AppState` construction, background task
//! spawning, axum serve loop) lives in `slm_doorman_server::run()` — see
//! `src/entrypoint.rs` — so this crate can also be embedded as a library
//! dependency by a bundling binary
//! (`BRIEF-os-totebox-platform.md` §8/§10, Phase 2) without duplicating any
//! of this logic. Environment variable documentation lives in
//! `src/entrypoint.rs`'s module doc comment.

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    slm_doorman_server::run().await
}
