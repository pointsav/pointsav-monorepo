// SPDX-License-Identifier: LicenseRef-PointSav-Proprietary

//! Shared circuit breaker per Yo-Yo node — the single source of truth for
//! node health across every connected archive.
//!
//! ## Why this lives at the chassis
//!
//! Without a central breaker, each archive's Doorman keeps its own circuit
//! state. When a shared Yo-Yo node dies, the first archive opens its circuit
//! after a handful of failures — but the other nine archives know nothing and
//! keep dispatching, each paying the full timeout before their own circuits
//! open. Ten archives times the per-request timeout is a long, expensive
//! cascade against a node that is already down.
//!
//! Brokering through the chassis collapses that: there is one breaker per
//! labeled node, shared by all archives. The first failures open it once, and
//! every archive's next request fails fast until the node recovers.
//!
//! ## State machine
//!
//! ```text
//!   Closed ──(failure_threshold consecutive failures)──► Open
//!     ▲                                                    │
//!     │ (probe succeeds)                                   │ (cooldown elapsed)
//!     │                                                    ▼
//!   HalfOpen ◄───────────────────(allow one probe)──── HalfOpen
//!     │
//!     └──(probe fails)──► Open
//! ```
//!
//! The breaker is in-process state, rebuilt on chassis restart — consistent
//! with the stateless rule (no persistent data of its own).

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

/// Consecutive failures that trip a closed circuit to open.
pub const DEFAULT_FAILURE_THRESHOLD: u32 = 5;

/// How long an open circuit waits before allowing a half-open probe.
pub const DEFAULT_COOLDOWN: Duration = Duration::from_secs(300);

/// The three circuit states.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CircuitState {
    /// Normal operation — requests flow.
    Closed,
    /// Tripped — requests fail fast until the cooldown elapses.
    Open,
    /// Cooldown elapsed — one probe request is allowed to test recovery.
    HalfOpen,
}

impl CircuitState {
    /// Display/wire label.
    pub fn as_str(self) -> &'static str {
        match self {
            CircuitState::Closed => "closed",
            CircuitState::Open => "open",
            CircuitState::HalfOpen => "half-open",
        }
    }
}

#[derive(Debug)]
struct Inner {
    state: CircuitState,
    consecutive_failures: u32,
    opened_at: Option<Instant>,
    /// Set true while a half-open probe is outstanding so a second concurrent
    /// caller does not also slip through.
    probe_in_flight: bool,
}

/// A circuit breaker for one Yo-Yo node label.
#[derive(Debug)]
pub struct NodeCircuit {
    label: String,
    inner: Mutex<Inner>,
    failure_threshold: u32,
    cooldown: Duration,
}

impl NodeCircuit {
    /// Create a closed circuit for `label` with default thresholds.
    pub fn new(label: impl Into<String>) -> Self {
        Self::with_config(label, DEFAULT_FAILURE_THRESHOLD, DEFAULT_COOLDOWN)
    }

    /// Create a closed circuit with explicit threshold and cooldown.
    pub fn with_config(
        label: impl Into<String>,
        failure_threshold: u32,
        cooldown: Duration,
    ) -> Self {
        Self {
            label: label.into(),
            inner: Mutex::new(Inner {
                state: CircuitState::Closed,
                consecutive_failures: 0,
                opened_at: None,
                probe_in_flight: false,
            }),
            failure_threshold: failure_threshold.max(1),
            cooldown,
        }
    }

    /// The node label.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Whether a request may proceed right now. A `Closed` circuit always
    /// allows. An `Open` circuit allows exactly one probe once the cooldown has
    /// elapsed (transitioning to `HalfOpen`); otherwise it refuses. The caller
    /// MUST report the outcome via [`NodeCircuit::record_success`] /
    /// [`NodeCircuit::record_failure`].
    pub fn allow_request(&self) -> bool {
        let mut g = self.inner.lock().expect("node circuit lock poisoned");
        match g.state {
            CircuitState::Closed => true,
            CircuitState::HalfOpen => {
                // Only one probe at a time.
                if g.probe_in_flight {
                    false
                } else {
                    g.probe_in_flight = true;
                    true
                }
            }
            CircuitState::Open => {
                let elapsed = g.opened_at.map(|t| t.elapsed()).unwrap_or_default();
                if elapsed >= self.cooldown {
                    g.state = CircuitState::HalfOpen;
                    g.probe_in_flight = true;
                    true
                } else {
                    false
                }
            }
        }
    }

    /// Report a successful dispatch. Resets the breaker to `Closed`.
    pub fn record_success(&self) {
        let mut g = self.inner.lock().expect("node circuit lock poisoned");
        g.state = CircuitState::Closed;
        g.consecutive_failures = 0;
        g.opened_at = None;
        g.probe_in_flight = false;
    }

    /// Report a failed dispatch. In `Closed`, increments the failure counter
    /// and opens at the threshold. In `HalfOpen`, re-opens immediately (the
    /// probe failed). In `Open`, no-op.
    pub fn record_failure(&self) {
        let mut g = self.inner.lock().expect("node circuit lock poisoned");
        match g.state {
            CircuitState::Closed => {
                g.consecutive_failures = g.consecutive_failures.saturating_add(1);
                if g.consecutive_failures >= self.failure_threshold {
                    g.state = CircuitState::Open;
                    g.opened_at = Some(Instant::now());
                }
            }
            CircuitState::HalfOpen => {
                g.state = CircuitState::Open;
                g.opened_at = Some(Instant::now());
                g.probe_in_flight = false;
            }
            CircuitState::Open => {}
        }
    }

    /// Current state (for the `/readyz` panel).
    pub fn state(&self) -> CircuitState {
        self.inner.lock().expect("node circuit lock poisoned").state
    }

    /// Seconds since the circuit opened, if it is open or half-open.
    pub fn opened_for_secs(&self) -> Option<u64> {
        let g = self.inner.lock().expect("node circuit lock poisoned");
        g.opened_at.map(|t| t.elapsed().as_secs())
    }
}

/// A registry of node circuits, keyed by Yo-Yo label. Shared across the chassis
/// so every archive sees the same health view.
#[derive(Debug, Default)]
pub struct CircuitRegistry {
    circuits: HashMap<String, NodeCircuit>,
}

impl CircuitRegistry {
    /// Build a registry with a default circuit for each label.
    pub fn new<I, S>(labels: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let circuits = labels
            .into_iter()
            .map(|l| {
                let label = l.into();
                (label.clone(), NodeCircuit::new(label))
            })
            .collect();
        Self { circuits }
    }

    /// Get the circuit for a label, if registered.
    pub fn get(&self, label: &str) -> Option<&NodeCircuit> {
        self.circuits.get(label)
    }

    /// Snapshot of every circuit's state for the status panel.
    pub fn snapshot(&self) -> HashMap<String, CircuitState> {
        self.circuits
            .iter()
            .map(|(k, v)| (k.clone(), v.state()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn closed_allows_requests() {
        let c = NodeCircuit::new("trainer");
        assert!(c.allow_request());
        assert_eq!(c.state(), CircuitState::Closed);
    }

    #[test]
    fn opens_after_threshold_failures() {
        let c = NodeCircuit::with_config("trainer", 3, Duration::from_secs(300));
        for _ in 0..3 {
            assert!(c.allow_request());
            c.record_failure();
        }
        assert_eq!(c.state(), CircuitState::Open);
        // Now fails fast.
        assert!(!c.allow_request());
    }

    #[test]
    fn success_resets_failure_count() {
        let c = NodeCircuit::with_config("trainer", 3, Duration::from_secs(300));
        c.record_failure();
        c.record_failure();
        c.record_success();
        c.record_failure();
        c.record_failure();
        // Only two consecutive failures since the reset — still closed.
        assert_eq!(c.state(), CircuitState::Closed);
    }

    #[test]
    fn half_open_probe_after_cooldown() {
        let c = NodeCircuit::with_config("trainer", 1, Duration::from_millis(5));
        c.record_failure(); // opens immediately (threshold 1)
        assert_eq!(c.state(), CircuitState::Open);
        assert!(!c.allow_request()); // cooldown not elapsed
        std::thread::sleep(Duration::from_millis(8));
        // Cooldown elapsed — one probe allowed.
        assert!(c.allow_request());
        assert_eq!(c.state(), CircuitState::HalfOpen);
    }

    #[test]
    fn half_open_success_closes() {
        let c = NodeCircuit::with_config("trainer", 1, Duration::from_millis(5));
        c.record_failure();
        std::thread::sleep(Duration::from_millis(8));
        assert!(c.allow_request()); // half-open probe
        c.record_success();
        assert_eq!(c.state(), CircuitState::Closed);
        assert!(c.allow_request());
    }

    #[test]
    fn half_open_failure_reopens() {
        let c = NodeCircuit::with_config("trainer", 1, Duration::from_millis(5));
        c.record_failure();
        std::thread::sleep(Duration::from_millis(8));
        assert!(c.allow_request()); // half-open probe
        c.record_failure(); // probe fails
        assert_eq!(c.state(), CircuitState::Open);
    }

    #[test]
    fn only_one_half_open_probe_at_a_time() {
        let c = NodeCircuit::with_config("trainer", 1, Duration::from_millis(5));
        c.record_failure();
        std::thread::sleep(Duration::from_millis(8));
        assert!(c.allow_request()); // first probe claims the slot
                                    // A concurrent caller must not also slip through.
        assert!(!c.allow_request());
    }

    #[test]
    fn opened_for_secs_reports_while_open() {
        let c = NodeCircuit::with_config("trainer", 1, Duration::from_secs(300));
        assert!(c.opened_for_secs().is_none());
        c.record_failure();
        assert!(c.opened_for_secs().is_some());
    }

    #[test]
    fn registry_isolates_per_label() {
        let reg = CircuitRegistry::new(["trainer", "graph"]);
        let trainer = reg.get("trainer").unwrap();
        // Trip only the trainer.
        for _ in 0..DEFAULT_FAILURE_THRESHOLD {
            trainer.record_failure();
        }
        assert_eq!(reg.get("trainer").unwrap().state(), CircuitState::Open);
        // graph is unaffected — a dead trainer does not blind the graph node.
        assert_eq!(reg.get("graph").unwrap().state(), CircuitState::Closed);
    }

    #[test]
    fn registry_snapshot_lists_all() {
        let reg = CircuitRegistry::new(["trainer", "graph", "proxy"]);
        let snap = reg.snapshot();
        assert_eq!(snap.len(), 3);
        assert_eq!(snap.get("trainer"), Some(&CircuitState::Closed));
    }

    #[test]
    fn cascade_prevention_one_open_serves_all() {
        // Simulate the CRIT-2 scenario: a shared registry; the node dies; the
        // first archive's failures open the breaker; every subsequent archive
        // fails fast rather than re-discovering the death independently.
        let reg = CircuitRegistry::new(["graph"]);
        let c = reg.get("graph").unwrap();
        for _ in 0..DEFAULT_FAILURE_THRESHOLD {
            assert!(c.allow_request());
            c.record_failure();
        }
        // Now every other archive sharing this registry fails fast.
        for _ in 0..50 {
            assert!(!c.allow_request());
        }
    }
}
