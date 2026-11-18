// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.
//! Language probe, read-only. Reads a document once and lists its language as small units
//! (headings, paragraphs, table cells, defined terms) and classifies its file name
//! (`AREA_ENTITY_YYYY_MM_DD_Type_Name`). Deterministic, no model. It never writes anything and
//! its report holds counts only: no unit text, no names, no entity codes.

use std::collections::{BTreeMap, BTreeSet, HashMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Heading,
    Paragraph,
    Cell,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unit {
    pub kind: Kind,
    /// Whitespace-collapsed, lower-cased text used for matching.
    pub text: String,
    /// The same text with its original case, used to tell an acronym or a name from a term.
    pub raw: String,
}

const NAMED: &[(&str, &str)] = &[
    ("nbsp", " "),
    ("amp", "&"),
    ("lt", "<"),
    ("gt", ">"),
    ("quot", "\""),
    ("apos", "'"),
    ("rsquo", "\u{2019}"),
    ("lsquo", "\u{2018}"),
    ("ldquo", "\u{201c}"),
    ("rdquo", "\u{201d}"),
    ("sbquo", "\u{201a}"),
    ("bdquo", "\u{201e}"),
    ("sect", "\u{a7}"),
    ("para", "\u{b6}"),
    ("mdash", "\u{2014}"),
    ("ndash", "\u{2013}"),
    ("hellip", "\u{2026}"),
    ("bull", "\u{2022}"),
    ("middot", "\u{b7}"),
    ("copy", "\u{a9}"),
    ("reg", "\u{ae}"),
    ("trade", "\u{2122}"),
    ("deg", "\u{b0}"),
    ("euro", "\u{20ac}"),
    ("pound", "\u{a3}"),
    ("cent", "\u{a2}"),
    ("times", "\u{d7}"),
    ("divide", "\u{f7}"),
    ("laquo", "\u{ab}"),
    ("raquo", "\u{bb}"),
    ("thinsp", " "),
    ("ensp", " "),
    ("emsp", " "),
    ("shy", ""),
    ("aacute", "\u{e1}"),
    ("eacute", "\u{e9}"),
    ("iacute", "\u{ed}"),
    ("oacute", "\u{f3}"),
    ("uacute", "\u{fa}"),
    ("Aacute", "\u{c1}"),
    ("Eacute", "\u{c9}"),
    ("Iacute", "\u{cd}"),
    ("Oacute", "\u{d3}"),
    ("Uacute", "\u{da}"),
    ("ntilde", "\u{f1}"),
    ("Ntilde", "\u{d1}"),
    ("ccedil", "\u{e7}"),
    ("Ccedil", "\u{c7}"),
    ("agrave", "\u{e0}"),
    ("egrave", "\u{e8}"),
    ("ograve", "\u{f2}"),
    ("auml", "\u{e4}"),
    ("ouml", "\u{f6}"),
    ("uuml", "\u{fc}"),
    ("acirc", "\u{e2}"),
    ("ecirc", "\u{ea}"),
    ("ocirc", "\u{f4}"),
    ("szlig", "\u{df}"),
    ("iquest", "\u{bf}"),
    ("iexcl", "\u{a1}"),
];

/// Decode named and numeric HTML entities; an unknown entity is left as written.
fn decode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut rest = s;
    while let Some(p) = rest.find('&') {
        out.push_str(&rest[..p]);
        let tail = &rest[p..];
        let end = tail.find(';').filter(|e| *e <= 10);
        let decoded = end.and_then(|e| {
            let name = &tail[1..e];
            if let Some(num) = name.strip_prefix('#') {
                let n = match num.strip_prefix(['x', 'X']) {
                    Some(h) => u32::from_str_radix(h, 16).ok(),
                    None => num.parse().ok(),
                };
                n.and_then(char::from_u32).map(|c| c.to_string())
            } else {
                NAMED
                    .iter()
                    .find(|(k, _)| *k == name)
                    .map(|(_, v)| v.to_string())
            }
        });
        match (decoded, end) {
            (Some(d), Some(e)) => {
                out.push_str(&d);
                rest = &tail[e + 1..];
            }
            _ => {
                out.push('&');
                rest = &tail[1..];
            }
        }
    }
    out.push_str(rest);
    out
}

fn is_block_tag(name: &str) -> bool {
    matches!(
        name,
        "p" | "br"
            | "li"
            | "ul"
            | "ol"
            | "td"
            | "th"
            | "tr"
            | "table"
            | "thead"
            | "tbody"
            | "div"
            | "section"
            | "article"
            | "header"
            | "footer"
            | "blockquote"
            | "hr"
            | "h1"
            | "h2"
            | "h3"
            | "h4"
            | "h5"
            | "h6"
            | "caption"
            | "dd"
            | "dt"
            | "pre"
    )
}

/// Remove tags. A block-level tag (or a line break) separates words; an inline tag (bold,
/// emphasis, span, link, mark) does not, so `"<b>x</b>)"` keeps its bracket against the word.
fn strip_tags(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut tag = String::new();
    let mut in_tag = false;
    for c in s.chars() {
        match c {
            '<' => {
                in_tag = true;
                tag.clear();
            }
            '>' if in_tag => {
                in_tag = false;
                let name: String = tag
                    .trim_start_matches(['/', '!'])
                    .chars()
                    .take_while(|c| c.is_ascii_alphanumeric())
                    .collect::<String>()
                    .to_ascii_lowercase();
                if is_block_tag(&name) {
                    out.push(' ');
                }
            }
            _ if in_tag => tag.push(c),
            _ => out.push(c),
        }
    }
    out
}

fn collapse(s: &str) -> String {
    decode(&strip_tags(s))
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

/// Remove `<script>` and `<style>` blocks (case-insensitive) before reading units.
fn drop_blocks(html: &str) -> String {
    let lower = html.to_ascii_lowercase();
    let mut out = String::with_capacity(html.len());
    let mut at = 0;
    while at < html.len() {
        let next = ["<script", "<style"]
            .iter()
            .filter_map(|t| lower[at..].find(t).map(|p| (at + p, *t)))
            .min_by_key(|(p, _)| *p);
        match next {
            None => {
                out.push_str(&html[at..]);
                break;
            }
            Some((p, tag)) => {
                out.push_str(&html[at..p]);
                let close = format!("</{}>", &tag[1..]);
                at = match lower[p..].find(&close) {
                    Some(e) => p + e + close.len(),
                    None => html.len(),
                };
            }
        }
    }
    out
}

/// Units of an HTML document, in order: headings, paragraphs (`p`, `li`) and table cells.
pub fn html_units(html: &str) -> Vec<Unit> {
    let h = drop_blocks(html);
    let lower = h.to_ascii_lowercase();
    let bytes = lower.as_bytes();
    let mut units = Vec::new();
    let mut i = 0;
    while let Some(rel) = lower[i..].find('<') {
        let start = i + rel;
        let name_end = lower[start + 1..]
            .find(|c: char| !c.is_ascii_alphanumeric())
            .map(|p| start + 1 + p)
            .unwrap_or(lower.len());
        let name = &lower[start + 1..name_end];
        let kind = match name {
            "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => Some(Kind::Heading),
            "p" | "li" => Some(Kind::Paragraph),
            "td" | "th" => Some(Kind::Cell),
            _ => None,
        };
        i = start + 1;
        if let Some(kind) = kind {
            let close = format!("</{name}>");
            if let Some(e) = lower[name_end..].find(&close) {
                let inner_start = lower[name_end..]
                    .find('>')
                    .map(|p| name_end + p + 1)
                    .unwrap_or(name_end);
                if inner_start <= name_end + e {
                    let raw = collapse(&h[inner_start..name_end + e]);
                    if !raw.is_empty() {
                        units.push(Unit {
                            kind,
                            text: raw.to_lowercase(),
                            raw,
                        });
                    }
                }
                i = name_end + e + close.len();
                if i >= bytes.len() {
                    break;
                }
            }
        }
    }
    units
}

/// One quoted definition found in a text.
struct Scan {
    surface: String,
    /// Up to 90 characters before the opening quote.
    before: String,
    /// Introduced in a parenthesis: `(the "X")`, `("X")`, `(a "X")`.
    paren: bool,
    /// Introduced by a verb: `"X" means ...`.
    verb: bool,
}

fn scan(text: &str) -> Vec<Scan> {
    let chars: Vec<char> = text.chars().collect();
    let mut out = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        let open = chars[i];
        if open == '"' || open == '\u{201c}' {
            let close_ch = if open == '"' { '"' } else { '\u{201d}' };
            if let Some(len) = chars[i + 1..].iter().take(61).position(|c| *c == close_ch) {
                let surface: String = chars[i + 1..i + 1 + len].iter().collect();
                let lo = i.saturating_sub(90);
                let before: String = chars[lo..i].iter().collect();
                let after: String = chars[i + 2 + len..].iter().take(20).collect();
                let tail: String = before
                    .chars()
                    .rev()
                    .take(6)
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                    .collect();
                let tl = tail.to_lowercase();
                let paren =
                    tl.trim_end().ends_with('(') || tl.contains("(the ") || tl.contains("(a ");
                let verb = {
                    let a = after.trim_start().to_lowercase();
                    a.starts_with("means")
                        || a.starts_with("has the meaning")
                        || a.starts_with("shall mean")
                };
                if (paren || verb) && surface.chars().next().is_some_and(|c| c.is_alphabetic()) {
                    out.push(Scan {
                        surface,
                        before,
                        paren,
                        verb,
                    });
                }
                i += len + 2;
                continue;
            }
        }
        i += 1;
    }
    out
}

/// Defined terms (lower-cased): a quoted capitalised phrase introduced as `(the "X")`, `("X")` or
/// followed by `means` / `has the meaning` / `shall mean`. Both straight and curly quotes.
pub fn defined_terms(text: &str) -> Vec<String> {
    scan(text)
        .into_iter()
        .map(|s| s.surface.to_lowercase())
        .collect()
}

/// Whether a quoted short name is a defined term or the name of a party or entity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TermClass {
    Term,
    Party,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Defined {
    /// Lower-cased.
    pub term: String,
    pub class: TermClass,
}

/// Abbreviations that mark a legal entity (they may end with a full stop).
const ENTITY_ABBREVIATIONS: &[&str] = &[
    "inc.", "inc", "ltd.", "ltd", "llc", "lp", "l.p.", "s.a.", "s.l.", "corp.", "corp", "plc",
    "gmbh", "b.v.", "n.v.", "s.c.", "co.",
];
/// Whole words that mark a legal entity (not when they end a sentence).
const ENTITY_WORDS: &[&str] = &[
    "limited",
    "corporation",
    "company",
    "trust",
    "fideicomiso",
    "partnership",
    "sicc",
    "socimi",
    "fibra",
];

fn is_entity_word(w: &str) -> bool {
    let w = w.trim_end_matches(',').to_lowercase();
    ENTITY_ABBREVIATIONS.contains(&w.as_str()) || ENTITY_WORDS.contains(&w.as_str())
}

/// Classify one found definition. A party is an acronym in capitals, a name that ENDS with an
/// entity word, or a short name introduced in a parenthesis immediately after a legal name (the
/// word before the parenthesis is an entity word, and not one that ends the previous sentence).
fn classify(s: &Scan) -> TermClass {
    let t = s.surface.trim();
    let letters = t.chars().filter(|c| c.is_alphabetic()).count();
    let acronym =
        (2..=8).contains(&letters) && t.chars().all(|c| !c.is_alphabetic() || c.is_uppercase());
    // the name ENDS with an entity word ("... Example LP"), not merely contains one ("LP units")
    let in_term = t.split_whitespace().last().is_some_and(is_entity_word);
    let after_name = s.paren && !s.verb && {
        let upto = s.before.rfind('(').unwrap_or(s.before.len());
        // the word immediately before the parenthesis must itself be the end of a legal name
        s.before[..upto]
            .split_whitespace()
            .last()
            .is_some_and(is_entity_word)
    };
    if acronym || in_term || after_name {
        TermClass::Party
    } else {
        TermClass::Term
    }
}

/// Definitions in a text with their class. Pass the ORIGINAL-case text.
pub fn defined_items(raw: &str) -> Vec<Defined> {
    scan(raw)
        .iter()
        .map(|s| Defined {
            term: s.surface.to_lowercase(),
            class: classify(s),
        })
        .collect()
}

/// The classification a file name carries: `AREA_ENTITY_YYYY_MM_DD_Type_Name` (the name here
/// is the base without its version chain).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NameClass {
    pub area: String,
    pub entity: String,
    pub date: String,
    pub kind: String,
}

pub fn classify_base(base: &str) -> Option<NameClass> {
    let t: Vec<&str> = base.split('_').collect();
    if t.len() < 6 {
        return None;
    }
    let digits = |s: &str, n: usize| s.len() == n && s.bytes().all(|b| b.is_ascii_digit());
    if !digits(t[2], 4) || !digits(t[3], 2) || !digits(t[4], 2) {
        return None;
    }
    let upper = |s: &str| !s.is_empty() && s.bytes().all(|b| b.is_ascii_uppercase() || b == b'-');
    let code = |s: &str| {
        !s.is_empty()
            && s.bytes()
                .all(|b| b.is_ascii_uppercase() || b.is_ascii_digit() || b == b'-')
    };
    if !upper(t[0]) || !code(t[1]) {
        return None;
    }
    Some(NameClass {
        area: t[0].to_string(),
        entity: t[1].to_string(),
        date: format!("{}-{}-{}", t[2], t[3], t[4]),
        kind: t[5].to_string(),
    })
}

fn fnv(s: &str) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in s.as_bytes() {
        h ^= *b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

fn shingles(text: &str) -> BTreeSet<u64> {
    let w: Vec<&str> = text.split(' ').collect();
    if w.len() < 4 {
        return BTreeSet::from([fnv(text)]);
    }
    w.windows(4).map(|x| fnv(&x.join(" "))).collect()
}

/// Counts only.
#[derive(Debug, Default)]
pub struct Report {
    pub documents: usize,
    pub headings: usize,
    pub paragraphs: usize,
    pub cells: usize,
    /// paragraphs of at least `min_words` words considered for twin counting
    pub considered: usize,
    pub exact_twin: usize,
    pub near_twin: usize,
    pub unique: usize,
    pub docs_sharing_exact: usize,
    pub defined_terms: usize,
    pub terms_in_2plus_docs: usize,
    pub names_classified: usize,
    pub entities: usize,
    pub entities_in_2plus_docs: usize,
    pub max_docs_one_entity: usize,
    pub kinds: BTreeMap<String, usize>,
}

/// One document: its base name (for classification) and its HTML.
pub fn report(docs: &[(String, String)], min_words: usize, near: f64) -> Report {
    let mut r = Report {
        documents: docs.len(),
        ..Default::default()
    };
    let mut paras: Vec<(usize, String)> = Vec::new();
    let mut term_docs: HashMap<String, BTreeSet<usize>> = HashMap::new();
    let mut ent_docs: HashMap<String, usize> = HashMap::new();
    for (d, (base, html)) in docs.iter().enumerate() {
        if let Some(c) = classify_base(base) {
            r.names_classified += 1;
            *ent_docs.entry(c.entity).or_default() += 1;
            *r.kinds.entry(c.kind).or_default() += 1;
        }
        for u in html_units(html) {
            match u.kind {
                Kind::Heading => r.headings += 1,
                Kind::Cell => r.cells += 1,
                Kind::Paragraph => r.paragraphs += 1,
            }
            if u.text.split(' ').count() >= min_words {
                for t in defined_terms(&u.text) {
                    term_docs.entry(t).or_default().insert(d);
                }
                paras.push((d, u.text));
            }
        }
        // terms in the raw text too (quotes are lost on lower-casing only for case, not for marks)
    }
    r.considered = paras.len();
    r.defined_terms = term_docs.len();
    r.terms_in_2plus_docs = term_docs.values().filter(|s| s.len() > 1).count();
    r.entities = ent_docs.len();
    r.entities_in_2plus_docs = ent_docs.values().filter(|n| **n > 1).count();
    r.max_docs_one_entity = ent_docs.values().copied().max().unwrap_or(0);
    // exact twins
    let mut by_text: HashMap<&str, BTreeSet<usize>> = HashMap::new();
    for (d, t) in &paras {
        by_text.entry(t.as_str()).or_default().insert(*d);
    }
    let exact: Vec<bool> = paras
        .iter()
        .map(|(_, t)| by_text[t.as_str()].len() > 1)
        .collect();
    r.exact_twin = exact.iter().filter(|b| **b).count();
    let mut sharing: BTreeSet<usize> = BTreeSet::new();
    for docs_of in by_text.values().filter(|s| s.len() > 1) {
        sharing.extend(docs_of.iter().copied());
    }
    r.docs_sharing_exact = sharing.len();
    // near twins: 4-word shingles, candidate pairs through an inverted index
    let sh: Vec<BTreeSet<u64>> = paras.iter().map(|(_, t)| shingles(t)).collect();
    let mut inv: HashMap<u64, Vec<usize>> = HashMap::new();
    for (i, s) in sh.iter().enumerate() {
        for x in s {
            inv.entry(*x).or_default().push(i);
        }
    }
    let mut cand: HashMap<(usize, usize), usize> = HashMap::new();
    for l in inv.values().filter(|l| l.len() > 1 && l.len() <= 40) {
        for a in 0..l.len() {
            for b in a + 1..l.len() {
                if paras[l[a]].0 != paras[l[b]].0 {
                    *cand.entry((l[a], l[b])).or_default() += 1;
                }
            }
        }
    }
    let mut near_set: BTreeSet<usize> = BTreeSet::new();
    for ((a, b), c) in cand {
        if c >= 3 {
            let inter = sh[a].intersection(&sh[b]).count() as f64;
            let uni = sh[a].union(&sh[b]).count() as f64;
            let j = inter / uni;
            if (near..1.0).contains(&j) {
                near_set.insert(a);
                near_set.insert(b);
            }
        }
    }
    r.near_twin = near_set.iter().filter(|i| !exact[**i]).count();
    r.unique = r.considered - r.exact_twin - r.near_twin;
    r
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_headings_paragraphs_and_cells_once() {
        let html = "<html><head><style>p{color:red}</style></head><body><h1>Title &amp; More</h1><p>Hello   <b>world</b>.</p><ul><li>one</li></ul><table><tr><th>H</th><td>cell</td></tr></table><script>var x='<p>no</p>';</script></body></html>";
        let u = html_units(html);
        let kinds: Vec<Kind> = u.iter().map(|x| x.kind).collect();
        assert_eq!(
            kinds,
            vec![
                Kind::Heading,
                Kind::Paragraph,
                Kind::Paragraph,
                Kind::Cell,
                Kind::Cell
            ]
        );
        assert_eq!(u[0].text, "title & more");
        assert_eq!(u[1].text, "hello world.");
        assert!(
            u.iter().all(|x| x.text != "no"),
            "script content is ignored"
        );
    }

    #[test]
    fn finds_defined_terms_in_both_quote_styles() {
        let t = "This agreement (the \u{201c}Agreement\u{201d}) is made. \u{201c}Affiliate\u{201d} means a person. The \"Other\" thing. (\"Fund\") holds.";
        let d = defined_terms(t);
        assert!(d.contains(&"agreement".to_string()));
        assert!(d.contains(&"affiliate".to_string()));
        assert!(d.contains(&"fund".to_string()));
        assert!(
            !d.contains(&"other".to_string()),
            "a bare quoted word is not a definition"
        );
    }

    #[test]
    fn classifies_a_file_name_without_opening_the_file() {
        let c = classify_base("COMPLIANCE_ENT-01-X_2026_10_24_Memo_Some-Name").unwrap();
        assert_eq!(
            (
                c.area.as_str(),
                c.entity.as_str(),
                c.date.as_str(),
                c.kind.as_str()
            ),
            ("COMPLIANCE", "ENT-01-X", "2026-10-24", "Memo")
        );
        assert!(classify_base("Memo_A").is_none());
        assert!(classify_base("COMPLIANCE_ENT-01_2026_1_24_Memo_X").is_none());
    }

    #[test]
    fn counts_exact_near_and_unique_paragraphs_across_documents() {
        let shared = "the manager shall act in the best interests of the partnership at all times and shall not engage in any competing business";
        let adapted = "the manager shall act in the best interests of the partnership at all times and shall not engage in any competing activity";
        let own_a = "this paragraph appears only in the first document and talks about something entirely different from the others here";
        let own_b = "completely separate wording that is found in the second document and in no other place at all for this test";
        let a = format!("<p>{shared}</p><p>{own_a}</p>");
        let b = format!("<p>{shared}</p><p>{own_b}</p>");
        let c = format!("<p>{adapted}</p>");
        let docs = vec![
            ("A".to_string(), a),
            ("B".to_string(), b),
            ("C".to_string(), c),
        ];
        let r = report(&docs, 12, 0.5);
        assert_eq!(r.considered, 5);
        assert_eq!(r.exact_twin, 2, "the shared paragraph in A and B");
        assert_eq!(r.near_twin, 1, "the adapted paragraph in C");
        assert_eq!(r.unique, 2);
        assert_eq!(r.docs_sharing_exact, 2);
    }

    #[test]
    fn entity_and_term_connections_are_counted() {
        let t = "<p>This agreement (the \"Fund\") is a long enough paragraph to be counted by the probe for sure, yes indeed.</p>";
        let docs = vec![
            (
                "COMPLIANCE_ENT-01_2026_01_02_Agreement_A".to_string(),
                t.to_string(),
            ),
            (
                "COMPLIANCE_ENT-01_2026_01_03_Memo_B".to_string(),
                t.to_string(),
            ),
            (
                "COMPLIANCE_ENT-02_2026_01_04_Letter_C".to_string(),
                "<p>x</p>".to_string(),
            ),
        ];
        let r = report(&docs, 12, 0.5);
        assert_eq!(
            (
                r.names_classified,
                r.entities,
                r.entities_in_2plus_docs,
                r.max_docs_one_entity
            ),
            (3, 2, 1, 2)
        );
        assert_eq!((r.defined_terms, r.terms_in_2plus_docs), (1, 1));
        assert_eq!(r.kinds.get("Memo"), Some(&1));
    }

    #[test]
    fn separates_parties_from_defined_terms() {
        let t = "Acme Holdings Inc. (the \u{201c}Acme\u{201d}) and the Comisi\u{f3}n (\"CNBV\") agree. \u{201c}Holder\u{201d} means a person. Example Partners LP (\u{201c}Example Partners LP\u{201d}) is a partnership. (the \u{201c}Closing\u{201d}) occurs later.";
        let d: std::collections::BTreeMap<String, TermClass> = defined_items(t)
            .into_iter()
            .map(|x| (x.term, x.class))
            .collect();
        assert_eq!(
            d["acme"],
            TermClass::Party,
            "short name right after a legal name"
        );
        assert_eq!(d["cnbv"], TermClass::Party, "acronym");
        assert_eq!(
            d["example partners lp"],
            TermClass::Party,
            "entity word in the name"
        );
        assert_eq!(d["holder"], TermClass::Term, "a means-definition is a term");
        assert_eq!(
            d["closing"],
            TermClass::Term,
            "a paren definition with no legal name before it"
        );
    }

    #[test]
    fn instruments_and_contract_names_are_terms_not_parties() {
        let t = "The deal between Acme Capital Inc. and Beta Holdings (the \u{201c}Advisory Services Agreement\u{201d}) closes. The \u{201c}LP Units\u{201d} means units. \u{201c}Trust Units\u{201d} means units. The Partnership Agreement (the \u{201c}Partnership Agreement\u{201d}) applies.";
        let d: std::collections::BTreeMap<String, TermClass> = defined_items(t)
            .into_iter()
            .map(|x| (x.term, x.class))
            .collect();
        assert_eq!(
            d["advisory services agreement"],
            TermClass::Term,
            "a legal name earlier in the sentence is not enough"
        );
        assert_eq!(
            d["lp units"],
            TermClass::Term,
            "contains an entity word but does not end with one"
        );
        assert_eq!(d["trust units"], TermClass::Term);
        assert_eq!(d["partnership agreement"], TermClass::Term);
    }
}
