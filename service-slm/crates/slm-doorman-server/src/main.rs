// SPDX-License-Identifier: Apache-2.0

//! `slm-doorman-server` — thin binary entry point for the service-slm Doorman.
//!
//! Full boot sequence (env parsing, `AppState` construction, background task
//! spawning, axum serve loop) lives in `slm_doorman_server::run()` — see
//! `src/entrypoint.rs` — so this crate can also be embedded as a library
//! dependency by a bundling binary
//! (`BRIEF-os-totebox-platform.md` §8/§10, Phase 2) without duplicating any
//! of this logic. Environment variable documentation lives in
//! `src/entrypoint.rs`'s module doc comment.

/// Builds the runtime. On the os-totebox seL4/Microkit guest
/// (`FOUNDRY_NO_UNIX_SIGNAL` set by its /init) the kernel lacks AF_UNIX, and
/// `enable_all()` (what `#[tokio::main]` uses) builds the signal driver
/// eagerly, which panics during runtime construction (found 2026-09-01); there
/// only the io and time drivers are enabled, as in `os-totebox/src/main.rs`
/// and `service-content`. Everywhere else the full driver set is kept, since
/// without the signal driver `shutdown_signal()` cannot install its SIGTERM
/// handler and graceful drain would be lost.
fn build_runtime() -> std::io::Result<tokio::runtime::Runtime> {
    let mut b = tokio::runtime::Builder::new_multi_thread();
    if std::env::var_os("FOUNDRY_NO_UNIX_SIGNAL").is_some() {
        b.enable_io().enable_time();
    } else {
        b.enable_all();
    }
    b.build()
}

fn main() -> anyhow::Result<()> {
    build_runtime()?.block_on(slm_doorman_server::run())
}

#[cfg(test)]
mod tests {
    use super::build_runtime;

    #[test]
    fn runtime_builds_with_and_without_the_guest_switch() {
        // Both branches must produce a working runtime on a normal host. The
        // AF_UNIX-less guest itself is not reachable from a unit test.
        std::env::remove_var("FOUNDRY_NO_UNIX_SIGNAL");
        assert_eq!(build_runtime().unwrap().block_on(async { 7 }), 7);
        std::env::set_var("FOUNDRY_NO_UNIX_SIGNAL", "1");
        let out = build_runtime().unwrap().block_on(async { 7 });
        std::env::remove_var("FOUNDRY_NO_UNIX_SIGNAL");
        assert_eq!(out, 7);
    }
}
