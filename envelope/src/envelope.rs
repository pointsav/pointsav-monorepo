// SPDX-License-Identifier: Apache-2.0

//! The Envelope record itself.
//!
//! Per `BRIEF-os-totebox-communication-complex-2026-09.md` decision 8:
//! one versioned, signed schema for "an assertion about entities at a
//! time, with source, observed-vs-attested state, and lineage."
//! Unifies three independently-invented partial schemes already in
//! this codebase: `service-people`'s Person/Anchor/Claim model
//! (`state`/`confidence`/`source` below mirror Anchor/Claim's
//! observed-vs-attested + confidence + source shape), the
//! apprenticeship-corpus (brief/attempt/verdict) triple, and the
//! training-pipeline provenance-header tuple (`tenant`/`created`
//! below mirror its provenance fields).
//!
//! **Signed-bytes discipline** (decision 9's "never re-serialize a
//! signed Envelope" rule, which starts here, not just at the wire
//! layer): [`Envelope::canonical_bytes`] produces the exact bytes that
//! get signed and that a verifier must re-derive identically.
//! Once an `Envelope` has been signed, store and forward
//! `canonical_bytes()`'s output verbatim alongside it (or the whole
//! signed struct, re-encoded via the *same* canonical encoding) --
//! never a value round-tripped through a different encoder or a
//! mutated copy, or the signature silently stops verifying.
//!
//! **Known, deliberate scope limit for Phase 0:** true forward-
//! compatible unknown-field retention (decision 9's "a v2 Envelope
//! passing through a v1 node re-emits byte-identically") is NOT yet
//! implemented here -- this version's `canonical_bytes` round-trips
//! only the fields this struct itself knows about. That refinement is
//! `totebox-protocol`'s problem to solve once there's a real second
//! version to be compatible with (solving it before that exists is
//! exactly the kind of premature abstraction this workspace's own
//! conventions warn against) -- tracked in NEXT.md, not silently
//! dropped.

use serde::{Deserialize, Serialize};

use crate::signing::{sign_envelope, verify_envelope_signature, SigningError, VerifyError};

/// Whether an Envelope's payload was merely observed (recorded without
/// asserting it belongs to/was authored by anyone in particular) or
/// attested (a specific person, identified in `attested_by`, has
/// signed off on it). Mirrors `service-people`'s Anchor (observed) vs.
/// Claim (attested, with confidence + source) distinction, generalized
/// to any entity/record type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EnvelopeState {
    Observed,
    Attested,
}

/// Which adapter produced this Envelope, and that adapter's own
/// reference for the original record (e.g. an email Message-ID, a
/// CalDAV UID, a file path) -- for tracing a record back through the
/// `system-protocol-*` adapter that ingested it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Source {
    pub adapter: String,
    pub origin_ref: String,
}

/// One DataGraph entity this Envelope references, and the role it
/// plays in this specific record (e.g. "sender", "attendee", "payee",
/// "location") -- role is an edge property, not a per-service field,
/// per the BRIEF's framing of why this avoids every service
/// re-inventing Person/Place/Time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityRef {
    pub entity_id: String,
    pub role: String,
}

/// Who attested this record, and their signature over it. Required
/// (by convention, not enforced at the type level -- see
/// `Envelope::validate`) when `state == Attested`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttestedBy {
    pub person_id: String,
    /// PEM-armored SSHSIG signature bytes (base64 text, not raw
    /// binary) over this Envelope's `canonical_bytes()` -- the
    /// person's own attestation, distinct from `Envelope::signature`
    /// (which is the producing adapter/service's signature over the
    /// whole record, always present; this one is the human sign-off,
    /// present only for attested records).
    pub signature: String,
}

/// The record's actual content. `body` is deliberately
/// `serde_json::Value` (not a typed enum) at this layer -- per-service
/// `payload.type` sub-schemas are defined where each service is
/// built, not centralized here (see decision 15: ingestion, and by
/// extension payload shape, is per-service, converging only on this
/// Envelope wrapper).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payload {
    #[serde(rename = "type")]
    pub payload_type: String,
    pub body: serde_json::Value,
}

/// An attachment stored as a hash pointer, never inline -- decision 7:
/// large media/attachments go to content-addressed blob storage
/// (`worm_ledger::blob`), the ledger/Envelope only carries the
/// pointer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachmentRef {
    pub hash: String,
    pub size: u64,
    pub mime: String,
    pub blob_ref: String,
}

/// Version-history links: what this Envelope supersedes (a strict
/// replacement) and/or what it was derived from (a computed/composed
/// view over other Envelopes, not necessarily replacing them).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Lineage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supersedes: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub derived_from: Vec<String>,
}

/// The Envelope itself. See the module doc for the full rationale.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Envelope {
    /// ULID, assigned at creation. Caller-supplied (this crate doesn't
    /// generate IDs itself, to avoid a ulid-crate dependency for
    /// Phase 0 -- any ULID-shaped string is accepted).
    pub envelope_id: String,
    /// Semver string, e.g. `"0.1.0"`. This crate's own schema version,
    /// not the producing service's version.
    pub envelope_version: String,
    pub tenant: String,
    pub created: chrono::DateTime<chrono::Utc>,
    pub source: Source,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entities: Vec<EntityRef>,
    pub state: EnvelopeState,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attested_by: Option<AttestedBy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f64>,
    pub payload: Payload,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attachments: Vec<AttachmentRef>,
    #[serde(default, skip_serializing_if = "is_default_lineage")]
    pub lineage: Lineage,
    /// PEM-armored SSHSIG signature bytes (as a UTF-8 string, since
    /// the armored form is already text) over `canonical_bytes()` of
    /// every OTHER field -- see `Envelope::sign`/`verify`. Empty
    /// string for an as-yet-unsigned Envelope under construction;
    /// never empty on a stored/transmitted one.
    #[serde(default)]
    pub signature: String,
}

fn is_default_lineage(l: &Lineage) -> bool {
    l.supersedes.is_none() && l.derived_from.is_empty()
}

#[derive(Debug)]
pub enum EnvelopeError {
    /// `state == Attested` but `attested_by` is unset.
    MissingAttestation,
    Encoding(String),
    Signing(SigningError),
    Verify(VerifyError),
}

impl std::fmt::Display for EnvelopeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EnvelopeError::MissingAttestation => {
                write!(f, "state is Attested but attested_by is unset")
            }
            EnvelopeError::Encoding(e) => write!(f, "canonical encoding failed: {e}"),
            EnvelopeError::Signing(e) => write!(f, "{e}"),
            EnvelopeError::Verify(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for EnvelopeError {}

impl From<SigningError> for EnvelopeError {
    fn from(e: SigningError) -> Self {
        EnvelopeError::Signing(e)
    }
}

impl From<VerifyError> for EnvelopeError {
    fn from(e: VerifyError) -> Self {
        EnvelopeError::Verify(e)
    }
}

impl Envelope {
    /// The invariant `state == Attested` implies `attested_by.is_some()`
    /// is a convention, not structurally enforced by the type (an
    /// `Option` has to be `Option` for `Observed` records) -- this
    /// checks it explicitly. Call before `sign`/at deserialization
    /// boundaries.
    pub fn validate(&self) -> Result<(), EnvelopeError> {
        if self.state == EnvelopeState::Attested && self.attested_by.is_none() {
            return Err(EnvelopeError::MissingAttestation);
        }
        Ok(())
    }

    /// Canonical CBOR bytes of every field EXCEPT `signature` --
    /// what gets signed, and what a verifier must re-derive
    /// identically. Field order is this struct's declaration order
    /// (deterministic for a fixed schema version; see the module doc
    /// for the Phase-0 scope limit on cross-version unknown-field
    /// retention).
    pub fn canonical_bytes(&self) -> Result<Vec<u8>, EnvelopeError> {
        let mut unsigned = self.clone();
        unsigned.signature = String::new();
        let mut buf = Vec::new();
        ciborium::into_writer(&unsigned, &mut buf)
            .map_err(|e| EnvelopeError::Encoding(e.to_string()))?;
        Ok(buf)
    }

    /// Sign this Envelope in place under `signing_key_path`, setting
    /// `self.signature`. Validates first (an unattested-but-claimed-
    /// Attested envelope never gets signed).
    pub fn sign(&mut self, signing_key_path: &std::path::Path) -> Result<(), EnvelopeError> {
        self.validate()?;
        let payload = self.canonical_bytes()?;
        let sig = sign_envelope(signing_key_path, &payload)?;
        self.signature = String::from_utf8(sig)
            .map_err(|e| EnvelopeError::Encoding(format!("signature not UTF-8: {e}")))?;
        Ok(())
    }

    /// Verify this Envelope's `signature` under `ssh_pubkey`/
    /// `signer_identity`. Does not mutate `self`. Recomputes
    /// `canonical_bytes()` fresh (with `signature` cleared, matching
    /// what `sign` signed) -- so this only verifies correctly against
    /// bytes actually produced by this same struct's encoding, per
    /// the module doc's "never re-serialize a signed Envelope"
    /// discipline: a re-serialized-then-verified round trip on a
    /// future differently-shaped version would silently fail to
    /// verify, which is the intended fail-closed behavior, not a bug
    /// to work around here.
    pub fn verify(&self, ssh_pubkey: &str, signer_identity: &str) -> Result<bool, EnvelopeError> {
        let payload = self.canonical_bytes()?;
        Ok(verify_envelope_signature(
            self.signature.as_bytes(),
            &payload,
            ssh_pubkey,
            signer_identity,
        )?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;
    use tempfile::TempDir;

    fn make_keypair(dir: &TempDir, name: &str) -> (std::path::PathBuf, String) {
        let priv_path = dir.path().join(name);
        let pub_path = dir.path().join(format!("{name}.pub"));
        let status = Command::new("ssh-keygen")
            .args([
                "-t", "ed25519", "-f", priv_path.to_str().unwrap(), "-N", "", "-C",
                &format!("{name}@envelope-test"), "-q",
            ])
            .status()
            .unwrap();
        assert!(status.success());
        let pubkey = std::fs::read_to_string(&pub_path).unwrap().trim().to_string();
        (priv_path, pubkey)
    }

    fn sample_envelope(state: EnvelopeState) -> Envelope {
        Envelope {
            envelope_id: "01JAAAA0000000000000000000".to_string(),
            envelope_version: "0.1.0".to_string(),
            tenant: "woodfine".to_string(),
            created: chrono::DateTime::parse_from_rfc3339("2026-09-23T00:00:00Z")
                .unwrap()
                .with_timezone(&chrono::Utc),
            source: Source {
                adapter: "minutebook-manual".to_string(),
                origin_ref: "meeting-2026-09-23".to_string(),
            },
            entities: vec![EntityRef {
                entity_id: "person-jennifer".to_string(),
                role: "attendee".to_string(),
            }],
            state,
            attested_by: None,
            confidence: None,
            payload: Payload {
                payload_type: "minutebook.entry".to_string(),
                body: serde_json::json!({"summary": "quarterly review"}),
            },
            attachments: vec![],
            lineage: Lineage::default(),
            signature: String::new(),
        }
    }

    #[test]
    fn observed_envelope_validates_without_attestation() {
        let env = sample_envelope(EnvelopeState::Observed);
        assert!(env.validate().is_ok());
    }

    #[test]
    fn attested_envelope_without_attested_by_fails_validation() {
        let env = sample_envelope(EnvelopeState::Attested);
        assert!(matches!(env.validate(), Err(EnvelopeError::MissingAttestation)));
    }

    #[test]
    fn canonical_bytes_is_deterministic() {
        let env = sample_envelope(EnvelopeState::Observed);
        let b1 = env.canonical_bytes().unwrap();
        let b2 = env.canonical_bytes().unwrap();
        assert_eq!(b1, b2);
    }

    #[test]
    fn canonical_bytes_excludes_signature_field() {
        let mut env = sample_envelope(EnvelopeState::Observed);
        let before = env.canonical_bytes().unwrap();
        env.signature = "some-signature-bytes".to_string();
        let after = env.canonical_bytes().unwrap();
        assert_eq!(before, after, "signature field must not affect what's signed");
    }

    #[test]
    fn sign_then_verify_round_trips() {
        let dir = TempDir::new().unwrap();
        let (priv_path, pubkey) = make_keypair(&dir, "k1");
        let mut env = sample_envelope(EnvelopeState::Observed);

        env.sign(&priv_path).expect("sign succeeds");
        assert!(!env.signature.is_empty());

        let ok = env.verify(&pubkey, "k1@envelope-test").expect("verify call ok");
        assert!(ok, "freshly signed envelope must verify");
    }

    #[test]
    fn tampering_after_signing_fails_verification() {
        let dir = TempDir::new().unwrap();
        let (priv_path, pubkey) = make_keypair(&dir, "k2");
        let mut env = sample_envelope(EnvelopeState::Observed);
        env.sign(&priv_path).expect("sign succeeds");

        env.payload.body = serde_json::json!({"summary": "TAMPERED"});
        let ok = env.verify(&pubkey, "k2@envelope-test").expect("verify call ok");
        assert!(!ok, "tampered envelope must NOT verify");
    }

    #[test]
    fn sign_refuses_attested_without_attested_by() {
        let dir = TempDir::new().unwrap();
        let (priv_path, _pubkey) = make_keypair(&dir, "k3");
        let mut env = sample_envelope(EnvelopeState::Attested);
        let result = env.sign(&priv_path);
        assert!(matches!(result, Err(EnvelopeError::MissingAttestation)));
    }

    #[test]
    fn attested_envelope_with_attested_by_signs_and_verifies() {
        let dir = TempDir::new().unwrap();
        let (priv_path, pubkey) = make_keypair(&dir, "k4");
        let mut env = sample_envelope(EnvelopeState::Attested);
        env.attested_by = Some(AttestedBy {
            person_id: "person-jennifer".to_string(),
            signature: "placeholder-human-signature".to_string(),
        });
        env.sign(&priv_path).expect("sign succeeds");
        let ok = env.verify(&pubkey, "k4@envelope-test").expect("verify call ok");
        assert!(ok);
    }

    #[test]
    fn json_round_trip_preserves_fields() {
        let env = sample_envelope(EnvelopeState::Observed);
        let json = serde_json::to_string(&env).unwrap();
        let parsed: Envelope = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.envelope_id, env.envelope_id);
        assert_eq!(parsed.entities.len(), 1);
        assert_eq!(parsed.entities[0].role, "attendee");
        assert_eq!(parsed.payload.payload_type, "minutebook.entry");
    }

    #[test]
    fn lineage_and_attachments_default_to_empty() {
        let env = sample_envelope(EnvelopeState::Observed);
        assert!(env.lineage.supersedes.is_none());
        assert!(env.lineage.derived_from.is_empty());
        assert!(env.attachments.is_empty());
    }
}
