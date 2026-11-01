// SPDX-License-Identifier: Apache-2.0

//! The mountable `Submit` handler — decision 15's "the one genuinely
//! shared ingestion surface... a library, not a process." Every
//! "system of record" service mounts this against its own
//! `worm_ledger` instance; it is not a shared daemon any service
//! writes through (that would recreate the confused-deputy problem
//! decision 6 already moved away from) — it's a function each
//! service's own process calls against its own ledger + vocabulary.

use envelope::Envelope;
use worm_ledger::guard::GuardError;
use worm_ledger::retention::RetentionVocabulary;
use worm_ledger::LedgerBackend;

use crate::frames::{EnvelopeBytes, RefusalReason, Response};

#[derive(Debug)]
pub enum SubmitError {
    /// The envelope's `canonical_bytes()`/CBOR parse failed — not
    /// well-formed.
    Malformed(String),
    /// `Envelope::validate()` failed (e.g. `Attested` with no
    /// `attested_by`).
    Invalid(String),
    /// Signature verification failed or the signer isn't recognized
    /// for this tenant (caller-supplied verification, see
    /// `SubmitContext`).
    SignatureInvalid,
    /// The envelope's `payload.type` didn't declare a `retention_class`
    /// the caller-supplied context could resolve — see
    /// `SubmitContext::retention_class_for`. Distinct from
    /// `GuardError::UnknownRetentionClass`, which fires one step
    /// later (the class was named but isn't in the vocabulary).
    NoRetentionClass,
    Guard(GuardError),
}

impl std::fmt::Display for SubmitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SubmitError::Malformed(e) => write!(f, "malformed envelope: {e}"),
            SubmitError::Invalid(e) => write!(f, "invalid envelope: {e}"),
            SubmitError::SignatureInvalid => write!(f, "envelope signature did not verify"),
            SubmitError::NoRetentionClass => {
                write!(f, "no retention class resolved for this envelope's payload type")
            }
            SubmitError::Guard(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for SubmitError {}

/// What the mounting service supplies to gate/route a `Submit` call —
/// deliberately a trait, not a struct, so each service can implement
/// its own `payload.type` -> retention-class mapping and signer
/// verification without `totebox-protocol` needing to know about
/// every service's schema.
pub trait SubmitContext {
    /// Resolve the retention class for this envelope's
    /// `payload.type`. Returning `None` refuses the submit outright
    /// (decision 16: every "system of record" append must declare a
    /// known class — no silent default).
    fn retention_class_for(&self, envelope: &Envelope) -> Option<String>;

    /// Verify the envelope's own signature (via
    /// `envelope::Envelope::verify`, using whatever pubkey/identity
    /// lookup this service's tenant configuration provides for
    /// `envelope.source.adapter`). Returning `false` refuses the
    /// submit — a `Submit` request is never trusted on transport
    /// authentication alone, the envelope's own signature is checked
    /// independently.
    fn verify_signature(&self, envelope: &Envelope) -> bool;
}

/// Handle one `Submit` request against `ledger` (already selected by
/// the caller — see `capability::Capability::allows_ledger`, checked
/// upstream of this function, not repeated here) using `vocab` for
/// retention-class validation and `ctx` for the service-specific
/// class-resolution/signature-verification hooks.
pub fn handle_submit(
    ledger: &dyn LedgerBackend,
    vocab: &RetentionVocabulary,
    ctx: &dyn SubmitContext,
    envelope_bytes: &EnvelopeBytes,
) -> Result<Response, SubmitError> {
    let envelope = envelope_bytes
        .parse()
        .map_err(|e| SubmitError::Malformed(e.to_string()))?;

    envelope
        .validate()
        .map_err(|e| SubmitError::Invalid(e.to_string()))?;

    if !ctx.verify_signature(&envelope) {
        return Err(SubmitError::SignatureInvalid);
    }

    let retention_class = ctx
        .retention_class_for(&envelope)
        .ok_or(SubmitError::NoRetentionClass)?;

    let guarded = worm_ledger::GuardedLedger::new(ledger, vocab);
    let payload_json = serde_json::to_value(&envelope).map_err(|e| SubmitError::Malformed(e.to_string()))?;
    let cursor = guarded
        .append_with_retention(&envelope.envelope_id, &payload_json, &retention_class)
        .map_err(SubmitError::Guard)?;

    Ok(Response::Submitted { cursor })
}

/// Build the `Response::Refused` a caller should return when
/// `capability::Capability::allows`/`allows_ledger` already rejected
/// a request before it reached `handle_submit` at all — kept here so
/// callers construct the same shape consistently rather than each
/// hand-rolling a `Response::Refused` literal.
pub fn refuse(reason: RefusalReason) -> Response {
    Response::Refused { reason }
}

#[cfg(test)]
mod tests {
    use super::*;
    use envelope::{EntityRef, EnvelopeState, Lineage, Payload, Source};
    use worm_ledger::InMemoryLedger;

    struct TestCtx {
        class: Option<String>,
        signature_ok: bool,
    }

    impl SubmitContext for TestCtx {
        fn retention_class_for(&self, _envelope: &Envelope) -> Option<String> {
            self.class.clone()
        }
        fn verify_signature(&self, _envelope: &Envelope) -> bool {
            self.signature_ok
        }
    }

    fn sample_envelope() -> Envelope {
        Envelope {
            envelope_id: "01JSUBMIT000000000000000000".to_string(),
            envelope_version: "0.1.0".to_string(),
            tenant: "woodfine".to_string(),
            created: chrono::Utc::now(),
            source: Source { adapter: "test".to_string(), origin_ref: "x".to_string() },
            entities: vec![EntityRef { entity_id: "p1".to_string(), role: "attendee".to_string() }],
            state: EnvelopeState::Observed,
            attested_by: None,
            confidence: None,
            payload: Payload {
                payload_type: "minutebook.entry".to_string(),
                body: serde_json::json!({"summary": "test"}),
            },
            attachments: vec![],
            lineage: Lineage::default(),
            signature: String::new(),
        }
    }

    // Per-call uniqueness needs more than std::process::id() -- Rust's
    // test runner executes #[test] functions in parallel THREADS
    // within one process, so every test in this module shares the
    // same pid. Without a counter, two tests racing on the identical
    // temp path can torn-read each other's std::fs::write (verified
    // real flake, not theoretical: this file's own vocab helper hit
    // it before this fix). Matches worm-ledger's own established
    // AtomicU64-counter pattern for exactly this reason.
    static TMPCTR: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

    fn vocab_with_finra_6y() -> RetentionVocabulary {
        let n = TMPCTR.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let path = std::env::temp_dir().join(format!(
            "totebox-protocol-submit-test-{}-{}.csv",
            std::process::id(),
            n
        ));
        std::fs::write(&path, "class,duration_days,regime,notes\nfinra-6y,2192,FINRA,x\n").unwrap();
        worm_ledger::retention::load_retention_classes(path.to_str().unwrap())
    }

    fn tmp_ledger() -> InMemoryLedger {
        let n = TMPCTR.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let dir = std::env::temp_dir().join(format!(
            "totebox-protocol-submit-ledger-{}-{}",
            std::process::id(),
            n
        ));
        InMemoryLedger::open(dir, "test-tenant").unwrap()
    }

    #[test]
    fn well_formed_envelope_with_known_class_submits_successfully() {
        let ledger = tmp_ledger();
        let vocab = vocab_with_finra_6y();
        let ctx = TestCtx { class: Some("finra-6y".to_string()), signature_ok: true };
        let bytes = EnvelopeBytes::new(sample_envelope().canonical_bytes().unwrap());

        let resp = handle_submit(&ledger, &vocab, &ctx, &bytes).unwrap();
        assert!(matches!(resp, Response::Submitted { cursor: 1 }));
    }

    #[test]
    fn invalid_signature_is_rejected() {
        let ledger = tmp_ledger();
        let vocab = vocab_with_finra_6y();
        let ctx = TestCtx { class: Some("finra-6y".to_string()), signature_ok: false };
        let bytes = EnvelopeBytes::new(sample_envelope().canonical_bytes().unwrap());

        let err = handle_submit(&ledger, &vocab, &ctx, &bytes).unwrap_err();
        assert!(matches!(err, SubmitError::SignatureInvalid));
        assert_eq!(ledger.read_since(0).unwrap().len(), 0, "nothing appended on rejection");
    }

    #[test]
    fn missing_retention_class_resolution_is_rejected() {
        let ledger = tmp_ledger();
        let vocab = vocab_with_finra_6y();
        let ctx = TestCtx { class: None, signature_ok: true };
        let bytes = EnvelopeBytes::new(sample_envelope().canonical_bytes().unwrap());

        let err = handle_submit(&ledger, &vocab, &ctx, &bytes).unwrap_err();
        assert!(matches!(err, SubmitError::NoRetentionClass));
    }

    #[test]
    fn unknown_retention_class_is_rejected_by_the_guard() {
        let ledger = tmp_ledger();
        let vocab = vocab_with_finra_6y();
        let ctx = TestCtx { class: Some("no-such-class".to_string()), signature_ok: true };
        let bytes = EnvelopeBytes::new(sample_envelope().canonical_bytes().unwrap());

        let err = handle_submit(&ledger, &vocab, &ctx, &bytes).unwrap_err();
        assert!(matches!(err, SubmitError::Guard(GuardError::UnknownRetentionClass(_))));
    }

    #[test]
    fn attested_without_attested_by_is_rejected_before_signature_check() {
        let ledger = tmp_ledger();
        let vocab = vocab_with_finra_6y();
        let ctx = TestCtx { class: Some("finra-6y".to_string()), signature_ok: true };
        let mut env = sample_envelope();
        env.state = EnvelopeState::Attested;
        let bytes = EnvelopeBytes::new(env.canonical_bytes().unwrap());

        let err = handle_submit(&ledger, &vocab, &ctx, &bytes).unwrap_err();
        assert!(matches!(err, SubmitError::Invalid(_)));
    }

    #[test]
    fn malformed_bytes_are_rejected_not_panicking() {
        let ledger = tmp_ledger();
        let vocab = vocab_with_finra_6y();
        let ctx = TestCtx { class: Some("finra-6y".to_string()), signature_ok: true };
        let bytes = EnvelopeBytes::new(vec![0xff, 0x00]);

        let err = handle_submit(&ledger, &vocab, &ctx, &bytes).unwrap_err();
        assert!(matches!(err, SubmitError::Malformed(_)));
    }
}
