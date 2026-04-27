//! system-core — substrate primitives for The Capability Ledger Substrate.
//!
//! Implements [`Capability`] per Doctrine claim #33 and [`WitnessRecord`]
//! per Mechanism A (Time-Bound Capabilities). Specification:
//! `~/Foundry/conventions/system-substrate-doctrine.md` §3.1 + §5.
//!
//! The [`checkpoint`] submodule implements the C2SP signed-note
//! checkpoint primitive (apex-cosigning per convention §4). The
//! [`inclusion_proof`] submodule implements RFC 9162 v2 Merkle
//! inclusion proofs (compatible with C2SP tlog-tiles).

pub mod checkpoint;
pub mod inclusion_proof;
pub use checkpoint::{Checkpoint, NoteSignature, SignedCheckpoint};
pub use inclusion_proof::{
    rfc9162_internal_hash, rfc9162_leaf_hash, InclusionProof, InclusionVerifyError,
};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// What the capability authorises. Open variant set per convention §5.1;
/// additions land via doctrine MINOR.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CapabilityType {
    Endpoint,
    Memory,
    Irq,
    Notification,
    CNode,
}

/// Permitted operations. Composable; `grant` and `revoke` are
/// kernel-mediated authority transfers per seL4 semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Right {
    Read,
    Write,
    Invoke,
    Grant,
    Revoke,
}

/// SHA-256 baseline per `worm-ledger-design.md` §3 D3. Algorithm-agility
/// is a future MINOR.
pub type Hash256 = [u8; 32];

/// Anchor into the customer-rooted Merkle log. References a C2SP
/// signed-note checkpoint by tree size + root hash.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LedgerAnchor {
    /// C2SP signed-note origin (e.g., `foundry.<module-id>.capability-ledger`).
    pub origin: String,
    /// Tree size at which this capability was anchored.
    pub tree_size: u64,
    /// Merkle root hash at `tree_size`.
    pub root_hash: Hash256,
}

/// Kernel-mediated authorisation token, ledger-bound. Per Doctrine
/// claim #33 the kernel verifies this capability against the current
/// Merkle root before honoring an invocation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Capability {
    pub cap_type: CapabilityType,
    pub rights: Vec<Right>,
    /// Seconds since UNIX epoch (UTC). `None` = no built-in expiry
    /// (seL4 default). `Some(t)` = MUST NOT be honored after `t`
    /// unless extended by a [`WitnessRecord`] per Mechanism A.
    pub expiry_t: Option<u64>,
    /// SSH-format public key authorised to extend `expiry_t`.
    /// `None` = non-extensible (pure time-bound, no witness model).
    pub witness_pubkey: Option<String>,
    pub ledger_anchor: LedgerAnchor,
}

impl Capability {
    /// Canonical SHA-256 hash for witness-record binding. v0.1.x:
    /// serde JSON; future MINOR may swap to canonical CBOR per
    /// `worm-ledger-design.md` §3 D3.
    pub fn hash(&self) -> Hash256 {
        let bytes = serde_json::to_vec(self).expect("Capability serializable");
        let mut hasher = Sha256::new();
        hasher.update(&bytes);
        hasher.finalize().into()
    }
}

/// Extends a capability past `expiry_t`. Per Mechanism A (convention
/// §5.1): the kernel MUST verify both (a) `signature` against the
/// capability's `witness_pubkey` AND (b) that this record's hash
/// appears in the current ledger Merkle root before extending.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WitnessRecord {
    pub capability_hash: Hash256,
    /// MUST be greater than the previous `expiry_t`.
    pub new_expiry_t: u64,
    /// `ssh-keygen -Y sign` over `(capability_hash || new_expiry_t.to_be_bytes())`.
    /// Namespace tag: `capability-witness-v1`.
    pub signature: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture_anchor() -> LedgerAnchor {
        LedgerAnchor {
            origin: "foundry.test.capability-ledger".to_string(),
            tree_size: 42,
            root_hash: [0xAA; 32],
        }
    }

    fn fixture_capability() -> Capability {
        Capability {
            cap_type: CapabilityType::Endpoint,
            rights: vec![Right::Invoke, Right::Read],
            expiry_t: Some(1_730_000_000),
            witness_pubkey: Some("ssh-ed25519 AAAA...".to_string()),
            ledger_anchor: fixture_anchor(),
        }
    }

    #[test]
    fn capability_serialises_round_trip() {
        let cap = fixture_capability();
        let json = serde_json::to_string(&cap).unwrap();
        let restored: Capability = serde_json::from_str(&json).unwrap();
        assert_eq!(cap, restored);
    }

    #[test]
    fn capability_hash_is_deterministic() {
        let cap = fixture_capability();
        assert_eq!(cap.hash(), cap.hash());
    }

    #[test]
    fn capability_hash_changes_with_expiry() {
        let cap1 = fixture_capability();
        let mut cap2 = fixture_capability();
        cap2.expiry_t = Some(1_730_000_001);
        assert_ne!(cap1.hash(), cap2.hash());
    }

    #[test]
    fn capability_hash_changes_with_anchor() {
        let cap1 = fixture_capability();
        let mut cap2 = fixture_capability();
        cap2.ledger_anchor.tree_size = 43;
        assert_ne!(cap1.hash(), cap2.hash());
    }

    #[test]
    fn witness_record_serialises_round_trip() {
        let wr = WitnessRecord {
            capability_hash: [0xBB; 32],
            new_expiry_t: 1_731_000_000,
            signature: vec![0x01, 0x02, 0x03],
        };
        let json = serde_json::to_string(&wr).unwrap();
        let restored: WitnessRecord = serde_json::from_str(&json).unwrap();
        assert_eq!(wr, restored);
    }

    #[test]
    fn ledger_anchor_serialises_round_trip() {
        let a = fixture_anchor();
        let json = serde_json::to_string(&a).unwrap();
        let restored: LedgerAnchor = serde_json::from_str(&json).unwrap();
        assert_eq!(a, restored);
    }
}
