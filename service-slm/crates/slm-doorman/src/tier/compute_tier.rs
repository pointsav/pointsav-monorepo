// SPDX-License-Identifier: Apache-2.0

//! The boundary the router uses to reach a remote compute node.
//!
//! `Doorman` routes Tier B requests through `dyn ComputeTier` and never
//! names a concrete client. The Yo-Yo client implements it today; code that
//! manages a remote fleet can implement it from outside this crate and be
//! loaded as a plug-in. Nothing here is specific to one provider.

use std::sync::Arc;

use async_trait::async_trait;

use crate::error::Result;
use slm_core::{ComputeRequest, ComputeResponse};

/// Runtime state of one compute node, for health reporting (`/readyz`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TierStatus {
    /// Last health probe outcome.
    pub health_up: bool,
    /// Seconds since the probe last went down; `None` while it is up.
    pub health_down_secs: Option<u64>,
    /// Circuit state label: `"closed"`, `"open"` or `"half-open"`.
    pub circuit: &'static str,
    /// Seconds the circuit has been open; `None` when it is not open.
    pub opened_for_secs: Option<u64>,
    /// Zone the node runs in, when known.
    pub zone: Option<String>,
}

/// One remote compute node the router can send a request to.
#[async_trait]
pub trait ComputeTier: Send + Sync {
    /// True when the node is currently accepting requests (health probe up
    /// and circuit not open). Cheap; called before every dispatch.
    fn allow_request(&self) -> bool;

    /// Run one request on the node.
    async fn complete(&self, req: &ComputeRequest) -> Result<ComputeResponse>;

    /// Current health and circuit state.
    fn status(&self) -> TierStatus;
}

/// Wrap a concrete node as the shared handle `DoormanConfig` holds.
pub fn shared<T: ComputeTier + 'static>(tier: T) -> Arc<dyn ComputeTier> {
    Arc::new(tier)
}
