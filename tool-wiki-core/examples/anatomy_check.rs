//! Structural conformance check (`BRIEF-tool-wiki-core.md` roadmap item): per-article
//! anatomy checks that need no token engine --
//!   1. a lead paragraph before the first `##` heading (TOPIC-shaped articles only -- see
//!      `lead_paragraph_check_applies`),
//!   2. a `short_description` frontmatter field of reasonable length (40-400 characters --
//!      flags both empty/missing and suspiciously long),
//!   3. an `.es.md` pair with the same number of `##` headings as its English original -- a
//!      rough structural-parity signal, not exact translation checking.
//!
//! Usage: `cargo run --example anatomy_check -- <path-to-wiki-content-root>`

use std::collections::{BTreeMap, HashMap};
use std::env;
use std::path::Path;

const MIN_SHORT_DESCRIPTION_LEN: usize = 40;
const MAX_SHORT_DESCRIPTION_LEN: usize = 400;

/// `content_type` values whose articles are TOPIC-shaped and therefore genuinely owe a lead
/// paragraph before their first `##` heading (`guide-reference.md`'s TOPIC shape).
///
/// The lead-paragraph rule is a **TOPIC** rule, not a universal one. A GUIDE
/// (`content_type: how-to`) correctly opens with `## Prerequisites` per
/// `guide-how-to.md`'s Purpose-then-Prerequisites lead -- flagging it for "no lead
/// paragraph" was a false positive, not a finding (56 of them on
/// `media-knowledge-documentation` alone, every one a `how-to`). `guide` is accepted
/// alongside `how-to` defensively: the corpus uses `how-to` exclusively today (verified
/// 2026-09-06 across all three wikis -- 1232 `topic`, 62 `how-to`, 19 `page`, 2 `research`,
/// zero `guide`), but the artifact vocabulary calls these GUIDEs, so a future retag would
/// otherwise silently reintroduce the false positives.
///
/// `page` (static pages: `page-privacy`, `page-disclaimer`, `CONTRIBUTING`) and `research`
/// (JOURNAL papers, which open with an abstract block the render contract owns) are also
/// out of scope -- neither is governed by the TOPIC lead rule.
const LEAD_PARAGRAPH_CONTENT_TYPES: &[&str] = &["topic"];

/// True when the TOPIC lead-paragraph rule governs this article's `content_type`.
/// An article with no `content_type` at all is out of scope: the check cannot know which
/// shape rule applies, and guessing "topic" is what produced the GUIDE false positives.
fn lead_paragraph_check_applies(content_type: Option<&str>) -> bool {
    matches!(content_type, Some(ct) if LEAD_PARAGRAPH_CONTENT_TYPES.contains(&ct))
}

fn main() {
    let root = env::args().nth(1).expect("usage: anatomy_check <path>");
    let root = Path::new(&root);

    let crawl = tool_wiki_core::crawl(root).expect("crawl failed");
    let docs = &crawl.documents;
    println!("crawled {} documents\n", docs.len());

    // --- 1. Missing lead paragraph before the first `##` heading (TOPIC-shaped only) ---
    let mut no_lead: Vec<String> = Vec::new();
    let mut skipped_by_type: BTreeMap<&str, usize> = BTreeMap::new();
    for d in docs {
        let content_type = d.frontmatter.content_type();
        if !lead_paragraph_check_applies(content_type) {
            *skipped_by_type.entry(content_type.unwrap_or("<none>")).or_insert(0) += 1;
            continue;
        }
        let lead_text = match d.headings.iter().find(|h| h.level == 2) {
            Some(h2) => &d.body[..h2.offset],
            None => &d.body[..],
        };
        if lead_text.trim().is_empty() {
            no_lead.push(d.path.display().to_string());
        }
    }
    no_lead.sort();
    println!("=== articles with no lead paragraph before the first `##` heading ({}) ===", no_lead.len());
    for p in &no_lead {
        println!("  {p}");
    }
    let skipped_total: usize = skipped_by_type.values().sum();
    let breakdown: Vec<String> = skipped_by_type
        .iter()
        .map(|(ct, n)| format!("{ct}={n}"))
        .collect();
    println!(
        "  (lead-paragraph check is TOPIC-only; {skipped_total} non-TOPIC documents not \
         checked -- {})",
        if breakdown.is_empty() { "none".to_string() } else { breakdown.join(", ") }
    );

    // --- 2. `short_description` shape ---
    let mut missing_short_desc: Vec<String> = Vec::new();
    let mut bad_length: Vec<(String, usize)> = Vec::new();
    for d in docs {
        match d.frontmatter.get_str("short_description") {
            None => missing_short_desc.push(d.path.display().to_string()),
            Some(desc) => {
                let len = desc.chars().count();
                if len == 0 {
                    missing_short_desc.push(d.path.display().to_string());
                } else if !(MIN_SHORT_DESCRIPTION_LEN..=MAX_SHORT_DESCRIPTION_LEN).contains(&len) {
                    bad_length.push((d.path.display().to_string(), len));
                }
            }
        }
    }
    missing_short_desc.sort();
    bad_length.sort();
    println!(
        "\n=== articles with no (or empty) `short_description` ({}) ===",
        missing_short_desc.len()
    );
    for p in &missing_short_desc {
        println!("  {p}");
    }
    println!(
        "\n=== `short_description` outside {MIN_SHORT_DESCRIPTION_LEN}-{MAX_SHORT_DESCRIPTION_LEN} chars ({}) ===",
        bad_length.len()
    );
    for (p, len) in &bad_length {
        let note = if *len < MIN_SHORT_DESCRIPTION_LEN { "too short" } else { "too long" };
        println!("  {p} -- {len} chars ({note})");
    }

    // --- 3. EN/ES `##` heading-count parity ---
    let h2_counts: HashMap<String, usize> = docs
        .iter()
        .map(|d| {
            let key = d.path.to_string_lossy().replace('\\', "/");
            let count = d.headings.iter().filter(|h| h.level == 2).count();
            (key, count)
        })
        .collect();

    let mut parity_mismatches: Vec<(String, usize, usize)> = Vec::new();
    for d in docs {
        let path_str = d.path.to_string_lossy().replace('\\', "/");
        let Some(stem) = path_str.strip_suffix(".es.md") else { continue };
        let en_path = format!("{stem}.md");
        let Some(&en_count) = h2_counts.get(&en_path) else { continue }; // no EN pair -- reported by category_census
        let es_count = h2_counts[&path_str];
        if en_count != es_count {
            parity_mismatches.push((path_str.clone(), en_count, es_count));
        }
    }
    parity_mismatches.sort();
    println!(
        "\n=== `.es.md` `##` heading-count parity mismatches vs. English pair ({}) ===",
        parity_mismatches.len()
    );
    for (es_path, en_count, es_count) in &parity_mismatches {
        println!("  {es_path} -- English has {en_count} `##` headings, Spanish has {es_count}");
    }
}

#[cfg(test)]
mod lead_paragraph_scope_tests {
    use super::*;
    use std::path::PathBuf;

    fn lead_is_empty(content: &str) -> bool {
        let d = tool_wiki_core::Document::parse(PathBuf::from("probe.md"), content).unwrap();
        if !lead_paragraph_check_applies(d.frontmatter.content_type()) {
            return false; // out of scope -- never reported
        }
        let lead_text = match d.headings.iter().find(|h| h.level == 2) {
            Some(h2) => &d.body[..h2.offset],
            None => &d.body[..],
        };
        lead_text.trim().is_empty()
    }

    /// The real corpus shape that produced the false positives: a GUIDE opening directly on
    /// `## Prerequisites`, exactly as `guide-how-to.md` requires.
    const GUIDE_NO_LEAD: &str = "---\nschema: foundry-doc-v1\ntitle: \"Add a node to a running fleet\"\nslug: add-a-fleet-node\ncategory: how-to\ncontent_type: how-to\n---\n\n## Prerequisites\n\n- A running fleet controller.\n\n## Steps\n\n1. Do the thing.\n";

    #[test]
    fn how_to_opening_on_prerequisites_is_not_flagged() {
        assert!(!lead_is_empty(GUIDE_NO_LEAD));
    }

    #[test]
    fn guide_content_type_is_also_out_of_scope() {
        let content = GUIDE_NO_LEAD.replace("content_type: how-to", "content_type: guide");
        assert!(!lead_is_empty(&content));
    }

    #[test]
    fn topic_with_no_lead_is_still_flagged() {
        let content = "---\ntitle: \"X\"\nslug: x\ncontent_type: topic\n---\n\n## First Section\n\nBody.\n";
        assert!(lead_is_empty(content), "a real TOPIC defect must survive the content_type filter");
    }

    #[test]
    fn topic_with_a_lead_is_not_flagged() {
        let content = "---\ntitle: \"X\"\nslug: x\ncontent_type: topic\n---\n\nA real lead paragraph.\n\n## First Section\n\nBody.\n";
        assert!(!lead_is_empty(content));
    }

    #[test]
    fn page_and_research_and_untyped_are_out_of_scope() {
        for ct in ["page", "research"] {
            let content = format!("---\ntitle: \"X\"\nslug: x\ncontent_type: {ct}\n---\n\n## Only Heading\n");
            assert!(!lead_is_empty(&content), "content_type `{ct}` must not be lead-checked");
        }
        let untyped = "---\ntitle: \"X\"\nslug: x\n---\n\n## Only Heading\n";
        assert!(!lead_is_empty(untyped), "an untyped document must not be guessed into TOPIC scope");
    }
}
