//! Glossary-consistency check (`BRIEF-category-index-guide-redesign.md` Queue #26, check 6 —
//! spec: `.agent/audit/category-redesign-opus-pass.md` §R6.10 `glossary_term_conformance`
//! and §R6.11 `glossary_coverage`).
//!
//! Each wiki defines its vocabulary in glossary/terminology articles. This checks that the
//! rest of the wiki uses the *defined form* of those terms rather than a drifted variant.
//!
//! Usage: `cargo run --example glossary_consistency -- <path-to-wiki-content-root> [--list]`
//!
//! # Where headwords come from
//!
//! Files whose stem contains `glossary`, `terminology`, or `vocabulary`. The corpus writes
//! its glossaries in **four different shapes** — all four are parsed; see [`headwords`] for
//! each one and the file it was found in. Assuming a single shape is not safe here: the first
//! version of this check supported only the bolded-em-dash shape and reported "no glossary
//! articles found" for the entire documentation wiki while extracting one term from projects.
//!
//! # Sub-checks
//!
//! 1. **`drifted_form`** — a body phrase whose letters-and-digits match a headword exactly but
//!    whose punctuation or spacing does not: `Direct Hold Solution` where the glossary defines
//!    `Direct-Hold Solution`. This is the mechanical core of "a drifted synonym, not the
//!    defined form". Case is ignored (ordinary sentence-case prose is not drift); plurals are
//!    not folded (a plural is legitimate usage, not a variant spelling).
//! 2. **`near_synonym_name`** — an article `title` or a `categories.yaml` category `name` that
//!    is a near-synonym of a headword without being it: the same head noun with a different
//!    modifier (`Development Program` where `Rollout Program` is defined). This is §R6.10's
//!    check, and the one that would have caught four of the seven naming conflicts that pass
//!    records.
//!
//! # Simplifications against the written spec, and why
//!
//! * §R6.10 specifies "normalised edit distance below a threshold **or** a shared head noun
//!   with differing modifier". Only the second half is implemented. Edit distance over a
//!   corpus of ~200 headwords x ~1,300 titles produces mostly noise at any threshold loose
//!   enough to catch a real synonym (`Rollout Program`/`Development Program` are 9 edits
//!   apart — further than many unrelated pairs). The head-noun rule is precise, cheap, and
//!   catches the documented examples; a distance pass can be added later behind a flag if the
//!   head-noun rule proves too narrow.
//! * §R6.11 `glossary_coverage` (banned-jargon terms used in `short_description` with no
//!   glossary entry) is **not** duplicated here — `plain_language_surface` already reads the
//!   banned-vocabulary tokens and reports those uses. Adding the "is it defined?" column
//!   belongs there, next to the term list, not in a second binary with a second copy of the
//!   list. Noted rather than silently dropped.
//! * The glossary articles themselves are excluded from sub-check 1: a glossary legitimately
//!   discusses variant spellings when defining a term.
//!
//! Report-only.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::env;
use std::fs;
use std::path::Path;

use tool_wiki_core::Document;

/// Longest headword, in words, that sub-check 1 will look for in body prose.
const MAX_HEADWORD_WORDS: usize = 5;
/// Shortest headword worth checking — one- and two-letter terms match everywhere.
const MIN_HEADWORD_CHARS: usize = 4;
const GLOSSARY_STEMS: &[&str] = &["glossary", "terminology", "vocabulary"];

fn is_glossary_file(doc: &Document) -> bool {
    doc.path
        .file_stem()
        .and_then(|s| s.to_str())
        .map(|s| {
            let s = s.to_lowercase();
            GLOSSARY_STEMS.iter().any(|g| s.contains(g))
        })
        .unwrap_or(false)
}

fn is_spanish(doc: &Document) -> bool {
    doc.path.to_string_lossy().ends_with(".es.md")
}

/// Letters and digits only, lowercased: `Direct-Hold Solution` and `Direct Hold Solution`
/// both become `directholdsolution`.
fn normalize(s: &str) -> String {
    s.chars()
        .filter(|c| c.is_alphanumeric())
        .flat_map(|c| c.to_lowercase())
        .collect()
}

/// The form two strings must differ in for the difference to count as *drift*: lowercased,
/// internal whitespace collapsed to one space, leading/trailing punctuation trimmed.
///
/// This is what separates a real variant spelling from three kinds of non-finding that
/// dominated the first run against the live corpus: a trailing sentence period (`Woodfine
/// Capital Projects Inc` in prose vs. `... Inc.` in the glossary), a line wrap inside a
/// multi-word term, and ordinary sentence case. Only an *internal* punctuation or spacing
/// difference survives — which is exactly `Direct Hold Solution` vs `Direct-Hold Solution`.
fn compare_key(s: &str) -> String {
    let collapsed: String = s.split_whitespace().collect::<Vec<_>>().join(" ");
    collapsed
        .trim_matches(|c: char| !c.is_alphanumeric())
        .to_lowercase()
}

/// Byte spans of `[[...]]` wikilinks and `](...)` link targets — slug text, not prose. A
/// wikilink's slug (`corporate-structure`) normalises to the same key as the term it points
/// at (`Corporate Structure`), so without this every correct cross-reference reads as drift.
fn link_spans(body: &str) -> Vec<(usize, usize)> {
    let mut spans = Vec::new();
    let mut from = 0usize;
    while let Some(rel) = body[from..].find("[[") {
        let start = from + rel;
        let end = body[start..]
            .find("]]")
            .map(|r| start + r + 2)
            .unwrap_or(body.len());
        spans.push((start, end));
        from = end;
    }
    let mut from = 0usize;
    while let Some(rel) = body[from..].find("](") {
        let start = from + rel;
        let end = body[start..]
            .find(')')
            .map(|r| start + r + 1)
            .unwrap_or(body.len());
        spans.push((start, end));
        from = end;
    }
    spans
}

/// Headings that structure a glossary rather than name a term.
const STRUCTURAL_HEADINGS: &[&str] = &[
    "see also", "what this is not", "overview", "references", "sources", "further reading",
    "notes", "how to use this glossary", "scope",
    "véase también", "vease tambien", "lo que esto no es", "resumen", "referencias",
    "fuentes", "notas", "alcance",
];
/// A heading longer than this is a section title, not a term.
const MAX_HEADING_HEADWORD_WORDS: usize = 6;

/// Extract every defined term from a glossary article body.
///
/// **Four shapes exist in the corpus and all four are supported.** Each was found by running
/// this check against the live wikis, not assumed:
///
/// * **Bolded-lead, dash** — `**Accredited Investor** — An investor meeting ...`
///   (`corporate/reference/corporate-glossary.md`).
/// * **Bolded-lead, period** — `**Buying Program.** The portfolio-wide procurement ...`
///   (`projects/buildings/woodfine-development-vocabulary.md`).
/// * **H3-headword** — `### Cascading Style Sheet or CSS`
///   (`documentation/reference/glossary-documentation.md`). Supporting only the first shape
///   made this check report "no glossary articles found" for that entire wiki.
/// * **H2-headword with a bolded definition sentence** — `## Power Centre` followed by
///   `A **Power Centre** is a large-format retail node ...`
///   (`projects/reference/site-selection-terminology.md`). Without this, the projects wiki
///   yielded exactly one headword.
///
/// Alternates in a heading are split on ` or `/` o ` and on commas
/// (`## Primary Target, Secondary Target, Tertiary Target` is three terms). A parenthetical
/// expansion yields both forms (`Adjusted Funds From Operations (AFFO)`). Headings on
/// `STRUCTURAL_HEADINGS`, or longer than `MAX_HEADING_HEADWORD_WORDS`, are not terms.
fn headwords(body: &str) -> Vec<String> {
    let mut out = Vec::new();
    for line in body.lines() {
        let line = line.trim();

        if let Some(heading) = line
            .strip_prefix("### ")
            .or_else(|| line.strip_prefix("## "))
        {
            let heading = heading.trim();
            let lower = heading.to_lowercase();
            if STRUCTURAL_HEADINGS.iter().any(|s| lower == *s)
                || heading.split_whitespace().count() > MAX_HEADING_HEADWORD_WORDS
            {
                continue;
            }
            for part in heading.split(',') {
                for alt in split_alternates(part.trim()) {
                    push_headword(&mut out, &alt);
                }
            }
            continue;
        }

        // Every bolded run on the line, so a paragraph packing several definitions
        // (`gis-cluster-scoring-glossary.md` puts three on one line) yields all of them.
        let mut rest = line;
        while let Some(open) = rest.find("**") {
            let after_open = &rest[open + 2..];
            let Some(close) = after_open.find("**") else { break };
            let term = after_open[..close].trim();
            let after = after_open[close + 2..].trim_start();
            let dash_led = after.starts_with('—') || after.starts_with("--") || after.starts_with('–');
            let period_inside = term.ends_with('.');
            if dash_led || period_inside {
                push_headword(&mut out, term.trim_end_matches('.').trim());
            }
            rest = &after_open[close + 2..];
        }
    }
    out.sort();
    out.dedup();
    out
}

fn push_headword(out: &mut Vec<String>, term: &str) {
    let term = term.trim();
    if term.is_empty() || term.chars().count() < MIN_HEADWORD_CHARS {
        return;
    }
    // `Adjusted Funds From Operations (AFFO)` -> both forms.
    if let (Some(open), Some(shut)) = (term.rfind('('), term.rfind(')')) {
        if open < shut {
            let long = term[..open].trim();
            let abbrev = term[open + 1..shut].trim();
            if long.chars().count() >= MIN_HEADWORD_CHARS {
                out.push(long.to_string());
            }
            if abbrev.chars().count() >= MIN_HEADWORD_CHARS {
                out.push(abbrev.to_string());
            }
            return;
        }
    }
    out.push(term.to_string());
}

/// `Cascading Style Sheet or CSS` -> `["Cascading Style Sheet", "CSS"]`; also handles the
/// Spanish `o` separator used in the `.es.md` pair.
fn split_alternates(heading: &str) -> Vec<String> {
    let mut parts = vec![String::new()];
    for word in heading.split_whitespace() {
        if word.eq_ignore_ascii_case("or") || word == "o" || word == "ó" {
            parts.push(String::new());
            continue;
        }
        let last = parts.last_mut().expect("always non-empty");
        if !last.is_empty() {
            last.push(' ');
        }
        last.push_str(word);
    }
    parts.into_iter().filter(|p| !p.is_empty()).collect()
}

/// The last word of a term, lowercased and de-pluralised — its head noun.
fn head_noun(term: &str) -> String {
    let last = term
        .split_whitespace()
        .next_back()
        .unwrap_or("")
        .trim_matches(|c: char| !c.is_alphanumeric())
        .to_lowercase();
    last.strip_suffix('s').map(str::to_string).unwrap_or(last)
}

fn modifier(term: &str) -> String {
    let words: Vec<&str> = term.split_whitespace().collect();
    if words.len() < 2 {
        return String::new();
    }
    words[..words.len() - 1].join(" ").to_lowercase()
}

/// Word spans of `text`, as (start, end) byte offsets, so an n-gram can be sliced back out
/// with its original punctuation intact.
fn word_spans(text: &str) -> Vec<(usize, usize)> {
    let mut spans = Vec::new();
    let mut start: Option<usize> = None;
    for (i, ch) in text.char_indices() {
        // A word may contain an internal hyphen or apostrophe but never begin with one --
        // otherwise a markdown list bullet (`- Corporate structure`) becomes part of the
        // phrase and every list item reads as drift.
        let is_inner = ch.is_alphanumeric() || ch == '-' || ch == '\'' || ch == '’';
        let starts_word = ch.is_alphanumeric();
        match (is_inner, starts_word, start) {
            (_, true, None) => start = Some(i),
            (true, _, Some(_)) => {}
            (false, _, Some(s)) => {
                spans.push((s, i));
                start = None;
            }
            _ => {}
        }
    }
    if let Some(s) = start {
        spans.push((s, text.len()));
    }
    // Trim any trailing hyphen/apostrophe a word ended on.
    spans
        .into_iter()
        .map(|(s, e)| {
            let mut e = e;
            while e > s && !text[..e].chars().next_back().unwrap().is_alphanumeric() {
                e -= text[..e].chars().next_back().unwrap().len_utf8();
            }
            (s, e)
        })
        .filter(|(s, e)| e > s)
        .collect()
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let list_terms = args.iter().any(|a| a == "--list");
    let root = args
        .iter()
        .find(|a| !a.starts_with("--"))
        .expect("usage: glossary_consistency <path> [--list]")
        .clone();
    let root = Path::new(&root);

    let crawl = tool_wiki_core::crawl(root).expect("crawl failed");

    // --- Collect headwords, per language (the ES glossary defines Spanish forms) ---
    let mut defined: BTreeMap<bool, BTreeMap<String, String>> = BTreeMap::new(); // is_es -> norm -> defined form
    let mut sources: Vec<String> = Vec::new();
    for d in crawl.documents.iter().filter(|d| is_glossary_file(d)) {
        let terms = headwords(&d.body);
        if terms.is_empty() {
            continue;
        }
        sources.push(format!("{} ({} terms)", d.path.display(), terms.len()));
        let bucket = defined.entry(is_spanish(d)).or_default();
        for t in terms {
            bucket.entry(normalize(&t)).or_insert(t);
        }
    }

    if sources.is_empty() {
        println!("no glossary/terminology/vocabulary articles found under {} — nothing to check against", root.display());
        return;
    }
    sources.sort();
    println!("glossary sources:");
    for s in &sources {
        println!("  {s}");
    }
    let en_terms = defined.get(&false).map(BTreeMap::len).unwrap_or(0);
    let es_terms = defined.get(&true).map(BTreeMap::len).unwrap_or(0);
    println!("\n{en_terms} distinct English headwords, {es_terms} Spanish\n");
    if list_terms {
        for (es, bucket) in &defined {
            println!("--- {} headwords ---", if *es { "ES" } else { "EN" });
            for t in bucket.values() {
                println!("  {t}");
            }
        }
        println!();
    }

    // --- 1. drifted_form ---
    let mut drift: Vec<(String, String, String)> = Vec::new(); // path, used, defined
    for d in &crawl.documents {
        if is_glossary_file(d) {
            continue; // a glossary may legitimately discuss variant spellings
        }
        let Some(bucket) = defined.get(&is_spanish(d)) else { continue };
        let body = &d.body;
        let spans = word_spans(body);
        // Regions that are markup, not prose: link/slug text and the machine-generated
        // membership blocks (whose bullets echo slugs verbatim).
        let mut skip: Vec<(usize, usize)> = link_spans(body);
        skip.extend(d.auto_generated_blocks.iter().map(|b| (b.offset, b.end)));
        let mut seen: BTreeSet<(String, String)> = BTreeSet::new();
        for i in 0..spans.len() {
            for n in 1..=MAX_HEADWORD_WORDS {
                let Some(&(_, end)) = spans.get(i + n - 1) else { break };
                let start = spans[i].0;
                if skip.iter().any(|&(lo, hi)| start < hi && end > lo) {
                    continue;
                }
                let phrase = &body[start..end];
                if phrase.chars().count() < MIN_HEADWORD_CHARS {
                    continue;
                }
                let Some(canonical) = bucket.get(&normalize(phrase)) else { continue };
                if compare_key(phrase) != compare_key(canonical) {
                    if seen.insert((phrase.to_string(), canonical.clone())) {
                        drift.push((
                            d.path.display().to_string(),
                            phrase.split_whitespace().collect::<Vec<_>>().join(" "),
                            canonical.clone(),
                        ));
                    }
                }
            }
        }
    }
    drift.sort();
    println!("=== body text using a drifted form of a defined term ({}) ===", drift.len());
    for (path, used, canonical) in &drift {
        println!("  {path}: \"{used}\" — glossary defines \"{canonical}\"");
    }

    // --- 2. near_synonym_name ---
    // Index headwords by head noun so a name can be compared against same-noun terms only.
    let mut by_head: HashMap<String, Vec<&String>> = HashMap::new();
    for bucket in defined.values() {
        for term in bucket.values() {
            if term.split_whitespace().count() >= 2 {
                by_head.entry(head_noun(term)).or_default().push(term);
            }
        }
    }

    let mut names: Vec<(String, String)> = Vec::new(); // (label, name)
    if let Ok(yaml) = fs::read_to_string(root.join("categories.yaml")) {
        if let Ok(value) = serde_yaml::from_str::<serde_yaml::Value>(&yaml) {
            if let Some(cats) = value.get("categories").and_then(|c| c.as_sequence()) {
                for c in cats {
                    if let (Some(id), Some(name)) = (
                        c.get("id").and_then(|v| v.as_str()),
                        c.get("name").and_then(|v| v.as_str()),
                    ) {
                        names.push((format!("categories.yaml `{id}`.name"), name.to_string()));
                    }
                }
            }
        }
    }
    for d in &crawl.documents {
        if is_glossary_file(d) {
            continue;
        }
        if let Some(title) = d.frontmatter.title() {
            names.push((d.path.display().to_string(), title.to_string()));
        }
    }

    let mut near: Vec<(String, String, String)> = Vec::new();
    for (label, name) in &names {
        if name.split_whitespace().count() < 2 {
            continue;
        }
        let norm = normalize(name);
        if defined.values().any(|b| b.contains_key(&norm)) {
            continue; // exact match -- conformant
        }
        let Some(candidates) = by_head.get(&head_noun(name)) else { continue };
        let name_mod = modifier(name);
        for c in candidates {
            let c_mod = modifier(c);
            if c_mod.is_empty() || name_mod.is_empty() || c_mod == name_mod {
                continue;
            }
            // Same head noun, different modifier, and neither modifier contains the other
            // (`Regional Market` vs `Top-400 Regional Market` is a specialisation, not drift).
            if c_mod.contains(&name_mod) || name_mod.contains(&c_mod) {
                continue;
            }
            // Same length, differing in exactly one word. Without this, any long title
            // ending in a common noun pairs with every glossary term ending in that noun --
            // "Press releases and material change reports" vs "Technical report" was the
            // dominant false positive on the first live run, 36 of them on corporate alone.
            // Opus §R6.10's own examples ("Development Program" vs the defined "Rollout
            // Program") are all same-length, one-word-apart pairs.
            let nw: Vec<&str> = name.split_whitespace().collect();
            let cw: Vec<&str> = c.split_whitespace().collect();
            if nw.len() != cw.len() {
                continue;
            }
            let differing = nw
                .iter()
                .zip(cw.iter())
                .filter(|(a, b)| a.to_lowercase() != b.to_lowercase())
                .count();
            if differing != 1 {
                continue;
            }
            near.push((label.clone(), name.clone(), (*c).clone()));
        }
    }
    near.sort();
    near.dedup();
    println!(
        "\n=== names that are a near-synonym of a defined term ({}) ===",
        near.len()
    );
    for (label, name, term) in &near {
        println!("  {label}: \"{name}\" — glossary defines \"{term}\" (same head noun, different modifier)");
    }

    println!("\n{} findings", drift.len() + near.len());
}

#[cfg(test)]
mod glossary_tests {
    use super::*;

    const REAL_GLOSSARY: &str = "\
## A

**Accredited Investor** — An investor meeting the income, net asset, or professional-qualification thresholds set out in NI 45-106.

**Adjusted Funds From Operations (AFFO)** — Funds from operations less normalized maintenance capital expenditure.

## D

**Direct-Hold Solution** — The legal structure under which each investor holds equity units in a single named vehicle.

**Rollout Program** — The staged development sequence described in the offering documents.

Not a headword: **bolded lead-in** with no dash definition following it.
";

    #[test]
    fn extracts_the_real_headword_shape_and_splits_expansions() {
        let terms = headwords(REAL_GLOSSARY);
        assert!(terms.contains(&"Accredited Investor".to_string()));
        assert!(terms.contains(&"Direct-Hold Solution".to_string()));
        assert!(terms.contains(&"Rollout Program".to_string()));
        // The parenthetical expansion yields both forms.
        assert!(terms.contains(&"Adjusted Funds From Operations".to_string()));
        assert!(terms.contains(&"AFFO".to_string()));
        // A bolded phrase with no dash-led definition is not a headword.
        assert!(!terms.iter().any(|t| t.contains("bolded lead-in")));
    }

    #[test]
    fn normalize_collapses_hyphenation_and_case_but_not_plurals() {
        assert_eq!(normalize("Direct-Hold Solution"), normalize("Direct Hold Solution"));
        assert_eq!(normalize("Direct-Hold Solution"), normalize("direct hold solution"));
        assert_ne!(normalize("Direct-Hold Solution"), normalize("Direct-Hold Solutions"));
    }

    /// The core of sub-check 1: find a hyphenation variant in running prose, but leave
    /// ordinary sentence-case usage alone.
    #[test]
    fn finds_a_hyphenation_variant_and_ignores_mere_case() {
        let bucket: BTreeMap<String, String> = headwords(REAL_GLOSSARY)
            .into_iter()
            .map(|t| (normalize(&t), t))
            .collect();

        let hits = |body: &str| -> Vec<(String, String)> {
            let spans = word_spans(body);
            let mut out = Vec::new();
            for i in 0..spans.len() {
                for n in 1..=MAX_HEADWORD_WORDS {
                    let Some(&(_, end)) = spans.get(i + n - 1) else { break };
                    let phrase = &body[spans[i].0..end];
                    if let Some(c) = bucket.get(&normalize(phrase)) {
                        if compare_key(phrase) != compare_key(c) {
                            out.push((phrase.to_string(), c.clone()));
                        }
                    }
                }
            }
            out
        };

        let drifted = hits("Each investor holds units in a Direct Hold Solution.");
        assert_eq!(drifted.len(), 1);
        assert_eq!(drifted[0].0, "Direct Hold Solution");
        assert_eq!(drifted[0].1, "Direct-Hold Solution");

        assert!(
            hits("Each investor holds units in a direct-hold solution.").is_empty(),
            "sentence-case usage of the defined form is not drift"
        );
        assert!(
            hits("Two direct-hold solutions were formed.").is_empty(),
            "a plural is legitimate usage, not a variant spelling"
        );
    }

    /// §R6.10's documented example: "Development Program" against the defined
    /// "Rollout Program".
    #[test]
    fn same_head_noun_different_modifier_is_a_near_synonym() {
        assert_eq!(head_noun("Rollout Program"), head_noun("Development Program"));
        assert_ne!(modifier("Rollout Program"), modifier("Development Program"));
        // ... but a specialisation is not drift.
        assert!(modifier("Top-400 Regional Market").contains(&modifier("Regional Market")));
    }

    /// Guards the same-length / one-word-apart narrowing: without it, any long title ending
    /// in a common noun pairs with every glossary term ending in that noun.
    #[test]
    fn a_long_unrelated_title_is_not_a_near_synonym_of_a_short_term() {
        let name = "Press releases and material change reports";
        let term = "Technical report";
        assert_eq!(head_noun(name), head_noun(term), "the loose rule would pair these");
        assert_ne!(
            name.split_whitespace().count(),
            term.split_whitespace().count(),
            "the length rule rejects them"
        );

        let (a, b) = ("Development Program", "Rollout Program");
        assert_eq!(a.split_whitespace().count(), b.split_whitespace().count());
        let differing = a
            .split_whitespace()
            .zip(b.split_whitespace())
            .filter(|(x, y)| x.to_lowercase() != y.to_lowercase())
            .count();
        assert_eq!(differing, 1, "the documented real case survives the narrowing");
    }

    #[test]
    fn word_spans_slice_the_original_punctuation_back_out() {
        let text = "a Direct-Hold Solution, then";
        let spans = word_spans(text);
        let phrase = &text[spans[1].0..spans[2].1];
        assert_eq!(phrase, "Direct-Hold Solution");
    }
}

#[cfg(test)]
mod noise_suppression_tests {
    use super::*;

    #[test]
    fn a_trailing_sentence_period_is_not_drift() {
        // "Woodfine Capital Projects Inc." in the glossary; "...Inc" mid-sentence in prose.
        assert_eq!(
            compare_key("Woodfine Capital Projects Inc"),
            compare_key("Woodfine Capital Projects Inc.")
        );
    }

    #[test]
    fn a_line_wrap_inside_a_term_is_not_drift() {
        assert_eq!(
            compare_key("Woodfine Capital\nProjects Inc"),
            compare_key("Woodfine Capital Projects Inc.")
        );
    }

    #[test]
    fn an_internal_hyphen_difference_still_is_drift() {
        assert_ne!(compare_key("Direct Hold Solution"), compare_key("Direct-Hold Solution"));
        assert_eq!(
            normalize("Direct Hold Solution"),
            normalize("Direct-Hold Solution"),
            "same letters -- which is what makes it a variant rather than a different term"
        );
    }

    #[test]
    fn a_list_bullet_is_not_part_of_the_phrase() {
        let text = "- Corporate structure\n";
        let spans = word_spans(text);
        assert_eq!(&text[spans[0].0..spans[0].1], "Corporate");
        let phrase = &text[spans[0].0..spans[1].1];
        assert_eq!(phrase, "Corporate structure");
        assert_eq!(compare_key(phrase), compare_key("Corporate Structure"));
    }

    #[test]
    fn wikilink_slugs_and_auto_blocks_are_skipped() {
        let body = "See [[corporate-structure|Corporate structure]] and [x](/corporate-structure/).";
        let spans = link_spans(body);
        let slug_at = body.find("corporate-structure").unwrap();
        assert!(
            spans.iter().any(|&(lo, hi)| slug_at >= lo && slug_at < hi),
            "a wikilink slug normalises to the same key as the term it points at"
        );
        let url_at = body.rfind("corporate-structure").unwrap();
        assert!(spans.iter().any(|&(lo, hi)| url_at >= lo && url_at < hi));
    }
}

#[cfg(test)]
mod h3_headword_tests {
    use super::*;

    /// The documentation wiki's glossary shape. Supporting only the bolded-lead shape made
    /// this check report "no glossary articles found" for that entire wiki.
    const H3_GLOSSARY: &str = "\
## C

### Cascading Style Sheet or CSS

The stylesheet language used for presentation.

### Console OS

The purpose-built operating system for the console surface.

## A

### A
";

    #[test]
    fn h3_headwords_are_extracted_and_alternates_split() {
        let terms = headwords(H3_GLOSSARY);
        assert!(terms.contains(&"Cascading Style Sheet".to_string()));
        assert!(terms.contains(&"Console OS".to_string()));
        // The single-letter alphabet heading is below MIN_HEADWORD_CHARS, as intended.
        assert!(!terms.contains(&"A".to_string()));
        // So is "CSS" -- MIN_HEADWORD_CHARS is 4, so three-letter abbreviations (CSS, AIF,
        // PPN) are not tracked. A deliberate floor: short all-caps tokens appear in enough
        // unrelated contexts that drift/near-synonym matching on them is noise. Lowering the
        // floor for uppercase-only headwords is the obvious future refinement.
        assert!(!terms.contains(&"CSS".to_string()));
    }

    #[test]
    fn split_alternates_handles_both_languages() {
        assert_eq!(
            split_alternates("Artificial Intelligence or AI"),
            vec!["Artificial Intelligence".to_string(), "AI".to_string()]
        );
        assert_eq!(
            split_alternates("Inteligencia Artificial o IA"),
            vec!["Inteligencia Artificial".to_string(), "IA".to_string()]
        );
        assert_eq!(split_alternates("Console OS"), vec!["Console OS".to_string()]);
    }
}

#[cfg(test)]
mod headword_shape_tests {
    use super::*;

    /// `projects/buildings/woodfine-development-vocabulary.md`'s shape.
    #[test]
    fn bolded_lead_with_a_period_is_a_headword() {
        let body = "**Buying Program.** The portfolio-wide procurement mechanism through which Woodfine purchases directly.\n";
        assert_eq!(headwords(body), vec!["Buying Program".to_string()]);
    }

    /// `projects/reference/site-selection-terminology.md`'s shape.
    #[test]
    fn an_h2_headword_with_a_bolded_definition_sentence_is_a_headword() {
        let body = "## Power Centre\n\nA **Power Centre** is a large-format retail node anchored by National Retailers.\n";
        let terms = headwords(body);
        assert!(terms.contains(&"Power Centre".to_string()));
    }

    /// A comma-list heading defines several terms at once.
    #[test]
    fn a_comma_list_heading_yields_one_headword_per_term() {
        let terms = headwords("## Primary Target, Secondary Target, Tertiary Target\n");
        assert!(terms.contains(&"Primary Target".to_string()));
        assert!(terms.contains(&"Secondary Target".to_string()));
        assert!(terms.contains(&"Tertiary Target".to_string()));
    }

    /// `projects/reference/gis-cluster-scoring-glossary.md` packs three definitions onto one
    /// paragraph line; stopping at the first bolded run found only one of them.
    #[test]
    fn every_bolded_definition_on_a_line_is_extracted() {
        let body = "**Composition descriptor** — The anchor classes present. **Catchment rank** — A cluster's percentile position. **Scored cluster** — Any geo-located node evaluated.\n";
        let terms = headwords(body);
        assert!(terms.contains(&"Composition descriptor".to_string()));
        assert!(terms.contains(&"Catchment rank".to_string()));
        assert!(terms.contains(&"Scored cluster".to_string()));
    }

    #[test]
    fn structural_headings_are_not_terms() {
        let terms = headwords("## See also\n\n## What this is not\n\n## Véase también\n");
        assert!(terms.is_empty());
    }

    #[test]
    fn a_long_section_heading_is_not_a_term() {
        let terms = headwords("## Required Development Count and Site Shortlist for Every Jurisdiction\n");
        assert!(terms.is_empty(), "7 words -- a section title, not a headword");
    }

    #[test]
    fn a_bolded_run_that_is_neither_dash_led_nor_period_ended_is_not_a_headword() {
        let terms = headwords("This is **emphasis** in the middle of a sentence.\n");
        assert!(terms.is_empty());
    }
}
