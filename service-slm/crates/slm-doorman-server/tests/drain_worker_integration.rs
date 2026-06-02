// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Drain-worker end-to-end integration tests.
//
// These drive the real file-backed queue (enqueue → dequeue → classify →
// release) and assert the brief lands in the correct bucket. The headline test
// is the regression for the 2.5 h empty-diff stall: a brief with an empty
// actual_diff must drain straight to queue-done/ WITHOUT ever being dispatched
// to an inference tier.
//
// The tests own the tempdir base path, so they assert file locations directly
// via std::fs without needing the queue module's private dir accessors.

use slm_core::{ApprenticeshipBrief, BriefScope, SeniorRole};
use slm_doorman_server::drain::{classify_shadow_brief, DrainDecision};
use slm_doorman_server::queue::{
    dequeue_shadow, enqueue_shadow, release_shadow, QueueConfig, ReleaseOutcome, ShadowQueueEntry,
};
use std::time::{SystemTime, UNIX_EPOCH};

fn tmp_base(label: &str) -> std::path::PathBuf {
    std::env::temp_dir().join(format!(
        "drain-it-{label}-{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)
    ))
}

fn brief(id: &str) -> ApprenticeshipBrief {
    ApprenticeshipBrief {
        brief_id: id.to_string(),
        created: chrono::Utc::now(),
        senior_role: SeniorRole::Master,
        senior_identity: "ps-administrator".to_string(),
        task_type: "git-commit".to_string(),
        scope: BriefScope {
            cluster: None,
            files: vec![],
        },
        acceptance_test: String::new(),
        doctrine_citations: vec![],
        shadow: true,
        body: "git-commit diff: some change".to_string(),
    }
}

fn count_dir(p: &std::path::Path) -> usize {
    std::fs::read_dir(p).map(|d| d.count()).unwrap_or(0)
}

/// THE regression: a brief with an empty actual_diff drains to queue-done/
/// without ever being dispatched. Models the drain loop's skip branch exactly.
#[test]
fn empty_diff_brief_drains_to_done_without_dispatch() {
    let base = tmp_base("empty-skip");
    let cfg = QueueConfig::with_base_dir(&base);
    let entry = ShadowQueueEntry {
        brief: brief("01J9DRAINEMPTY00000000001"),
        actual_diff: String::new(), // the empty-diff stall trigger
    };

    enqueue_shadow(&cfg, &entry).expect("enqueue");
    let leased = dequeue_shadow(&cfg, "w0")
        .expect("dequeue ok")
        .expect("brief present");

    // The drain worker would consult classify_shadow_brief here; it MUST skip,
    // so no dispatcher / inference tier is constructed at all.
    assert_eq!(
        classify_shadow_brief(&leased.entry),
        DrainDecision::Skip,
        "empty actual_diff must classify as Skip (no OLMo dispatch)"
    );

    // Skip path → release as Done.
    release_shadow(&cfg, &leased, ReleaseOutcome::Done).expect("release done");

    assert_eq!(
        count_dir(&base.join("queue-done")),
        1,
        "empty-diff brief must land in queue-done/"
    );
    assert_eq!(
        count_dir(&base.join("queue-in-flight")),
        0,
        "no lease should remain in-flight"
    );
    let _ = std::fs::remove_dir_all(&base);
}

/// A brief with a real diff classifies as Dispatch (the drain would then call
/// the apprentice; that path is covered by slm-doorman apprenticeship tests).
#[test]
fn real_diff_brief_classifies_as_dispatch() {
    let base = tmp_base("real-dispatch");
    let cfg = QueueConfig::with_base_dir(&base);
    let entry = ShadowQueueEntry {
        brief: brief("01J9DRAINREAL000000000001"),
        actual_diff: "diff --git a/x b/x\n--- a/x\n+++ b/x\n@@ -1 +1 @@\n-a\n+b\n".to_string(),
    };

    enqueue_shadow(&cfg, &entry).expect("enqueue");
    let leased = dequeue_shadow(&cfg, "w0")
        .expect("dequeue ok")
        .expect("brief present");

    assert_eq!(
        classify_shadow_brief(&leased.entry),
        DrainDecision::Dispatch,
        "non-empty actual_diff must classify as Dispatch"
    );

    // Simulate a successful dispatch outcome.
    release_shadow(&cfg, &leased, ReleaseOutcome::Done).expect("release done");
    assert_eq!(count_dir(&base.join("queue-done")), 1);
    let _ = std::fs::remove_dir_all(&base);
}

/// A failed dispatch routed to Poison lands in queue-poison/ (release routing).
#[test]
fn poison_outcome_routes_to_poison_bucket() {
    let base = tmp_base("poison");
    let cfg = QueueConfig::with_base_dir(&base);
    let entry = ShadowQueueEntry {
        brief: brief("01J9DRAINPOISON0000000001"),
        actual_diff: "diff --git a/x b/x\n+y\n".to_string(),
    };

    enqueue_shadow(&cfg, &entry).expect("enqueue");
    let leased = dequeue_shadow(&cfg, "w0")
        .expect("dequeue ok")
        .expect("brief present");
    release_shadow(&cfg, &leased, ReleaseOutcome::Poison).expect("release poison");

    assert_eq!(
        count_dir(&base.join("queue-poison")),
        1,
        "poison outcome must land in queue-poison/"
    );
    assert_eq!(count_dir(&base.join("queue-done")), 0);
    let _ = std::fs::remove_dir_all(&base);
}

/// A batch of mixed briefs all resolve to a terminal bucket (none left stuck
/// in-flight) — the drain orchestration sanity check.
#[test]
fn batch_drains_all_briefs_no_leftovers() {
    let base = tmp_base("batch");
    let cfg = QueueConfig::with_base_dir(&base);

    for i in 0..6 {
        let diff = if i % 2 == 0 {
            String::new() // empty → skip → done
        } else {
            format!("diff --git a/f{i} b/f{i}\n+line\n") // real → dispatch → done
        };
        let entry = ShadowQueueEntry {
            brief: brief(&format!("01J9DRAINBATCH00000000{i:03}")),
            actual_diff: diff,
        };
        enqueue_shadow(&cfg, &entry).expect("enqueue");
    }

    let mut processed = 0;
    while let Some(leased) = dequeue_shadow(&cfg, "w0").expect("dequeue ok") {
        // Skip empties, "dispatch" reals — both terminate as Done here.
        let _ = classify_shadow_brief(&leased.entry);
        release_shadow(&cfg, &leased, ReleaseOutcome::Done).expect("release");
        processed += 1;
    }

    assert_eq!(processed, 6, "all 6 briefs must be processed");
    assert_eq!(count_dir(&base.join("queue-done")), 6);
    assert_eq!(
        count_dir(&base.join("queue-in-flight")),
        0,
        "no briefs left stuck in-flight"
    );
    assert_eq!(count_dir(&base.join("queue")), 0, "queue/ drained empty");
    let _ = std::fs::remove_dir_all(&base);
}
