//! BRIEF-index drift check (`BRIEF-tool-wiki-core.md` roadmap item). Checks every
//! `.agent/briefs/*.md` file's frontmatter (`brief-id`, `status`, `parent`, `related_briefs`)
//! for index drift, dangling cross-references, and soft-cap overrun.
//!
//! Usage: `cargo run --example brief_ledger -- <path-to-.agent-dir-or-briefs-dir>`
//!
//! Unlike the wiki-content checks, this one does NOT take a wiki content root — it takes
//! either an archive's `.agent/` directory (in which case it looks for a `briefs/`
//! subdirectory) or a `briefs/` directory directly. `.agent/briefs/archive/` is always
//! excluded, per the standing rule that archive/ holds foreign/misfiled BRIEFs from other
//! archives, not this archive's own work.
//!
//! **Deviation from the original spec, deliberate, evidence-based:** the original design
//! (`.agent/rules/brief-discipline.md`, and this check's own originating request) says to
//! check disk files against `.agent/briefs/README.md`'s active-briefs table. As of
//! 2026-09-04 that table no longer exists — `README.md` explicitly states it drifted five
//! times under hand-maintenance and was replaced by `BRIEF-editorial-programme-master.md`'s
//! `## §Ledger` section, which `BRIEF-tool-wiki-core.md` names as this exact check's real
//! target. Checking a table that has been intentionally deleted would report nothing;
//! checking the real, current authoritative source is what the spec-behind-the-spec (the
//! BRIEF that commissioned this tool) actually asks for. If a future session restores a
//! literal README table, the lookup in `main()` below (currently: find
//! `BRIEF-editorial-programme-master.md` and parse its `## §Ledger` section) should be
//! extended to prefer it.
//!
//! The `## §Ledger` section is semi-structured: several `###` subsections, some carrying a
//! markdown table (each with its *own* column headers -- `brief-id | Why active` for "Kept
//! active", `brief-id | Live edge now tracked at` for "Demoted to reference"), others
//! carrying a bare prose list of ids. Extraction is therefore per-subsection:
//!
//!   * A subsection **containing any table** contributes ids from its **table rows only**,
//!     and only from each row's **first cell** (the `brief-id` column). Every table under
//!     `## §Ledger` is parsed, not just the first one, and each inherits its own
//!     subsection's status rule.
//!   * A subsection with **no table** ("Reference-grade", "Standalone runbook",
//!     "Superseded", "Resolved by evidence") is a prose list, and is scanned token-wise.
//!   * "Housekeeping" subsections (documenting `archive/`'s foreign BRIEFs) are skipped
//!     entirely — narrative, not ledger rows.
//!
//! Ids are matched by *shape* (see `looks_like_brief_id`), deliberately not filtered to
//! tokens matching a real on-disk `brief-id`, so a ledger citation of an id that does not
//! exist (a typo, a stale rename) is itself a finding rather than silently invisible. Each
//! reference's status comes from its subsection heading unless an explicit
//! `` `status: <word>` `` token appears on the same row/line ("Resolved by evidence" uses
//! exactly this pattern for entries whose disposition doesn't match the subsection default).
//!
//! **Why first-cell-only, and why per-subsection (fixed 2026-09-06).** The previous parser
//! scanned every line of the ledger section uniformly. That produced two false-positive
//! classes against the real master BRIEF:
//!   1. `### Kept active (...)`'s heading wraps onto several continuation lines, and those
//!      lines cite other brief-ids in prose. Under a uniform scan they inherited the
//!      "Kept active" default, so `project-editorial-artifact-triage-sweep` — correctly
//!      listed in the *Demoted to `reference`* table below — was recorded as `active` and
//!      reported as a status mismatch against its own (correct) `status: reference`
//!      frontmatter.
//!   2. A brief's "Why active" cell can cite a non-BRIEF slug in backticks; the JOURNAL slug
//!      `capability-secured-session-orchestration` was read as a ledger row and reported as
//!      "no matching file on disk".
//! Both vanish once only a table row's first cell counts as a row id.
//!
//! **The other reported symptom was not a parser bug at all.**
//! `project-editorial-jennifer-to-wiki-backfill` was reported as a ledger row with no file
//! on disk (and 9 `parent:`/`related_briefs:` references to it as dangling) even though
//! `BRIEF-jennifer-to-wiki-backfill.md` carries exactly that `brief-id`. Root cause: that
//! file's frontmatter was invalid YAML (a multi-line plain scalar containing `authoring: `,
//! which YAML reads as a nested mapping key), so the crawler put it in `skipped` and this
//! check silently dropped it. Silently is the defect: an unparseable file is invisible to
//! every check in this crate, and its absence then manifests as a *content* finding
//! somewhere else. `main()` now reports unparseable and frontmatter-less files as a
//! first-class finding before anything else.

use std::collections::{HashMap, HashSet};
use std::env;
use std::path::{Path, PathBuf};

use tool_wiki_core::document::DocumentError;
use tool_wiki_core::frontmatter::FrontmatterError;
use tool_wiki_core::Document;

const SOFT_CAP: usize = 5;
const LEDGER_HEADING: &str = "## §Ledger";
const MASTER_BRIEF_FILENAME: &str = "BRIEF-editorial-programme-master.md";

struct Brief {
    path: PathBuf,
    id: String,
    status: Option<String>,
    parent: Option<String>,
    related: Vec<String>,
}

fn main() {
    let root_arg = env::args()
        .nth(1)
        .expect("usage: brief_ledger <path-to-.agent-dir-or-briefs-dir>");
    let root = Path::new(&root_arg);
    let briefs_dir = if root.join("briefs").is_dir() {
        root.join("briefs")
    } else {
        root.to_path_buf()
    };

    let crawl = tool_wiki_core::crawl(&briefs_dir).expect("crawl failed");

    // Exclude archive/ by path -- foreign/misfiled BRIEFs, not this archive's own ledger.
    let docs: Vec<&Document> = crawl
        .documents
        .iter()
        .filter(|d| !is_under_archive(&d.path))
        .collect();

    let briefs: Vec<Brief> = docs
        .iter()
        .filter_map(|d| {
            let id = d.frontmatter.get_str("brief-id")?.to_string();
            let status = d.frontmatter.status().map(str::to_string);
            let parent = d.frontmatter.get_str("parent").map(str::to_string);
            let related = string_list(d.frontmatter.get("related_briefs"));
            Some(Brief { path: d.path.clone(), id, status, parent, related })
        })
        .collect();

    println!(
        "scanned {} files under {} ({} carry a brief-id; archive/ excluded)\n",
        docs.len(),
        briefs_dir.display(),
        briefs.len()
    );

    // --- 0. Files the crawler could not parse (reported first: an unparseable file is
    // invisible to every check below, and its absence surfaces as a bogus content finding
    // somewhere else). Only Invalid-YAML skips are defects; a file with no frontmatter at
    // all (README.md) is expected in a briefs directory. ---
    let mut unparseable: Vec<(String, String)> = Vec::new();
    let mut no_frontmatter = 0usize;
    for (path, err) in &crawl.skipped {
        if is_under_archive(path) {
            continue;
        }
        match err {
            DocumentError::Frontmatter(FrontmatterError::Missing) => no_frontmatter += 1,
            other => unparseable.push((path.display().to_string(), other.to_string())),
        }
    }
    unparseable.sort();
    println!(
        "=== files whose frontmatter could not be parsed ({}) ===",
        unparseable.len()
    );
    for (path, err) in &unparseable {
        println!("  {path} -- {err}");
        println!("    (invisible to every check below: no brief-id, no status, no cross-refs)");
    }
    if unparseable.is_empty() {
        println!("  none");
    }
    println!("  ({no_frontmatter} further file(s) carry no frontmatter block at all -- expected for README.md)");

    let known_ids: HashSet<&str> = briefs.iter().map(|b| b.id.as_str()).collect();
    let by_id: HashMap<&str, &Brief> = briefs.iter().map(|b| (b.id.as_str(), b)).collect();

    // --- Locate and parse the ledger source ---
    let master = docs
        .iter()
        .find(|d| d.path.file_name().and_then(|n| n.to_str()) == Some(MASTER_BRIEF_FILENAME));

    let Some(master) = master else {
        println!(
            "no {MASTER_BRIEF_FILENAME} found under {} -- cannot check against a ledger; \
             reporting frontmatter-only checks below.",
            briefs_dir.display()
        );
        report_frontmatter_only(&briefs, &known_ids);
        return;
    };

    let ledger_text = extract_ledger_section(&master.body);
    let Some(ledger_text) = ledger_text else {
        println!(
            "{MASTER_BRIEF_FILENAME} has no `{LEDGER_HEADING}` section -- cannot check ledger \
             rows; reporting frontmatter-only checks below."
        );
        report_frontmatter_only(&briefs, &known_ids);
        return;
    };

    let ledger = parse_ledger(&ledger_text);
    let ledger_status = ledger.status;

    if !ledger.duplicates.is_empty() {
        println!(
            "\n=== brief-ids cited in more than one §Ledger subsection ({}) ===",
            ledger.duplicates.len()
        );
        for (id, first, second) in &ledger.duplicates {
            println!("  `{id}` -- first subsection says `{first}`, a later one says `{second}` (first wins)");
        }
    }

    // --- 1. Files on disk with no ledger row ---
    let mut missing_from_ledger: Vec<&Brief> =
        briefs.iter().filter(|b| !ledger_status.contains_key(b.id.as_str())).collect();
    missing_from_ledger.sort_by(|a, b| a.id.cmp(&b.id));
    println!("=== files with no row in {MASTER_BRIEF_FILENAME}'s §Ledger ({}) ===", missing_from_ledger.len());
    for b in &missing_from_ledger {
        println!("  {} -- brief-id `{}`, frontmatter status `{}`", b.path.display(), b.id, b.status.as_deref().unwrap_or("<missing>"));
    }

    // --- 2. Ledger rows with no matching file ---
    let mut ledger_orphans: Vec<&String> =
        ledger_status.keys().filter(|id| !by_id.contains_key(id.as_str())).collect();
    ledger_orphans.sort();
    println!("\n=== §Ledger rows with no matching file on disk ({}) ===", ledger_orphans.len());
    for id in &ledger_orphans {
        println!("  `{id}` -- cited in the ledger but no brief-id on disk matches exactly");
    }

    // --- 3. Status mismatches ---
    let mut mismatches: Vec<(&str, &str, &str)> = Vec::new();
    for b in &briefs {
        if let Some(ledger_st) = ledger_status.get(b.id.as_str()) {
            let fm_st = b.status.as_deref().unwrap_or("<missing>");
            if !status_equivalent(ledger_st, fm_st) {
                mismatches.push((b.id.as_str(), ledger_st.as_str(), fm_st));
            }
        }
    }
    println!("\n=== status mismatches: §Ledger vs. frontmatter ({}) ===", mismatches.len());
    for (id, ledger_st, fm_st) in &mismatches {
        println!("  `{id}` -- ledger says `{ledger_st}`, frontmatter says `{fm_st}`");
    }

    // --- 4. Dangling parent:/related_briefs: ---
    let dangling = find_dangling(&briefs, &known_ids);
    println!("\n=== dangling parent:/related_briefs: references ({}) ===", dangling.len());
    for (from, field, target) in &dangling {
        println!("  {from} -- {field}: `{target}` -- no brief-id on disk matches");
    }

    // --- 5. Active count vs. soft cap ---
    let active: Vec<&Brief> = briefs.iter().filter(|b| b.status.as_deref() == Some("active")).collect();
    println!(
        "\n=== active-status count vs. soft cap of {SOFT_CAP} ({}) ===",
        active.len()
    );
    if active.len() > SOFT_CAP {
        println!("  OVER CAP by {} -- active briefs:", active.len() - SOFT_CAP);
    } else {
        println!("  within cap -- active briefs:");
    }
    let mut active_ids: Vec<&str> = active.iter().map(|b| b.id.as_str()).collect();
    active_ids.sort();
    for id in active_ids {
        let in_ledger = ledger_status.get(id).map(String::as_str);
        match in_ledger {
            Some(st) if st == "active" => println!("    `{id}` (ledger-confirmed active)"),
            Some(st) => println!("    `{id}` (ledger says `{st}` -- mismatch, see above)"),
            None => println!("    `{id}` (NOT in ledger at all -- see missing-from-ledger above)"),
        }
    }
}

fn report_frontmatter_only(briefs: &[Brief], known_ids: &HashSet<&str>) {
    let dangling = find_dangling(briefs, known_ids);
    println!("\n=== dangling parent:/related_briefs: references ({}) ===", dangling.len());
    for (from, field, target) in &dangling {
        println!("  {from} -- {field}: `{target}` -- no brief-id on disk matches");
    }
    let active_count = briefs.iter().filter(|b| b.status.as_deref() == Some("active")).count();
    println!("\nactive-status count: {active_count} (soft cap {SOFT_CAP})");
}

fn find_dangling<'a>(briefs: &'a [Brief], known_ids: &HashSet<&str>) -> Vec<(String, &'static str, &'a str)> {
    let mut out = Vec::new();
    for b in briefs {
        if let Some(parent) = &b.parent {
            if !known_ids.contains(parent.as_str()) {
                out.push((b.path.display().to_string(), "parent", parent.as_str()));
            }
        }
        for rel in &b.related {
            if !known_ids.contains(rel.as_str()) {
                out.push((b.path.display().to_string(), "related_briefs", rel.as_str()));
            }
        }
    }
    out
}

/// `archived` (a real status value) and `archive/`-the-path are unrelated; only the path
/// matters here. `status: archived` files that still live outside `archive/` (e.g.
/// `BRIEF-trademark-changeover-mcorp-capability-geometry.md`) are correctly still in scope.
fn is_under_archive(path: &Path) -> bool {
    path.components().any(|c| c.as_os_str() == "archive")
}

fn string_list(value: Option<&serde_yaml::Value>) -> Vec<String> {
    match value {
        Some(serde_yaml::Value::Sequence(seq)) => {
            seq.iter().filter_map(|v| v.as_str().map(str::to_string)).collect()
        }
        Some(serde_yaml::Value::String(s)) => vec![s.clone()],
        _ => Vec::new(),
    }
}

/// Slice out the `## §Ledger` ... (next `## ` heading) section from a document body.
fn extract_ledger_section(body: &str) -> Option<String> {
    let start = body.find(LEDGER_HEADING)?;
    let after_start = &body[start..];
    // Find the next top-level (`## `) heading after the first line of this one.
    let mut end = after_start.len();
    for (i, line) in after_start.match_indices('\n') {
        let line_start = i + 1;
        if line_start >= after_start.len() {
            break;
        }
        if after_start[line_start..].starts_with("## ") && line_start > 0 {
            end = line_start;
            break;
        }
        let _ = line;
    }
    Some(after_start[..end].to_string())
}

/// `None` if the id was seen with only an inferred subsection default;
/// used only inside `parse_ledger` bookkeeping (kept as plain `String` in the returned map).
#[derive(Clone, Copy, PartialEq, Eq)]
enum SectionRule {
    Default(&'static str),
    ExplicitOnly,
    Skip,
}

fn section_rule_for_heading(heading_lower: &str) -> SectionRule {
    if heading_lower.contains("housekeeping") {
        SectionRule::Skip
    } else if heading_lower.contains("resolved by evidence") {
        SectionRule::ExplicitOnly
    } else if heading_lower.contains("kept active") {
        SectionRule::Default("active")
    } else if heading_lower.contains("demoted to") {
        SectionRule::Default("reference")
    } else if heading_lower.contains("reference-grade") {
        SectionRule::Default("reference")
    } else if heading_lower.contains("standalone runbook") {
        SectionRule::Default("reference")
    } else if heading_lower.contains("superseded") {
        SectionRule::Default("superseded")
    } else {
        // An unrecognised subsection heading -- be conservative and require an explicit
        // `status:` token rather than guessing a default that might be wrong.
        SectionRule::ExplicitOnly
    }
}

/// Extract every backtick-quoted token on a line, in left-to-right order.
fn backtick_tokens(line: &str) -> Vec<&str> {
    line.split('`').skip(1).step_by(2).collect()
}

/// A backtick token that has the *shape* of a `brief-id` -- lowercase ASCII, digits, and
/// hyphens only, at least 3 hyphens. This is deliberately a shape check, not a check
/// against the known-ids set collected from frontmatter -- the whole point of this parser
/// is to also catch a ledger citing an id that does NOT exist on disk (or exists under a
/// different spelling), which an exact-match-only filter would silently miss. The 3-hyphen
/// floor is calibrated against the real corpus: every real `project-editorial-*` brief-id
/// has >=3 hyphens (`project-editorial-programme-master` is the shortest); this floor is
/// exactly high enough to exclude incidental hyphenated prose tokens like `` `tool-wiki-core` ``
/// (2 hyphens) and `` `brief_ledger` `` (no hyphens) that appear in the same ledger prose.
fn looks_like_brief_id(tok: &str) -> bool {
    if tok.is_empty() || tok.starts_with('-') || tok.ends_with('-') {
        return false;
    }
    let valid_chars = tok
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-');
    valid_chars && tok.matches('-').count() >= 3
}

fn parse_status_token(tok: &str) -> Option<String> {
    let after = tok.trim().strip_prefix("status:")?;
    let word: String = after
        .trim()
        .chars()
        .take_while(|c| c.is_alphanumeric())
        .collect();
    if word.is_empty() {
        None
    } else {
        Some(word.to_lowercase())
    }
}

/// A markdown pipe-table row (`| ... | ... |`), as opposed to prose. The `§Ledger` tables
/// all start their rows flush at column 0, but `trim_start` keeps this tolerant of indented
/// tables inside a list item.
fn is_table_row(line: &str) -> bool {
    line.trim_start().starts_with('|')
}

/// A table's `|---|---|` separator row -- structural, never a ledger row.
fn is_separator_row(line: &str) -> bool {
    let t = line.trim();
    is_table_row(t)
        && t.trim_matches('|')
            .split('|')
            .all(|cell| !cell.trim().is_empty() && cell.trim().chars().all(|c| c == '-' || c == ':'))
}

/// The first cell of a pipe-table row -- the `brief-id` column in every `§Ledger` table.
fn first_cell(line: &str) -> &str {
    line.trim().trim_start_matches('|').split('|').next().unwrap_or("")
}

/// One `### `-delimited subsection of the ledger. The text before the first `### ` heading
/// is returned with an empty heading (it is the section's own preamble prose).
struct LedgerSection<'a> {
    heading: &'a str,
    lines: Vec<&'a str>,
}

fn split_sections(ledger_text: &str) -> Vec<LedgerSection<'_>> {
    let mut sections = vec![LedgerSection { heading: "", lines: Vec::new() }];
    for line in ledger_text.lines() {
        if let Some(heading) = line.trim_start().strip_prefix("### ") {
            sections.push(LedgerSection { heading, lines: Vec::new() });
        } else {
            sections.last_mut().expect("always non-empty").lines.push(line);
        }
    }
    sections
}

/// The parsed `§Ledger`: each cited brief-id's status, plus any id cited from more than one
/// subsection (an ambiguity the old first-wins parser resolved silently and arbitrarily).
struct Ledger {
    status: HashMap<String, String>,
    /// `(id, first section's status, second section's status)`.
    duplicates: Vec<(String, String, String)>,
}

fn parse_ledger(ledger_text: &str) -> Ledger {
    let mut status: HashMap<String, String> = HashMap::new();
    let mut duplicates: Vec<(String, String, String)> = Vec::new();

    let record = |id: &str, st: String, dups: &mut Vec<(String, String, String)>,
                  status: &mut HashMap<String, String>| {
        match status.get(id) {
            Some(existing) if *existing != st => {
                dups.push((id.to_string(), existing.clone(), st));
            }
            Some(_) => {}
            None => {
                status.insert(id.to_string(), st);
            }
        }
    };

    for section in split_sections(ledger_text) {
        let rule = section_rule_for_heading(&section.heading.to_lowercase());
        if rule == SectionRule::Skip {
            continue;
        }
        let has_table = section.lines.iter().any(|l| is_table_row(l));

        if has_table {
            // Table subsection: every row of every table under this heading, first cell only.
            for line in &section.lines {
                if !is_table_row(line) || is_separator_row(line) {
                    continue;
                }
                let Some(id) = backtick_tokens(first_cell(line))
                    .into_iter()
                    .find(|t| looks_like_brief_id(t))
                else {
                    continue; // header row (`| brief-id | Why active |`) or an unquoted cell
                };
                // An explicit `status: <word>` anywhere in the row overrides the default.
                let explicit = backtick_tokens(line).into_iter().find_map(parse_status_token);
                let st = match (explicit, rule) {
                    (Some(explicit), _) => Some(explicit),
                    (None, SectionRule::Default(d)) => Some(d.to_string()),
                    (None, _) => None,
                };
                if let Some(st) = st {
                    record(id, st, &mut duplicates, &mut status);
                }
            }
        } else {
            // Prose-list subsection: token scan, an explicit status attaching to the id
            // most recently seen on the same line or the line above.
            let mut last_id: Option<String> = None;
            for line in &section.lines {
                for tok in backtick_tokens(line) {
                    if let Some(explicit) = parse_status_token(tok) {
                        if let Some(id) = &last_id {
                            status.insert(id.clone(), explicit);
                        }
                        continue;
                    }
                    if looks_like_brief_id(tok) {
                        last_id = Some(tok.to_string());
                        if let SectionRule::Default(default_status) = rule {
                            record(tok, default_status.to_string(), &mut duplicates, &mut status);
                        }
                    }
                }
            }
        }
    }
    Ledger { status, duplicates }
}

/// `archived` and `superseded` both functionally mean "no longer live"; the ledger's own
/// "Resolved by evidence" prose uses `archived` for what closes out a BRIEF, matching
/// `conventions/brief-discipline.md`'s five-value vocabulary exactly -- no fuzz needed
/// beyond exact string match today, but centralised here in case that changes.
fn status_equivalent(a: &str, b: &str) -> bool {
    a == b
}

#[cfg(test)]
mod ledger_parser_tests {
    use super::*;

    /// A faithful reduction of the real `## §Ledger` section: a wrapped `###` heading whose
    /// continuation lines cite other brief-ids, two differently-shaped tables, a "Why active"
    /// cell citing a non-BRIEF slug, and two prose-list subsections.
    const REAL_SHAPE: &str = "\
## §Ledger — disposition of all 23 native BRIEFs

Machine-checkable target: every `BRIEF-*.md` in `.agent/briefs/` has a row here.

### Kept active (6 wiki-content + 4 in §Adjacent = 10 total — 5 over the soft cap of 5;
flagged, not silently forced under it. Corrected 2026-09-07: demoting
`project-editorial-artifact-triage-sweep` to `reference` (all 3 healing streams fully
drained) brought the count from 7 to 6.)

| brief-id | Why active |
|---|---|
| `project-editorial-programme-master` (this file) | The router |
| `project-editorial-journal-research-programme` | Live — `capability-secured-session-orchestration`'s remaining drafting is real open work |

### Demoted to `reference` (open items migrated to §Outstanding Queue below)

| brief-id | Live edge now tracked at |
|---|---|
| `project-editorial-jennifer-to-wiki-backfill` | Historical trunk; pointer only |
| `project-editorial-artifact-triage-sweep` | **Demoted 2026-09-07** — no live edge remains |

### Superseded (1, pre-existing)

`project-editorial-regional-markets-rebuild-and-wiki-integrity` — `superseded_by:
project-editorial-wiki-content-loop`, unchanged.

### Resolved by evidence this session (1 closed, 1 kept active)

`project-editorial-trademark-changeover-mcorp-capability-geometry` — **`status: archived`**,
`moved_to: .agent/rules/entity-naming-convention.md`.

### Housekeeping note — `briefs/archive/`

23 files live there, including `project-workplace-some-foreign-brief`. Excluded by path.
";

    #[test]
    fn parses_every_table_under_the_ledger_heading_not_just_the_first() {
        let l = parse_ledger(REAL_SHAPE);
        // First table ("Kept active").
        assert_eq!(l.status.get("project-editorial-programme-master").map(String::as_str), Some("active"));
        assert_eq!(
            l.status.get("project-editorial-journal-research-programme").map(String::as_str),
            Some("active")
        );
        // Second table ("Demoted to reference") -- differently shaped header, previously unparsed.
        assert_eq!(
            l.status.get("project-editorial-jennifer-to-wiki-backfill").map(String::as_str),
            Some("reference"),
            "a row in the second table under §Ledger must be recognised"
        );
    }

    /// Regression: the `### Kept active (...)` heading wraps, and its continuation lines cite
    /// `project-editorial-artifact-triage-sweep` in prose. Under the old uniform line scan
    /// that citation won (first-wins) and the brief was reported as a status mismatch against
    /// its correct `status: reference` frontmatter.
    #[test]
    fn a_wrapped_heading_s_prose_citation_does_not_outrank_a_real_table_row() {
        let l = parse_ledger(REAL_SHAPE);
        assert_eq!(
            l.status.get("project-editorial-artifact-triage-sweep").map(String::as_str),
            Some("reference"),
            "the Demoted table row is the ledger row; the Kept-active heading's prose is not"
        );
    }

    /// Regression: a non-BRIEF slug cited inside a table row's *second* cell was read as a
    /// ledger row and reported as "no matching file on disk".
    #[test]
    fn a_slug_cited_in_a_non_first_cell_is_not_a_ledger_row() {
        let l = parse_ledger(REAL_SHAPE);
        assert!(
            !l.status.contains_key("capability-secured-session-orchestration"),
            "only a row's first cell (the brief-id column) names a ledger row"
        );
    }

    #[test]
    fn prose_list_subsections_still_parse() {
        let l = parse_ledger(REAL_SHAPE);
        assert_eq!(
            l.status.get("project-editorial-regional-markets-rebuild-and-wiki-integrity").map(String::as_str),
            Some("superseded")
        );
        // Explicit inline `status:` beats the subsection default in a prose list.
        assert_eq!(
            l.status
                .get("project-editorial-trademark-changeover-mcorp-capability-geometry")
                .map(String::as_str),
            Some("archived")
        );
        // `superseded_by: <id>` must not enrol its target as a superseded ledger row.
        assert!(!l.status.contains_key("project-editorial-wiki-content-loop"));
    }

    #[test]
    fn housekeeping_subsections_are_skipped_entirely() {
        let l = parse_ledger(REAL_SHAPE);
        assert!(!l.status.contains_key("project-workplace-some-foreign-brief"));
    }

    #[test]
    fn table_header_and_separator_rows_are_never_ledger_rows() {
        let l = parse_ledger(REAL_SHAPE);
        assert!(!l.status.keys().any(|k| k.contains("---") || k == "brief-id"));
    }

    #[test]
    fn an_explicit_status_token_in_a_table_row_overrides_the_section_default() {
        let text = "\
### Kept active

| brief-id | Why active |
|---|---|
| `project-editorial-some-brief` | held open but `status: reference` in frontmatter |
";
        let l = parse_ledger(text);
        assert_eq!(l.status.get("project-editorial-some-brief").map(String::as_str), Some("reference"));
    }

    #[test]
    fn an_id_cited_in_two_subsections_with_different_statuses_is_reported() {
        let text = "\
### Kept active

| brief-id | Why active |
|---|---|
| `project-editorial-two-places` | here |

### Demoted to `reference`

| brief-id | Live edge now tracked at |
|---|---|
| `project-editorial-two-places` | and also here |
";
        let l = parse_ledger(text);
        assert_eq!(l.duplicates.len(), 1);
        assert_eq!(l.duplicates[0].0, "project-editorial-two-places");
        assert_eq!(l.status.get("project-editorial-two-places").map(String::as_str), Some("active"));
    }

    #[test]
    fn extract_ledger_section_stops_at_the_next_h2_and_keeps_every_h3() {
        let body = "## Something Else\n\ntext\n\n## §Ledger — x\n\n### A\n\nrow\n\n### B\n\nrow\n\n## After\n\nnot ledger\n";
        let section = extract_ledger_section(body).expect("finds the ledger");
        assert!(section.contains("### A") && section.contains("### B"));
        assert!(!section.contains("not ledger"));
        assert_eq!(split_sections(&section).len(), 3, "preamble + 2 `###` subsections");
    }
}
