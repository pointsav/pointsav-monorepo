// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Flow policy — runtime-switchable routing strategy.
//!
//! The flow policy decides which tier label a request without an explicit
//! `yoyo_label` is routed to, given its complexity. The operator changes the
//! policy at runtime (`POST /v1/flow/policy`) without restarting the Doorman;
//! the change takes effect on the next routed request.
//!
//! Four policies:
//!
//! - [`FlowPolicy::Balanced`] (default): low/medium complexity → Tier A;
//!   high complexity → the express GPU node. The everyday mode.
//! - [`FlowPolicy::DrainBatch`]: non-express work goes to the batch (L4)
//!   node; the express (A100) node stays stopped. Used to process a backlog
//!   cheaply overnight.
//! - [`FlowPolicy::DrainExpress`]: everything goes to the express (A100) node
//!   to clear a backlog as fast as possible.
//! - [`FlowPolicy::LocalOnly`]: everything stays on Tier A; both GPU nodes
//!   stay stopped. The cheapest mode and an emergency cost brake that still
//!   serves chat (extraction, which requires Tier B, is refused).
//!
//! The policy is held behind an `RwLock` so reads (every routed request) are
//! cheap and writes (operator toggles) are rare. On Doorman restart the
//! policy resets to the env-var default (`SLM_FLOW_POLICY`), so a crash never
//! leaves the system stuck in `drain-express` burning A100 time.

use std::sync::RwLock;

use serde::{Deserialize, Serialize};

/// The label of the batch (cheaper, L4) GPU node.
pub const BATCH_LABEL: &str = "batch";
/// The label of the express (faster, A100) GPU node.
pub const EXPRESS_LABEL: &str = "express";

/// Request complexity, supplied by the caller as a hint or inferred. Mirrors
/// the `X-Foundry-Complexity` header.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Complexity {
    Low,
    Medium,
    High,
}

impl Complexity {
    /// Parse a header value (`"low"`, `"medium"`, `"high"`), defaulting to
    /// `Medium` for any unrecognized or absent value.
    pub fn from_header(value: Option<&str>) -> Self {
        match value.map(|v| v.trim().to_ascii_lowercase()).as_deref() {
            Some("low") => Complexity::Low,
            Some("high") => Complexity::High,
            _ => Complexity::Medium,
        }
    }
}

/// The runtime routing policy.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum FlowPolicy {
    /// Low/medium → Tier A; high → express GPU node. The default.
    #[default]
    Balanced,
    /// Non-express work → batch (L4) node; express node stays stopped.
    DrainBatch,
    /// Everything → express (A100) node to clear a backlog fast.
    DrainExpress,
    /// Everything → Tier A; both GPU nodes stay stopped.
    LocalOnly,
}

/// Where a request should be routed under the current policy.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RouteTarget {
    /// Route to the local Tier A model.
    TierALocal,
    /// Route to the named GPU node label (e.g. `"batch"` or `"express"`).
    TierBNode(&'static str),
}

impl FlowPolicy {
    /// Parse the `SLM_FLOW_POLICY` env value, defaulting to `Balanced`.
    pub fn from_env_value(value: Option<&str>) -> Self {
        match value.map(|v| v.trim().to_ascii_lowercase()).as_deref() {
            Some("drain-batch") | Some("drain_batch") => FlowPolicy::DrainBatch,
            Some("drain-express") | Some("drain_express") => FlowPolicy::DrainExpress,
            Some("local-only") | Some("local_only") => FlowPolicy::LocalOnly,
            Some("balanced") => FlowPolicy::Balanced,
            _ => FlowPolicy::Balanced,
        }
    }

    /// Decide where a request of the given complexity is routed under this
    /// policy. This governs requests WITHOUT an explicit node label; an
    /// explicit label always wins over the policy (handled by the router).
    pub fn route(self, complexity: Complexity) -> RouteTarget {
        match self {
            FlowPolicy::Balanced => match complexity {
                Complexity::Low | Complexity::Medium => RouteTarget::TierALocal,
                Complexity::High => RouteTarget::TierBNode(EXPRESS_LABEL),
            },
            FlowPolicy::DrainBatch => RouteTarget::TierBNode(BATCH_LABEL),
            FlowPolicy::DrainExpress => RouteTarget::TierBNode(EXPRESS_LABEL),
            FlowPolicy::LocalOnly => RouteTarget::TierALocal,
        }
    }

    /// True when this policy keeps both GPU nodes stopped (no Tier B routing
    /// for unlabelled requests). The idle monitor uses this to decide whether
    /// to allow auto-start.
    pub fn is_local_only(self) -> bool {
        matches!(self, FlowPolicy::LocalOnly)
    }

    /// The kebab-case wire string for this policy.
    pub fn as_str(self) -> &'static str {
        match self {
            FlowPolicy::Balanced => "balanced",
            FlowPolicy::DrainBatch => "drain-batch",
            FlowPolicy::DrainExpress => "drain-express",
            FlowPolicy::LocalOnly => "local-only",
        }
    }
}

/// Thread-safe holder for the current policy. Construct once at startup,
/// share the `Arc<FlowPolicyState>` across handlers and the drain worker.
#[derive(Debug)]
pub struct FlowPolicyState {
    inner: RwLock<FlowPolicy>,
}

impl FlowPolicyState {
    /// Create a holder initialized to `initial` (typically from the env).
    pub fn new(initial: FlowPolicy) -> Self {
        Self {
            inner: RwLock::new(initial),
        }
    }

    /// Read the current policy. Cheap; takes a read lock.
    pub fn current(&self) -> FlowPolicy {
        *self.inner.read().expect("flow policy lock poisoned")
    }

    /// Replace the current policy. Returns the previous value.
    pub fn set(&self, policy: FlowPolicy) -> FlowPolicy {
        let mut guard = self.inner.write().expect("flow policy lock poisoned");
        std::mem::replace(&mut guard, policy)
    }
}

impl Default for FlowPolicyState {
    fn default() -> Self {
        Self::new(FlowPolicy::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn balanced_routes_by_complexity() {
        let p = FlowPolicy::Balanced;
        assert_eq!(p.route(Complexity::Low), RouteTarget::TierALocal);
        assert_eq!(p.route(Complexity::Medium), RouteTarget::TierALocal);
        assert_eq!(
            p.route(Complexity::High),
            RouteTarget::TierBNode(EXPRESS_LABEL)
        );
    }

    #[test]
    fn drain_batch_sends_all_to_batch() {
        let p = FlowPolicy::DrainBatch;
        for c in [Complexity::Low, Complexity::Medium, Complexity::High] {
            assert_eq!(p.route(c), RouteTarget::TierBNode(BATCH_LABEL));
        }
    }

    #[test]
    fn drain_express_sends_all_to_express() {
        let p = FlowPolicy::DrainExpress;
        for c in [Complexity::Low, Complexity::Medium, Complexity::High] {
            assert_eq!(p.route(c), RouteTarget::TierBNode(EXPRESS_LABEL));
        }
    }

    #[test]
    fn local_only_keeps_everything_local() {
        let p = FlowPolicy::LocalOnly;
        for c in [Complexity::Low, Complexity::Medium, Complexity::High] {
            assert_eq!(p.route(c), RouteTarget::TierALocal);
        }
        assert!(p.is_local_only());
    }

    #[test]
    fn env_parsing_handles_variants() {
        assert_eq!(
            FlowPolicy::from_env_value(Some("drain-express")),
            FlowPolicy::DrainExpress
        );
        assert_eq!(
            FlowPolicy::from_env_value(Some("DRAIN_BATCH")),
            FlowPolicy::DrainBatch
        );
        assert_eq!(
            FlowPolicy::from_env_value(Some("local-only")),
            FlowPolicy::LocalOnly
        );
        assert_eq!(FlowPolicy::from_env_value(None), FlowPolicy::Balanced);
        assert_eq!(
            FlowPolicy::from_env_value(Some("garbage")),
            FlowPolicy::Balanced
        );
    }

    #[test]
    fn complexity_header_parsing() {
        assert_eq!(Complexity::from_header(Some("low")), Complexity::Low);
        assert_eq!(Complexity::from_header(Some("HIGH")), Complexity::High);
        assert_eq!(Complexity::from_header(Some("weird")), Complexity::Medium);
        assert_eq!(Complexity::from_header(None), Complexity::Medium);
    }

    #[test]
    fn state_set_returns_previous() {
        let state = FlowPolicyState::new(FlowPolicy::Balanced);
        assert_eq!(state.current(), FlowPolicy::Balanced);
        let prev = state.set(FlowPolicy::DrainExpress);
        assert_eq!(prev, FlowPolicy::Balanced);
        assert_eq!(state.current(), FlowPolicy::DrainExpress);
    }

    #[test]
    fn wire_string_round_trips() {
        for p in [
            FlowPolicy::Balanced,
            FlowPolicy::DrainBatch,
            FlowPolicy::DrainExpress,
            FlowPolicy::LocalOnly,
        ] {
            assert_eq!(FlowPolicy::from_env_value(Some(p.as_str())), p);
        }
    }
}
