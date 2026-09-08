//! Plain-language / banned-term surface check (`BRIEF-category-index-guide-redesign.md`
//! Queue #26, check 3 — spec: `.agent/audit/category-redesign-fable-pass.md` §R2-6 Check 3,
//! and `category-redesign-opus-pass.md` §R6.6(b) `plain_language_scope`).
//!
//! Internal platform jargon on a **category surface** — a `categories.yaml` `name` or
//! `scope`, or an `_index.md` `short_description` — is a different defect from the same word
//! inside an article body. The documentation wiki's own `categories.yaml` audience statement
//! says its secondary finance audience "must be able to read every category name and scope
//! cold"; the corporate and projects wikis are non-technical throughout.
//!
//! Usage:
//! ```text
//! cargo run --example plain_language_surface -- <wiki-content-root> [<linguistic-token-dir>]
//! ```
//! The token directory defaults to `../pointsav-design-system/tokens/linguistic` relative to
//! the wiki root, which is where it sits in a `project-editorial` archive checkout.
//!
//! # Token source — resolved, not guessed
//!
//! The banned lists are **not** hardcoded here. They are read from the design-system
//! linguistic tokens this archive stewards per `.agent/rules/design-token-stewardship.md`:
//!
//! * `vocabulary-banned-corporate.yaml` — declares `metadata.wikis:
//!   [content-wiki-corporate, content-wiki-projects]`, 16 terms, no exceptions permitted.
//! * `vocabulary-banned-documentation.yaml` — `[content-wiki-documentation]`, 27 terms,
//!   permitted in article bodies when explicitly defined on first use.
//!
//! Each file's shape is `vocabulary_rules: [{term, replacements, exception, reason}]`.
//! Which file applies to which wiki is read from each file's own `metadata.wikis` list, not
//! inferred from its filename.
//!
//! **The token file the spec names does not exist.** Fable §R2-6 Check 3 specifies
//! `tokens/linguistic/vocabulary-banned-category-surface-<wiki>.yaml` — a *category-surface
//! tier* list, distinct from the whole-wiki lists above, seeded with terms like `PKS`, `VWH`,
//! `DBSCAN`, `WORM`, `seL4`, `Tier A/B/C`. No such file exists in
//! `pointsav-design-system/tokens/linguistic/` (verified 2026-09-06; the directory holds 38
//! files, two of them the whole-wiki lists named above). Creating it is a
//! `DESIGN-TOKEN-CHANGE` routed to project-design, which holds the commit gate — not
//! something this check may invent. So:
//!
//!   * The check consumes the **whole-wiki lists that do exist**, applied at the category
//!     tier where they are strictly correct (a term banned wiki-wide is certainly banned on
//!     a category surface).
//!   * `EXTRA_CATEGORY_SURFACE_TERMS` below carries the handful of category-tier terms Opus
//!     §R6.6(b) measured *in the live corpus* and Fable's seed list names, marked as such in
//!     the report. This is a stopgap with its provenance stated, not a second source of
//!     truth: when the specced token file lands, delete the constant and read the file.
//!
//! # Structural sub-checks folded in (Fable §R2-6 Check 3, "Structural sub-checks")
//!
//! * `short_description` missing entirely.
//! * `short_description` longer than `MAX_SHORT_DESCRIPTION` characters.
//!
//! # Severity
//!
//! Category-surface hit = ERROR on every wiki. Article-level `short_description` hit =
//! ERROR on corporate/projects (non-technical throughout), WARN on documentation (its
//! articles are declared technical, and the token file's own `exception` permits a defined
//! term in a body). Matching is case-insensitive on word boundaries; a term that is a
//! prefix of a longer word (`substrate` in `substrates`) matches, which is intended.
//!
//! Report-only.

use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use tool_wiki_core::Document;

const MAX_SHORT_DESCRIPTION: usize = 300;

/// Category-surface-tier terms from Fable §R2-6 Check 3's seed list and Opus §R6.6(b)'s
/// measured list that are NOT in either existing whole-wiki token file. Stopgap only — see
/// the module docs. Every finding sourced from here is labelled `(stopgap list)` in the
/// report so it is never mistaken for a ratified token.
const EXTRA_CATEGORY_SURFACE_TERMS: &[&str] = &[
    "seL4", "WORM", "PPN", "Diode", "TUI", "cartridge", "DTCG", "cgroup", "flock",
    "idempotent", "tlog", "predicate gate", "archetype", "DBSCAN", "elfloader", "msg-id",
    "QLoRA", "LoRA", "MCP",
];

#[derive(Debug, Clone)]
struct BannedTerm {
    term: String,
    replacement: Option<String>,
    /// The token file permits this term when explicitly defined on first use (documentation
    /// wiki). Never relaxes the category tier — only the article tier.
    has_body_exception: bool,
    stopgap: bool,
}

/// Read every `vocabulary-banned-*.yaml` in `dir` and keep the ones whose own
/// `metadata.wikis` list names this wiki.
fn load_banned_terms(dir: &Path, wiki_slug: &str) -> Result<(Vec<BannedTerm>, Vec<String>), String> {
    let entries = fs::read_dir(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    let mut terms: Vec<BannedTerm> = Vec::new();
    let mut sources: Vec<String> = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let name = entry.file_name().to_string_lossy().to_string();
        if !name.starts_with("vocabulary-banned-") || !name.ends_with(".yaml") {
            continue;
        }
        let text = fs::read_to_string(entry.path()).map_err(|e| e.to_string())?;
        let value: serde_yaml::Value =
            serde_yaml::from_str(&text).map_err(|e| format!("{name}: {e}"))?;
        let applies = value
            .get("metadata")
            .and_then(|m| m.get("wikis"))
            .and_then(|w| w.as_sequence())
            .map(|seq| {
                seq.iter()
                    .filter_map(|v| v.as_str())
                    .any(|w| w == wiki_slug || w.ends_with(wiki_slug))
            })
            .unwrap_or(false);
        if !applies {
            continue;
        }
        sources.push(name);
        let Some(rules) = value.get("vocabulary_rules").and_then(|r| r.as_sequence()) else {
            continue;
        };
        for rule in rules {
            let Some(term) = rule.get("term").and_then(|t| t.as_str()) else { continue };
            let replacement = rule
                .get("replacements")
                .and_then(|r| r.as_sequence())
                .and_then(|s| s.first())
                .and_then(|v| v.as_str())
                .map(str::to_string);
            let has_body_exception = rule
                .get("exception")
                .map(|e| !e.is_null())
                .unwrap_or(false);
            terms.push(BannedTerm {
                term: term.to_string(),
                replacement,
                has_body_exception,
                stopgap: false,
            });
        }
    }
    Ok((terms, sources))
}

/// Case-insensitive whole-token search. Returns the matched span's text in its original
/// casing so the report shows what is actually written.
fn find_term<'a>(haystack: &'a str, needle: &str) -> Option<&'a str> {
    let hay_lower = haystack.to_lowercase();
    let needle_lower = needle.to_lowercase();
    let mut from = 0usize;
    while let Some(rel) = hay_lower[from..].find(&needle_lower) {
        let start = from + rel;
        let end = start + needle_lower.len();
        let before_ok = start == 0
            || !hay_lower[..start]
                .chars()
                .next_back()
                .map(|c| c.is_alphanumeric())
                .unwrap_or(false);
        if before_ok && haystack.is_char_boundary(start) && haystack.is_char_boundary(end) {
            return Some(&haystack[start..end]);
        }
        from = end;
    }
    None
}

/// `media-knowledge-documentation` -> `content-wiki-documentation` (the slug the token files
/// use in `metadata.wikis`).
fn wiki_slug_for(root: &Path) -> String {
    let dir = root
        .canonicalize()
        .ok()
        .and_then(|p| p.file_name().map(|s| s.to_string_lossy().to_string()))
        .unwrap_or_default();
    match dir.rsplit('-').next() {
        Some(tail) => format!("content-wiki-{tail}"),
        None => dir,
    }
}

fn is_index(doc: &Document) -> bool {
    doc.path
        .file_stem()
        .and_then(|s| s.to_str())
        .map(|s| s == "_index" || s == "_index.es")
        .unwrap_or(false)
}

fn main() {
    let mut args = env::args().skip(1);
    let root_arg = args
        .next()
        .expect("usage: plain_language_surface <wiki-content-root> [<linguistic-token-dir>]");
    let root = Path::new(&root_arg);
    let token_dir: PathBuf = args
        .next()
        .map(PathBuf::from)
        .unwrap_or_else(|| root.join("../pointsav-design-system/tokens/linguistic"));

    let wiki_slug = wiki_slug_for(root);
    let technical_wiki = wiki_slug.ends_with("documentation");

    let (mut terms, sources) = match load_banned_terms(&token_dir, &wiki_slug) {
        Ok(v) => v,
        Err(e) => {
            // Report, never guess -- per the check's own brief.
            println!(
                "TOKEN SOURCE UNAVAILABLE: could not read banned-vocabulary tokens from {} ({e}).\n\
                 The lists are stewarded in pointsav-design-system/tokens/linguistic/ \
                 (.agent/rules/design-token-stewardship.md). Pass the directory as the second \
                 argument. Not falling back to a hardcoded list -- a drifted local copy is \
                 exactly the failure that rules file exists to prevent.",
                token_dir.display()
            );
            std::process::exit(2);
        }
    };
    if sources.is_empty() {
        println!(
            "TOKEN SOURCE EMPTY: {} holds no vocabulary-banned-*.yaml naming `{wiki_slug}` in \
             its metadata.wikis. Nothing to check against.",
            token_dir.display()
        );
        std::process::exit(2);
    }
    for t in EXTRA_CATEGORY_SURFACE_TERMS {
        terms.push(BannedTerm {
            term: t.to_string(),
            replacement: None,
            has_body_exception: false,
            stopgap: true,
        });
    }

    println!("wiki `{wiki_slug}`; token sources: {}", sources.join(", "));
    println!(
        "{} ratified terms + {} stopgap category-surface terms\n",
        terms.len() - EXTRA_CATEGORY_SURFACE_TERMS.len(),
        EXTRA_CATEGORY_SURFACE_TERMS.len()
    );

    let crawl = tool_wiki_core::crawl(root).expect("crawl failed");
    let mut errors = 0usize;
    let mut warnings = 0usize;

    let report = |errors: &mut usize,
                  warnings: &mut usize,
                  severity: &str,
                  location: &str,
                  t: &BannedTerm,
                  hit: &str,
                  context: &str| {
        if severity == "ERROR" {
            *errors += 1;
        } else {
            *warnings += 1;
        }
        let tag = if t.stopgap { " (stopgap list)" } else { "" };
        let fix = t
            .replacement
            .as_deref()
            .map(|r| format!(" — use \"{r}\""))
            .unwrap_or_default();
        println!("  {severity}{tag}: `{hit}` in {location}{fix}");
        println!("      \"{}\"", truncate(context, 160));
    };

    // --- 1. categories.yaml `name` and `scope` ---
    println!("=== category surfaces: categories.yaml `name` / `scope` ===");
    match fs::read_to_string(root.join("categories.yaml")) {
        Ok(yaml) => {
            let value: serde_yaml::Value = serde_yaml::from_str(&yaml).expect("parse categories.yaml");
            if let Some(cats) = value.get("categories").and_then(|c| c.as_sequence()) {
                for cat in cats {
                    let id = cat.get("id").and_then(|v| v.as_str()).unwrap_or("<no id>");
                    for field in ["name", "scope"] {
                        let Some(text) = cat.get(field).and_then(|v| v.as_str()) else { continue };
                        for t in &terms {
                            if let Some(hit) = find_term(text, &t.term) {
                                report(
                                    &mut errors,
                                    &mut warnings,
                                    "ERROR",
                                    &format!("categories.yaml `{id}`.{field}"),
                                    t,
                                    hit,
                                    text,
                                );
                            }
                        }
                    }
                }
            }
        }
        Err(_) => println!("  (no categories.yaml at this root)"),
    }

    // --- 2. `_index.md` short_description (category tier) and article short_description ---
    let mut by_tier: BTreeMap<bool, Vec<&Document>> = BTreeMap::new();
    for d in &crawl.documents {
        by_tier.entry(is_index(d)).or_default().push(d);
    }

    println!("\n=== category surfaces: `_index` short_description ===");
    let mut missing_sd: Vec<String> = Vec::new();
    let mut long_sd: Vec<(String, usize)> = Vec::new();
    for d in by_tier.get(&true).map(Vec::as_slice).unwrap_or(&[]) {
        let path = d.path.display().to_string();
        match d.frontmatter.get_str("short_description") {
            None => missing_sd.push(path.clone()),
            Some(sd) => {
                if sd.chars().count() > MAX_SHORT_DESCRIPTION {
                    long_sd.push((path.clone(), sd.chars().count()));
                }
                for t in &terms {
                    if let Some(hit) = find_term(sd, &t.term) {
                        report(&mut errors, &mut warnings, "ERROR", &path, t, hit, sd);
                    }
                }
            }
        }
    }

    println!("\n=== article-tier short_description ===");
    let article_severity = if technical_wiki { "WARN" } else { "ERROR" };
    for d in by_tier.get(&false).map(Vec::as_slice).unwrap_or(&[]) {
        let path = d.path.display().to_string();
        match d.frontmatter.get_str("short_description") {
            None => missing_sd.push(path.clone()),
            Some(sd) => {
                if sd.chars().count() > MAX_SHORT_DESCRIPTION {
                    long_sd.push((path.clone(), sd.chars().count()));
                }
                for t in &terms {
                    // The documentation token file's `exception` permits a defined term in a
                    // body/article context; it never relaxes the category tier.
                    if technical_wiki && t.has_body_exception {
                        continue;
                    }
                    if let Some(hit) = find_term(sd, &t.term) {
                        report(&mut errors, &mut warnings, article_severity, &path, t, hit, sd);
                    }
                }
            }
        }
    }

    missing_sd.sort();
    long_sd.sort();
    println!("\n=== structural: `short_description` missing ({}) ===", missing_sd.len());
    for p in &missing_sd {
        println!("  {p}");
    }
    println!(
        "\n=== structural: `short_description` over {MAX_SHORT_DESCRIPTION} chars ({}) ===",
        long_sd.len()
    );
    for (p, n) in &long_sd {
        println!("  {p} — {n} chars");
    }

    println!("\n{errors} errors, {warnings} warnings, {} structural findings", missing_sd.len() + long_sd.len());
}

fn truncate(s: &str, n: usize) -> String {
    if s.chars().count() <= n {
        s.replace('\n', " ")
    } else {
        let t: String = s.chars().take(n).collect();
        format!("{}…", t.replace('\n', " "))
    }
}

#[cfg(test)]
mod plain_language_tests {
    use super::*;

    #[test]
    fn word_boundary_matching_does_not_fire_mid_word() {
        assert_eq!(find_term("the substrate layer", "substrate"), Some("substrate"));
        // A term at the start of a longer word still matches (substrates) -- intended.
        assert_eq!(find_term("many substrates here", "substrate"), Some("substrate"));
        // But not when it is the tail of another word.
        assert_eq!(find_term("infrasubstrate", "substrate"), None);
    }

    #[test]
    fn matching_is_case_insensitive_but_reports_original_casing() {
        assert_eq!(find_term("The Doorman routes it", "doorman"), Some("Doorman"));
        assert_eq!(find_term("seL4 microkernel", "SEL4"), Some("seL4"));
    }

    #[test]
    fn multi_word_terms_match() {
        assert_eq!(find_term("crosses Ring 1 boundaries", "Ring 1"), Some("Ring 1"));
        assert_eq!(find_term("no rings here", "Ring 1"), None);
    }

    #[test]
    fn wiki_slug_maps_the_repo_directory_to_the_token_files_vocabulary() {
        // Directory names are `media-knowledge-<tail>`; token files say `content-wiki-<tail>`.
        let tmp = std::env::temp_dir().join("media-knowledge-documentation");
        std::fs::create_dir_all(&tmp).unwrap();
        assert_eq!(wiki_slug_for(&tmp), "content-wiki-documentation");
    }

    #[test]
    fn token_rules_parse_from_the_real_file_shape() {
        let dir = std::env::temp_dir().join(format!("plain-lang-tokens-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("vocabulary-banned-corporate.yaml"),
            "title: \"X\"\nmetadata:\n  wikis:\n    - content-wiki-corporate\n    - content-wiki-projects\nvocabulary_rules:\n  - term: \"substrate\"\n    replacements:\n      - \"the data layer\"\n    exception: null\n    reason: \"Internal metaphor.\"\n  - term: \"Doorman\"\n    replacements:\n      - \"the request router\"\n    exception: |\n      Allowed when defined.\n    reason: \"Internal name.\"\n",
        )
        .unwrap();

        let (terms, sources) = load_banned_terms(&dir, "content-wiki-corporate").unwrap();
        assert_eq!(sources.len(), 1);
        assert_eq!(terms.len(), 2);
        assert_eq!(terms[0].term, "substrate");
        assert_eq!(terms[0].replacement.as_deref(), Some("the data layer"));
        assert!(!terms[0].has_body_exception);
        assert!(terms[1].has_body_exception);

        // A wiki the file does not name gets nothing from it.
        let (none, no_sources) = load_banned_terms(&dir, "content-wiki-documentation").unwrap();
        assert!(none.is_empty() && no_sources.is_empty());

        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn a_missing_token_directory_is_reported_not_guessed_around() {
        let err = load_banned_terms(Path::new("/does/not/exist/tokens"), "content-wiki-corporate");
        assert!(err.is_err(), "the check must report an unavailable token source, never fall back");
    }
}
