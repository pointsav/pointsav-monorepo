// SPDX-License-Identifier: Apache-2.0

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// One business entity -- POI track
/// (`BRIEF-os-totebox-communication-complex-2026-09.md` decision 12).
/// The first service in this BRIEF's execution to hold a real
/// cross-service reference: `place_id` names a `service-places::Place`
/// by its own id, rather than duplicating an address here. Whether
/// that place actually exists is not this crate's concern to verify
/// (no network call to `service-places` happens in this crate at
/// all) -- decision 8's entities[]-by-reference design is exactly
/// meant to let services point at each other's records without a
/// runtime dependency between them; a DataGraph consumer reading both
/// ledgers is what actually resolves the reference.
///
/// `id` is content-derived (UUIDv5 of `name|place_id`), like `Place`:
/// the same business re-imported from the same POI feed at the same
/// location resolves to the same record.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Business {
    pub id: Uuid,
    pub name: String,
    /// Free-form at Phase 0, same posture as `Place::place_type` --
    /// e.g. "restaurant", "retail", "professional-services".
    pub category: String,
    /// The `service-places::Place` this business occupies, if known.
    pub place_id: Option<Uuid>,
    pub registration_number: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Business {
    pub fn new(name: impl Into<String>, category: impl Into<String>, place_id: Option<Uuid>) -> Self {
        let name = name.into();
        let key = format!("{}|{}", name, place_id.map(|p| p.to_string()).unwrap_or_default());
        let id = Uuid::new_v5(&Uuid::NAMESPACE_DNS, key.as_bytes());
        let now = Utc::now();
        Self {
            id,
            name,
            category: category.into(),
            place_id,
            registration_number: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn with_registration_number(mut self, number: impl Into<String>) -> Self {
        self.registration_number = Some(number.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_derives_deterministic_id_from_name_and_place() {
        let place_id = Uuid::new_v4();
        let a = Business::new("Acme Cafe", "restaurant", Some(place_id));
        let b = Business::new("Acme Cafe", "restaurant", Some(place_id));
        assert_eq!(a.id, b.id, "same name+place must produce the same id");
    }

    #[test]
    fn different_place_produces_different_id() {
        let a = Business::new("Acme Cafe", "restaurant", Some(Uuid::new_v4()));
        let b = Business::new("Acme Cafe", "restaurant", Some(Uuid::new_v4()));
        assert_ne!(a.id, b.id, "same name at a different location is a different business");
    }

    #[test]
    fn no_place_id_still_produces_a_stable_id() {
        let a = Business::new("Mobile Vendor", "retail", None);
        let b = Business::new("Mobile Vendor", "retail", None);
        assert_eq!(a.id, b.id);
    }

    #[test]
    fn with_registration_number_sets_field() {
        let business = Business::new("Acme Cafe", "restaurant", None).with_registration_number("BC1234567");
        assert_eq!(business.registration_number.as_deref(), Some("BC1234567"));
    }

    #[test]
    fn json_round_trip_preserves_all_fields() {
        let original = Business::new("Acme Cafe", "restaurant", Some(Uuid::new_v4()))
            .with_registration_number("BC1234567");
        let json = serde_json::to_string(&original).unwrap();
        let decoded: Business = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded, original);
    }
}
