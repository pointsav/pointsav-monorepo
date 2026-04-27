//! Apex history + post-handover invariant ("only P-new accepted from
//! N+3+" per convention §4). Skeleton — implementation lands per
//! task #11.

/// One apex identity in the ledger's history.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApexEntry {
    pub name: String,
    pub pubkey: [u8; 32],
    /// Ledger height at which this apex took effect.
    pub effective_from: u64,
    /// `Some(h)` once a handover to a new apex has been recorded;
    /// signatures from this apex MUST be refused on checkpoints at
    /// or above height `h + 1` (per §4 N+3+ invariant: handover at
    /// N+2 → N+3 onward only-new).
    pub effective_until: Option<u64>,
}

/// Append-only history of apex identities. The current apex is the
/// most recent entry with `effective_until = None`; before applying
/// a handover, the kernel verifier consults this history.
pub struct ApexHistory {
    entries: Vec<ApexEntry>,
}

impl ApexHistory {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn current(&self) -> Option<&ApexEntry> {
        self.entries
            .iter()
            .rev()
            .find(|e| e.effective_until.is_none())
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

impl Default for ApexHistory {
    fn default() -> Self {
        Self::new()
    }
}
