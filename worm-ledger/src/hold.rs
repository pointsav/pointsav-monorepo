// SPDX-License-Identifier: Apache-2.0
//
// Litigation-hold records + the read-only disposal-eligibility fold —
// Phase 2 of the ratified disposal design (BRIEF-cross-compliance-worm-
// design.md, Decisions locked #7, project-input). Holds are NOT a
// mutable flag: they are ordinary append-only ledger entries
// (`schema: "fs-hold-v1"`), and "is this record on hold" is answered by
// folding the ledger's own history at check time, never by reading or
// writing separate mutable state. This module performs no deletion and
// exposes no write path of its own beyond what `http.rs` appends via the
// existing `/v1/append` machinery — it only interprets already-appended
// entries.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::ledger::Entry;

pub const HOLD_SCHEMA: &str = "fs-hold-v1";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HoldEvent {
    Placed,
    Lifted,
}

/// A hold's scope predicate. Every `Some` field must match for a
/// record to be covered; an unset (`None`) field imposes no
/// constraint on that dimension. `content_sha256` alone expresses
/// "this one specific record"; `coa_stem_prefix` and/or the created-
/// date range express "everything relating to matter Z" — a real
/// hold arrives that way, before matching future records even exist,
/// which is why this is a predicate and not a per-record list.
///
/// Known current limitation: Phase 1's ledger payloads (`http.rs`'s
/// content-split) do not yet carry a `coa_stem` field — that's blocked
/// on the still-open chart-of-accounts-naming decision
/// (`BRIEF-service-input-file-naming-taxonomy.md`). A hold scoped by
/// `coa_stem_prefix` alone will not match any record until that field
/// exists; document this rather than pretending it works today.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct HoldScope {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_sha256: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coa_stem_prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_from: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_to: Option<DateTime<Utc>>,
}

impl HoldScope {
    /// Does this scope cover a record with the given content hash,
    /// (optional) chart-of-accounts stem, and creation time?
    pub fn covers(&self, content_sha256: &str, coa_stem: Option<&str>, created: DateTime<Utc>) -> bool {
        if let Some(want) = &self.content_sha256 {
            if want != content_sha256 {
                return false;
            }
        }
        if let Some(prefix) = &self.coa_stem_prefix {
            match coa_stem {
                Some(stem) if stem.starts_with(prefix.as_str()) => {}
                _ => return false,
            }
        }
        if let Some(from) = self.created_from {
            if created < from {
                return false;
            }
        }
        if let Some(to) = self.created_to {
            if created > to {
                return false;
            }
        }
        // A scope with every field `None` covers everything — a real,
        // if unusual, "hold the entire ledger" case; not rejected here,
        // since refusing it would be a silent policy decision this
        // module shouldn't make on the caller's behalf.
        true
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HoldRecord {
    pub event: HoldEvent,
    pub hold_id: String,
    pub scope: HoldScope,
    pub basis: String,
    pub actor: String,
    pub timestamp: DateTime<Utc>,
}

/// Parse a single ledger entry's payload as a hold record. Returns
/// `None` for any entry that isn't a well-formed `fs-hold-v1` record —
/// including every ordinary document/transaction entry, which is the
/// overwhelmingly common case when folding a real ledger.
pub fn parse_hold_record(payload: &serde_json::Value) -> Option<HoldRecord> {
    if payload.get("schema").and_then(|s| s.as_str()) != Some(HOLD_SCHEMA) {
        return None;
    }
    let event = match payload.get("event").and_then(|e| e.as_str()) {
        Some("placed") => HoldEvent::Placed,
        Some("lifted") => HoldEvent::Lifted,
        _ => return None,
    };
    let hold_id = payload.get("hold_id").and_then(|v| v.as_str())?.to_string();
    let basis = payload.get("basis").and_then(|v| v.as_str())?.to_string();
    let actor = payload.get("actor").and_then(|v| v.as_str())?.to_string();
    let timestamp = payload
        .get("timestamp")
        .and_then(|v| v.as_str())
        .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.with_timezone(&Utc))?;
    let scope: HoldScope = payload
        .get("scope")
        .cloned()
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default();

    Some(HoldRecord {
        event,
        hold_id,
        scope,
        basis,
        actor,
        timestamp,
    })
}

/// Fold a ledger's entries into the set of currently-active holds:
/// every `hold_id` with a `placed` event and no subsequent `lifted`
/// event for that same `hold_id`. Never cached — call this fresh at
/// every eligibility check, per the ratified "resolver, not replica"
/// posture; a hold placed after a record became retention-expired
/// must still block it.
pub fn active_holds(entries: &[Entry]) -> Vec<HoldRecord> {
    use std::collections::BTreeMap;
    // Last event per hold_id wins — `entries` is already cursor-ordered
    // by the ledger, so a later `lifted` overwrites an earlier `placed`
    // for the same hold_id, and vice versa (a hold can be re-placed
    // after being lifted, under a fresh justification).
    let mut latest: BTreeMap<String, HoldRecord> = BTreeMap::new();
    for entry in entries {
        if let Some(rec) = parse_hold_record(&entry.payload) {
            latest.insert(rec.hold_id.clone(), rec);
        }
    }
    latest
        .into_values()
        .filter(|r| r.event == HoldEvent::Placed)
        .collect()
}

/// Is a record (identified by content hash, optional coa_stem, and
/// creation time) covered by any currently-active hold?
pub fn is_on_hold(
    active: &[HoldRecord],
    content_sha256: &str,
    coa_stem: Option<&str>,
    created: DateTime<Utc>,
) -> bool {
    active
        .iter()
        .any(|h| h.scope.covers(content_sha256, coa_stem, created))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hold_payload(event: &str, hold_id: &str, scope: serde_json::Value, ts: &str) -> serde_json::Value {
        serde_json::json!({
            "schema": HOLD_SCHEMA,
            "event": event,
            "hold_id": hold_id,
            "scope": scope,
            "basis": "test basis",
            "actor": "test-actor",
            "timestamp": ts,
        })
    }

    fn entry(cursor: u64, payload: serde_json::Value) -> Entry {
        Entry {
            cursor,
            payload_id: format!("p{cursor}"),
            payload,
            this_hash: "unused-in-these-tests".to_string(),
        }
    }

    #[test]
    fn non_hold_entries_are_ignored() {
        let entries = vec![entry(1, serde_json::json!({"amount": 500}))];
        assert!(active_holds(&entries).is_empty());
    }

    #[test]
    fn placed_hold_with_no_lift_is_active() {
        let entries = vec![entry(
            1,
            hold_payload(
                "placed",
                "matter-1",
                serde_json::json!({"content_sha256": "abc"}),
                "2026-09-01T00:00:00Z",
            ),
        )];
        let active = active_holds(&entries);
        assert_eq!(active.len(), 1);
        assert_eq!(active[0].hold_id, "matter-1");
    }

    #[test]
    fn lifted_hold_is_not_active() {
        let entries = vec![
            entry(
                1,
                hold_payload("placed", "matter-1", serde_json::json!({}), "2026-09-01T00:00:00Z"),
            ),
            entry(
                2,
                hold_payload("lifted", "matter-1", serde_json::json!({}), "2026-09-02T00:00:00Z"),
            ),
        ];
        assert!(active_holds(&entries).is_empty());
    }

    #[test]
    fn re_placed_after_lift_is_active_again() {
        let entries = vec![
            entry(1, hold_payload("placed", "matter-1", serde_json::json!({}), "2026-09-01T00:00:00Z")),
            entry(2, hold_payload("lifted", "matter-1", serde_json::json!({}), "2026-09-02T00:00:00Z")),
            entry(3, hold_payload("placed", "matter-1", serde_json::json!({}), "2026-09-03T00:00:00Z")),
        ];
        assert_eq!(active_holds(&entries).len(), 1);
    }

    #[test]
    fn scope_matches_exact_content_hash_only() {
        let scope = HoldScope {
            content_sha256: Some("abc".to_string()),
            ..Default::default()
        };
        let created = Utc::now();
        assert!(scope.covers("abc", None, created));
        assert!(!scope.covers("xyz", None, created));
    }

    #[test]
    fn scope_matches_coa_stem_prefix() {
        let scope = HoldScope {
            coa_stem_prefix: Some("WCP-ADM-2026".to_string()),
            ..Default::default()
        };
        let created = Utc::now();
        assert!(scope.covers("any", Some("WCP-ADM-2026-financial-statement"), created));
        assert!(!scope.covers("any", Some("WCP-OPR-2026-lease"), created));
        assert!(
            !scope.covers("any", None, created),
            "no coa_stem on the record at all must not match a prefix-scoped hold"
        );
    }

    #[test]
    fn scope_matches_date_range() {
        let scope = HoldScope {
            created_from: Some(DateTime::parse_from_rfc3339("2026-01-01T00:00:00Z").unwrap().with_timezone(&Utc)),
            created_to: Some(DateTime::parse_from_rfc3339("2026-12-31T23:59:59Z").unwrap().with_timezone(&Utc)),
            ..Default::default()
        };
        let inside = DateTime::parse_from_rfc3339("2026-06-15T00:00:00Z").unwrap().with_timezone(&Utc);
        let before = DateTime::parse_from_rfc3339("2025-06-15T00:00:00Z").unwrap().with_timezone(&Utc);
        assert!(scope.covers("any", None, inside));
        assert!(!scope.covers("any", None, before));
    }

    #[test]
    fn is_on_hold_checks_all_active_holds() {
        let entries = vec![entry(
            1,
            hold_payload(
                "placed",
                "matter-1",
                serde_json::json!({"content_sha256": "target-hash"}),
                "2026-09-01T00:00:00Z",
            ),
        )];
        let active = active_holds(&entries);
        assert!(is_on_hold(&active, "target-hash", None, Utc::now()));
        assert!(!is_on_hold(&active, "other-hash", None, Utc::now()));
    }
}
