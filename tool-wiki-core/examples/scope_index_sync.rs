//! Scope-vs-index-group sync check (`BRIEF-category-index-guide-redesign.md` Queue #26,
//! check 4 — spec: `.agent/audit/category-redesign-fable-pass.md` §R2-6 Check 4, and
//! `category-redesign-opus-pass.md` §R6.6(a) `scope_parity`).
//!
//! A category is described in three places that must agree: its `categories.yaml` `scope`
//! text, its `_index.md` (`short_description` plus the H2 shelves and their AUTO-block
//! `index_group` markers), and the `index_group:` values its member articles actually carry.
//! When they drift, the landing page advertises a shape the corpus does not have.
//!
//! Usage: `cargo run --example scope_index_sync -- <path-to-wiki-content-root> [--info]`
//!
//! Sub-check 1's INFO tier (a group unnamed in a scope that does not enumerate at all) is
//! suppressed by default — it is context, not a defect, and at corpus scale it buries the
//! real findings. `--info` prints it.
//!
//! # The defect class this exists for
//!
//! `documentation/how-to`'s scope text names five thematic groups by name — "(Getting
//! started, Working in the console, Records & storage, Multi-entity scale, Integration &
//! data)" — and six exist on disk: `financial-construction-tools` carries 6 articles, renders
//! its own H2 shelf on the landing page, and is named nowhere in the scope. Opus §4.A found
//! this by hand. Sub-check 1 below finds it mechanically.
//!
//! # Sub-checks
//!
//! 1. **`group_unnamed_in_scope`** — a live `index_group` on disk whose slug does not appear
//!    in the slug-normalised `scope` text. INFO by default (see the limitation below);
//!    promoted to a finding when the scope *does* enumerate other groups, which is what makes
//!    the omission a real enumeration and not merely descriptive prose.
//! 2. **`index_group_drift`** — the set of `index_group` values the `_index`'s AUTO blocks
//!    declare vs. the set its live members carry. A block group no member declares is a shelf
//!    advertising content the category does not hold; a member group with no block is content
//!    that renders nowhere.
//! 3. **`count_claim`** — a numeral or number-word attached to a group/category noun in the
//!    scope or `short_description` that does not match the live group count.
//! 4. **`scope_parity`** — content-word Jaccard similarity between `scope` and the `_index`
//!    `short_description`. Below `MIN_JACCARD` the two describe different categories.
//!
//! # Simplification, deliberate
//!
//! Sub-check 1 detects an enumerated group name by slug-normalising the whole scope string
//! and substring-matching the group slug. That finds a comma-list enumeration (the real
//! shape, and the shape the how-to defect lives in) and does not find a group described in
//! scattered prose. Rather than guess, an unmatched group is reported as INFO unless at least
//! one *other* group in the same category did match — the signal that this scope enumerates
//! and simply left one out. The alternative (NLP-ish name extraction) is not mechanical and
//! would misreport more than it caught.
//!
//! Sub-check 4 compares EN scope against the EN `_index` only. The `.es.md` pair is a
//! strategic adaptation, not a translation (every style profile's "adapt structure, never
//! facts" rule), so a word-overlap score across languages measures nothing. Sub-checks 1–3
//! are facts-bearing and do run against both pairs.
//!
//! Report-only: `--apply` would mean regenerating the `_index` `short_description` from
//! `scope`, which is legitimate only once "`categories.yaml` is canonical" is ratified
//! (fable-pass R1 §0.2 / opus-pass §1.4). Not ratified.

use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::Path;

use tool_wiki_core::Document;

const MIN_JACCARD: f64 = 0.35;
const DEAD_STATUSES: &[&str] = &["archived", "retired", "superseded"];
const NUMBER_WORDS: &[(&str, usize)] = &[
    ("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5), ("six", 6),
    ("seven", 7), ("eight", 8), ("nine", 9), ("ten", 10), ("eleven", 11), ("twelve", 12),
    ("thirteen", 13), ("fourteen", 14), ("fifteen", 15), ("sixteen", 16),
];
/// Nouns a count in scope text may legitimately be counting groups with.
const GROUP_NOUNS: &[&str] = &["group", "groups", "thematic groups", "shelves", "shelf"];
/// Words too common to carry meaning in a similarity score.
const STOPWORDS: &[&str] = &[
    "the", "a", "an", "and", "or", "of", "to", "in", "for", "on", "that", "this", "these",
    "with", "is", "are", "as", "by", "it", "its", "not", "but", "from", "at", "each", "how",
    "what", "who", "where", "when", "they", "their", "them", "so", "own", "just", "own",
];

fn is_live(doc: &Document) -> bool {
    let archived = doc
        .path
        .components()
        .any(|c| c.as_os_str().to_str() == Some(".archive"));
    let dead = doc
        .frontmatter
        .status()
        .map(|s| DEAD_STATUSES.contains(&s))
        .unwrap_or(false);
    !archived && !dead
}

fn is_index(doc: &Document) -> bool {
    doc.path
        .file_stem()
        .and_then(|s| s.to_str())
        .map(|s| s == "_index" || s == "_index.es")
        .unwrap_or(false)
}

fn is_spanish(doc: &Document) -> bool {
    doc.path.to_string_lossy().ends_with(".es.md")
}

/// "Financial & construction tools" -> "financial-construction-tools", matching how the
/// corpus derives `index_group` slugs from shelf headings.
fn slugify(s: &str) -> String {
    let mut out = String::new();
    let mut prev_dash = true;
    for ch in s.chars() {
        if ch.is_alphanumeric() {
            for lc in ch.to_lowercase() {
                out.push(lc);
            }
            prev_dash = false;
        } else if !prev_dash {
            out.push('-');
            prev_dash = true;
        }
    }
    out.trim_matches('-').to_string()
}

fn content_words(s: &str) -> BTreeSet<String> {
    s.split(|c: char| !c.is_alphanumeric())
        .map(|w| w.to_lowercase())
        .filter(|w| w.len() > 2 && !STOPWORDS.contains(&w.as_str()))
        .collect()
}

fn jaccard(a: &BTreeSet<String>, b: &BTreeSet<String>) -> f64 {
    if a.is_empty() || b.is_empty() {
        return 1.0; // nothing to compare -- a missing field is another check's finding
    }
    let inter = a.intersection(b).count() as f64;
    let union = a.union(b).count() as f64;
    inter / union
}

/// Counts asserted about groups: `5 groups`, `five thematic groups`, `the other 5 groups`.
fn group_count_claims(text: &str) -> Vec<(usize, String)> {
    let lower = text.to_lowercase();
    let words: Vec<&str> = lower
        .split(|c: char| !c.is_alphanumeric() && c != '-')
        .filter(|w| !w.is_empty())
        .collect();
    let mut out = Vec::new();
    for (i, w) in words.iter().enumerate() {
        let n = w
            .parse::<usize>()
            .ok()
            .or_else(|| NUMBER_WORDS.iter().find(|(nw, _)| nw == w).map(|(_, n)| *n));
        let Some(n) = n else { continue };
        // Look ahead up to 2 words for a group noun ("5 groups", "five thematic groups").
        for j in 1..=2usize {
            if let Some(next) = words.get(i + j) {
                if GROUP_NOUNS.contains(next) {
                    let ctx: Vec<&str> = words[i..(i + j + 1).min(words.len())].to_vec();
                    out.push((n, ctx.join(" ")));
                    break;
                }
            }
        }
    }
    out
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let show_info = args.iter().any(|a| a == "--info");
    let root = args
        .iter()
        .find(|a| !a.starts_with("--"))
        .expect("usage: scope_index_sync <path> [--info]")
        .clone();
    let root = Path::new(&root);

    let Ok(yaml) = fs::read_to_string(root.join("categories.yaml")) else {
        println!("no categories.yaml at this root — nothing to check against");
        return;
    };
    let value: serde_yaml::Value = serde_yaml::from_str(&yaml).expect("parse categories.yaml");
    let mut scopes: BTreeMap<String, String> = BTreeMap::new();
    if let Some(cats) = value.get("categories").and_then(|c| c.as_sequence()) {
        for cat in cats {
            let (Some(id), Some(scope)) = (
                cat.get("id").and_then(|v| v.as_str()),
                cat.get("scope").and_then(|v| v.as_str()),
            ) else {
                continue;
            };
            scopes.insert(id.to_string(), scope.to_string());
        }
    }

    let crawl = tool_wiki_core::crawl(root).expect("crawl failed");
    let mut members: BTreeMap<(String, bool), Vec<&Document>> = BTreeMap::new();
    let mut indexes: BTreeMap<(String, bool), &Document> = BTreeMap::new();
    for d in crawl.documents.iter().filter(|d| is_live(d)) {
        let Some(cat) = d.frontmatter.category() else { continue };
        if tool_wiki_core::is_reserved_category(cat) {
            continue;
        }
        let key = (cat.to_string(), is_spanish(d));
        if is_index(d) {
            indexes.insert(key, d);
        } else {
            members.entry(key).or_default().push(d);
        }
    }

    println!(
        "crawled {} documents; {} categories with a `scope`\n",
        crawl.documents.len(),
        scopes.len()
    );

    let mut findings = 0usize;
    let mut infos = 0usize;

    for (key, index) in &indexes {
        let (cat_id, es) = (key.0.as_str(), key.1);
        let Some(scope) = scopes.get(cat_id) else { continue };
        let lang = if es { "es" } else { "en" };
        let mems = members.get(key).map(Vec::as_slice).unwrap_or(&[]);

        let member_groups: BTreeSet<String> = mems
            .iter()
            .filter_map(|d| d.frontmatter.get_str("index_group"))
            .map(str::to_string)
            .collect();
        let block_groups: BTreeSet<String> = index
            .auto_generated_blocks
            .iter()
            .map(|b| b.index_group.clone())
            .collect();

        let mut local: Vec<(bool, String)> = Vec::new(); // (is_finding, text)

        // --- 1. index_group not named in the scope enumeration ---
        let scope_slug = slugify(scope);
        let named: Vec<&String> = member_groups
            .iter()
            .filter(|g| scope_slug.contains(g.as_str()))
            .collect();
        let unnamed: Vec<&String> = member_groups
            .iter()
            .filter(|g| !scope_slug.contains(g.as_str()))
            .collect();
        // Only a real finding when the scope demonstrably enumerates (>=2 other groups match).
        let scope_enumerates = named.len() >= 2;
        for g in &unnamed {
            local.push((
                scope_enumerates,
                format!(
                    "(1) index_group `{g}` ({} articles) is not named in the scope text{}",
                    mems.iter()
                        .filter(|d| d.frontmatter.get_str("index_group") == Some(g.as_str()))
                        .count(),
                    if scope_enumerates {
                        format!(" — scope enumerates {} of {} groups", named.len(), member_groups.len())
                    } else {
                        " (scope does not appear to enumerate groups — INFO)".to_string()
                    }
                ),
            ));
        }

        // --- 2. AUTO-block groups vs member groups ---
        for g in block_groups.difference(&member_groups) {
            local.push((
                true,
                format!("(2) `_index` renders a shelf for index_group `{g}` that no live member declares"),
            ));
        }
        for g in member_groups.difference(&block_groups) {
            local.push((
                true,
                format!("(2) index_group `{g}` has members but no AUTO block on the `_index` — renders nowhere"),
            ));
        }

        // --- 3. numeric claims about group count ---
        let actual = member_groups.len();
        let sd = index.frontmatter.get_str("short_description").unwrap_or("");
        for (source, text) in [("scope", scope.as_str()), ("short_description", sd)] {
            for (claimed, ctx) in group_count_claims(text) {
                if claimed != actual {
                    local.push((
                        true,
                        format!("(3) {source} claims \"{ctx}\" but {actual} index_groups exist on disk"),
                    ));
                }
            }
        }

        // --- 4. scope vs short_description similarity (English pair only) ---
        if !es && !sd.is_empty() {
            let sim = jaccard(&content_words(scope), &content_words(sd));
            if sim < MIN_JACCARD {
                local.push((
                    true,
                    format!("(4) scope and `_index` short_description share only {sim:.2} of their content words — they describe different categories"),
                ));
            }
        }

        infos += local.iter().filter(|(f, _)| !f).count();
        let printable: Vec<&(bool, String)> =
            local.iter().filter(|(f, _)| *f || show_info).collect();
        if !printable.is_empty() {
            println!("{cat_id} [{lang}] ({} members, {actual} groups):", mems.len());
            for (is_finding, text) in printable {
                if *is_finding {
                    findings += 1;
                    println!("  {text}");
                } else {
                    println!("  INFO {text}");
                }
            }
        }
    }

    println!(
        "\n{findings} findings, {infos} info{}",
        if show_info { "" } else { " (suppressed — pass --info to see them)" }
    );
}

#[cfg(test)]
mod scope_sync_tests {
    use super::*;

    #[test]
    fn slugify_matches_the_corpus_index_group_derivation() {
        assert_eq!(slugify("Financial & construction tools"), "financial-construction-tools");
        assert_eq!(slugify("Records & storage"), "records-storage");
        assert_eq!(slugify("Working in the console"), "working-in-the-console");
        assert_eq!(slugify("Multi-entity scale"), "multi-entity-scale");
    }

    /// The live `documentation/how-to` defect, reduced: the scope enumerates five groups by
    /// name and six exist on disk.
    #[test]
    fn the_how_to_five_vs_six_defect_is_detected() {
        let scope = "Step-by-step instructions for the hands-on work. Houses the other 5 \
                     groups (Getting started, Working in the console, Records & storage, \
                     Multi-entity scale, Integration & data) until each gets the same \
                     treatment in a later round.";
        let scope_slug = slugify(scope);
        let on_disk = [
            "getting-started",
            "working-in-the-console",
            "records-storage",
            "multi-entity-scale",
            "integration-data",
            "financial-construction-tools",
        ];
        let named: Vec<&&str> = on_disk.iter().filter(|g| scope_slug.contains(**g)).collect();
        let unnamed: Vec<&&str> = on_disk.iter().filter(|g| !scope_slug.contains(**g)).collect();
        assert_eq!(named.len(), 5);
        assert_eq!(unnamed, vec![&"financial-construction-tools"]);
        assert!(named.len() >= 2, "the scope enumerates -- so the omission is a finding, not INFO");

        // And the numeric claim is independently wrong.
        let claims = group_count_claims(scope);
        assert_eq!(claims.len(), 1);
        assert_eq!(claims[0].0, 5);
        assert_ne!(claims[0].0, on_disk.len());
    }

    #[test]
    fn a_scope_that_does_not_enumerate_yields_info_not_a_finding() {
        let scope = "Everything about how the platform is built and why it is built that way.";
        let scope_slug = slugify(scope);
        let on_disk = ["platform-structure", "customer-ownership"];
        let named = on_disk.iter().filter(|g| scope_slug.contains(**g)).count();
        assert_eq!(named, 0, "no group matches -- so no enumeration -- so INFO, not a finding");
    }

    #[test]
    fn count_claims_read_digits_and_number_words() {
        assert_eq!(group_count_claims("the other 5 groups").first().map(|c| c.0), Some(5));
        assert_eq!(
            group_count_claims("five thematic groups live here").first().map(|c| c.0),
            Some(5)
        );
        // A number not attached to a group noun is not a claim about groups.
        assert!(group_count_claims("17 countries and 400 markets").is_empty());
    }

    #[test]
    fn jaccard_separates_a_restatement_from_two_different_descriptions() {
        let a = content_words("Cross-cutting platform architecture: the three-ring composition model and the security boundary.");
        let same = content_words("The three-ring composition model, the security boundary, and cross-cutting platform architecture.");
        let different = content_words("Quarterly distribution mechanics, redemption windows, and unitholder payment schedules.");
        assert!(jaccard(&a, &same) >= MIN_JACCARD);
        assert!(jaccard(&a, &different) < MIN_JACCARD);
    }
}
