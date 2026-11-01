// SPDX-License-Identifier: Apache-2.0

//! `service-email`'s own `worm-ledger` instance.
//!
//! `BRIEF-os-totebox-communication-complex-2026-09.md` decision 6/8:
//! replaces `FsClient` (an HTTP client to a remote `service-fs`)
//! entirely — persistence is now in-process, matching the cutover
//! already done for `service-people`/`service-places`. Every ingested
//! or sent message becomes an `email.message` Envelope (decision 8)
//! before appending.
//!
//! Same known, deliberate Phase-0/1 scope limit as `service-people`/
//! `service-places`: Envelopes are constructed and validated but NOT
//! signed — real Ed25519 key provisioning is deployment-time
//! infrastructure, not solved in this crate.
//!
//! **Attachments are carried inline as base64 in Phase 1**, not yet
//! split out to content-addressed blob storage with a hash pointer
//! (decision 7's `AttachmentRef`). Real gap, not silently assumed
//! away: large-attachment handling is deferred to whenever a real
//! message exercises it — the raw-EML body already carries the full
//! MIME structure (headers + all parts) as one blob, so nothing is
//! lost, just not yet content-addressed separately.

use envelope::{EntityRef, Envelope, EnvelopeState, Lineage, Payload, Source};
use worm_ledger::guard::{GuardError, GuardedLedger};
use worm_ledger::retention::RetentionVocabulary;
use worm_ledger::{LedgerBackend, LedgerError};

/// Retention class every record this crate writes declares. Ships its
/// own vocabulary (`retention-classes.csv`, shipped alongside this
/// crate) per decision 16 — the library never hardcodes a regime.
/// **Placeholder duration, not a legal retention decision**: real
/// business-email retention periods (regulatory, litigation-hold,
/// contractual) are an operator/counsel decision, not a crate-design
/// one — this class defaults to no-expiry until that decision is made.
pub const EMAIL_MESSAGE_CLASS: &str = "email-message";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Inbound,
    Outbound,
}

impl Direction {
    fn as_str(self) -> &'static str {
        match self {
            Direction::Inbound => "inbound",
            Direction::Outbound => "outbound",
        }
    }
}

#[derive(Debug)]
pub enum LedgerBackingError {
    Envelope(envelope::EnvelopeError),
    Serialize(serde_json::Error),
    Guard(GuardError),
    Ledger(LedgerError),
}

impl std::fmt::Display for LedgerBackingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LedgerBackingError::Envelope(e) => write!(f, "{e}"),
            LedgerBackingError::Serialize(e) => write!(f, "serialize error: {e}"),
            LedgerBackingError::Guard(e) => write!(f, "{e}"),
            LedgerBackingError::Ledger(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for LedgerBackingError {}

pub struct LedgerBacking {
    ledger: Box<dyn LedgerBackend + Send + Sync>,
    vocab: RetentionVocabulary,
    tenant: String,
}

impl LedgerBacking {
    pub fn new(
        ledger: Box<dyn LedgerBackend + Send + Sync>,
        vocab: RetentionVocabulary,
        tenant: impl Into<String>,
    ) -> Self {
        Self {
            ledger,
            vocab,
            tenant: tenant.into(),
        }
    }

    /// Append one email message to this service's own ledger. Returns
    /// the assigned ledger cursor.
    pub fn append_message(
        &self,
        message_id: &str,
        raw_eml: &[u8],
        direction: Direction,
    ) -> Result<u64, LedgerBackingError> {
        use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};

        let body = serde_json::json!({
            "message_id": message_id,
            "raw_eml_b64": BASE64.encode(raw_eml),
            "direction": direction.as_str(),
            "ingested_at": chrono::Utc::now().to_rfc3339(),
        });

        let envelope = Envelope {
            envelope_id: uuid::Uuid::new_v4().to_string(),
            envelope_version: "0.1.0".to_string(),
            tenant: self.tenant.clone(),
            created: chrono::Utc::now(),
            source: Source {
                adapter: "service-email".to_string(),
                origin_ref: message_id.to_string(),
            },
            entities: vec![EntityRef {
                entity_id: message_id.to_string(),
                role: "message".to_string(),
            }],
            state: EnvelopeState::Observed,
            attested_by: None,
            confidence: None,
            payload: Payload {
                payload_type: "email.message".to_string(),
                body,
            },
            attachments: vec![],
            lineage: Lineage::default(),
            signature: String::new(),
        };
        envelope.validate().map_err(LedgerBackingError::Envelope)?;

        let payload_json = serde_json::to_value(&envelope).map_err(LedgerBackingError::Serialize)?;
        let guarded = GuardedLedger::new(self.ledger.as_ref(), &self.vocab);
        guarded
            .append_with_retention(&envelope.envelope_id, &payload_json, EMAIL_MESSAGE_CLASS)
            .map_err(LedgerBackingError::Guard)
    }

    /// Read every `email.message` Envelope back out of the ledger,
    /// oldest first — the read-side half of the IMAP-serve round trip
    /// (see `imap.rs`).
    pub fn read_all_messages(&self) -> Result<Vec<Envelope>, LedgerBackingError> {
        let entries = self.ledger.read_since(0).map_err(LedgerBackingError::Ledger)?;
        let mut out = Vec::new();
        for entry in entries {
            let Ok(envelope) = serde_json::from_value::<Envelope>(entry.payload) else {
                continue;
            };
            if envelope.payload.payload_type == "email.message" {
                out.push(envelope);
            }
        }
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use worm_ledger::InMemoryLedger;

    static TMPCTR: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

    fn tmp_vocab() -> RetentionVocabulary {
        let n = TMPCTR.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let path = std::env::temp_dir().join(format!(
            "service-email-ledger-backing-test-{}-{}.csv",
            std::process::id(),
            n
        ));
        std::fs::write(
            &path,
            "class,duration_days,regime,notes\nemail-message,,internal email ledger,placeholder pending real retention decision\n",
        )
        .unwrap();
        worm_ledger::retention::load_retention_classes(path.to_str().unwrap())
    }

    /// Real regression test for a real bug found in a live seL4 guest
    /// boot (2026-09-24): the shipped fixture's `notes` field had an
    /// unquoted internal comma, and was also missing the `anchor`
    /// column entirely -- together this made the CSV parser
    /// misinterpret a fragment of the notes text as an unrecognized
    /// `anchor` value. The file being *readable* (the guest-staging
    /// fix, same commit) surfaced this separate, pre-existing content
    /// bug for the first time; this locks in that it's fixed.
    #[test]
    fn loads_the_real_shipped_fixture_with_zero_warnings() {
        let vocab = worm_ledger::retention::load_retention_classes(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/retention-classes.csv"
        ));
        assert!(
            vocab.warnings.is_empty(),
            "shipped fixture must load with zero warnings, got: {:?}",
            vocab.warnings
        );
        assert_eq!(vocab.classes.len(), 1);
    }

    fn tmp_backing() -> LedgerBacking {
        let n = TMPCTR.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let dir = std::env::temp_dir().join(format!(
            "service-email-ledger-backing-ledger-{}-{}",
            std::process::id(),
            n
        ));
        let ledger = Box::new(InMemoryLedger::open(dir, "test-tenant").unwrap());
        LedgerBacking::new(ledger, tmp_vocab(), "woodfine")
    }

    #[test]
    fn append_message_succeeds_with_known_class() {
        let backing = tmp_backing();
        let cursor = backing
            .append_message("msg-1", b"From: a@b.com\r\n\r\nhi", Direction::Inbound)
            .unwrap();
        assert_eq!(cursor, 1);
    }

    #[test]
    fn read_all_messages_round_trips() {
        let backing = tmp_backing();
        backing
            .append_message("msg-1", b"From: a@b.com\r\n\r\nhi", Direction::Inbound)
            .unwrap();
        backing
            .append_message("msg-2", b"From: c@d.com\r\n\r\nbye", Direction::Outbound)
            .unwrap();

        let messages = backing.read_all_messages().unwrap();
        assert_eq!(messages.len(), 2);
        assert_eq!(messages[0].payload.body["message_id"], "msg-1");
        assert_eq!(messages[0].payload.body["direction"], "inbound");
        assert_eq!(messages[1].payload.body["message_id"], "msg-2");
        assert_eq!(messages[1].payload.body["direction"], "outbound");
    }

    #[test]
    fn envelopes_are_written_unsigned_by_design() {
        let backing = tmp_backing();
        backing
            .append_message("msg-1", b"raw", Direction::Inbound)
            .unwrap();
        let messages = backing.read_all_messages().unwrap();
        assert_eq!(messages[0].signature, "");
    }
}
