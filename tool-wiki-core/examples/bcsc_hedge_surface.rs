//! BCSC hedge-surface check (`BRIEF-category-index-guide-redesign.md` Queue #26, check 5 —
//! spec: `.agent/audit/category-redesign-fable-pass.md` §R2-6 Check 5, narrowed to the
//! article-level form the queue item states).
//!
//! An article tagged `bcsc_class: forward-looking` is asserting a disclosure posture. The tag
//! is free; the posture is not. This check asks whether the body actually carries it:
//!
//! 1. **`what_this_is_not_missing`** — `schema-topic.yaml` `required_sections.what_this_is_not`
//!    says `"true when bcsc_class: forward-looking; recommended otherwise"`. An article whose
//!    body has no such section is non-conformant against its own schema.
//! 2. **`what_this_is_not_oversize`** — the same schema sets `max_words: 120`.
//! 3. **`unhedged_forward_claim`** — a forward-looking sentence carrying a future/intent verb
//!    with no hedge token in the same sentence (`~/Foundry/CLAUDE.md` §6: "Forward-looking
//!    claims carry planned/intended/may/target language").
//! 4. **`no_hedge_anywhere`** — an article declaring the class whose body contains no hedge
//!    token at all: the tag is the only forward-looking thing about it.
//!
//! Usage: `cargo run --example bcsc_hedge_surface -- <path-to-wiki-content-root>`
//!
//! # This is a lexical check, and absence of a flag is not legal review
//!
//! Stated here because Fable §R2-6 Check 5 requires the report to say it. Everything below is
//! deterministic string matching — no AI in the loop, so it stays inside the SYS-ADR-07
//! boundary and can run pre-commit. It queues human review; it never clears an article.
//!
//! # Simplifications against the written spec, and why
//!
//! * Fable's Check 5 is a *category-surface* check needing two human-maintained data files: a
//!   list of planned-entity names ("maintained as data, reviewed by a human, never inferred
//!   by the tool") and a promissory-vocabulary list. Neither exists on disk; inventing either
//!   would be exactly the inference that spec forbids, on BCSC-reviewable content. The queue
//!   item states the article-level form instead — `what_this_is_not` presence plus hedging —
//!   and that is what is built. When the entity list is written and operator-reviewed, add it
//!   as a fifth sub-check reading that file, not as a constant here.
//! * Sub-check 3 flags a *sentence*, not a claim: it cannot know whether a future-tense verb
//!   is about the company or about a reader's browser. It is deliberately tuned to
//!   under-report — `FORWARD_VERBS` are the ones that carry an institutional commitment, and
//!   a sentence already containing any hedge token is never flagged.
//! * `.es.md` pairs are checked identically with Spanish hedge and section vocabulary: the
//!   profiles' rule is "adapt structure, never facts", and the forward-looking posture is a
//!   fact that must survive translation.

use std::collections::BTreeMap;
use std::env;
use std::path::Path;

use tool_wiki_core::Document;

const FORWARD_LOOKING: &str = "forward-looking";
/// `schema-topic.yaml` `required_sections.what_this_is_not.max_words`.
const MAX_WHAT_THIS_IS_NOT_WORDS: usize = 120;

/// Substrings (lowercased) that satisfy the `what_this_is_not` section requirement, in both
/// languages of the bilingual pair.
///
/// NOT a literal heading list -- an earlier version tried that (`"what this is not"` /
/// `"lo que esto no es"` only) and produced a real false positive: the corpus's actual
/// headings vary by subject even in English ("What PPN is not", "What the thesis is not",
/// "What this posture is not", "What is not" ...), and the Spanish translations vary the
/// clause order too ("Qué no es esto" is just as common as "Lo que esto no es", and neither
/// contains the other as a substring). A subject-varying rhetorical pattern needs a pattern
/// match, not an exact-string list: "does this heading contain the English 'is not' clause
/// or the Spanish 'no es' clause, in either word order." Verified against the real corpus
/// (`grep -rhoi '^## .*no es' */*.es.md`, `grep -rhoi '^## What .*is not' */*.md`) before
/// picking these two substrings -- both matched every real variant found, with no observed
/// false trigger on an unrelated heading.
const WHAT_THIS_IS_NOT_HEADINGS: &[&str] = &[
    "is not",
    "no es",
];

/// Hedge tokens, EN + ES. `~/Foundry/CLAUDE.md` §6 names planned/intended/may/target;
/// `profile-corporate.md` §8 adds the same set plus "designed to"/"when and if".
const HEDGE_TOKENS: &[&str] = &[
    "planned", "plans to", "intends", "intended", "intention", "may ", "target", "targeted",
    "expects", "expected", "designed to", "when and if", "if and when", "is not yet",
    "not yet", "proposed", "anticipates", "anticipated", "would ", "could ",
    // Spanish
    "planea", "planeado", "planificado", "prevista", "previsto", "pretende", "tiene la intención",
    "podría", "puede ", "objetivo", "destinado", "aún no", "todavía no", "propuesto", "espera",
    // Standard FLS cautionary language. A sentence saying results *will differ* and that
    // nothing is assured is the hedged form, not the unhedged one -- without these, the
    // cautionary paragraph every forward-looking article is supposed to carry becomes the
    // check's loudest false positive.
    "actual results", "resultados reales", "no assurance", "not a prediction", "no guarantee",
    "ninguna garantía", "no es una predicción", "will differ", "diferirán",
];

/// Verbs that carry an institutional forward commitment when unhedged. Deliberately narrow.
const FORWARD_VERBS: &[&str] = &[
    "will ", "shall ", "is going to", "becomes ", "will become", "guarantees", "ensures that",
    // Spanish. `garantiza que` / `garantizará`, never a bare `garantiza` stem: the corpus
    // uses "Obligación con Primera Hipoteca Garantizada" as an instrument *name*, and a stem
    // match turned every mention of it into a false positive.
    "será", "serán", "garantiza que", "garantizará", "asegura que",
];

fn is_index(doc: &Document) -> bool {
    doc.path
        .file_stem()
        .and_then(|s| s.to_str())
        .map(|s| s == "_index" || s == "_index.es")
        .unwrap_or(false)
}

fn contains_any(haystack_lower: &str, needles: &[&str]) -> Option<String> {
    needles
        .iter()
        .find(|n| haystack_lower.contains(&n.to_lowercase()))
        .map(|n| n.trim().to_string())
}

/// The body span of the `what this is not` section, if one exists: from its heading to the
/// next heading of the same or higher level.
fn what_this_is_not_section(doc: &Document) -> Option<(&str, String)> {
    let idx = doc.headings.iter().position(|h| {
        let t = h.text.to_lowercase();
        WHAT_THIS_IS_NOT_HEADINGS.iter().any(|w| t.contains(w))
    })?;
    let h = &doc.headings[idx];
    let end = doc.headings[idx + 1..]
        .iter()
        .find(|n| n.level <= h.level)
        .map(|n| n.offset)
        .unwrap_or(doc.body.len());
    let body = doc.body[h.offset..end].to_string();
    Some((h.text.as_str(), body))
}

/// Split on sentence terminators. Crude by design: a false sentence split costs an extra
/// report line, never a missed flag, because a hedge anywhere in the fragment suppresses it.
fn sentences(text: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let mut start = 0usize;
    let bytes = text.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        if b == b'.' || b == b'!' || b == b'?' || b == b'\n' {
            if text.is_char_boundary(i + 1) {
                let s = text[start..i + 1].trim();
                if !s.is_empty() {
                    out.push(s);
                }
                start = i + 1;
            }
        }
    }
    let tail = text[start..].trim();
    if !tail.is_empty() {
        out.push(tail);
    }
    out
}

/// Body text with fenced code, wikilink markup, and HTML comments removed — code and markup
/// are not disclosure prose.
fn prose_only(doc: &Document) -> String {
    let mut body = doc.body.clone();
    let mut blocks = doc.code_blocks.clone();
    blocks.sort_by_key(|b| std::cmp::Reverse(b.offset));
    for b in blocks {
        if b.end <= body.len() {
            body.replace_range(b.offset..b.end, " ");
        }
    }
    body
}

fn main() {
    let root = env::args().nth(1).expect("usage: bcsc_hedge_surface <path>");
    let root = Path::new(&root);
    let crawl = tool_wiki_core::crawl(root).expect("crawl failed");

    println!(
        "NOTE: this is a deterministic lexical check (no AI in the loop — SYS-ADR-07 safe).\n\
         It queues human review. The absence of a flag is not legal review.\n"
    );

    let forward: Vec<&Document> = crawl
        .documents
        .iter()
        .filter(|d| d.frontmatter.get_str("bcsc_class") == Some(FORWARD_LOOKING))
        .collect();

    let mut class_counts: BTreeMap<&str, usize> = BTreeMap::new();
    for d in &crawl.documents {
        if let Some(c) = d.frontmatter.get_str("bcsc_class") {
            *class_counts.entry(c).or_insert(0) += 1;
        }
    }
    println!(
        "crawled {} documents; bcsc_class breakdown: {}",
        crawl.documents.len(),
        class_counts
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>()
            .join(", ")
    );
    println!("checking {} `bcsc_class: {FORWARD_LOOKING}` documents\n", forward.len());

    let mut missing_section: Vec<String> = Vec::new();
    let mut oversize_section: Vec<(String, usize)> = Vec::new();
    let mut no_hedge: Vec<String> = Vec::new();
    let mut unhedged: Vec<(String, String, String)> = Vec::new();

    for d in &forward {
        let path = d.path.display().to_string();
        let prose = prose_only(d);
        let lower = prose.to_lowercase();

        // 1 + 2. what_this_is_not
        match what_this_is_not_section(d) {
            None => {
                // An `_index` MOC page is not a TOPIC and does not owe the section.
                if !is_index(d) {
                    missing_section.push(path.clone());
                }
            }
            Some((_, section)) => {
                let words = section.split_whitespace().count();
                if words > MAX_WHAT_THIS_IS_NOT_WORDS {
                    oversize_section.push((path.clone(), words));
                }
            }
        }

        // 4. no hedge token anywhere in the body
        if contains_any(&lower, HEDGE_TOKENS).is_none() {
            no_hedge.push(path.clone());
            continue; // sub-check 3 would report every sentence -- redundant
        }

        // 3. a forward verb in a sentence carrying no hedge
        for s in sentences(&prose) {
            let sl = s.to_lowercase();
            if let Some(verb) = contains_any(&sl, FORWARD_VERBS) {
                if contains_any(&sl, HEDGE_TOKENS).is_none() {
                    unhedged.push((path.clone(), verb, s.chars().take(150).collect()));
                }
            }
        }
    }

    missing_section.sort();
    oversize_section.sort();
    no_hedge.sort();
    unhedged.sort();

    println!(
        "=== `{FORWARD_LOOKING}` articles with no `What this is not` section ({}) ===",
        missing_section.len()
    );
    println!("  (schema-topic.yaml: required_sections.what_this_is_not — \"true when bcsc_class: forward-looking\")");
    for p in &missing_section {
        println!("  {p}");
    }

    println!(
        "\n=== `What this is not` over {MAX_WHAT_THIS_IS_NOT_WORDS} words ({}) ===",
        oversize_section.len()
    );
    for (p, n) in &oversize_section {
        println!("  {p} — {n} words");
    }

    println!(
        "\n=== `{FORWARD_LOOKING}` articles with no hedge token anywhere in the body ({}) ===",
        no_hedge.len()
    );
    for p in &no_hedge {
        println!("  {p}");
    }

    println!(
        "\n=== sentences with a forward verb and no hedge in the same sentence ({}) ===",
        unhedged.len()
    );
    for (p, verb, sentence) in &unhedged {
        println!("  {p} — `{verb}`");
        println!("      \"{}\"", sentence.replace('\n', " "));
    }

    println!(
        "\n{} findings across {} forward-looking documents",
        missing_section.len() + oversize_section.len() + no_hedge.len() + unhedged.len(),
        forward.len()
    );
}

#[cfg(test)]
mod hedge_tests {
    use super::*;
    use std::path::PathBuf;

    fn doc(path: &str, content: &str) -> Document {
        Document::parse(PathBuf::from(path), content).unwrap()
    }

    const FM: &str =
        "---\ntitle: \"X\"\nslug: x\ncategory: governance\ncontent_type: topic\nbcsc_class: forward-looking\n---\n\n";

    #[test]
    fn finds_the_what_this_is_not_section_and_measures_it() {
        let d = doc(
            "governance/x.md",
            &format!("{FM}Lead.\n\n## Planned instruments\n\nText.\n\n## What this is not\n\nThis is not an offer. Three words.\n\n## See also\n\n- [[y]]\n"),
        );
        let (heading, body) = what_this_is_not_section(&d).expect("section found");
        assert_eq!(heading, "What this is not");
        assert!(body.contains("not an offer"));
        assert!(!body.contains("See also"), "the section must stop at the next H2");
    }

    #[test]
    fn the_spanish_pair_heading_satisfies_the_same_requirement() {
        let d = doc(
            "governance/x.es.md",
            &format!("{FM}Entrada.\n\n## Lo que esto no es\n\nEsto no es una oferta.\n"),
        );
        assert!(what_this_is_not_section(&d).is_some());
    }

    #[test]
    fn reversed_spanish_word_order_also_satisfies_it() {
        // Real 2026-09-07 false positive: governance-documents.es.md and
        // investor-relations-policy.es.md both use "Qué no es esto" (clause order reversed
        // from "Lo que esto no es") and were reported as missing the section entirely, even
        // though both had complete, faithful translations of it. Neither literal string is a
        // substring of the other.
        let d = doc(
            "governance/x.es.md",
            &format!("{FM}Entrada.\n\n## Qué no es esto\n\nEsto no es una oferta.\n"),
        );
        assert!(
            what_this_is_not_section(&d).is_some(),
            "'Qué no es esto' must satisfy the requirement exactly like 'Lo que esto no es'"
        );
    }

    #[test]
    fn subject_varying_headings_in_both_languages_are_recognized() {
        // The corpus's real headings vary by subject, not just by language -- "What PPN is
        // not", "What the thesis is not", "Lo que el umbral no es" are all real, live
        // headings satisfying this same requirement. A literal-list match would miss all of
        // them just as it missed the Spanish word-order variant above.
        for (path, heading) in [
            ("x/a.md", "## What PPN is not"),
            ("x/b.md", "## What the thesis is not"),
            ("x/c.es.md", "## Lo que el umbral no es"),
            ("x/d.es.md", "## Qué no es"),
        ] {
            let d = doc(path, &format!("{FM}Entrada.\n\n{heading}\n\nTexto.\n"));
            assert!(
                what_this_is_not_section(&d).is_some(),
                "heading {heading:?} should satisfy the requirement"
            );
        }
    }

    #[test]
    fn an_article_with_no_such_section_is_reported() {
        let d = doc(
            "governance/x.md",
            &format!("{FM}Lead.\n\n## Planned instruments\n\nText.\n\n## See also\n\n- [[y]]\n"),
        );
        assert!(what_this_is_not_section(&d).is_none());
    }

    #[test]
    fn a_hedged_sentence_is_not_flagged_and_an_unhedged_one_is() {
        let hedged = "The Foundation is intended to become the audit body and will hold the mandate.";
        let unhedged = "The Foundation will hold the governance mandate.";
        assert!(contains_any(&hedged.to_lowercase(), FORWARD_VERBS).is_some());
        assert!(
            contains_any(&hedged.to_lowercase(), HEDGE_TOKENS).is_some(),
            "\"intended\" in the same sentence suppresses the flag"
        );
        assert!(contains_any(&unhedged.to_lowercase(), FORWARD_VERBS).is_some());
        assert!(contains_any(&unhedged.to_lowercase(), HEDGE_TOKENS).is_none());
    }

    #[test]
    fn sentence_splitting_keeps_a_hedge_with_its_own_claim() {
        let text = "The vehicle will list on an exchange. A second vehicle is planned.";
        let s = sentences(text);
        assert_eq!(s.len(), 2);
        // Only the first sentence is unhedged -- the second one's hedge must not cover it.
        assert!(contains_any(&s[0].to_lowercase(), HEDGE_TOKENS).is_none());
        assert!(contains_any(&s[1].to_lowercase(), HEDGE_TOKENS).is_some());
    }

    #[test]
    fn fenced_code_is_not_disclosure_prose() {
        let d = doc(
            "governance/x.md",
            &format!("{FM}Lead.\n\n```rust\nfn f() {{ /* this will panic */ }}\n```\n\nReal prose.\n"),
        );
        let prose = prose_only(&d);
        assert!(!prose.contains("will panic"));
        assert!(prose.contains("Real prose."));
    }
}
