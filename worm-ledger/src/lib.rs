// SPDX-License-Identifier: Apache-2.0

//! `worm-ledger` — the WORM ledger's durable contract, storage
//! backends, and compliance layer.
//!
//! Extracted from `service-fs` per `BRIEF-os-totebox-communication-
//! complex-2026-09.md` decisions 6 and 16 — see `ledger.rs`'s module
//! doc for the storage rationale and `guard.rs`'s for the compliance
//! rationale. Every service that needs a WORM-ledger-backed store
//! links this crate directly and owns its own instance, rather than
//! writing through one shared daemon process.
//!
//! Layering (matches the L1/L2/L3 vocabulary already used in
//! `ledger.rs`'s own doc comment; L3 -- the HTTP wire layer -- lives
//! in the separate `worm-ledger-http` crate, not here):
//! - **L1 storage**: `ledger` (the `LedgerBackend` trait + entry/
//!   checkpoint types), `posix_tile` (on-disk backend), `blob`
//!   (content-addressed blob store for attachments — never store
//!   large binaries in the ledger's own tile log).
//! - **L2 policy**: `retention` (retention-class vocabulary + expiry),
//!   `hold` (litigation-hold records, folded from ledger history —
//!   never a mutable flag), `disposition` (the propose/execute split
//!   for actually disposing of expired/erasure-eligible records),
//!   `guard` (`GuardedLedger` — the append-time enforcement point
//!   tying `ledger` and `retention` together).
//!
//! Decision 16's rule for a new service deciding whether it needs L2:
//! *if this store were deleted tonight, would any legal, tax, or
//! contractual obligation go unmet, or any fact become unprovable?*
//! Yes → "system of record": declare a retention class per record
//! type and write through `guard::GuardedLedger`. No → "derived view":
//! `ledger`/`posix_tile` only, must be rebuildable from its source,
//! must never be the only copy of anything.

pub mod blob;
pub mod disposition;
pub mod guard;
pub mod hold;
pub mod ledger;
pub mod posix_tile;
pub mod retention;

pub use guard::{GuardError, GuardedLedger};
pub use ledger::{
    now_unix_seconds, verify_checkpoint_signature, Checkpoint, ConsistencyProof, Entry,
    InMemoryLedger, InclusionProof, LedgerBackend, LedgerError,
};
pub use posix_tile::PosixTileLedger;
pub use retention::{
    compute_expiry, load_retention_classes, RetentionAnchor, RetentionClass, RetentionVocabulary,
};

/// Open a `PosixTileLedger` and box it as a `LedgerBackend` trait object.
/// Convenience wrapper for callers (e.g. a daemon's `main.rs`, or
/// integration-test setup) that need a
/// `Box<dyn LedgerBackend + Send + Sync>` without importing `posix_tile`
/// directly. Carried over unchanged from `service-fs::posix_tile_open`.
pub fn posix_tile_open(
    root: impl Into<std::path::PathBuf>,
    origin: impl Into<String>,
    signing_key_path: Option<impl AsRef<std::path::Path>>,
) -> Result<Box<dyn LedgerBackend + Send + Sync>, LedgerError> {
    Ok(Box::new(PosixTileLedger::open(
        root,
        origin,
        signing_key_path,
    )?))
}
