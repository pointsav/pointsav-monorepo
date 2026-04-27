//! system-ledger — the substrate-tier consumer of the Capability
//! Ledger Substrate primitives in `system-core`.
//!
//! Per `~/Foundry/conventions/system-substrate-doctrine.md` §3.1:
//! "Before the kernel honors any capability invocation, it consults
//! the ledger for: current revocation status of the invoking
//! capability; time-bound expiry per Mechanism A; apex root validity."
//!
//! This crate owns the **state machine** that performs that
//! consultation. The cryptographic primitives (Capability,
//! WitnessRecord, SignedCheckpoint) live in `system-core`; this
//! crate composes them into a kernel-side verifier.
//!
//! # Module layout
//!
//! - [`cache`] — recent-N checkpoint cache (LRU)
//! - [`revocation`] — revoked-capability set
//! - [`apex`] — apex history + post-handover invariant
//! - [`witness`] — `ssh-keygen -Y verify` wrapper for witness records
//!
//! # Public API
//!
//! [`LedgerConsumer`] is the kernel-facing trait. v0.1.x ships
//! [`InMemoryLedger`] as the concrete impl; future MINOR may add
//! `MoonshotDatabaseLedger` once `moonshot-database` ships.

pub mod apex;
pub mod cache;
pub mod revocation;
pub mod witness;

use system_core::{Capability, SignedCheckpoint, WitnessRecord};

/// Kernel verifier verdict on a capability invocation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verdict {
    /// Honor the invocation. Capability is current and unexpired.
    Allow,
    /// Refuse with structured reason.
    Refuse(RefuseReason),
    /// Honor the invocation AND log the witness extension into the
    /// ledger so future invocations see the new expiry. Caller MUST
    /// append the witness record to the ledger before honoring.
    ExtendThenAllow { new_expiry_t: u64 },
}

/// Why an invocation was refused.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RefuseReason {
    /// Capability is in the revocation set.
    Revoked,
    /// `expiry_t` reached and no valid witness extension presented.
    Expired,
    /// Witness signature failed verification.
    WitnessSignatureInvalid,
    /// Witness record's hash is not in the current Merkle root.
    WitnessNotInLedger,
    /// Witness presented but capability has no `witness_pubkey`
    /// (non-extensible by construction).
    NotExtensible,
    /// `current_root` is not signed by the current apex.
    ApexInvalid,
    /// Post-handover invariant: only P-new accepted from N+3+;
    /// presented checkpoint signed only by P-old.
    StaleApex,
}

/// Errors that prevent the verifier from rendering a verdict.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConsultError {
    /// Internal cache or state inconsistency.
    InconsistentState(String),
    /// `ssh-keygen -Y verify` invocation failed.
    WitnessVerifyFailed(String),
}

/// Errors applying state changes to the ledger.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LedgerError {
    /// Apex handover refused — handover checkpoint malformed or
    /// fails the both-signatures-required predicate.
    InvalidHandover(String),
    /// Revocation event references an unknown capability.
    UnknownCapability,
}

/// Kernel-facing consumer of the Capability Ledger Substrate. The
/// kernel calls into this trait to decide whether to honor a
/// capability invocation; the concrete impl owns the cache /
/// revocation / apex / witness state.
pub trait LedgerConsumer {
    /// Consult the ledger about a capability invocation. Returns a
    /// [`Verdict`] or a [`ConsultError`] if the consultation itself
    /// fails (cache miss without on-disk fallback, witness-verify
    /// shellout failure, etc).
    ///
    /// `now` is unix seconds (UTC). `witness` is optional; if the
    /// capability is past `expiry_t` and no witness is supplied,
    /// the verdict is [`Verdict::Refuse`] with [`RefuseReason::Expired`].
    fn consult_capability(
        &self,
        cap: &Capability,
        current_root: &SignedCheckpoint,
        now: u64,
        witness: Option<&WitnessRecord>,
    ) -> Result<Verdict, ConsultError>;

    /// Apply a revocation event to the ledger state. After this
    /// call, subsequent [`consult_capability`] for the revoked
    /// capability returns [`Verdict::Refuse`] with [`RefuseReason::Revoked`].
    fn apply_revocation(&mut self, event: revocation::RevocationEvent) -> Result<(), LedgerError>;

    /// Apply an apex handover. After this call, only `new_apex`
    /// signatures are accepted on subsequent checkpoints
    /// (post-handover invariant per convention §4 height-N+3+).
    fn apply_apex_handover(
        &mut self,
        old_apex_name: &str,
        old_apex_pubkey: &[u8; 32],
        new_apex_name: &str,
        new_apex_pubkey: &[u8; 32],
        handover_checkpoint: &SignedCheckpoint,
    ) -> Result<(), LedgerError>;
}

/// In-memory [`LedgerConsumer`] for v0.1.x. Single-writer; not
/// thread-safe — matches the kernel-side single-threaded substrate
/// model. Future MINOR may add Mutex wrapping for shared deployments
/// or a `MoonshotDatabaseLedger` impl backed by `moonshot-database`.
pub struct InMemoryLedger {
    pub cache: cache::CheckpointCache,
    pub revocations: revocation::RevocationSet,
    pub apex: apex::ApexHistory,
}

impl InMemoryLedger {
    pub fn new() -> Self {
        Self {
            cache: cache::CheckpointCache::with_capacity(64),
            revocations: revocation::RevocationSet::new(),
            apex: apex::ApexHistory::new(),
        }
    }
}

impl Default for InMemoryLedger {
    fn default() -> Self {
        Self::new()
    }
}

// LedgerConsumer impl on InMemoryLedger lands once cache /
// revocation / apex / witness modules are filled in. Stub for
// skeleton commit; filled in subsequent commits per task #20.
