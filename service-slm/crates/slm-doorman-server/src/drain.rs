// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Drain-worker decision logic, extracted from `main.rs` so it is unit-testable.
//!
//! The brief-queue drain worker dequeues one shadow brief per cycle and must
//! decide whether to dispatch it to the apprentice (Tier A/B inference) or skip
//! it. The skip case exists because of a real production stall: on 2026-06-01 a
//! `git-commit` brief captured with an empty `actual_diff` was dispatched to
//! OLMo, which never terminated on the out-of-distribution prompt and wedged the
//! serial drain for 2.5 hours. An empty `actual_diff` carries no ground-truth
//! reference, so the apprentice's output could never be a useful training tuple
//! — the only correct action is to skip it before any inference call.
//!
//! Keeping this decision in a pure function (no I/O, no async) lets the
//! regression be asserted directly in unit tests; `main.rs`'s inline loop and
//! the `drain_worker_integration` test both route through it.

use crate::queue::ShadowQueueEntry;

/// What the drain worker should do with a dequeued shadow brief.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DrainDecision {
    /// `actual_diff` is empty/whitespace — no training signal possible. Move the
    /// brief straight to `queue-done/` without dispatching to any inference tier.
    Skip,
    /// `actual_diff` is present — dispatch to the apprentice for a shadow attempt.
    Dispatch,
}

/// Decide whether a dequeued brief should be skipped or dispatched.
///
/// An `actual_diff` that is empty or whitespace-only yields [`DrainDecision::Skip`]
/// — this is the guard against the empty-diff drain stall. Anything else yields
/// [`DrainDecision::Dispatch`].
pub fn classify_shadow_brief(entry: &ShadowQueueEntry) -> DrainDecision {
    if entry.actual_diff.trim().is_empty() {
        DrainDecision::Skip
    } else {
        DrainDecision::Dispatch
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::queue::ShadowQueueEntry;
    use slm_core::{ApprenticeshipBrief, BriefScope, SeniorRole};

    fn brief() -> ApprenticeshipBrief {
        ApprenticeshipBrief {
            brief_id: "01J9TESTBRIEF0000000000000".into(),
            created: chrono::Utc::now(),
            senior_role: SeniorRole::Master,
            senior_identity: "pwoodfine".into(),
            task_type: "git-commit".into(),
            scope: BriefScope {
                cluster: None,
                files: vec![],
            },
            acceptance_test: String::new(),
            doctrine_citations: vec![],
            shadow: true,
            body: "git-commit diff: some change".into(),
        }
    }

    fn entry_with(diff: &str) -> ShadowQueueEntry {
        ShadowQueueEntry {
            brief: brief(),
            actual_diff: diff.to_string(),
        }
    }

    #[test]
    fn classify_empty_diff_skips() {
        // The exact 2.5h-stall regression: empty actual_diff must never reach OLMo.
        assert_eq!(classify_shadow_brief(&entry_with("")), DrainDecision::Skip);
    }

    #[test]
    fn classify_whitespace_diff_skips() {
        assert_eq!(
            classify_shadow_brief(&entry_with("  \n\t  ")),
            DrainDecision::Skip
        );
    }

    #[test]
    fn classify_real_diff_dispatches() {
        let diff = "diff --git a/x b/x\n--- a/x\n+++ b/x\n@@ -1 +1 @@\n-old\n+new\n";
        assert_eq!(
            classify_shadow_brief(&entry_with(diff)),
            DrainDecision::Dispatch
        );
    }
}
