// SPDX-License-Identifier: Apache-2.0 OR MIT

//! VM lifecycle — the seven-state machine that governs a burst GPU node.
//!
//! A burst GPU node (Yo-Yo) is stopped when idle to save cost and started on
//! demand. The lifecycle is more than a boolean "up/down": a node can be
//! booting, wedged, or failed-to-start, and each state drives a different
//! routing decision. This machine names all seven states explicitly so the
//! operator console and the router can reason about exactly where a node is.
//!
//! ```text
//!  Unknown ──probe──► Stopped ──request──► Staging ──health──► Available
//!     │                  ▲                    │                    │
//!     │                  │                    │ timeout/3×fail     │ idle
//!     │ probe-fail       └────────────────────┤                    ▼
//!     ▼                                        ▼                 Stopped
//!  FailedStart ◄──boot timeout── Staging   Zombie ◄─3× health fail (Running/Available)
//!     │                                        │
//!     └──operator reset──► Unknown ◄──operator reset──┘
//! ```
//!
//! ## States
//!
//! - [`VmState::Unknown`] — startup; the true state must be probed from the
//!   cloud provider before any routing decision.
//! - [`VmState::Stopped`] — the VM is off. No billing. A Tier B request
//!   triggers a start.
//! - [`VmState::Staging`] — a start was issued; the VM is booting and loading
//!   the model. Requests queue or receive a 202.
//! - [`VmState::Running`] — the VM is on but the health probe has not yet
//!   confirmed readiness.
//! - [`VmState::Available`] — the VM is on and healthy; dispatch immediately.
//! - [`VmState::FailedStart`] — a start was issued but the boot deadline
//!   passed or the provider returned an error. Requires operator reset; falls
//!   back to Tier A meanwhile.
//! - [`VmState::Zombie`] — the VM is running but unresponsive (repeated health
//!   failures). Requires operator reset; falls back to Tier A.
//!
//! ## Double-start prevention
//!
//! Two concurrent requests must not both issue a provider start (which would
//! cost money and confuse the operator about which VM is authoritative). The
//! [`VmLifecycle::try_begin_start`] method uses an atomic compare-exchange so
//! exactly one caller wins the right to start; the others observe the state
//! transition to `Staging` and wait.
//!
//! ## Async readiness
//!
//! State changes are broadcast on a [`tokio::sync::watch`] channel.
//! [`VmLifecycle::subscribe`] hands out a receiver that the express lane and
//! drain worker `await` to learn when the VM reaches `Available`.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::time::{Duration, Instant};

use serde::Serialize;
use tokio::sync::watch;

/// Number of consecutive health-probe failures that move a Running/Available
/// node to [`VmState::Zombie`].
pub const ZOMBIE_FAILURE_THRESHOLD: u32 = 3;

/// The seven lifecycle states of a burst GPU node.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum VmState {
    Unknown,
    Stopped,
    Staging,
    Running,
    Available,
    FailedStart,
    Zombie,
}

impl VmState {
    /// Wire/display string for this state.
    pub fn as_str(self) -> &'static str {
        match self {
            VmState::Unknown => "unknown",
            VmState::Stopped => "stopped",
            VmState::Staging => "staging",
            VmState::Running => "running",
            VmState::Available => "available",
            VmState::FailedStart => "failed-start",
            VmState::Zombie => "zombie",
        }
    }

    /// True when a request may be dispatched directly to this node.
    pub fn is_dispatchable(self) -> bool {
        matches!(self, VmState::Available)
    }

    /// True when the node needs an operator reset before it can serve again
    /// (it will not self-recover).
    pub fn needs_reset(self) -> bool {
        matches!(self, VmState::FailedStart | VmState::Zombie)
    }

    /// Whether a transition from `self` to `to` is permitted.
    fn can_transition_to(self, to: VmState) -> bool {
        use VmState::*;
        match (self, to) {
            // From Unknown, the probe can resolve to any concrete state.
            (Unknown, _) => true,
            // Stopped starts booting.
            (Stopped, Staging) => true,
            // Staging resolves to ready, running-but-unprobed, wedged, or failed.
            (Staging, Available | Running | Zombie | FailedStart) => true,
            // Running becomes Available on a good probe, or wedges to Zombie.
            (Running, Available | Zombie) => true,
            // Available may go idle (Stopped) or wedge (Zombie); a transient
            // probe miss can drop it back to Running.
            (Available, Stopped | Zombie | Running) => true,
            // Terminal states require an explicit operator reset to Unknown,
            // or a direct Stopped (operator forced the VM off).
            (FailedStart, Unknown | Stopped) => true,
            (Zombie, Unknown | Stopped) => true,
            // Idempotent self-transition is always allowed (no-op).
            (a, b) if a == b => true,
            _ => false,
        }
    }
}

/// Mutable interior state, guarded by a short-held `std::sync::Mutex`.
#[derive(Debug)]
struct Inner {
    state: VmState,
    health_failures: u32,
    /// Deadline by which a `Staging` node must reach `Available`, else it is
    /// moved to `FailedStart`. Set when `Staging` is entered.
    boot_deadline: Option<Instant>,
}

/// The lifecycle handle for a single burst GPU node. Share via `Arc`.
#[derive(Debug)]
pub struct VmLifecycle {
    label: String,
    inner: Mutex<Inner>,
    /// Lock-free fast path for the hot router check: true iff state is
    /// `Available`. Mirrors `inner.state == Available`.
    is_available: AtomicBool,
    /// Double-start guard. `true` means a start is in flight; only the caller
    /// that flips it false→true may issue the provider start.
    start_in_flight: AtomicBool,
    /// Broadcasts every state change to async waiters.
    state_tx: watch::Sender<VmState>,
    /// How long a `Staging` node has to reach `Available` before it is failed.
    boot_timeout: Duration,
}

impl VmLifecycle {
    /// Create a lifecycle for `label`, starting in [`VmState::Unknown`].
    /// Returns the handle plus a [`watch::Receiver`] pre-subscribed to state
    /// changes (additional subscribers via [`VmLifecycle::subscribe`]).
    pub fn new(
        label: impl Into<String>,
        boot_timeout: Duration,
    ) -> (Self, watch::Receiver<VmState>) {
        let (state_tx, state_rx) = watch::channel(VmState::Unknown);
        let me = Self {
            label: label.into(),
            inner: Mutex::new(Inner {
                state: VmState::Unknown,
                health_failures: 0,
                boot_deadline: None,
            }),
            is_available: AtomicBool::new(false),
            start_in_flight: AtomicBool::new(false),
            state_tx,
            boot_timeout,
        };
        (me, state_rx)
    }

    /// The node label this lifecycle governs.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Current state (takes the short lock).
    pub fn current(&self) -> VmState {
        self.inner.lock().expect("vm lifecycle lock poisoned").state
    }

    /// Lock-free check for the hot router path: true iff `Available`.
    pub fn is_available_fast(&self) -> bool {
        self.is_available.load(Ordering::Acquire)
    }

    /// Subscribe to state-change notifications. The express lane awaits
    /// `rx.changed()` and re-reads to learn when the node is `Available`.
    pub fn subscribe(&self) -> watch::Receiver<VmState> {
        self.state_tx.subscribe()
    }

    /// Attempt a transition to `new`. Returns the previous state on success,
    /// or `None` if the transition is not permitted (the caller can log and
    /// ignore — an invalid transition is a no-op, never a panic).
    pub fn transition(&self, new: VmState) -> Option<VmState> {
        let mut guard = self.inner.lock().expect("vm lifecycle lock poisoned");
        let old = guard.state;
        if !old.can_transition_to(new) {
            return None;
        }
        guard.state = new;
        // Entering Staging arms the boot deadline; leaving it disarms.
        match new {
            VmState::Staging => guard.boot_deadline = Some(Instant::now() + self.boot_timeout),
            VmState::Available | VmState::Running => {
                guard.boot_deadline = None;
                guard.health_failures = 0;
            }
            _ => guard.boot_deadline = None,
        }
        drop(guard);

        // Mirror the lock-free availability flag.
        self.is_available
            .store(new == VmState::Available, Ordering::Release);
        // Broadcast (ignore error: no receivers is fine).
        let _ = self.state_tx.send(new);
        Some(old)
    }

    /// Try to claim the right to issue a provider start. Returns `true` for
    /// exactly one caller when the node is `Stopped`/`Unknown`/`FailedStart`;
    /// the winner must call [`VmLifecycle::finish_start`] when done. Returns
    /// `false` if a start is already in flight or the node is not in a
    /// startable state.
    ///
    /// On success the node transitions to `Staging` and the boot deadline is
    /// armed.
    pub fn try_begin_start(&self) -> bool {
        // Only start from a state where starting makes sense.
        let state = self.current();
        if !matches!(
            state,
            VmState::Stopped | VmState::Unknown | VmState::FailedStart
        ) {
            return false;
        }
        // Atomically claim the start slot.
        if self
            .start_in_flight
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .is_err()
        {
            return false;
        }
        // We won — move to Staging.
        if self.transition(VmState::Staging).is_none() {
            // Lost a race on the state; release the slot.
            self.start_in_flight.store(false, Ordering::Release);
            return false;
        }
        true
    }

    /// Release the start slot after a start attempt concludes. `succeeded`
    /// transitions the node to `Available`; failure transitions it to
    /// `FailedStart`. Always clears the in-flight guard so a future start can
    /// proceed.
    pub fn finish_start(&self, succeeded: bool) {
        if succeeded {
            self.transition(VmState::Available);
        } else {
            self.transition(VmState::FailedStart);
        }
        self.start_in_flight.store(false, Ordering::Release);
    }

    /// Record a health-probe result. A success on a `Staging`/`Running` node
    /// promotes it to `Available` and resets the failure counter. Consecutive
    /// failures increment a counter; at [`ZOMBIE_FAILURE_THRESHOLD`] a
    /// `Running`/`Available` node is moved to `Zombie`.
    pub fn record_health(&self, ok: bool) {
        if ok {
            // Reset failures; promote to Available from Staging/Running.
            let state = self.current();
            if matches!(state, VmState::Staging | VmState::Running) {
                self.transition(VmState::Available);
            } else {
                let mut g = self.inner.lock().expect("vm lifecycle lock poisoned");
                g.health_failures = 0;
            }
            return;
        }
        // Failure path.
        let (state, failures) = {
            let mut g = self.inner.lock().expect("vm lifecycle lock poisoned");
            g.health_failures = g.health_failures.saturating_add(1);
            (g.state, g.health_failures)
        };
        if failures >= ZOMBIE_FAILURE_THRESHOLD
            && matches!(
                state,
                VmState::Running | VmState::Available | VmState::Staging
            )
        {
            // Staging that never comes up is a failed start; a Running/Available
            // node that stops responding is a zombie.
            if state == VmState::Staging {
                self.finish_start(false);
            } else {
                self.transition(VmState::Zombie);
            }
        }
    }

    /// Check whether a `Staging` node has blown its boot deadline; if so, fail
    /// the start. Call this periodically from the lifecycle monitor. Returns
    /// `true` if the node was just failed.
    pub fn check_boot_deadline(&self) -> bool {
        let expired = {
            let g = self.inner.lock().expect("vm lifecycle lock poisoned");
            g.state == VmState::Staging
                && g.boot_deadline.map(|d| Instant::now() > d).unwrap_or(false)
        };
        if expired {
            self.finish_start(false);
            return true;
        }
        false
    }

    /// Operator reset: move a `FailedStart`/`Zombie` node back to `Unknown` so
    /// the next probe can re-establish its true state. Also clears any stuck
    /// in-flight start guard. Returns the previous state.
    pub fn reset(&self) -> VmState {
        let prev = self.current();
        self.start_in_flight.store(false, Ordering::Release);
        self.transition(VmState::Unknown);
        prev
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lc() -> VmLifecycle {
        VmLifecycle::new("test", Duration::from_secs(120)).0
    }

    #[test]
    fn starts_unknown_not_available() {
        let l = lc();
        assert_eq!(l.current(), VmState::Unknown);
        assert!(!l.is_available_fast());
    }

    #[test]
    fn probe_resolves_unknown_to_stopped() {
        let l = lc();
        assert_eq!(l.transition(VmState::Stopped), Some(VmState::Unknown));
        assert_eq!(l.current(), VmState::Stopped);
    }

    #[test]
    fn invalid_transition_is_noop() {
        let l = lc();
        l.transition(VmState::Stopped);
        // Stopped cannot jump straight to Available.
        assert_eq!(l.transition(VmState::Available), None);
        assert_eq!(l.current(), VmState::Stopped);
    }

    #[test]
    fn begin_start_moves_to_staging() {
        let l = lc();
        l.transition(VmState::Stopped);
        assert!(l.try_begin_start());
        assert_eq!(l.current(), VmState::Staging);
    }

    #[test]
    fn double_start_prevented() {
        let l = lc();
        l.transition(VmState::Stopped);
        assert!(l.try_begin_start());
        // A second concurrent caller must lose.
        assert!(!l.try_begin_start());
    }

    #[test]
    fn finish_start_success_makes_available() {
        let l = lc();
        l.transition(VmState::Stopped);
        l.try_begin_start();
        l.finish_start(true);
        assert_eq!(l.current(), VmState::Available);
        assert!(l.is_available_fast());
        // The start slot is released, so a future start can proceed after stop.
        l.transition(VmState::Stopped);
        assert!(l.try_begin_start());
    }

    #[test]
    fn finish_start_failure_makes_failed() {
        let l = lc();
        l.transition(VmState::Stopped);
        l.try_begin_start();
        l.finish_start(false);
        assert_eq!(l.current(), VmState::FailedStart);
        assert!(l.current().needs_reset());
        assert!(!l.is_available_fast());
    }

    #[test]
    fn health_success_promotes_staging_to_available() {
        let l = lc();
        l.transition(VmState::Stopped);
        l.try_begin_start();
        assert_eq!(l.current(), VmState::Staging);
        l.record_health(true);
        assert_eq!(l.current(), VmState::Available);
    }

    #[test]
    fn three_health_failures_zombie_a_running_node() {
        let l = lc();
        l.transition(VmState::Stopped);
        l.try_begin_start();
        l.finish_start(true); // Available
        for _ in 0..ZOMBIE_FAILURE_THRESHOLD {
            l.record_health(false);
        }
        assert_eq!(l.current(), VmState::Zombie);
        assert!(l.current().needs_reset());
    }

    #[test]
    fn health_failures_reset_on_success() {
        let l = lc();
        l.transition(VmState::Stopped);
        l.try_begin_start();
        l.finish_start(true);
        l.record_health(false);
        l.record_health(false);
        l.record_health(true); // resets counter
        l.record_health(false);
        l.record_health(false);
        // Only two consecutive failures since the reset — not a zombie.
        assert_eq!(l.current(), VmState::Available);
    }

    #[test]
    fn boot_deadline_failure() {
        // Zero timeout: the staging node is immediately past deadline.
        let l = VmLifecycle::new("test", Duration::from_millis(0)).0;
        l.transition(VmState::Stopped);
        l.try_begin_start();
        // Deadline already elapsed.
        std::thread::sleep(Duration::from_millis(2));
        assert!(l.check_boot_deadline());
        assert_eq!(l.current(), VmState::FailedStart);
    }

    #[test]
    fn reset_returns_to_unknown() {
        let l = lc();
        l.transition(VmState::Stopped);
        l.try_begin_start();
        l.finish_start(false); // FailedStart
        let prev = l.reset();
        assert_eq!(prev, VmState::FailedStart);
        assert_eq!(l.current(), VmState::Unknown);
        // After reset, a start can begin again once we know it is stopped.
        l.transition(VmState::Stopped);
        assert!(l.try_begin_start());
    }

    #[tokio::test]
    async fn subscribers_observe_available() {
        let l = lc();
        let mut rx = l.subscribe();
        l.transition(VmState::Stopped);
        l.try_begin_start();
        l.finish_start(true);
        // The receiver should observe a change ending at Available.
        rx.changed().await.unwrap();
        // Drain to the latest value.
        let mut latest = *rx.borrow();
        while rx.has_changed().unwrap_or(false) {
            rx.changed().await.unwrap();
            latest = *rx.borrow();
        }
        assert_eq!(latest, VmState::Available);
    }
}
