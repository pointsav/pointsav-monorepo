// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Golden-fixture suite (SPEC-journal-wiki-render-contract.md §0.5): locks
//! citation numbering and reference formatting to a normalized text summary
//! any conforming renderer — this crate's Rust engine, or project-gis's
//! independent Python one — can reproduce and diff byte-for-byte, without
//! requiring the two to share render code or match raw HTML whitespace and
//! attribute order (a full-page HTML diff would be meaningless across two
//! completely different template systems).
//!
//! Fixture location is crate-local for now (`tests/fixtures/journal/`), not
//! yet the shared workspace path SPEC §0's "one copy, both renderers read
//! it" model calls for — that placement is an open cross-project question
//! (parallel to the still-open notice-text-data-source question, see
//! `notice_banner`'s doc comment), not decided unilaterally here.

use app_mediakit_knowledge::citations::CitationRegistry;
use app_mediakit_knowledge::content::{parse, resolve_citations};

fn fixture_path(name: &str) -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/journal")
        .join(name)
}

/// Build the normalized golden-comparable summary for one fixture: citation
/// ids in first-appearance order, then the References list each would
/// generate (SPEC §1.4) — the two things SPEC §0's conformance surface
/// requires every renderer to agree on. A fixture with an unresolved
/// citation is a fixture bug (a golden case must be fully resolvable), so
/// that's an assertion failure here, not part of the comparable summary.
fn golden_summary(source_md: &str, registry: &CitationRegistry) -> String {
    let doc = parse(source_md);
    let (_, order, unresolved) = resolve_citations(&doc.body_md, registry);
    assert!(
        unresolved.is_empty(),
        "fixture must not contain unresolved citations: {unresolved:?}"
    );
    let mut out = String::new();
    out.push_str(&format!("citations_in_order: {}\n", order.join(", ")));
    out.push_str("references:\n");
    for (i, id) in order.iter().enumerate() {
        let entry = registry.get(id).expect("resolved above");
        out.push_str(&format!("{}. {}\n", i + 1, entry.bibliography_line()));
    }
    out
}

#[test]
fn paper_1_matches_expected_normalized_summary() {
    let source = std::fs::read_to_string(fixture_path("paper-1.md")).unwrap();
    let registry = CitationRegistry::load(&fixture_path("paper-1.citations.yaml"));
    let expected = std::fs::read_to_string(fixture_path("paper-1.expected.txt")).unwrap();
    let actual = golden_summary(&source, &registry);
    assert_eq!(
        actual, expected,
        "golden fixture drift — if this change is intentional, update paper-1.expected.txt"
    );
}
