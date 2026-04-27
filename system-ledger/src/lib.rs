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

use std::collections::HashSet;
use system_core::{Capability, Hash256, SignedCheckpoint, WitnessRecord};
use sha2::{Digest, Sha256};

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

    /// Record a witness record as having been logged in the ledger.
    /// This is a precondition for [`consult_capability`] to honor
    /// a witness-extension verdict — per Mechanism A the witness's
    /// hash MUST appear in the current Merkle root before the
    /// kernel honors the extension. The substrate-tier consumer
    /// calls this when an apex-cosigned witness entry has been
    /// committed to the WORM ledger.
    fn apply_witness_record(&mut self, record: WitnessRecord) -> Result<(), LedgerError>;
}

/// In-memory [`LedgerConsumer`] for v0.1.x. Single-writer; not
/// thread-safe — matches the kernel-side single-threaded substrate
/// model. Future MINOR may add Mutex wrapping for shared deployments
/// or a `MoonshotDatabaseLedger` impl backed by `moonshot-database`.
pub struct InMemoryLedger {
    pub cache: cache::CheckpointCache,
    pub revocations: revocation::RevocationSet,
    pub apex: apex::ApexHistory,
    /// Hashes of witness records known to be in the current ledger
    /// Merkle root. Per Mechanism A, the kernel verifier honors a
    /// witness extension only if the record's hash appears in the
    /// current root. v0.1.x relies on the consumer to call
    /// [`apply_witness_record`] when an entry lands; future MINOR
    /// will replace this with a Merkle inclusion-proof check
    /// against [`SignedCheckpoint`] once
    /// [`system-core`] gains the proof machinery.
    witnessed: HashSet<Hash256>,
    /// Identity used in the `allowed_signers` lookup for witness
    /// signature verification. v0.1.x: a fixed `"witness"` label;
    /// the actual binding is via the SSH-format pubkey carried on
    /// the [`Capability`].
    witness_identity: String,
}

impl InMemoryLedger {
    pub fn new() -> Self {
        Self {
            cache: cache::CheckpointCache::with_capacity(64),
            revocations: revocation::RevocationSet::new(),
            apex: apex::ApexHistory::new(),
            witnessed: HashSet::new(),
            witness_identity: "witness".to_string(),
        }
    }

    /// Hash a witness record to its canonical 32-byte identity.
    /// SHA-256 of the JSON serialisation; matches the
    /// `Capability::hash` discipline.
    fn witness_record_hash(record: &WitnessRecord) -> Hash256 {
        let bytes = serde_json::to_vec(record).expect("WitnessRecord serializable");
        let mut hasher = Sha256::new();
        hasher.update(&bytes);
        hasher.finalize().into()
    }

    fn is_witness_logged(&self, record: &WitnessRecord) -> bool {
        self.witnessed.contains(&Self::witness_record_hash(record))
    }
}

impl Default for InMemoryLedger {
    fn default() -> Self {
        Self::new()
    }
}

impl LedgerConsumer for InMemoryLedger {
    fn consult_capability(
        &self,
        cap: &Capability,
        current_root: &SignedCheckpoint,
        now: u64,
        witness: Option<&WitnessRecord>,
    ) -> Result<Verdict, ConsultError> {
        // Step 1 — verify current_root is signed by the apex(es)
        // valid at its tree_size.
        let height = current_root.checkpoint.tree_size;
        let apex_verdict = self.apex.check_height(height);
        let apex_ok = match &apex_verdict {
            apex::ApexVerdict::NoApex => {
                return Ok(Verdict::Refuse(RefuseReason::ApexInvalid));
            }
            apex::ApexVerdict::Single { apex } => current_root
                .verify_signer(&apex.name, &apex.pubkey)
                .map_err(|e| {
                    ConsultError::InconsistentState(format!("verify_signer: {e:?}"))
                })?,
            apex::ApexVerdict::Handover { old_apex, new_apex } => current_root
                .verify_apex_handover(
                    &old_apex.name,
                    &old_apex.pubkey,
                    &new_apex.name,
                    &new_apex.pubkey,
                )
                .map_err(|e| {
                    ConsultError::InconsistentState(format!("verify_handover: {e:?}"))
                })?,
        };
        if !apex_ok {
            return Ok(Verdict::Refuse(RefuseReason::StaleApex));
        }

        // Step 2 — revocation check.
        let cap_hash = cap.hash();
        if self.revocations.contains(&cap_hash) {
            return Ok(Verdict::Refuse(RefuseReason::Revoked));
        }

        // Step 3 — expiry check.
        match cap.expiry_t {
            None => return Ok(Verdict::Allow),
            Some(t) if now < t => return Ok(Verdict::Allow),
            Some(_) => {} // past expiry; fall through to step 4
        }

        // Step 4 — witness extension path.
        let witness = match witness {
            Some(w) => w,
            None => return Ok(Verdict::Refuse(RefuseReason::Expired)),
        };
        let witness_pubkey = match cap.witness_pubkey.as_deref() {
            Some(pk) => pk,
            None => return Ok(Verdict::Refuse(RefuseReason::NotExtensible)),
        };
        // Witness must bind THIS capability.
        if witness.capability_hash != cap_hash {
            return Ok(Verdict::Refuse(RefuseReason::WitnessSignatureInvalid));
        }
        // Witness must extend (not retract) expiry.
        if let Some(prev_expiry) = cap.expiry_t {
            if witness.new_expiry_t <= prev_expiry {
                return Ok(Verdict::Refuse(RefuseReason::WitnessSignatureInvalid));
            }
        }
        // Witness must have been logged in the ledger.
        if !self.is_witness_logged(witness) {
            return Ok(Verdict::Refuse(RefuseReason::WitnessNotInLedger));
        }
        // Verify the SSH signature over (capability_hash || new_expiry_t.to_be_bytes()).
        let mut payload = Vec::with_capacity(40);
        payload.extend_from_slice(&witness.capability_hash);
        payload.extend_from_slice(&witness.new_expiry_t.to_be_bytes());
        let sig_ok = witness::verify_witness_signature(
            &witness.signature,
            &payload,
            witness_pubkey,
            &self.witness_identity,
        )
        .map_err(|e| ConsultError::WitnessVerifyFailed(format!("{e:?}")))?;
        if !sig_ok {
            return Ok(Verdict::Refuse(RefuseReason::WitnessSignatureInvalid));
        }
        Ok(Verdict::ExtendThenAllow {
            new_expiry_t: witness.new_expiry_t,
        })
    }

    fn apply_revocation(&mut self, event: revocation::RevocationEvent) -> Result<(), LedgerError> {
        self.revocations.apply_revocation(event);
        Ok(())
    }

    fn apply_apex_handover(
        &mut self,
        old_apex_name: &str,
        old_apex_pubkey: &[u8; 32],
        new_apex_name: &str,
        new_apex_pubkey: &[u8; 32],
        handover_checkpoint: &SignedCheckpoint,
    ) -> Result<(), LedgerError> {
        // Verify the handover checkpoint carries both signatures.
        let ok = handover_checkpoint
            .verify_apex_handover(
                old_apex_name,
                old_apex_pubkey,
                new_apex_name,
                new_apex_pubkey,
            )
            .map_err(|e| LedgerError::InvalidHandover(format!("verify_apex_handover: {e:?}")))?;
        if !ok {
            return Err(LedgerError::InvalidHandover(
                "handover checkpoint does not carry both apex signatures".to_string(),
            ));
        }
        // Apply to apex history.
        let height = handover_checkpoint.checkpoint.tree_size;
        self.apex
            .apply_handover(old_apex_pubkey, new_apex_name, *new_apex_pubkey, height)
            .map_err(|e| LedgerError::InvalidHandover(format!("apex.apply_handover: {e:?}")))?;
        // Cache the handover checkpoint for fast subsequent lookup.
        self.cache.insert(handover_checkpoint.clone());
        Ok(())
    }

    fn apply_witness_record(&mut self, record: WitnessRecord) -> Result<(), LedgerError> {
        let h = Self::witness_record_hash(&record);
        self.witnessed.insert(h);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::{Signer, SigningKey};
    use system_core::{
        Capability, CapabilityType, Checkpoint, LedgerAnchor, NoteSignature, Right,
        SignedCheckpoint, WitnessRecord,
    };

    fn keypair(seed: u8) -> (SigningKey, [u8; 32]) {
        let sk = SigningKey::from_bytes(&[seed; 32]);
        let pk = sk.verifying_key().to_bytes();
        (sk, pk)
    }

    fn fixture_anchor() -> LedgerAnchor {
        LedgerAnchor {
            origin: "foundry.test.cap-ledger".to_string(),
            tree_size: 1,
            root_hash: [0xAA; 32],
        }
    }

    fn fixture_capability(expiry_t: Option<u64>) -> Capability {
        Capability {
            cap_type: CapabilityType::Endpoint,
            rights: vec![Right::Invoke],
            expiry_t,
            witness_pubkey: None,
            ledger_anchor: fixture_anchor(),
        }
    }

    fn signed_checkpoint(
        tree_size: u64,
        root_byte: u8,
        signers: &[(&str, &SigningKey)],
    ) -> SignedCheckpoint {
        let cp = Checkpoint {
            origin: "foundry.test.cap-ledger".to_string(),
            tree_size,
            root_hash: [root_byte; 32],
            extensions: vec![],
        };
        let body = cp.body_bytes();
        let signatures = signers
            .iter()
            .map(|(name, sk)| {
                let pk = sk.verifying_key().to_bytes();
                let key_hash = NoteSignature::derive_key_hash(name, &pk);
                let sig = sk.sign(&body).to_bytes();
                NoteSignature {
                    signer_name: name.to_string(),
                    key_hash,
                    signature: sig,
                }
            })
            .collect();
        SignedCheckpoint {
            checkpoint: cp,
            signatures,
        }
    }

    fn ledger_with_genesis(apex_name: &str, sk: &SigningKey) -> InMemoryLedger {
        let mut ledger = InMemoryLedger::new();
        let pk = sk.verifying_key().to_bytes();
        ledger.apex.record_genesis(apex_name, pk, 0).unwrap();
        ledger
    }

    #[test]
    fn no_apex_refuses_with_apex_invalid() {
        let ledger = InMemoryLedger::new();
        let cap = fixture_capability(None);
        let (sk, _pk) = keypair(0x11);
        let root = signed_checkpoint(0, 0xAA, &[("apex", &sk)]);
        let v = ledger.consult_capability(&cap, &root, 1000, None).unwrap();
        assert_eq!(v, Verdict::Refuse(RefuseReason::ApexInvalid));
    }

    #[test]
    fn unsigned_root_refuses_with_stale_apex() {
        let (sk_apex, _pk_apex) = keypair(0x11);
        let (sk_other, _pk_other) = keypair(0x22);
        let ledger = ledger_with_genesis("apex", &sk_apex);
        let cap = fixture_capability(None);
        // Sign root under "apex" name but with the WRONG key — key_hash
        // won't match → verify_signer returns false → StaleApex.
        let root = signed_checkpoint(5, 0xAA, &[("apex", &sk_other)]);
        let v = ledger.consult_capability(&cap, &root, 1000, None).unwrap();
        assert_eq!(v, Verdict::Refuse(RefuseReason::StaleApex));
    }

    #[test]
    fn signed_root_with_no_expiry_allows() {
        let (sk_apex, _pk) = keypair(0x11);
        let ledger = ledger_with_genesis("apex", &sk_apex);
        let cap = fixture_capability(None);
        let root = signed_checkpoint(5, 0xAA, &[("apex", &sk_apex)]);
        let v = ledger.consult_capability(&cap, &root, 1000, None).unwrap();
        assert_eq!(v, Verdict::Allow);
    }

    #[test]
    fn revoked_capability_refuses() {
        let (sk_apex, _pk) = keypair(0x11);
        let mut ledger = ledger_with_genesis("apex", &sk_apex);
        let cap = fixture_capability(None);
        ledger
            .apply_revocation(revocation::RevocationEvent {
                capability_hash: cap.hash(),
                revoked_at: 999,
                signed_by: "apex".to_string(),
                ledger_height: 4,
            })
            .unwrap();
        let root = signed_checkpoint(5, 0xAA, &[("apex", &sk_apex)]);
        let v = ledger.consult_capability(&cap, &root, 1000, None).unwrap();
        assert_eq!(v, Verdict::Refuse(RefuseReason::Revoked));
    }

    #[test]
    fn unexpired_capability_with_expiry_allows() {
        let (sk_apex, _pk) = keypair(0x11);
        let ledger = ledger_with_genesis("apex", &sk_apex);
        let cap = fixture_capability(Some(2000));
        let root = signed_checkpoint(5, 0xAA, &[("apex", &sk_apex)]);
        let v = ledger.consult_capability(&cap, &root, 1000, None).unwrap();
        assert_eq!(v, Verdict::Allow);
    }

    #[test]
    fn expired_capability_no_witness_refuses() {
        let (sk_apex, _pk) = keypair(0x11);
        let ledger = ledger_with_genesis("apex", &sk_apex);
        let cap = fixture_capability(Some(500));
        let root = signed_checkpoint(5, 0xAA, &[("apex", &sk_apex)]);
        let v = ledger.consult_capability(&cap, &root, 1000, None).unwrap();
        assert_eq!(v, Verdict::Refuse(RefuseReason::Expired));
    }

    #[test]
    fn expired_capability_no_witness_pubkey_refuses_not_extensible() {
        let (sk_apex, _pk) = keypair(0x11);
        let ledger = ledger_with_genesis("apex", &sk_apex);
        let mut cap = fixture_capability(Some(500));
        cap.witness_pubkey = None; // explicit
        let witness = WitnessRecord {
            capability_hash: cap.hash(),
            new_expiry_t: 2000,
            signature: vec![0; 100], // doesn't matter; refused before sig check
        };
        let root = signed_checkpoint(5, 0xAA, &[("apex", &sk_apex)]);
        let v = ledger
            .consult_capability(&cap, &root, 1000, Some(&witness))
            .unwrap();
        assert_eq!(v, Verdict::Refuse(RefuseReason::NotExtensible));
    }

    #[test]
    fn witness_with_wrong_cap_hash_refuses() {
        let (sk_apex, _pk) = keypair(0x11);
        let ledger = ledger_with_genesis("apex", &sk_apex);
        let mut cap = fixture_capability(Some(500));
        cap.witness_pubkey = Some("ssh-ed25519 AAAA".to_string());
        let witness = WitnessRecord {
            capability_hash: [0xFF; 32], // doesn't match cap.hash()
            new_expiry_t: 2000,
            signature: vec![],
        };
        let root = signed_checkpoint(5, 0xAA, &[("apex", &sk_apex)]);
        let v = ledger
            .consult_capability(&cap, &root, 1000, Some(&witness))
            .unwrap();
        assert_eq!(v, Verdict::Refuse(RefuseReason::WitnessSignatureInvalid));
    }

    #[test]
    fn witness_not_logged_refuses() {
        let (sk_apex, _pk) = keypair(0x11);
        let ledger = ledger_with_genesis("apex", &sk_apex);
        let mut cap = fixture_capability(Some(500));
        cap.witness_pubkey = Some("ssh-ed25519 AAAA".to_string());
        let witness = WitnessRecord {
            capability_hash: cap.hash(),
            new_expiry_t: 2000,
            signature: vec![],
        };
        let root = signed_checkpoint(5, 0xAA, &[("apex", &sk_apex)]);
        // Witness has NOT been registered via apply_witness_record.
        let v = ledger
            .consult_capability(&cap, &root, 1000, Some(&witness))
            .unwrap();
        assert_eq!(v, Verdict::Refuse(RefuseReason::WitnessNotInLedger));
    }

    #[test]
    fn witness_extending_to_earlier_expiry_refuses() {
        let (sk_apex, _pk) = keypair(0x11);
        let mut ledger = ledger_with_genesis("apex", &sk_apex);
        let mut cap = fixture_capability(Some(1500));
        cap.witness_pubkey = Some("ssh-ed25519 AAAA".to_string());
        let witness = WitnessRecord {
            capability_hash: cap.hash(),
            new_expiry_t: 1000, // EARLIER than current expiry
            signature: vec![],
        };
        ledger.apply_witness_record(witness.clone()).unwrap();
        let root = signed_checkpoint(5, 0xAA, &[("apex", &sk_apex)]);
        let v = ledger
            .consult_capability(&cap, &root, 2000, Some(&witness))
            .unwrap();
        assert_eq!(v, Verdict::Refuse(RefuseReason::WitnessSignatureInvalid));
    }

    #[test]
    fn apex_handover_application_succeeds() {
        let (sk_old, pk_old) = keypair(0x11);
        let (_sk_new, pk_new) = keypair(0x22);
        let mut ledger = ledger_with_genesis("apex-old", &sk_old);
        // Sign handover checkpoint at height 100 with both keys.
        let (sk_old_clone, _) = keypair(0x11);
        let (sk_new_again, _) = keypair(0x22);
        let handover = signed_checkpoint(
            100,
            0xCD,
            &[("apex-old", &sk_old_clone), ("apex-new", &sk_new_again)],
        );
        let r = ledger.apply_apex_handover(
            "apex-old",
            &pk_old,
            "apex-new",
            &pk_new,
            &handover,
        );
        assert!(r.is_ok());
        // Current apex is now apex-new.
        let cur = ledger.apex.current().unwrap();
        assert_eq!(cur.name, "apex-new");
    }

    #[test]
    fn apex_handover_with_one_missing_signature_refused() {
        let (sk_old, pk_old) = keypair(0x11);
        let (_sk_new, pk_new) = keypair(0x22);
        let mut ledger = ledger_with_genesis("apex-old", &sk_old);
        // Handover checkpoint signed ONLY by apex-old (missing apex-new).
        let (sk_old_clone, _) = keypair(0x11);
        let handover = signed_checkpoint(100, 0xCD, &[("apex-old", &sk_old_clone)]);
        let r = ledger.apply_apex_handover(
            "apex-old",
            &pk_old,
            "apex-new",
            &pk_new,
            &handover,
        );
        assert!(matches!(r, Err(LedgerError::InvalidHandover(_))));
    }

    #[test]
    fn full_handover_ceremony_end_to_end() {
        // Inbox brief Phase 1A item 4 — END-TO-END ceremony.
        // Synthesize deployment, append revocation entry by P-old,
        // append checkpoint with both P-old + P-new sigs, verify
        // kernel verifier accepts the handover, verify subsequent
        // checkpoints require only P-new (P-old at N+3 REFUSED).
        let (sk_old, pk_old) = keypair(0x11);
        let (sk_new, pk_new) = keypair(0x22);
        let mut ledger = ledger_with_genesis("apex-old", &sk_old);

        // Pre-handover: checkpoint at height 50 signed by P-old works
        // for capability consultation.
        let (sk_old_clone1, _) = keypair(0x11);
        let pre_root = signed_checkpoint(50, 0xAA, &[("apex-old", &sk_old_clone1)]);
        let cap = fixture_capability(None);
        assert_eq!(
            ledger
                .consult_capability(&cap, &pre_root, 100, None)
                .unwrap(),
            Verdict::Allow
        );

        // Append revocation entry by P-old (a separate revocation
        // for some unrelated capability — this just exercises the
        // revocation API as part of the ceremony).
        ledger
            .apply_revocation(revocation::RevocationEvent {
                capability_hash: [0xEE; 32],
                revoked_at: 100,
                signed_by: "apex-old".to_string(),
                ledger_height: 99,
            })
            .unwrap();

        // Apply the handover at height 100.
        let (sk_old_clone2, _) = keypair(0x11);
        let (sk_new_clone, _) = keypair(0x22);
        let handover = signed_checkpoint(
            100,
            0xCD,
            &[("apex-old", &sk_old_clone2), ("apex-new", &sk_new_clone)],
        );
        ledger
            .apply_apex_handover("apex-old", &pk_old, "apex-new", &pk_new, &handover)
            .unwrap();

        // Handover checkpoint (height 100) consults successfully.
        assert_eq!(
            ledger
                .consult_capability(&cap, &handover, 200, None)
                .unwrap(),
            Verdict::Allow
        );

        // Subsequent checkpoint (height 101) signed ONLY by P-new — accepted.
        let post_new = signed_checkpoint(101, 0xDD, &[("apex-new", &sk_new)]);
        assert_eq!(
            ledger
                .consult_capability(&cap, &post_new, 300, None)
                .unwrap(),
            Verdict::Allow
        );

        // Subsequent checkpoint (height 101) signed ONLY by P-old —
        // MUST BE REFUSED. This is the §4 N+3+ invariant.
        let post_old = signed_checkpoint(101, 0xDE, &[("apex-old", &sk_old)]);
        assert_eq!(
            ledger
                .consult_capability(&cap, &post_old, 300, None)
                .unwrap(),
            Verdict::Refuse(RefuseReason::StaleApex)
        );
    }
}

