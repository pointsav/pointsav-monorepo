// SPDX-License-Identifier: Apache-2.0
//
// Phase 3 of the ratified disposal design (BRIEF-cross-compliance-worm-
// design.md, Decisions locked #7, project-input): `fs-disposition-v1`
// reasoning-logging records, built for the `propose` half of the
// propose/execute split. This module performs no deletion of any kind
// -- that is Phase 4's `execute` path. Every decision, granted or
// withheld, is logged with its specific legal basis at decision time
// (Decisions locked #6) -- a state flag alone never satisfies that
// requirement, so the decision and its basis are always derived here
// from the record's actual current state, never accepted as caller
// input.

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

use crate::ledger::Entry;

pub const DISPOSITION_SCHEMA: &str = "fs-disposition-v1";

/// Standard notice window between propose and execute (Decisions
/// locked #7).
pub const STANDARD_NOTICE_DAYS: i64 = 7;
/// Extended window for a GDPR erasure-request grant, or for a record
/// with a hold lifted in the last 12 months (either condition widens
/// it independently; Decisions locked #7).
pub const EXTENDED_NOTICE_DAYS: i64 = 30;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DispositionTrigger {
    RetentionExpiry,
    GdprErasureRequest,
}

impl DispositionTrigger {
    /// Stable wire string, used both for serde and as the exact bytes
    /// an authority signs over — kept separate from `{:?}` Debug
    /// output so a future derive-macro or enum-repr change can never
    /// silently change what a previously-issued signature verifies
    /// against.
    pub fn as_str(&self) -> &'static str {
        match self {
            DispositionTrigger::RetentionExpiry => "retention-expiry",
            DispositionTrigger::GdprErasureRequest => "gdpr-erasure-request",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DispositionDecision {
    Granted,
    Withheld,
}

/// Which half of the propose/execute split this record represents —
/// mirrors `hold.rs`'s `HoldEvent` (one schema, an event/action field
/// distinguishing sub-events) rather than a second schema constant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DispositionAction {
    Propose,
    Execute,
}

/// Everything needed to decide, and to explain, one record's
/// disposition proposal — assembled by the HTTP layer from the
/// ledger's actual current state, never from caller-supplied claims.
#[derive(Debug, Clone)]
pub struct RecordDisposalContext {
    pub content_sha256: String,
    pub retention_class: String,
    pub expires: Option<DateTime<Utc>>,
    pub on_hold: bool,
    pub blocking_hold_ids: Vec<String>,
    /// True if any hold whose scope could cover this record was
    /// lifted within the last 12 months. See `hold_lifted_within`.
    pub hold_lifted_within_12_months: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DispositionRecord {
    pub schema: String,
    pub content_sha256: String,
    pub trigger: DispositionTrigger,
    pub decision: DispositionDecision,
    /// The specific legal basis at decision time (Decisions locked
    /// #6) — never just the decision enum by itself.
    pub legal_basis: String,
    pub notice_window_days: i64,
    pub proposed_at: DateTime<Utc>,
    /// `Some` only when `decision == Granted`. Phase 4's `execute`
    /// must still re-check eligibility at this time regardless of what
    /// this field says — a hold placed during the window overrides it
    /// (Decisions locked #7: "re-checked at execute time, not just at
    /// propose time").
    pub executable_after: Option<DateTime<Utc>>,
    /// Hex-encoded 32-byte Ed25519 public key of the configured
    /// disposal authority that signed this proposal.
    pub authority_pubkey: String,
    /// `Propose` for a `propose` call, `Execute` for an `execute` call.
    /// `#[serde(default)]` so this deserializes cleanly even against a
    /// record shape from before this field existed (none are known to
    /// exist outside throwaway test instances, but the load-bearing
    /// rule in this codebase is "don't assume, default safely").
    #[serde(default)]
    pub action: DispositionAction,
    /// `Some` only on an `Execute` record — when the actual unlink was
    /// attempted, and its outcome recorded alongside the decision.
    #[serde(default)]
    pub executed_at: Option<DateTime<Utc>>,
    /// `Some` only on an `Execute` record: `true` if the blob was
    /// actually removed (or was already absent — idempotent), `false`
    /// if the re-checked decision came back `Withheld` and nothing was
    /// deleted.
    #[serde(default)]
    pub blob_deleted: Option<bool>,
}

impl Default for DispositionAction {
    /// Pre-Phase-4 records (propose-only) had no `action` field at
    /// all; they were always proposals. Never used for a real
    /// `execute` record — those always set it explicitly.
    fn default() -> Self {
        DispositionAction::Propose
    }
}

/// Build the exact legal-basis string for one decision. An active
/// hold takes priority over both trigger-specific shapes — it
/// suspends both triggers identically (Decisions locked #6), and the
/// suspension itself must be logged under the same reasoning
/// requirement, not silently folded into a generic "withheld."
fn legal_basis(
    trigger: DispositionTrigger,
    decision: DispositionDecision,
    ctx: &RecordDisposalContext,
) -> String {
    if ctx.on_hold {
        return format!(
            "withheld: litigation hold(s) {} active on this record — suspends both the \
             retention-expiry and GDPR erasure-request triggers identically",
            ctx.blocking_hold_ids.join(", ")
        );
    }
    match (trigger, decision) {
        (DispositionTrigger::RetentionExpiry, DispositionDecision::Granted) => {
            let when = ctx
                .expires
                .map(|d| d.to_rfc3339())
                .unwrap_or_else(|| "unknown".to_string());
            format!(
                "retention-class {} expired {} per this deployment's retention-classes.csv schedule",
                ctx.retention_class, when
            )
        }
        (DispositionTrigger::RetentionExpiry, DispositionDecision::Withheld) => match ctx.expires {
            Some(exp) => format!(
                "withheld: retention-class {} has not yet expired, retained until {}",
                ctx.retention_class,
                exp.to_rfc3339()
            ),
            None => format!(
                "withheld: retention-class {} carries no expiry (permanent record)",
                ctx.retention_class
            ),
        },
        (DispositionTrigger::GdprErasureRequest, DispositionDecision::Granted) => {
            "granted: Art 17(1), no retention exemption applies".to_string()
        }
        (DispositionTrigger::GdprErasureRequest, DispositionDecision::Withheld) => match ctx.expires
        {
            Some(exp) => format!(
                "withheld: Art 17(3)(b), exempted under retention-class {}, retained until {}",
                ctx.retention_class,
                exp.to_rfc3339()
            ),
            None => format!(
                "withheld: Art 17(3)(b), exempted under retention-class {} (no expiry — permanent record)",
                ctx.retention_class
            ),
        },
    }
}

/// Decide granted/withheld purely from the record's actual current
/// state — never accepted as caller input (Decisions locked #6: a
/// state flag alone never satisfies the reasoning-logging
/// requirement; the "why" has to come from real state).
fn decide(
    trigger: DispositionTrigger,
    ctx: &RecordDisposalContext,
    now: DateTime<Utc>,
) -> DispositionDecision {
    if ctx.on_hold {
        return DispositionDecision::Withheld;
    }
    match trigger {
        DispositionTrigger::RetentionExpiry => match ctx.expires {
            Some(exp) if now >= exp => DispositionDecision::Granted,
            _ => DispositionDecision::Withheld,
        },
        DispositionTrigger::GdprErasureRequest => match ctx.expires {
            // Retention class still provides an active exemption.
            Some(exp) if now < exp => DispositionDecision::Withheld,
            // Already expired, or no retention class recorded at all
            // (e.g. a non-file structured record) — no exemption
            // applies either way.
            _ => DispositionDecision::Granted,
        },
    }
}

fn notice_window_days(
    trigger: DispositionTrigger,
    decision: DispositionDecision,
    ctx: &RecordDisposalContext,
) -> i64 {
    let gdpr_grant =
        trigger == DispositionTrigger::GdprErasureRequest && decision == DispositionDecision::Granted;
    if gdpr_grant || ctx.hold_lifted_within_12_months {
        EXTENDED_NOTICE_DAYS
    } else {
        STANDARD_NOTICE_DAYS
    }
}

/// Build a full disposition record for a propose call. Pure function
/// — no I/O, no signature verification (that belongs to the HTTP
/// layer, which owns the configured-authority set) — so it is
/// directly unit-testable against synthetic contexts.
pub fn propose(
    trigger: DispositionTrigger,
    ctx: &RecordDisposalContext,
    now: DateTime<Utc>,
    authority_pubkey_hex: &str,
) -> DispositionRecord {
    let decision = decide(trigger, ctx, now);
    let legal_basis = legal_basis(trigger, decision, ctx);
    let notice_window_days = notice_window_days(trigger, decision, ctx);
    let executable_after = match decision {
        DispositionDecision::Granted => Some(now + Duration::days(notice_window_days)),
        DispositionDecision::Withheld => None,
    };
    DispositionRecord {
        schema: DISPOSITION_SCHEMA.to_string(),
        content_sha256: ctx.content_sha256.clone(),
        trigger,
        decision,
        legal_basis,
        notice_window_days,
        proposed_at: now,
        executable_after,
        authority_pubkey: authority_pubkey_hex.to_string(),
        action: DispositionAction::Propose,
        executed_at: None,
        blob_deleted: None,
    }
}

/// Build an `execute` record for the most recent proposal on a
/// record. The decision is re-derived fresh against `ctx` (Phase 4's
/// re-check-at-execute-time requirement — a hold placed during the
/// notice window flips this to `Withheld` even though the original
/// proposal granted it). `blob_deleted` is filled in by the caller
/// after actually attempting the unlink (this function performs no
/// I/O) — pass `None` and set it afterward, or pass the outcome
/// directly if already known.
pub fn execute(
    original: &DispositionRecord,
    ctx: &RecordDisposalContext,
    now: DateTime<Utc>,
    authority_pubkey_hex: &str,
) -> DispositionRecord {
    let decision = decide(original.trigger, ctx, now);
    let legal_basis = legal_basis(original.trigger, decision, ctx);
    DispositionRecord {
        schema: DISPOSITION_SCHEMA.to_string(),
        content_sha256: ctx.content_sha256.clone(),
        trigger: original.trigger,
        decision,
        legal_basis,
        notice_window_days: original.notice_window_days,
        proposed_at: original.proposed_at,
        executable_after: original.executable_after,
        authority_pubkey: authority_pubkey_hex.to_string(),
        action: DispositionAction::Execute,
        executed_at: Some(now),
        blob_deleted: None,
    }
}

/// Find the most recent `propose`-action disposition record for a
/// given `content_sha256`, folding raw ledger entries directly (cursor
/// order — the last matching entry wins). Prior `execute` records for
/// the same hash are ignored here; `execute`'s own eligibility
/// re-check, not this lookup, is what prevents a meaningless repeat
/// deletion (an already-deleted blob's second unlink attempt is
/// treated as idempotent success, not specially blocked here).
pub fn latest_proposal(entries: &[Entry], content_sha256: &str) -> Option<DispositionRecord> {
    entries
        .iter()
        .filter_map(|e| parse_disposition_record(&e.payload))
        .rfind(|r| r.action == DispositionAction::Propose && r.content_sha256 == content_sha256)
}

/// Parse a single ledger entry's payload as a disposition record.
/// Returns `None` for any entry that isn't a well-formed
/// `fs-disposition-v1` record — the overwhelmingly common case when
/// folding a real ledger.
pub fn parse_disposition_record(payload: &serde_json::Value) -> Option<DispositionRecord> {
    if payload.get("schema").and_then(|s| s.as_str()) != Some(DISPOSITION_SCHEMA) {
        return None;
    }
    serde_json::from_value(payload.clone()).ok()
}

/// Was any hold whose scope could cover this record lifted within the
/// last `days` days? Scans raw ledger entries directly, not the
/// already-folded active-holds view — that view only answers "active
/// right now"; this needs the history of *lifted* events specifically,
/// which is exactly the case Decisions locked #7 widens the notice
/// window for.
pub fn hold_lifted_within(
    entries: &[Entry],
    content_sha256: &str,
    coa_stem: Option<&str>,
    created: DateTime<Utc>,
    now: DateTime<Utc>,
    days: i64,
) -> bool {
    let cutoff = now - Duration::days(days);
    entries.iter().any(|entry| {
        let Some(rec) = crate::hold::parse_hold_record(&entry.payload) else {
            return false;
        };
        rec.event == crate::hold::HoldEvent::Lifted
            && rec.timestamp >= cutoff
            && rec.scope.covers(content_sha256, coa_stem, created)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx(expires: Option<DateTime<Utc>>) -> RecordDisposalContext {
        RecordDisposalContext {
            content_sha256: "abc123".to_string(),
            retention_class: "finra-6y".to_string(),
            expires,
            on_hold: false,
            blocking_hold_ids: Vec::new(),
            hold_lifted_within_12_months: false,
        }
    }

    fn dt(s: &str) -> DateTime<Utc> {
        DateTime::parse_from_rfc3339(s).unwrap().with_timezone(&Utc)
    }

    #[test]
    fn retention_expiry_grants_when_already_expired() {
        let now = dt("2026-09-05T00:00:00Z");
        let c = ctx(Some(dt("2026-01-01T00:00:00Z")));
        let rec = propose(DispositionTrigger::RetentionExpiry, &c, now, "deadbeef");
        assert_eq!(rec.decision, DispositionDecision::Granted);
        assert!(rec.legal_basis.contains("expired"));
        assert_eq!(rec.notice_window_days, STANDARD_NOTICE_DAYS);
        assert_eq!(rec.executable_after, Some(now + Duration::days(7)));
    }

    #[test]
    fn retention_expiry_withholds_when_not_yet_expired() {
        let now = dt("2026-09-05T00:00:00Z");
        let c = ctx(Some(dt("2030-01-01T00:00:00Z")));
        let rec = propose(DispositionTrigger::RetentionExpiry, &c, now, "deadbeef");
        assert_eq!(rec.decision, DispositionDecision::Withheld);
        assert!(rec.legal_basis.contains("has not yet expired"));
        assert_eq!(rec.executable_after, None);
    }

    #[test]
    fn retention_expiry_withholds_for_no_expiry_class() {
        let now = dt("2026-09-05T00:00:00Z");
        let c = ctx(None);
        let rec = propose(DispositionTrigger::RetentionExpiry, &c, now, "deadbeef");
        assert_eq!(rec.decision, DispositionDecision::Withheld);
        assert!(rec.legal_basis.contains("no expiry"));
    }

    #[test]
    fn gdpr_erasure_grants_when_expired() {
        let now = dt("2026-09-05T00:00:00Z");
        let c = ctx(Some(dt("2026-01-01T00:00:00Z")));
        let rec = propose(DispositionTrigger::GdprErasureRequest, &c, now, "deadbeef");
        assert_eq!(rec.decision, DispositionDecision::Granted);
        assert_eq!(rec.legal_basis, "granted: Art 17(1), no retention exemption applies");
        assert_eq!(rec.notice_window_days, EXTENDED_NOTICE_DAYS, "GDPR grant widens the window");
    }

    #[test]
    fn gdpr_erasure_grants_when_no_retention_recorded_at_all() {
        let now = dt("2026-09-05T00:00:00Z");
        let c = ctx(None);
        let rec = propose(DispositionTrigger::GdprErasureRequest, &c, now, "deadbeef");
        assert_eq!(rec.decision, DispositionDecision::Granted, "no retention exemption applies");
    }

    #[test]
    fn gdpr_erasure_withholds_under_active_retention_exemption() {
        let now = dt("2026-09-05T00:00:00Z");
        let c = ctx(Some(dt("2030-01-01T00:00:00Z")));
        let rec = propose(DispositionTrigger::GdprErasureRequest, &c, now, "deadbeef");
        assert_eq!(rec.decision, DispositionDecision::Withheld);
        assert!(rec.legal_basis.starts_with("withheld: Art 17(3)(b)"));
        assert_eq!(rec.executable_after, None);
    }

    #[test]
    fn active_hold_withholds_regardless_of_trigger() {
        let now = dt("2026-09-05T00:00:00Z");
        let mut c = ctx(Some(dt("2026-01-01T00:00:00Z"))); // otherwise-expired
        c.on_hold = true;
        c.blocking_hold_ids = vec!["matter-42".to_string()];
        let rec = propose(DispositionTrigger::RetentionExpiry, &c, now, "deadbeef");
        assert_eq!(rec.decision, DispositionDecision::Withheld);
        assert!(rec.legal_basis.contains("litigation hold(s) matter-42"));
        assert_eq!(rec.executable_after, None);

        let rec2 = propose(DispositionTrigger::GdprErasureRequest, &c, now, "deadbeef");
        assert_eq!(rec2.decision, DispositionDecision::Withheld, "hold suspends the GDPR trigger too");
    }

    #[test]
    fn hold_lifted_recently_widens_window_even_for_retention_grant() {
        let now = dt("2026-09-05T00:00:00Z");
        let mut c = ctx(Some(dt("2026-01-01T00:00:00Z")));
        c.hold_lifted_within_12_months = true;
        let rec = propose(DispositionTrigger::RetentionExpiry, &c, now, "deadbeef");
        assert_eq!(rec.decision, DispositionDecision::Granted);
        assert_eq!(rec.notice_window_days, EXTENDED_NOTICE_DAYS);
    }

    #[test]
    fn disposition_record_round_trips_through_json() {
        let now = dt("2026-09-05T00:00:00Z");
        let c = ctx(Some(dt("2026-01-01T00:00:00Z")));
        let rec = propose(DispositionTrigger::RetentionExpiry, &c, now, "deadbeef");
        let payload = serde_json::to_value(&rec).unwrap();
        assert_eq!(payload["schema"], DISPOSITION_SCHEMA);
        let parsed = parse_disposition_record(&payload).expect("must parse back");
        assert_eq!(parsed.content_sha256, rec.content_sha256);
        assert_eq!(parsed.decision, rec.decision);
        assert_eq!(parsed.legal_basis, rec.legal_basis);
    }

    #[test]
    fn parse_disposition_record_rejects_other_schemas() {
        let payload = serde_json::json!({"schema": "fs-hold-v1", "event": "placed"});
        assert!(parse_disposition_record(&payload).is_none());
    }

    fn hold_entry(cursor: u64, event: &str, hold_id: &str, scope: serde_json::Value, ts: &str) -> Entry {
        Entry {
            cursor,
            payload_id: format!("p{cursor}"),
            payload: serde_json::json!({
                "schema": crate::hold::HOLD_SCHEMA,
                "event": event,
                "hold_id": hold_id,
                "scope": scope,
                "basis": "test",
                "actor": "test",
                "timestamp": ts,
            }),
            this_hash: "unused".to_string(),
        }
    }

    #[test]
    fn hold_lifted_within_detects_recent_lift_in_scope() {
        let entries = vec![hold_entry(
            1,
            "lifted",
            "matter-1",
            serde_json::json!({"content_sha256": "abc123"}),
            "2026-06-01T00:00:00Z",
        )];
        let now = dt("2026-09-05T00:00:00Z");
        assert!(hold_lifted_within(&entries, "abc123", None, now, now, 365));
        assert!(!hold_lifted_within(&entries, "other-hash", None, now, now, 365));
    }

    #[test]
    fn hold_lifted_within_ignores_lift_older_than_window() {
        let entries = vec![hold_entry(
            1,
            "lifted",
            "matter-1",
            serde_json::json!({"content_sha256": "abc123"}),
            "2024-01-01T00:00:00Z",
        )];
        let now = dt("2026-09-05T00:00:00Z");
        assert!(!hold_lifted_within(&entries, "abc123", None, now, now, 365));
    }

    #[test]
    fn hold_lifted_within_ignores_still_active_hold() {
        let entries = vec![hold_entry(
            1,
            "placed",
            "matter-1",
            serde_json::json!({"content_sha256": "abc123"}),
            "2026-06-01T00:00:00Z",
        )];
        let now = dt("2026-09-05T00:00:00Z");
        assert!(
            !hold_lifted_within(&entries, "abc123", None, now, now, 365),
            "a placed (not lifted) event must not count"
        );
    }

    // --- Phase 4: execute() and latest_proposal() ---

    fn disposition_entry(cursor: u64, rec: &DispositionRecord) -> Entry {
        Entry {
            cursor,
            payload_id: format!("p{cursor}"),
            payload: serde_json::to_value(rec).unwrap(),
            this_hash: "unused".to_string(),
        }
    }

    #[test]
    fn execute_grants_and_deletes_when_still_eligible() {
        let now = dt("2026-09-05T00:00:00Z");
        let c = ctx(Some(dt("2026-01-01T00:00:00Z")));
        let proposal = propose(DispositionTrigger::RetentionExpiry, &c, now, "deadbeef");
        assert_eq!(proposal.action, DispositionAction::Propose);

        let later = dt("2026-09-13T00:00:00Z"); // past the 7-day window
        let exec = execute(&proposal, &c, later, "deadbeef");
        assert_eq!(exec.action, DispositionAction::Execute);
        assert_eq!(exec.decision, DispositionDecision::Granted);
        assert_eq!(exec.executed_at, Some(later));
        assert_eq!(exec.proposed_at, proposal.proposed_at, "traceability back to the proposal");
        assert_eq!(exec.blob_deleted, None, "caller fills this in after the actual unlink");
    }

    #[test]
    fn execute_flips_to_withheld_if_hold_placed_during_window() {
        let now = dt("2026-09-05T00:00:00Z");
        let c = ctx(Some(dt("2026-01-01T00:00:00Z")));
        let proposal = propose(DispositionTrigger::RetentionExpiry, &c, now, "deadbeef");
        assert_eq!(proposal.decision, DispositionDecision::Granted);

        let mut c_at_execute = c.clone();
        c_at_execute.on_hold = true;
        c_at_execute.blocking_hold_ids = vec!["matter-99".to_string()];

        let later = dt("2026-09-13T00:00:00Z");
        let exec = execute(&proposal, &c_at_execute, later, "deadbeef");
        assert_eq!(
            exec.decision,
            DispositionDecision::Withheld,
            "a hold placed during the notice window must block execute even though propose granted it"
        );
        assert!(exec.legal_basis.contains("litigation hold(s) matter-99"));
    }

    #[test]
    fn latest_proposal_finds_most_recent_matching_propose_record() {
        let now = dt("2026-09-05T00:00:00Z");
        let c_a = RecordDisposalContext { content_sha256: "aa".to_string(), ..ctx(Some(now)) };
        let c_b = RecordDisposalContext { content_sha256: "bb".to_string(), ..ctx(Some(now)) };

        let first_a = propose(DispositionTrigger::RetentionExpiry, &c_a, dt("2026-09-01T00:00:00Z"), "k1");
        let for_b = propose(DispositionTrigger::RetentionExpiry, &c_b, dt("2026-09-02T00:00:00Z"), "k1");
        let second_a = propose(DispositionTrigger::GdprErasureRequest, &c_a, dt("2026-09-03T00:00:00Z"), "k1");

        let entries = vec![
            disposition_entry(1, &first_a),
            disposition_entry(2, &for_b),
            disposition_entry(3, &second_a),
        ];

        let found = latest_proposal(&entries, "aa").expect("must find a proposal for aa");
        assert_eq!(found.trigger, DispositionTrigger::GdprErasureRequest, "the later of the two aa proposals");
        assert_eq!(latest_proposal(&entries, "bb").unwrap().trigger, DispositionTrigger::RetentionExpiry);
        assert!(latest_proposal(&entries, "cc").is_none());
    }

    #[test]
    fn latest_proposal_ignores_execute_records() {
        let now = dt("2026-09-05T00:00:00Z");
        let c = ctx(Some(now));
        let proposal = propose(DispositionTrigger::RetentionExpiry, &c, now, "k1");
        let exec = execute(&proposal, &c, dt("2026-09-13T00:00:00Z"), "k1");

        let entries = vec![disposition_entry(1, &proposal), disposition_entry(2, &exec)];
        let found = latest_proposal(&entries, &c.content_sha256).unwrap();
        assert_eq!(found.action, DispositionAction::Propose, "must not return the execute record");
    }

    #[test]
    fn disposition_action_defaults_to_propose_when_field_missing() {
        // Simulates a record shape from before `action` existed.
        let payload = serde_json::json!({
            "schema": DISPOSITION_SCHEMA,
            "content_sha256": "abc",
            "trigger": "retention-expiry",
            "decision": "granted",
            "legal_basis": "test",
            "notice_window_days": 7,
            "proposed_at": "2026-09-05T00:00:00Z",
            "executable_after": "2026-09-12T00:00:00Z",
            "authority_pubkey": "deadbeef",
        });
        let rec = parse_disposition_record(&payload).expect("must parse");
        assert_eq!(rec.action, DispositionAction::Propose);
    }
}
