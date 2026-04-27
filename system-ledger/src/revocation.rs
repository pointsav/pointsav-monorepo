//! Revoked-capability set. Skeleton — implementation lands in the
//! next commit per task #19.

use std::collections::{HashMap, HashSet};
use system_core::Hash256;

/// One revocation entry — the audit detail behind a "this capability
/// is no longer honored" decision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RevocationEvent {
    pub capability_hash: Hash256,
    /// Unix seconds (UTC) when the revocation was recorded in the
    /// ledger.
    pub revoked_at: u64,
    /// Apex (or delegated witness) that signed the revocation.
    pub signed_by: String,
    /// Ledger height at which the revocation entry was anchored.
    pub ledger_height: u64,
}

/// Revoked-capability set. O(1) membership check via `HashSet`;
/// audit detail in a sidecar `HashMap`.
pub struct RevocationSet {
    revoked: HashSet<Hash256>,
    /// Audit detail keyed by capability_hash. Read accessor lands
    /// in task #19; field exists in skeleton so the next commit is
    /// a pure expansion rather than a structural change.
    #[allow(dead_code)]
    detail: HashMap<Hash256, RevocationEvent>,
}

impl RevocationSet {
    pub fn new() -> Self {
        Self {
            revoked: HashSet::new(),
            detail: HashMap::new(),
        }
    }

    pub fn contains(&self, capability_hash: &Hash256) -> bool {
        self.revoked.contains(capability_hash)
    }

    pub fn len(&self) -> usize {
        self.revoked.len()
    }

    pub fn is_empty(&self) -> bool {
        self.revoked.is_empty()
    }
}

impl Default for RevocationSet {
    fn default() -> Self {
        Self::new()
    }
}
