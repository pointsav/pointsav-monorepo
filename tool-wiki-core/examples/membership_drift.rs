//! Real-world drift check (BRIEF-tool-wiki plan Part 4 item 3 verification): crawl a wiki
//! content root and report every AUTO-GENERATED MEMBERSHIP bullet whose text no longer
//! matches its linked article's current `short_description`.
//!
//! Usage:
//!   `cargo run --example membership_drift -- <path-to-wiki-content-root>` (report only)
//!   `cargo run --example membership_drift -- <path> --apply --from-report <report.txt>`
//!     (report AND rewrite files -- see the safety gate below)
//!
//! `--apply` skips anything under `.archive/` (historical record, never auto-edited) and
//! refuses a fix whose `actual_line` text isn't unique within its file (ambiguous
//! replacement target -- safer to skip and report than to guess which occurrence).
//!
//! ## Safety gate (added after a 2026-09 finding: `--apply` had no guard at all -- a single
//! flag could bulk-rewrite files sight-unseen; the only reason a 682-fix pass was safe earlier
//! was that the harness classifier happened to block the flag and force a reviewed script
//! instead, an accident of the sandbox, not a tool invariant)
//!
//! `--apply` now REQUIRES `--from-report <path>`, naming a prior report-only run's saved
//! stdout. A human must have generated and (implicitly, by naming it) looked at that report
//! before any file gets written. Each fix is applied only if its exact
//! `(index_group, slug, actual_line)` triple appears verbatim in the named report -- a fix
//! that wasn't in the report a human reviewed (e.g. content changed between the report run
//! and the apply run) is skipped, not guessed at.

use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use tool_wiki_core::check_membership;

fn is_spanish(path: &Path) -> bool {
    path.file_name()
        .and_then(|n| n.to_str())
        .map(|n| n.ends_with(".es.md"))
        .unwrap_or(false)
}

/// Parses a saved report (this tool's own stdout from a prior report-only run) into the set
/// of `(index_group, slug, actual_line)` triples it showed a human. Only these exact,
/// previously-displayed findings are eligible for `--apply` to act on.
fn parse_report(report_text: &str) -> HashSet<(String, String, String)> {
    let mut allowed = HashSet::new();
    let mut current_group_slug: Option<(String, String)> = None;
    for line in report_text.lines() {
        let trimmed = line.trim_start();
        if let Some(rest) = trimmed.strip_prefix('[') {
            if let Some((group, slug)) = rest.split_once(']') {
                current_group_slug = Some((group.to_string(), slug.trim().to_string()));
            }
            continue;
        }
        if let Some(actual) = trimmed.strip_prefix("actual:") {
            let actual = actual.trim();
            if actual == "<missing>" {
                continue;
            }
            if let Some((group, slug)) = &current_group_slug {
                allowed.insert((group.clone(), slug.clone(), actual.to_string()));
            }
        }
    }
    allowed
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let root = args
        .get(1)
        .expect("usage: membership_drift <path> [--apply --from-report <report.txt>]")
        .clone();
    let apply = args.iter().any(|a| a == "--apply");
    let from_report_path = args
        .iter()
        .position(|a| a == "--from-report")
        .and_then(|i| args.get(i + 1))
        .cloned();

    if apply && from_report_path.is_none() {
        eprintln!(
            "Error: --apply requires --from-report <path-to-a-prior-report-only-run's-saved-output>.\n\
             Run this tool WITHOUT --apply first, save its stdout to a file, review it, then re-run\n\
             with --apply --from-report <that-file>. This gate exists so a human has actually seen\n\
             every finding before any file is rewritten -- see this file's module doc comment."
        );
        std::process::exit(1);
    }
    let allowed_from_report: Option<HashSet<(String, String, String)>> = from_report_path
        .as_ref()
        .map(|p| {
            let text = fs::read_to_string(p)
                .unwrap_or_else(|e| panic!("could not read --from-report file {p}: {e}"));
            parse_report(&text)
        });

    let result = tool_wiki_core::crawl(Path::new(&root)).expect("crawl failed");

    // Group documents by (parent directory, language), so each _index.md is only compared
    // against its own category's *same-language* siblings. EN and ES pairs share the same
    // `slug:` value (distinguished by file suffix and `paired_with:`, not by slug), so a
    // language-blind grouping silently collapses e.g. `commuter.md` and `commuter.es.md`
    // into one map entry -- found the hard way: an earlier version of this example reported
    // 209 "drifted" bullets that were entirely EN-vs-ES cross-matches, not real staleness.
    let mut by_dir_lang: HashMap<(PathBuf, bool), Vec<&tool_wiki_core::Document>> = HashMap::new();
    for doc in &result.documents {
        let dir = doc.path.parent().unwrap_or(Path::new("")).to_path_buf();
        let is_es = is_spanish(&doc.path);
        by_dir_lang.entry((dir, is_es)).or_default().push(doc);
    }

    let mut total_drift = 0usize;
    for doc in &result.documents {
        // Any document with an AUTO block is eligible, not just files literally named
        // `_index.md` -- the crawler doesn't assume where these blocks can appear.
        if doc.auto_generated_blocks.is_empty() {
            continue;
        }
        let dir = doc.path.parent().unwrap_or(Path::new("")).to_path_buf();
        let is_es = is_spanish(&doc.path);
        let siblings: Vec<tool_wiki_core::Document> = by_dir_lang
            .get(&(dir, is_es))
            .map(|v| v.iter().map(|d| (*d).clone()).collect())
            .unwrap_or_default();
        let drift = check_membership(doc, &siblings);
        if !drift.is_empty() {
            println!("{}:", doc.path.display());
            for d in &drift {
                println!("  [{}] {}", d.index_group, d.slug);
                println!("    actual:   {}", d.actual_line.as_deref().unwrap_or("<missing>"));
                println!("    expected: {}", d.expected_line);
            }
            total_drift += drift.len();

            if apply {
                apply_fixes(
                    Path::new(&root),
                    &doc.path,
                    &drift,
                    allowed_from_report.as_ref(),
                );
            }
        }
    }
    println!("\ntotal drifted bullets: {total_drift}");
}

fn apply_fixes(
    root: &Path,
    rel_path: &Path,
    drift: &[tool_wiki_core::MembershipDrift],
    allowed_from_report: Option<&HashSet<(String, String, String)>>,
) {
    let path_str = rel_path.to_string_lossy();
    if path_str.contains(".archive") {
        println!("  SKIP (archived, never auto-edited): {}", rel_path.display());
        return;
    }
    let abs_path = root.join(rel_path);
    let content = fs::read_to_string(&abs_path).expect("read for apply");
    let mut new_content = content.clone();
    let mut applied = 0usize;
    for d in drift {
        let Some(actual) = &d.actual_line else { continue };
        if let Some(allowed) = allowed_from_report {
            let key = (d.index_group.clone(), d.slug.clone(), actual.clone());
            if !allowed.contains(&key) {
                println!(
                    "  SKIP (not present in the named --from-report -- content changed since \
                     that report was generated; re-run the report and review again): {}",
                    d.slug
                );
                continue;
            }
        }
        let occurrences = new_content.matches(actual.as_str()).count();
        if occurrences != 1 {
            println!(
                "  SKIP (actual_line not unique in file, {occurrences} occurrences): {}",
                d.slug
            );
            continue;
        }
        new_content = new_content.replacen(actual.as_str(), &d.expected_line, 1);
        applied += 1;
    }
    if applied > 0 {
        fs::write(&abs_path, new_content).expect("write for apply");
        println!("  APPLIED {applied} fix(es) to {}", rel_path.display());
    }
}

#[cfg(test)]
mod safety_gate_tests {
    use super::*;

    #[test]
    fn parse_report_extracts_exact_triples() {
        let report = "\
governance/_index.md:
  [conflicts-and-required-disclosures] legal-proceedings
    actual:   - [[legal-proceedings]] — old description here.
    expected: - [[legal-proceedings]] — new description here.

total drifted bullets: 1
";
        let allowed = parse_report(report);
        assert_eq!(allowed.len(), 1);
        assert!(allowed.contains(&(
            "conflicts-and-required-disclosures".to_string(),
            "legal-proceedings".to_string(),
            "- [[legal-proceedings]] — old description here.".to_string(),
        )));
    }

    #[test]
    fn parse_report_skips_missing_actual_lines() {
        let report = "\
substrate/_index.md:
  [core] some-slug
    actual:   <missing>
    expected: - [[some-slug]] — a description.
";
        let allowed = parse_report(report);
        assert!(allowed.is_empty(), "a <missing> actual_line must never become an allowed fix target");
    }

    #[test]
    fn apply_skips_a_fix_not_present_in_the_named_report() {
        let allowed: HashSet<(String, String, String)> = HashSet::from([(
            "some-group".to_string(),
            "known-slug".to_string(),
            "- [[known-slug]] — the exact line the report showed.".to_string(),
        )]);
        let drift = vec![tool_wiki_core::MembershipDrift {
            index_group: "some-group".to_string(),
            slug: "unreported-slug".to_string(),
            actual_line: Some("- [[unreported-slug]] — a line never shown in any report.".to_string()),
            expected_line: "- [[unreported-slug]] — the corrected line.".to_string(),
        }];
        let dir = std::env::temp_dir().join(format!("membership-drift-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let rel = Path::new("probe.md");
        std::fs::write(dir.join(rel), "- [[unreported-slug]] — a line never shown in any report.\n").unwrap();
        apply_fixes(&dir, rel, &drift, Some(&allowed));
        let after = std::fs::read_to_string(dir.join(rel)).unwrap();
        assert!(
            after.contains("a line never shown in any report"),
            "a fix whose (index_group, slug, actual_line) triple isn't in the named report must \
             never be written, even though the drift-check logic found it independently"
        );
        std::fs::remove_dir_all(&dir).ok();
    }
}
