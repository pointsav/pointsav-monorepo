// SPDX-License-Identifier: LicenseRef-PointSav-Proprietary

//! Chassis flow gate — per-label and global kill switch for the broker.
//!
//! The operator can pause brokering to any Yo-Yo node label, or globally,
//! without restarting the chassis. A closed gate refuses proxy dispatch for
//! that label; in-flight requests complete. This is the chassis-side billing
//! control that complements each archive's own (file-backed) gate.
//!
//! Per the stateless rule, the chassis gate is in-memory only — it resets to
//! open on restart. A durable pause belongs in the per-archive Doorman gate;
//! the chassis gate is an operational lever for the shared fleet.

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};

/// The reserved label for the chassis-wide kill switch.
pub const GLOBAL_LABEL: &str = "global";

/// In-memory per-label gates plus the global gate.
#[derive(Debug)]
pub struct ChassisFlowGate {
    gates: HashMap<String, AtomicBool>,
}

impl ChassisFlowGate {
    /// Build gates for the given labels plus the implicit global gate. All
    /// start open.
    pub fn new<I, S>(labels: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let mut gates = HashMap::new();
        gates.insert(GLOBAL_LABEL.to_string(), AtomicBool::new(false));
        for l in labels {
            gates
                .entry(l.into())
                .or_insert_with(|| AtomicBool::new(false));
        }
        Self { gates }
    }

    /// True when dispatch to `label` is permitted (global open AND label open).
    pub fn is_open(&self, label: &str) -> bool {
        if self.is_closed_raw(GLOBAL_LABEL) {
            return false;
        }
        !self.is_closed_raw(label)
    }

    /// The label blocking dispatch to `label` (`"global"`, the label itself),
    /// or `None` when permitted.
    pub fn blocking_label(&self, label: &str) -> Option<String> {
        if self.is_closed_raw(GLOBAL_LABEL) {
            return Some(GLOBAL_LABEL.to_string());
        }
        if self.is_closed_raw(label) {
            return Some(label.to_string());
        }
        None
    }

    /// Set a gate closed (`true`) or open (`false`). Creates the gate if the
    /// label was not declared at construction.
    pub fn set(&mut self, label: &str, closed: bool) {
        self.gates
            .entry(label.to_string())
            .or_insert_with(|| AtomicBool::new(false))
            .store(closed, Ordering::SeqCst);
    }

    /// Set a gate via shared reference (gates are atomics; no `&mut` needed
    /// when the label already exists). Returns false if the label is unknown.
    pub fn set_existing(&self, label: &str, closed: bool) -> bool {
        match self.gates.get(label) {
            Some(a) => {
                a.store(closed, Ordering::SeqCst);
                true
            }
            None => false,
        }
    }

    /// Snapshot of every gate, label → closed.
    pub fn snapshot(&self) -> HashMap<String, bool> {
        self.gates
            .iter()
            .map(|(k, v)| (k.clone(), v.load(Ordering::SeqCst)))
            .collect()
    }

    fn is_closed_raw(&self, label: &str) -> bool {
        self.gates
            .get(label)
            .map(|a| a.load(Ordering::SeqCst))
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gates_start_open() {
        let g = ChassisFlowGate::new(["trainer", "graph"]);
        assert!(g.is_open("trainer"));
        assert!(g.is_open("graph"));
        assert!(g.blocking_label("trainer").is_none());
    }

    #[test]
    fn per_label_close() {
        let g = ChassisFlowGate::new(["trainer", "graph"]);
        assert!(g.set_existing("trainer", true));
        assert!(!g.is_open("trainer"));
        assert!(g.is_open("graph"));
        assert_eq!(g.blocking_label("trainer").as_deref(), Some("trainer"));
    }

    #[test]
    fn global_close_blocks_all() {
        let g = ChassisFlowGate::new(["trainer", "graph"]);
        g.set_existing(GLOBAL_LABEL, true);
        assert!(!g.is_open("trainer"));
        assert!(!g.is_open("graph"));
        assert_eq!(g.blocking_label("graph").as_deref(), Some("global"));
    }

    #[test]
    fn set_creates_unknown_label() {
        let mut g = ChassisFlowGate::new(["trainer"]);
        g.set("new-node", true);
        assert!(!g.is_open("new-node"));
    }

    #[test]
    fn set_existing_rejects_unknown() {
        let g = ChassisFlowGate::new(["trainer"]);
        assert!(!g.set_existing("unknown", true));
    }

    #[test]
    fn snapshot_includes_global() {
        let g = ChassisFlowGate::new(["trainer"]);
        let snap = g.snapshot();
        assert_eq!(snap.get("global"), Some(&false));
        assert_eq!(snap.get("trainer"), Some(&false));
    }
}
