// SPDX-License-Identifier: Apache-2.0

//! `service-business`'s own `worm-ledger` instance.
//!
//! `BRIEF-os-totebox-communication-complex-2026-09.md` decision 6/12:
//! own `worm-ledger` instance, `Business` wrapped as a signed Envelope
//! (`payload.type: "business.record"`). `entities[]` carries `subject`
//! always and `location` (the referenced `service-places::Place` id)
//! when known -- the first real cross-service entity reference in
//! this BRIEF's execution (`service-publishing`/`service-research`'s
//! extra roles pointed at people, not another service's own records).

use envelope::{EntityRef, Envelope, EnvelopeState, Lineage, Payload, Source};
use worm_ledger::guard::{GuardError, GuardedLedger};
use worm_ledger::retention::RetentionVocabulary;
use worm_ledger::{LedgerBackend, LedgerError};

use crate::business::Business;

pub const BUSINESS_RECORD_CLASS: &str = "business-record";

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

    pub fn append_business(&self, business: &Business) -> Result<u64, LedgerBackingError> {
        let mut entities = vec![EntityRef {
            entity_id: business.id.to_string(),
            role: "subject".to_string(),
        }];
        if let Some(place_id) = business.place_id {
            entities.push(EntityRef {
                entity_id: place_id.to_string(),
                role: "location".to_string(),
            });
        }
        let body = serde_json::to_value(business).map_err(LedgerBackingError::Serialize)?;

        let envelope = Envelope {
            envelope_id: uuid::Uuid::new_v4().to_string(),
            envelope_version: "0.1.0".to_string(),
            tenant: self.tenant.clone(),
            created: chrono::Utc::now(),
            source: Source {
                adapter: "service-business".to_string(),
                origin_ref: business.id.to_string(),
            },
            entities,
            state: EnvelopeState::Observed,
            attested_by: None,
            confidence: None,
            payload: Payload {
                payload_type: "business.record".to_string(),
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
            .append_with_retention(&envelope.envelope_id, &payload_json, BUSINESS_RECORD_CLASS)
            .map_err(LedgerBackingError::Guard)
    }

    pub fn replay_into(&self, store: &crate::business_store::BusinessStore) -> Result<usize, LedgerBackingError> {
        let entries = self.ledger.read_since(0).map_err(LedgerBackingError::Ledger)?;
        let mut replayed = 0;
        for entry in entries {
            let Ok(envelope) = serde_json::from_value::<Envelope>(entry.payload) else {
                continue;
            };
            if envelope.payload.payload_type != "business.record" {
                continue;
            }
            let Ok(business) = serde_json::from_value::<Business>(envelope.payload.body) else {
                continue;
            };
            if store.append(business).is_ok() {
                replayed += 1;
            }
        }
        Ok(replayed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;
    use worm_ledger::InMemoryLedger;

    static TMPCTR: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

    fn tmp_vocab() -> RetentionVocabulary {
        let n = TMPCTR.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let path = std::env::temp_dir().join(format!(
            "service-business-ledger-backing-test-{}-{}.csv",
            std::process::id(),
            n
        ));
        std::fs::write(
            &path,
            "class,duration_days,regime,notes\nbusiness-record,,internal identity ledger,permanent per D11 no-expiry posture\n",
        )
        .unwrap();
        worm_ledger::retention::load_retention_classes(path.to_str().unwrap())
    }

    fn tmp_backing() -> LedgerBacking {
        let n = TMPCTR.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let dir = std::env::temp_dir().join(format!(
            "service-business-ledger-backing-ledger-{}-{}",
            std::process::id(),
            n
        ));
        let ledger = Box::new(InMemoryLedger::open(dir, "test-tenant").unwrap());
        LedgerBacking::new(ledger, tmp_vocab(), "woodfine")
    }

    #[test]
    fn append_business_succeeds_with_known_class() {
        let backing = tmp_backing();
        let business = Business::new("Acme Cafe", "restaurant", None);
        let cursor = backing.append_business(&business).unwrap();
        assert_eq!(cursor, 1);
    }

    #[test]
    fn location_entity_present_only_when_place_known() {
        let backing = tmp_backing();
        let no_place = Business::new("Mobile Vendor", "retail", None);
        backing.append_business(&no_place).unwrap();

        let with_place = Business::new("Acme Cafe", "restaurant", Some(Uuid::new_v4()));
        backing.append_business(&with_place).unwrap();
    }

    #[test]
    fn replay_into_rebuilds_business_store() {
        let backing = tmp_backing();
        let a = Business::new("Acme Cafe", "restaurant", None);
        let b = Business::new("Bookstore", "retail", Some(Uuid::new_v4()));
        backing.append_business(&a).unwrap();
        backing.append_business(&b).unwrap();

        let store = crate::business_store::BusinessStore::new();
        let replayed = backing.replay_into(&store).unwrap();
        assert_eq!(replayed, 2);
        assert_eq!(store.lookup_by_id(a.id).unwrap().id, a.id);
        assert_eq!(store.lookup_by_id(b.id).unwrap().id, b.id);
    }
}
