// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.
//! Find: where is this used? Read-only views over documents already read once. The cross-document
//! glossary lists every defined term with the documents that define it, the documents that merely
//! use it, and how many distinct wordings its definition has. Wordings are grouped, never judged:
//! a difference is shown, not flagged. Computed from the files; nothing is stored.

use crate::language::{classify_base, defined_items, TermClass, Unit};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone)]
pub struct Doc {
    /// The file name shown to the person running the command.
    pub name: String,
    /// The instrument base (the name without its version chain).
    pub base: String,
    pub units: Vec<Unit>,
}

/// Who says a name is a party: an entity register, the person's residue list, or only the rules.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Source {
    /// Only the rules (acronym, entity word, name after a legal name): a suggestion.
    Rule,
    /// Found in an entity register; carries the entity code ("" when the register has none).
    Register(String),
    /// Marked by the person in the residue list.
    List,
}

/// The vocabulary a person or a register contributes. The tool only reads it.
#[derive(Debug, Clone, Default)]
pub struct Names {
    /// lower-cased entity title, alias or role -> entity code
    pub register: BTreeMap<String, String>,
    pub force_party: BTreeSet<String>,
    pub force_term: BTreeSet<String>,
}

/// Read the names of an entity register (any CSV): every value in a column whose header contains
/// title, name, alias, short or role, with the entity code from a code or node column when present.
pub fn read_register(csv_text: &str, into: &mut Names) {
    let rows = crate::csv::parse(csv_text);
    let Some(head) = rows.first() else { return };
    let h: Vec<String> = head
        .iter()
        .map(|c| c.trim_start_matches('\u{feff}').to_lowercase())
        .collect();
    let is = |c: &str, ws: &[&str]| ws.iter().any(|w| c.contains(w));
    let name_cols: Vec<usize> = (0..h.len())
        .filter(|i| is(&h[*i], &["title", "name", "alias", "short", "role"]))
        .collect();
    // a real code column ("op_code", "code") is preferred over a node id
    let coded: Vec<usize> = (0..h.len()).filter(|i| is(&h[*i], &["code"])).collect();
    let nodes: Vec<usize> = (0..h.len())
        .filter(|i| is(&h[*i], &["node"]) && !coded.contains(i))
        .collect();
    let code_cols: Vec<usize> = coded.into_iter().chain(nodes).collect();
    for r in &rows[1..] {
        let code = code_cols
            .iter()
            .filter_map(|i| r.get(*i))
            .map(|v| v.trim())
            .find(|v| !v.is_empty())
            .unwrap_or("")
            .to_string();
        for i in &name_cols {
            for piece in r.get(*i).map_or("", String::as_str).split([';', '|']) {
                let n = piece.trim().to_lowercase();
                if n.len() >= 2 {
                    into.register.entry(n).or_insert_with(|| code.clone());
                }
            }
        }
    }
}

/// Read a residue list: one name per line, `+name` marks a party, `-name` a term, `#` a comment.
pub fn read_residue(text: &str, into: &mut Names) {
    for line in text.lines() {
        let l = line.trim();
        if let Some(n) = l.strip_prefix('+') {
            into.force_party.insert(n.trim().to_lowercase());
        } else if let Some(n) = l.strip_prefix('-') {
            into.force_term.insert(n.trim().to_lowercase());
        }
    }
}

fn classify_with(term: &str, by_rule: TermClass, names: &Names) -> (TermClass, Source) {
    if names.force_term.contains(term) {
        return (TermClass::Term, Source::List);
    }
    if names.force_party.contains(term) {
        return (TermClass::Party, Source::List);
    }
    if let Some(code) = names.register.get(term) {
        return (TermClass::Party, Source::Register(code.clone()));
    }
    // a short name that begins a registered title ("acme" for "acme holdings inc.")
    if term.len() >= 4 {
        let lead = format!("{term} ");
        if let Some((_, code)) = names.register.iter().find(|(k, _)| k.starts_with(&lead)) {
            return (
                TermClass::Party,
                Source::Register(format!("{code} (partial)")),
            );
        }
    }
    (by_rule, Source::Rule)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub term: String,
    /// A defined term, or the name of a party or entity.
    pub class: TermClass,
    /// Where that classification comes from.
    pub source: Source,
    /// Documents (indexes) that define the term.
    pub defining: BTreeSet<usize>,
    /// Documents that use the term without defining it.
    pub using: BTreeSet<usize>,
    /// Definitions grouped by wording: each group is the documents whose definition reads the
    /// same (or nearly the same). More than one group means the wording differs.
    pub wordings: Vec<BTreeSet<usize>>,
    /// The defining sentence of each wording, as written (same order as `wordings`).
    pub texts: Vec<String>,
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

fn similar(a: &BTreeSet<u64>, b: &BTreeSet<u64>, near: f64) -> bool {
    if a == b {
        return true;
    }
    let i = a.intersection(b).count() as f64;
    let u = a.union(b).count() as f64;
    u > 0.0 && i / u >= near
}

/// Count whole-word occurrences of `term` (already lower-cased) in `text` (lower-cased).
pub fn occurrences(text: &str, term: &str) -> usize {
    if term.is_empty() {
        return 0;
    }
    let mut n = 0;
    let mut from = 0;
    while let Some(p) = text[from..].find(term) {
        let s = from + p;
        let e = s + term.len();
        let before_ok = text[..s]
            .chars()
            .next_back()
            .is_none_or(|c| !c.is_alphanumeric());
        let after_ok = text[e..]
            .chars()
            .next()
            .is_none_or(|c| !c.is_alphanumeric());
        if before_ok && after_ok {
            n += 1;
        }
        from = e;
    }
    n
}

/// The sentence of `text` that contains the quoted `term` (so two documents are compared on how
/// they DEFINE a term, not on the rest of the paragraph around it). Falls back to the whole text.
pub fn defining_sentence(text: &str, term: &str) -> String {
    match sentence_bounds(text, term) {
        Some((s, e)) => text
            .chars()
            .skip(s)
            .take(e - s)
            .collect::<String>()
            .trim()
            .to_string(),
        None => text.to_string(),
    }
}

/// The same sentence cut from the ORIGINAL-case text, so it can be read as written. `text` is
/// the lower-cased copy the bounds are found in; when the two differ in length it falls back to it.
pub fn defining_sentence_raw(raw: &str, text: &str, term: &str) -> String {
    match sentence_bounds(text, term) {
        Some((s, e)) if raw.chars().count() == text.chars().count() => raw
            .chars()
            .skip(s)
            .take(e - s)
            .collect::<String>()
            .trim()
            .to_string(),
        _ => defining_sentence(text, term),
    }
}

/// Character positions (start, end) of the sentence in `text` that contains the quoted `term`.
fn sentence_bounds(text: &str, term: &str) -> Option<(usize, usize)> {
    let chars: Vec<char> = text.chars().collect();
    let t: Vec<char> = term.chars().collect();
    let at = (0..chars.len()).find(|&i| {
        matches!(chars[i], '\u{201c}' | '"' | '\u{2018}')
            && chars.len() >= i + 1 + t.len()
            && chars[i + 1..i + 1 + t.len()] == t[..]
    })?;
    let is_end = |i: usize| {
        i + 1 < chars.len() && matches!(chars[i], '.' | ';' | ':') && chars[i + 1] == ' '
    };
    let start = (0..at).rev().find(|&i| is_end(i)).map_or(0, |i| i + 2);
    let end = (at..chars.len())
        .find(|&i| is_end(i))
        .map_or(chars.len(), |i| i + 1);
    Some((start, end))
}

/// The cross-document glossary over `docs`. `near` groups definitions that are at least that
/// similar (0.5 to 0.8 are useful); identical definitions always share a group.
pub fn glossary(docs: &[Doc], near: f64) -> Vec<Entry> {
    glossary_with(docs, near, &Names::default())
}

/// term -> (document -> definition texts as (lowercase, as written)).
type ByDoc = BTreeMap<usize, Vec<(String, String)>>;
/// Every defined term with its definitions per document, and how often each class was seen.
type Definitions = BTreeMap<String, (ByDoc, BTreeMap<TermClass, usize>)>;

fn collect_definitions(docs: &[Doc]) -> Definitions {
    let mut defs: Definitions = BTreeMap::new();
    for (d, doc) in docs.iter().enumerate() {
        for u in &doc.units {
            for item in defined_items(&u.raw) {
                let lower = defining_sentence(&u.text, &item.term);
                let shown = defining_sentence_raw(&u.raw, &u.text, &item.term);
                let e = defs.entry(item.term).or_default();
                e.0.entry(d).or_default().push((lower, shown));
                *e.1.entry(item.class).or_default() += 1;
            }
        }
    }
    defs
}

/// Group the first definition of each document by wording (documents whose definitions are at
/// least `near` similar share a group); biggest groups first.
fn group_wordings(by_doc: &ByDoc, near: f64) -> Vec<(BTreeSet<usize>, String)> {
    let reps: Vec<(usize, BTreeSet<u64>, String)> = by_doc
        .iter()
        .map(|(d, v)| (*d, shingles(&v[0].0), v[0].1.clone()))
        .collect();
    let mut groups: Vec<(BTreeSet<u64>, BTreeSet<usize>, String)> = Vec::new();
    for (d, sh, shown) in reps {
        match groups.iter_mut().find(|(g, _, _)| similar(g, &sh, near)) {
            Some((_, docs_in, _)) => {
                docs_in.insert(d);
            }
            None => groups.push((sh, BTreeSet::from([d]), shown)),
        }
    }
    let mut pairs: Vec<(BTreeSet<usize>, String)> =
        groups.into_iter().map(|(_, g, t)| (g, t)).collect();
    pairs.sort_by_key(|(g, _)| std::cmp::Reverse(g.len()));
    pairs
}

/// The glossary, with a register and a residue list applied to the classification.
pub fn glossary_with(docs: &[Doc], near: f64, names: &Names) -> Vec<Entry> {
    let mut out = Vec::new();
    for (term, (by_doc, classes)) in collect_definitions(docs) {
        let by_rule = if classes.get(&TermClass::Party).unwrap_or(&0)
            > classes.get(&TermClass::Term).unwrap_or(&0)
        {
            TermClass::Party
        } else {
            TermClass::Term
        };
        let (class, source) = classify_with(&term, by_rule, names);
        let defining: BTreeSet<usize> = by_doc.keys().copied().collect();
        let using: BTreeSet<usize> = docs
            .iter()
            .enumerate()
            .filter(|(d, _)| !defining.contains(d))
            .filter(|(_, doc)| doc.units.iter().any(|u| occurrences(&u.text, &term) > 0))
            .map(|(d, _)| d)
            .collect();
        let (wordings, texts): (Vec<_>, Vec<_>) = group_wordings(&by_doc, near).into_iter().unzip();
        out.push(Entry {
            term,
            class,
            source,
            defining,
            using,
            wordings,
            texts,
        });
    }
    out
}

/// A word-level difference between two wordings: shared words as written, each differing span as
/// `{words in the first -> words in the second}` (`∅` when one side has none). Deterministic.
pub fn word_diff(a: &str, b: &str) -> String {
    let x: Vec<&str> = a.split_whitespace().take(150).collect();
    let y: Vec<&str> = b.split_whitespace().take(150).collect();
    let (n, m) = (x.len(), y.len());
    let mut l = vec![vec![0u16; m + 1]; n + 1];
    for i in (0..n).rev() {
        for j in (0..m).rev() {
            l[i][j] = if x[i] == y[j] {
                l[i + 1][j + 1] + 1
            } else {
                l[i + 1][j].max(l[i][j + 1])
            };
        }
    }
    fn flush(out: &mut Vec<String>, del: &mut Vec<&str>, ins: &mut Vec<&str>) {
        if !del.is_empty() || !ins.is_empty() {
            let side = |v: &Vec<&str>| {
                if v.is_empty() {
                    "\u{2205}".to_string()
                } else {
                    v.join(" ")
                }
            };
            out.push(format!("{{{} \u{2192} {}}}", side(del), side(ins)));
            del.clear();
            ins.clear();
        }
    }
    let (mut i, mut j) = (0, 0);
    let (mut out, mut del, mut ins): (Vec<String>, Vec<&str>, Vec<&str>) = (vec![], vec![], vec![]);
    while i < n || j < m {
        if i < n && j < m && x[i] == y[j] {
            flush(&mut out, &mut del, &mut ins);
            out.push(x[i].to_string());
            i += 1;
            j += 1;
        } else if j < m && (i == n || l[i][j + 1] >= l[i + 1][j]) {
            ins.push(y[j]);
            j += 1;
        } else {
            del.push(x[i]);
            i += 1;
        }
    }
    flush(&mut out, &mut del, &mut ins);
    out.join(" ")
}

/// Documents whose file name carries this entity code (case-insensitive).
pub fn by_entity(docs: &[Doc], entity: &str) -> Vec<usize> {
    docs.iter()
        .enumerate()
        .filter(|(_, d)| {
            classify_base(&d.base).is_some_and(|c| c.entity.eq_ignore_ascii_case(entity))
        })
        .map(|(i, _)| i)
        .collect()
}

/// Units containing `phrase` (case-insensitive), as (document, number of units, number of hits).
pub fn by_text(docs: &[Doc], phrase: &str) -> Vec<(usize, usize, usize)> {
    let p = phrase.to_lowercase();
    docs.iter()
        .enumerate()
        .filter_map(|(i, d)| {
            let hits: Vec<usize> = d
                .units
                .iter()
                .map(|u| u.text.matches(&p).count())
                .filter(|n| *n > 0)
                .collect();
            (!hits.is_empty()).then(|| (i, hits.len(), hits.iter().sum()))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::language::{html_units, Kind};

    fn doc(name: &str, html: &str) -> Doc {
        Doc {
            name: name.to_string(),
            base: name.to_string(),
            units: html_units(html),
        }
    }

    #[test]
    fn counts_whole_word_occurrences_only() {
        assert_eq!(
            occurrences(
                "the affiliate and an affiliates list affiliate.",
                "affiliate"
            ),
            2
        );
        assert_eq!(occurrences("nothing here", "affiliate"), 0);
        assert_eq!(occurrences("x", ""), 0);
    }

    #[test]
    fn glossary_separates_defining_from_using_and_groups_wordings() {
        let a = doc("A", "<p>\u{201c}Affiliate\u{201d} means a person who controls or is controlled by another person under common control.</p>");
        let b = doc("B", "<p>\u{201c}Affiliate\u{201d} means a person who controls or is controlled by another person under common control.</p>");
        let c = doc("C", "<p>\u{201c}Affiliate\u{201d} means any entity that is a subsidiary of the sponsor or its parent company.</p>");
        let d = doc("D", "<p>The manager may appoint an affiliate to perform services on the usual commercial terms.</p>");
        let g = glossary(&[a, b, c, d], 0.5);
        assert_eq!(g.len(), 1);
        let e = &g[0];
        assert_eq!(e.term, "affiliate");
        assert_eq!(e.defining, BTreeSet::from([0, 1, 2]));
        assert_eq!(e.using, BTreeSet::from([3]));
        assert_eq!(e.wordings.len(), 2, "A and B read the same; C differs");
        assert_eq!(e.wordings[0], BTreeSet::from([0, 1]));
        assert_eq!(e.wordings[1], BTreeSet::from([2]));
    }

    #[test]
    fn finds_by_entity_code_and_by_phrase() {
        let mut a = doc(
            "COMPLIANCE_ENT-01_2026_01_02_Memo_A",
            "<p>The governing law is Ontario.</p><p>Ontario again.</p>",
        );
        a.base = "COMPLIANCE_ENT-01_2026_01_02_Memo_A".into();
        let mut b = doc(
            "COMPLIANCE_ENT-02_2026_01_03_Letter_B",
            "<p>Nothing relevant.</p>",
        );
        b.base = "COMPLIANCE_ENT-02_2026_01_03_Letter_B".into();
        let docs = vec![a, b];
        assert_eq!(by_entity(&docs, "ent-01"), vec![0]);
        assert_eq!(by_text(&docs, "ontario"), vec![(0, 2, 2)]);
        assert!(by_text(&docs, "absent phrase").is_empty());
        assert_eq!(docs[0].units[0].kind, Kind::Paragraph);
    }

    #[test]
    fn compares_the_defining_sentence_not_the_whole_paragraph() {
        let a = doc("A", "<p>Intro about something else entirely and unrelated to the term at hand. \u{201c}Holder\u{201d} means a person who holds units on the register of the partnership. A closing sentence that is only in the first document.</p>");
        let b = doc("B", "<p>A completely different opening sentence that has nothing in common at all. \u{201c}Holder\u{201d} means a person who holds units on the register of the partnership. Another different ending for the second one.</p>");
        let g = glossary(&[a, b], 0.8);
        assert_eq!(g.len(), 1);
        assert_eq!(
            g[0].wordings.len(),
            1,
            "same definition, different surrounding paragraph"
        );
        assert_eq!(
            defining_sentence("one. \u{201c}x\u{201d} means y; and more. last.", "x"),
            "\u{201c}x\u{201d} means y;"
        );
    }

    #[test]
    fn shows_each_wording_as_written() {
        let a = doc(
            "A",
            "<p>Intro. \u{201c}Holder\u{201d} means a Person who holds Units on the Register.</p>",
        );
        let c = doc("C", "<p>\u{201c}Holder\u{201d} means the registered owner of a unit, as shown in the register of the partnership.</p>");
        let g = glossary(&[a, c], 0.8);
        assert_eq!(g[0].wordings.len(), 2);
        assert!(
            g[0].texts
                .iter()
                .any(|t| t
                    == "\u{201c}Holder\u{201d} means a Person who holds Units on the Register."),
            "{:?}",
            g[0].texts
        );
        assert!(g[0]
            .texts
            .iter()
            .any(|t| t.starts_with("\u{201c}Holder\u{201d} means the registered owner")));
        assert_eq!(g[0].texts.len(), g[0].wordings.len());
    }

    #[test]
    fn word_diff_shows_the_slots() {
        let d = word_diff(
            "a holder of LP Units (a \u{201c}holder\u{201d}) is a limited partner, not a shareholder.",
            "a holder of Shares (a \u{201c}holder\u{201d}) is a shareholder of the SICC.",
        );
        assert!(d.contains("{LP Units \u{2192} Shares}"), "{d}");
        assert!(d.starts_with("a holder of"), "{d}");
        assert!(
            d.contains("a \u{201c}holder\u{201d}) is a"),
            "shared words stay as written: {d}"
        );
        assert_eq!(
            word_diff("same words here", "same words here"),
            "same words here"
        );
        assert_eq!(
            word_diff("one two", "one two three"),
            "one two {\u{2205} \u{2192} three}"
        );
    }

    #[test]
    fn entities_and_inline_tags_read_as_written() {
        let u = html_units("<p>GP Agreement &sect;3.1 &mdash; <b>&ldquo;holder&rdquo;</b>) &#9679; caf&eacute; &#x27;x&#x27; &unknown;</p>");
        assert_eq!(u[0].text, "gp agreement \u{a7}3.1 \u{2014} \u{201c}holder\u{201d}) \u{25cf} caf\u{e9} 'x' &unknown;");
    }

    #[test]
    fn registers_and_the_residue_list_override_the_rules() {
        let intro = |t: &str| {
            let mut s = String::from("<p>The agreement is made with Some Body Inc. (the \u{201c}");
            s.push_str(t);
            s.push_str("\u{201d}) and it has a few more words here.</p>");
            s
        };
        let rest = "<p>\u{201c}Mou\u{201d} means a memorandum. \u{201c}ACRO\u{201d} means something.</p><p>(the \u{201c}Indeval\u{201d}) is named.</p>";
        let docs = vec![
            doc("A", &format!("{}{}", intro("Acme"), rest)),
            doc("B", &format!("{}{}", intro("Acme"), rest)),
        ];
        let mut names = Names::default();
        read_register(
            "NODE_ID,ENTITY_TITLE,OP_CODE,ALIAS_OR_ROLE\n1,Acme Holdings Inc.,ACM-01,Sponsor; The Advisor\n",
            &mut names,
        );
        read_residue(
            "# parties and terms the rules get wrong\n+indeval\n-acro\n",
            &mut names,
        );
        let g = glossary_with(&docs, 0.5, &names);
        let by = |t: &str| {
            g.iter()
                .find(|e| e.term == t)
                .unwrap_or_else(|| panic!("{t}"))
        };
        assert_eq!(
            (by("acme").class, by("acme").source.clone()),
            (
                TermClass::Party,
                Source::Register("ACM-01 (partial)".into())
            ),
            "begins a registered title"
        );
        assert_eq!(
            (by("indeval").class, by("indeval").source.clone()),
            (TermClass::Party, Source::List)
        );
        assert_eq!(
            (by("acro").class, by("acro").source.clone()),
            (TermClass::Term, Source::List),
            "an acronym the person says is a term"
        );
        assert_eq!(
            (by("mou").class, by("mou").source.clone()),
            (TermClass::Term, Source::Rule)
        );
        assert_eq!(
            names.register.get("the advisor").map(String::as_str),
            Some("ACM-01")
        );
        assert_eq!(
            names.register.get("sponsor").map(String::as_str),
            Some("ACM-01")
        );
    }
}
