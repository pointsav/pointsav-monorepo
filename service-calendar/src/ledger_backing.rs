// SPDX-License-Identifier: Apache-2.0

//! `service-calendar`'s own `worm-ledger` instance.
//!
//! Same pattern as every other Phase 0/1/2 service: own in-process
//! `worm-ledger`, `CalendarEvent`s wrapped as `calendar.event`
//! Envelopes (decision 8), `state: Observed`, unsigned (same
//! deliberate Phase-0/1/2 scope limit documented in `service-people`/
//! `service-places`/`service-email`'s own `ledger_backing.rs`).
//!
//! A deletion is its own append-only tombstone Envelope
//! (`body.deleted: true`), never a mutation of the original —
//! consistent with WORM.

use envelope::{EntityRef, Envelope, EnvelopeState, Lineage, Payload, Source};
use worm_ledger::guard::{GuardError, GuardedLedger};
use worm_ledger::retention::RetentionVocabulary;
use worm_ledger::{LedgerBackend, LedgerError};

use crate::event::{parse_ics, CalendarEvent, ParseError};
use crate::event_store::EventStore;

/// Retention class every record this crate writes declares.
/// **Placeholder duration, not a legal retention decision** — same
/// framing as `service-email`'s `email-message` class.
pub const CALENDAR_EVENT_CLASS: &str = "calendar-event";

#[derive(Debug)]
pub enum LedgerBackingError {
    Parse(ParseError),
    Envelope(envelope::EnvelopeError),
    Serialize(serde_json::Error),
    Guard(GuardError),
    Ledger(LedgerError),
}

impl std::fmt::Display for LedgerBackingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LedgerBackingError::Parse(e) => write!(f, "{e}"),
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

    fn submit(&self, envelope: &Envelope) -> Result<u64, LedgerBackingError> {
        envelope.validate().map_err(LedgerBackingError::Envelope)?;
        let payload_json = serde_json::to_value(envelope).map_err(LedgerBackingError::Serialize)?;
        let guarded = GuardedLedger::new(self.ledger.as_ref(), &self.vocab);
        guarded
            .append_with_retention(&envelope.envelope_id, &payload_json, CALENDAR_EVENT_CLASS)
            .map_err(LedgerBackingError::Guard)
    }

    /// Parses `raw_ics` (must contain exactly one VEVENT, per RFC4791
    /// "one calendar object resource per event"), appends it as a
    /// `calendar.event` Envelope, and returns the assigned cursor plus
    /// the parsed `CalendarEvent`. A second PUT of the same UID is a
    /// normal update -- creates a new Envelope, does not mutate the
    /// old one (WORM); `EventStore::upsert` naturally shows the latest.
    pub fn put_event(&self, raw_ics: &str) -> Result<(u64, CalendarEvent), LedgerBackingError> {
        let (uid, summary, dtstart, dtend) = parse_ics(raw_ics).map_err(LedgerBackingError::Parse)?;
        let now = chrono::Utc::now();
        let event = CalendarEvent {
            uid: uid.clone(),
            raw_ics: raw_ics.to_string(),
            summary,
            dtstart,
            dtend,
            created_at: now,
            updated_at: now,
        };

        let body = serde_json::to_value(&event).map_err(LedgerBackingError::Serialize)?;
        let envelope = Envelope {
            envelope_id: uuid_v4(),
            envelope_version: "0.1.0".to_string(),
            tenant: self.tenant.clone(),
            created: now,
            source: Source {
                adapter: "service-calendar".to_string(),
                origin_ref: uid.clone(),
            },
            entities: vec![EntityRef {
                entity_id: uid,
                role: "subject".to_string(),
            }],
            state: EnvelopeState::Observed,
            attested_by: None,
            confidence: None,
            payload: Payload {
                payload_type: "calendar.event".to_string(),
                body,
            },
            attachments: vec![],
            lineage: Lineage::default(),
            signature: String::new(),
        };
        let cursor = self.submit(&envelope)?;
        Ok((cursor, event))
    }

    /// Appends a tombstone Envelope for `uid` -- deletion is a new
    /// append, never a mutation (WORM).
    pub fn delete_event(&self, uid: &str) -> Result<u64, LedgerBackingError> {
        let body = serde_json::json!({ "uid": uid, "deleted": true });
        let envelope = Envelope {
            envelope_id: uuid_v4(),
            envelope_version: "0.1.0".to_string(),
            tenant: self.tenant.clone(),
            created: chrono::Utc::now(),
            source: Source {
                adapter: "service-calendar".to_string(),
                origin_ref: uid.to_string(),
            },
            entities: vec![EntityRef {
                entity_id: uid.to_string(),
                role: "subject".to_string(),
            }],
            state: EnvelopeState::Observed,
            attested_by: None,
            confidence: None,
            payload: Payload {
                payload_type: "calendar.event".to_string(),
                body,
            },
            attachments: vec![],
            lineage: Lineage::default(),
            signature: String::new(),
        };
        self.submit(&envelope)
    }

    /// Replays every `calendar.event` Envelope into `store`, in
    /// ledger order -- a tombstone removes the event; anything else
    /// upserts it. Survives a restart the same way every other
    /// Phase 0/1/2 service's replay does.
    pub fn replay_into(&self, store: &EventStore) -> Result<usize, LedgerBackingError> {
        let entries = self.ledger.read_since(0).map_err(LedgerBackingError::Ledger)?;
        let mut replayed = 0;
        for entry in entries {
            let Ok(envelope) = serde_json::from_value::<Envelope>(entry.payload) else {
                continue;
            };
            if envelope.payload.payload_type != "calendar.event" {
                continue;
            }
            let is_deleted = envelope
                .payload
                .body
                .get("deleted")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            if is_deleted {
                if let Some(uid) = envelope.payload.body.get("uid").and_then(|v| v.as_str()) {
                    store.remove(uid);
                }
            } else if let Ok(event) = serde_json::from_value::<CalendarEvent>(envelope.payload.body.clone()) {
                store.upsert(event);
            }
            replayed += 1;
        }
        Ok(replayed)
    }
}

fn uuid_v4() -> String {
    uuid::Uuid::new_v4().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use worm_ledger::InMemoryLedger;

    static TMPCTR: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

    const SIMPLE_EVENT: &str = "BEGIN:VCALENDAR\r\nVERSION:2.0\r\nPRODID:-//test//EN\r\nBEGIN:VEVENT\r\nUID:event-1@example.com\r\nDTSTART:20260101T120000Z\r\nDTEND:20260101T130000Z\r\nSUMMARY:New Year Meeting\r\nEND:VEVENT\r\nEND:VCALENDAR\r\n";

    fn tmp_vocab() -> RetentionVocabulary {
        let n = TMPCTR.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let path = std::env::temp_dir().join(format!(
            "service-calendar-ledger-backing-test-{}-{}.csv",
            std::process::id(),
            n
        ));
        std::fs::write(
            &path,
            "class,duration_days,regime,notes\ncalendar-event,,internal calendar ledger,placeholder\n",
        )
        .unwrap();
        worm_ledger::retention::load_retention_classes(path.to_str().unwrap())
    }

    fn tmp_backing() -> LedgerBacking {
        let n = TMPCTR.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let dir = std::env::temp_dir().join(format!(
            "service-calendar-ledger-backing-ledger-{}-{}",
            std::process::id(),
            n
        ));
        let ledger = Box::new(InMemoryLedger::open(dir, "test-tenant").unwrap());
        LedgerBacking::new(ledger, tmp_vocab(), "woodfine")
    }

    #[test]
    fn put_event_succeeds_with_known_class() {
        let backing = tmp_backing();
        let (cursor, event) = backing.put_event(SIMPLE_EVENT).unwrap();
        assert_eq!(cursor, 1);
        assert_eq!(event.uid, "event-1@example.com");
    }

    #[test]
    fn replay_into_rebuilds_event_store() {
        let backing = tmp_backing();
        backing.put_event(SIMPLE_EVENT).unwrap();
        let store = EventStore::new();
        let replayed = backing.replay_into(&store).unwrap();
        assert_eq!(replayed, 1);
        assert!(store.lookup_by_uid("event-1@example.com").is_some());
    }

    #[test]
    fn delete_then_replay_removes_from_store() {
        let backing = tmp_backing();
        backing.put_event(SIMPLE_EVENT).unwrap();
        backing.delete_event("event-1@example.com").unwrap();

        let store = EventStore::new();
        let replayed = backing.replay_into(&store).unwrap();
        assert_eq!(replayed, 2, "put + tombstone both replay");
        assert!(store.lookup_by_uid("event-1@example.com").is_none(), "tombstone must remove it");
    }
}
