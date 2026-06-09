// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Flow gate — the operator's billing kill switch.
//!
//! A flow gate is a per-label boolean that, when **closed**, refuses all
//! dispatch to the tier or node behind that label. Closing a gate stops
//! the associated GPU node from being started, so the operator's bill drops
//! to zero. In-flight requests complete; new requests are refused with
//! [`DoormanError::FlowGateClosed`]. Queued work accumulates and drains when
//! the gate re-opens.
//!
//! ## Invariant: nothing bypasses the kill switch
//!
//! The express lane bypasses the file-backed priority queue for
//! time-sensitive work, but it still checks the flow gate first. There is no
//! code path that reaches a tier whose gate is closed. This is the operator's
//! guarantee against unwanted billing.
//!
//! ## Persistence
//!
//! Each gate's state is mirrored to a small file on disk
//! (`<dir>/flow-gate-<label>.lock`, where presence-of-file = CLOSED). The
//! in-memory [`AtomicBool`] is the hot path; the file is the durable record
//! so the gate state survives a Doorman restart. An operator can also toggle
//! a gate from the shell by creating or removing the lock file directly; the
//! [`FlowGate::reconcile_from_disk`] method re-reads the files so the
//! in-memory state catches up. (A future revision may add an inotify watcher;
//! the current design reconciles on each status poll, which is sufficient.)
//!
//! Semantics chosen deliberately: a *present* lock file means CLOSED. This is
//! fail-safe — if the directory is wiped or unreadable, gates default to OPEN
//! (serving), never silently CLOSED (which would look like an outage).

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::error::{DoormanError, Result};

/// The reserved label for the global kill switch. Closing this gate refuses
/// dispatch to every tier and node regardless of their individual gates.
pub const GLOBAL_LABEL: &str = "global";

/// A single named gate. `closed == true` means the gate refuses dispatch.
#[derive(Debug)]
struct Gate {
    closed: AtomicBool,
    lock_path: PathBuf,
}

/// The flow-gate registry. Holds one [`Gate`] per label plus the global gate.
///
/// Cloneable via `Arc`: clone the `Arc<FlowGate>` to share the same gates
/// across the HTTP handlers and the background drain worker.
#[derive(Debug)]
pub struct FlowGate {
    gates: HashMap<String, Gate>,
    dir: PathBuf,
}

impl FlowGate {
    /// Build a flow-gate registry for the given labels plus the implicit
    /// [`GLOBAL_LABEL`]. The `dir` is where lock files are written; it is
    /// created if absent. Initial state is read from disk: a present lock
    /// file means the gate starts CLOSED.
    ///
    /// Duplicate labels (including an explicit `"global"`) are de-duplicated;
    /// the global gate always exists.
    pub fn new<I, S>(dir: impl AsRef<Path>, labels: I) -> Result<Arc<Self>>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let dir = dir.as_ref().to_path_buf();
        std::fs::create_dir_all(&dir).map_err(|e| DoormanError::PriorityQueueIo {
            path: dir.display().to_string(),
            reason: format!("create flow-gate dir: {e}"),
        })?;

        let mut gates = HashMap::new();
        let mut all_labels: Vec<String> = vec![GLOBAL_LABEL.to_string()];
        for l in labels {
            let l = l.as_ref().trim().to_string();
            if !l.is_empty() && !all_labels.contains(&l) {
                all_labels.push(l);
            }
        }

        for label in all_labels {
            let lock_path = dir.join(format!("flow-gate-{label}.lock"));
            let closed = lock_path.exists();
            gates.insert(
                label,
                Gate {
                    closed: AtomicBool::new(closed),
                    lock_path,
                },
            );
        }

        Ok(Arc::new(Self { gates, dir }))
    }

    /// Return true if dispatch to `label` is currently permitted.
    ///
    /// A dispatch is permitted only when BOTH the global gate AND the
    /// label's own gate are open. An unknown label is governed by the global
    /// gate alone (so a node added at runtime is not accidentally blocked by
    /// a missing per-label gate).
    pub fn is_open(&self, label: &str) -> bool {
        if self.is_closed_raw(GLOBAL_LABEL) {
            return false;
        }
        !self.is_closed_raw(label)
    }

    /// Return the label that is blocking dispatch to `label`, if any.
    /// Returns `Some("global")` when the global gate is closed, `Some(label)`
    /// when the label's own gate is closed, or `None` when dispatch is
    /// permitted. Useful for constructing a precise [`DoormanError::FlowGateClosed`].
    pub fn blocking_label(&self, label: &str) -> Option<String> {
        if self.is_closed_raw(GLOBAL_LABEL) {
            return Some(GLOBAL_LABEL.to_string());
        }
        if self.is_closed_raw(label) {
            return Some(label.to_string());
        }
        None
    }

    /// Convenience: return `Ok(())` if dispatch is permitted, or
    /// `Err(DoormanError::FlowGateClosed)` naming the blocking gate.
    pub fn check(&self, label: &str) -> Result<()> {
        match self.blocking_label(label) {
            None => Ok(()),
            Some(blocking) => Err(DoormanError::FlowGateClosed { label: blocking }),
        }
    }

    /// Set the gate for `label` to closed (`true`) or open (`false`).
    /// Persists the change to disk immediately. Setting a label that was not
    /// declared at construction creates a gate for it on the fly (so the
    /// operator can pre-emptively close a node label before it is wired).
    pub fn set(&self, label: &str, closed: bool) -> Result<()> {
        let label = label.trim();
        // For a known gate, flip the atomic and sync the file.
        if let Some(gate) = self.gates.get(label) {
            gate.closed.store(closed, Ordering::SeqCst);
            return Self::sync_lock_file(&gate.lock_path, closed);
        }
        // Unknown label: persist a lock file so the state is durable. It will
        // be picked up as a gate on the next `new()` / reconcile. We cannot
        // mutate the HashMap behind `&self` without interior mutability, so
        // the file is the source of truth until reconcile or restart.
        let lock_path = self.dir.join(format!("flow-gate-{label}.lock"));
        Self::sync_lock_file(&lock_path, closed)
    }

    /// Re-read every known gate's lock file from disk and update the
    /// in-memory atomic to match. Call this on a status poll so a shell-side
    /// toggle (operator `touch`/`rm` of a lock file) is reflected. Unknown
    /// labels created on disk are not adopted here (that requires a restart
    /// or a future inotify watcher); known labels always reconcile.
    pub fn reconcile_from_disk(&self) {
        for gate in self.gates.values() {
            let on_disk = gate.lock_path.exists();
            gate.closed.store(on_disk, Ordering::SeqCst);
        }
    }

    /// Snapshot of every gate's state, label → closed. Includes the global
    /// gate. Used by the status endpoint and the console.
    pub fn snapshot(&self) -> HashMap<String, bool> {
        self.gates
            .iter()
            .map(|(k, v)| (k.clone(), v.closed.load(Ordering::SeqCst)))
            .collect()
    }

    fn is_closed_raw(&self, label: &str) -> bool {
        self.gates
            .get(label)
            .map(|g| g.closed.load(Ordering::SeqCst))
            .unwrap_or(false)
    }

    /// Make the on-disk lock file match `closed`: create it when closing,
    /// remove it when opening. Idempotent.
    fn sync_lock_file(lock_path: &Path, closed: bool) -> Result<()> {
        if closed {
            // Create (truncate) the file to mark CLOSED.
            std::fs::write(lock_path, b"closed\n").map_err(|e| DoormanError::PriorityQueueIo {
                path: lock_path.display().to_string(),
                reason: format!("write flow-gate lock: {e}"),
            })
        } else {
            // Remove the file to mark OPEN. Absence-is-open, so a missing
            // file is success.
            match std::fs::remove_file(lock_path) {
                Ok(()) => Ok(()),
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
                Err(e) => Err(DoormanError::PriorityQueueIo {
                    path: lock_path.display().to_string(),
                    reason: format!("remove flow-gate lock: {e}"),
                }),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp_dir(label: &str) -> PathBuf {
        let p = std::env::temp_dir().join(format!(
            "slm-flow-gate-{label}-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&p).unwrap();
        p
    }

    #[test]
    fn gates_default_open() {
        let dir = tmp_dir("open");
        let fg = FlowGate::new(&dir, ["batch", "express"]).unwrap();
        assert!(fg.is_open("batch"));
        assert!(fg.is_open("express"));
        assert!(fg.is_open("global"));
        assert!(fg.check("batch").is_ok());
    }

    #[test]
    fn closing_label_blocks_only_that_label() {
        let dir = tmp_dir("per-label");
        let fg = FlowGate::new(&dir, ["batch", "express"]).unwrap();
        fg.set("batch", true).unwrap();
        assert!(!fg.is_open("batch"));
        assert!(fg.is_open("express"));
        assert_eq!(fg.blocking_label("batch").as_deref(), Some("batch"));
        assert!(matches!(
            fg.check("batch"),
            Err(DoormanError::FlowGateClosed { .. })
        ));
        assert!(fg.check("express").is_ok());
    }

    #[test]
    fn global_gate_blocks_everything() {
        let dir = tmp_dir("global");
        let fg = FlowGate::new(&dir, ["batch", "express"]).unwrap();
        fg.set(GLOBAL_LABEL, true).unwrap();
        assert!(!fg.is_open("batch"));
        assert!(!fg.is_open("express"));
        // Global takes precedence in the blocking label.
        assert_eq!(fg.blocking_label("batch").as_deref(), Some("global"));
    }

    #[test]
    fn state_persists_across_reopen() {
        let dir = tmp_dir("persist");
        {
            let fg = FlowGate::new(&dir, ["batch"]).unwrap();
            fg.set("batch", true).unwrap();
        }
        // New registry over the same dir: batch should start CLOSED.
        let fg = FlowGate::new(&dir, ["batch"]).unwrap();
        assert!(!fg.is_open("batch"));
    }

    #[test]
    fn reopen_removes_lock_file() {
        let dir = tmp_dir("reopen");
        let fg = FlowGate::new(&dir, ["batch"]).unwrap();
        fg.set("batch", true).unwrap();
        assert!(dir.join("flow-gate-batch.lock").exists());
        fg.set("batch", false).unwrap();
        assert!(!dir.join("flow-gate-batch.lock").exists());
        assert!(fg.is_open("batch"));
    }

    #[test]
    fn shell_side_toggle_reconciles() {
        let dir = tmp_dir("shell");
        let fg = FlowGate::new(&dir, ["express"]).unwrap();
        assert!(fg.is_open("express"));
        // Operator closes the gate from the shell by creating the lock file.
        std::fs::write(dir.join("flow-gate-express.lock"), b"closed\n").unwrap();
        // Until reconcile, the in-memory atomic is stale.
        assert!(fg.is_open("express"));
        fg.reconcile_from_disk();
        assert!(!fg.is_open("express"));
    }

    #[test]
    fn snapshot_includes_global() {
        let dir = tmp_dir("snap");
        let fg = FlowGate::new(&dir, ["batch"]).unwrap();
        let snap = fg.snapshot();
        assert_eq!(snap.get("global"), Some(&false));
        assert_eq!(snap.get("batch"), Some(&false));
    }

    #[test]
    fn unknown_label_governed_by_global_only() {
        let dir = tmp_dir("unknown");
        let fg = FlowGate::new(&dir, ["batch"]).unwrap();
        // A label that was never declared: open while global is open.
        assert!(fg.is_open("some-new-node"));
        fg.set(GLOBAL_LABEL, true).unwrap();
        assert!(!fg.is_open("some-new-node"));
    }
}
