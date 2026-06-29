// SPDX-License-Identifier: LicenseRef-PointSav-Proprietary

//! Child process supervisor — spawns and monitors `app-orchestration-slm`.
//!
//! If `COMMAND_SLM_BINARY` is not set, the supervisor is disabled and `/readyz`
//! reports `slm_child: "not_configured"`. When set, the supervisor spawns the
//! binary on startup and restarts it up to 3 times on exit before giving up.

use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use tokio::process::Command;
use tracing::{info, warn};

/// Supervisor state accessible to the HTTP layer.
pub struct ChildSupervisor {
    pub running: Arc<AtomicBool>,
    pub configured: bool,
}

impl ChildSupervisor {
    /// Returns a disabled supervisor when no binary path is configured.
    pub fn not_configured() -> Self {
        Self {
            running: Arc::new(AtomicBool::new(false)),
            configured: false,
        }
    }

    /// Spawn the supervisor loop as a background task. Returns immediately.
    /// Call this from the server's `main` after creating the supervisor.
    pub fn spawn(binary_path: PathBuf, running: Arc<AtomicBool>) {
        tokio::spawn(async move {
            let mut attempts = 0u32;
            loop {
                info!(binary = %binary_path.display(), attempt = attempts + 1, "starting slm child");
                let result = Command::new(&binary_path).status().await;
                running.store(false, Ordering::SeqCst);

                match result {
                    Ok(status) if status.success() => {
                        info!("slm child exited cleanly — not restarting");
                        break;
                    }
                    Ok(status) => {
                        warn!(code = ?status.code(), "slm child exited with error");
                    }
                    Err(e) => {
                        warn!(error = %e, "slm child failed to start");
                    }
                }

                attempts += 1;
                if attempts >= 3 {
                    warn!("slm child failed 3 times — giving up");
                    break;
                }

                // Brief back-off before restart.
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                running.store(true, Ordering::SeqCst);
            }
        });
    }
}

/// Resolve and launch the child supervisor from the environment.
/// Returns `(supervisor, should_spawn)`.
pub fn from_env() -> (ChildSupervisor, bool) {
    match std::env::var("COMMAND_SLM_BINARY") {
        Ok(path) if !path.trim().is_empty() => {
            let running = Arc::new(AtomicBool::new(true));
            let sup = ChildSupervisor {
                running: Arc::clone(&running),
                configured: true,
            };
            (sup, true)
        }
        _ => (ChildSupervisor::not_configured(), false),
    }
}
