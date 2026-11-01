// SPDX-License-Identifier: Apache-2.0
//
// Guarded append -- the enforcement point flagged (independent Opus
// research pass, BRIEF-os-totebox-communication-complex-2026-09.md) as
// the single highest-value part of moving hold/retention/disposition
// into worm-ledger: today the "unknown retention class is a hard 400"
// check lives only in service-fs's own http.rs, which means each of
// the new "system of record" services would have to remember to
// re-implement it themselves -- exactly the kind of guarantee that
// must be structural, not a convention every future author has to
// recall. `GuardedLedger` wraps a `LedgerBackend` + `RetentionVocabulary`
// so that guarantee lives once, in the library.

use crate::ledger::{LedgerBackend, LedgerError};
use crate::retention::RetentionVocabulary;

#[derive(Debug)]
pub enum GuardError {
    /// The declared retention class isn't in the loaded vocabulary.
    /// Advisory-at-load, strict-at-use (per retention.rs's own
    /// module doc) -- this is the "strict-at-use" half.
    UnknownRetentionClass(String),
    Ledger(LedgerError),
}

impl std::fmt::Display for GuardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GuardError::UnknownRetentionClass(c) => {
                write!(f, "unknown retention class {c:?} — refusing append")
            }
            GuardError::Ledger(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for GuardError {}

impl From<LedgerError> for GuardError {
    fn from(e: LedgerError) -> Self {
        GuardError::Ledger(e)
    }
}

/// A `LedgerBackend` wrapper that refuses to append a record declaring
/// an unrecognized retention class.
///
/// Does NOT check holds at append time — a hold gates *disposal*, not
/// creation; placing a hold never blocks a new record from being
/// written, it only blocks that record from later being deleted (see
/// `disposition::decide`, which already checks this at disposal time
/// via `hold::is_on_hold`). A service building a disposal/execute path
/// must call `hold::is_on_hold` itself before appending a
/// disposition-execute record that results in an actual blob
/// deletion — this wrapper covers the append-time class-validation
/// guarantee only, deliberately not a second copy of the disposal
/// gate `disposition.rs` already owns.
pub struct GuardedLedger<'a> {
    inner: &'a dyn LedgerBackend,
    vocab: &'a RetentionVocabulary,
}

impl<'a> GuardedLedger<'a> {
    pub fn new(inner: &'a dyn LedgerBackend, vocab: &'a RetentionVocabulary) -> Self {
        Self { inner, vocab }
    }

    /// Append a record that declares a retention class. Every "system
    /// of record" service's write path should call this, not
    /// `LedgerBackend::append` directly, so an unknown class can never
    /// silently become a permanent, un-disposable ledger entry.
    pub fn append_with_retention(
        &self,
        payload_id: &str,
        payload: &serde_json::Value,
        retention_class: &str,
    ) -> Result<u64, GuardError> {
        if self.vocab.get(retention_class).is_none() {
            return Err(GuardError::UnknownRetentionClass(
                retention_class.to_string(),
            ));
        }
        Ok(self.inner.append(payload_id, payload)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ledger::InMemoryLedger;
    use crate::retention::load_retention_classes;
    use std::io::Write;

    fn tmp_vocab_csv() -> std::path::PathBuf {
        static CTR: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let n = CTR.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let path = std::env::temp_dir().join(format!("worm-ledger-guard-test-{}-{}.csv", std::process::id(), n));
        let mut f = std::fs::File::create(&path).unwrap();
        f.write_all(b"class,duration_days,regime,notes\nfinra-6y,2192,FINRA,x\n")
            .unwrap();
        path
    }

    fn tmp_ledger_dir() -> std::path::PathBuf {
        static CTR: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let n = CTR.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let dir = std::env::temp_dir().join(format!("worm-ledger-guard-ledger-{}-{}", std::process::id(), n));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn known_class_appends_successfully() {
        let vocab = load_retention_classes(tmp_vocab_csv().to_str().unwrap());
        let ledger = InMemoryLedger::open(tmp_ledger_dir(), "test").unwrap();
        let guarded = GuardedLedger::new(&ledger, &vocab);
        let cursor = guarded
            .append_with_retention("p1", &serde_json::json!({"x": 1}), "finra-6y")
            .unwrap();
        assert_eq!(cursor, 1);
    }

    #[test]
    fn unknown_class_is_rejected_before_touching_the_ledger() {
        let vocab = load_retention_classes(tmp_vocab_csv().to_str().unwrap());
        let ledger = InMemoryLedger::open(tmp_ledger_dir(), "test").unwrap();
        let guarded = GuardedLedger::new(&ledger, &vocab);
        let result = guarded.append_with_retention("p1", &serde_json::json!({"x": 1}), "no-such-class");
        assert!(matches!(result, Err(GuardError::UnknownRetentionClass(c)) if c == "no-such-class"));
        // Nothing was appended.
        assert_eq!(ledger.read_since(0).unwrap().len(), 0);
    }
}
