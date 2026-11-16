// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.
//! Lineage read from file names, read-only. The current rule (operator, 2026-11-15, confirmed by
//! Command): a version file is `<base>_<INITIALS><N>.<ext>` with ONE running number per file shared
//! by all people (`JW25`, then `MW26`, then `JW27`), initials 2 to 6 letters; a `_FIN` file has no
//! marker (`<base>_FIN.<ext>`, a later date in the base for a later `_FIN`). The earlier chained form
//! `<base>_<INITIALS><N>[_<INITIALS><N>...][_<VARIANT>...]` stays readable as history: a chain of
//! (who, that person's counter) segments, and optional variant words such as `_FIN`. This module only READS names (and optionally draw rows); it
//! never writes, renames or opens a document. The parent of a version is taken from a draw
//! row when one exists, otherwise inferred from the name and marked inferred.

use std::collections::{BTreeMap, BTreeSet};

/// Initials accepted by default. A tool may pass its own registry.
pub const DEFAULT_INITIALS: &[&str] = &["JW", "MW", "AI"];

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Seg {
    pub who: String,
    pub n: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parsed {
    pub base: String,
    pub chain: Vec<Seg>,
    /// Words after the chain, such as `FIN` or `BOOKLET` (not part of the version).
    pub variants: Vec<String>,
    /// Everything after the first dot of the last token (`html`, `docx`, `facts.toml`).
    pub ext: String,
}

impl Parsed {
    /// The version label: the chain without its leading underscore, for example `JW3_MW1`.
    pub fn label(&self) -> String {
        label_of(&self.chain)
    }
    /// The author of this file: the initials of the last segment.
    pub fn author(&self) -> &str {
        self.chain.last().map(|s| s.who.as_str()).unwrap_or("")
    }
}

pub fn label_of(chain: &[Seg]) -> String {
    if chain.is_empty() {
        return "FIN".to_string();
    }
    chain
        .iter()
        .map(|s| format!("{}{}", s.who, s.n))
        .collect::<Vec<_>>()
        .join("_")
}

fn seg_of(token: &str, initials: &[&str]) -> Option<Seg> {
    let split = token.find(|c: char| c.is_ascii_digit())?;
    let (who, digits) = token.split_at(split);
    if !(2..=6).contains(&who.len())
        || !who.bytes().all(|b| b.is_ascii_uppercase())
        || !initials.contains(&who)
        || digits.is_empty()
        || !digits.bytes().all(|b| b.is_ascii_digit())
    {
        return None;
    }
    Some(Seg {
        who: who.to_string(),
        n: digits.parse().ok()?,
    })
}

fn is_variant_word(token: &str) -> bool {
    !token.is_empty()
        && token.starts_with(|c: char| c.is_ascii_alphabetic())
        && token.bytes().all(|b| b.is_ascii_alphabetic() || b == b'-')
}

/// Parse a file name (not a path). `None` when it carries no version chain.
pub fn parse_name(file: &str, initials: &[&str]) -> Option<Parsed> {
    let last_us = file.rfind('_')?;
    let tail = &file[last_us + 1..];
    let dot = tail.find('.')?;
    let stem = &file[..last_us + 1 + dot];
    let ext = &file[last_us + 1 + dot + 1..];
    if ext.is_empty() {
        return None;
    }
    let tokens: Vec<&str> = stem.split('_').collect();
    let mut i = tokens.len();
    // trailing variant words
    while i > 0 && is_variant_word(tokens[i - 1]) && seg_of(tokens[i - 1], initials).is_none() {
        i -= 1;
    }
    let variants_start = i;
    // the chain, read right to left
    let mut chain: Vec<Seg> = Vec::new();
    while i > 0 {
        match seg_of(tokens[i - 1], initials) {
            Some(s) => {
                chain.push(s);
                i -= 1;
            }
            None => break,
        }
    }
    if chain.is_empty() || i == 0 {
        return None;
    }
    chain.reverse();
    Some(Parsed {
        base: tokens[..i].join("_"),
        chain,
        variants: tokens[variants_start..]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        ext: ext.to_string(),
    })
}

/// A marker-less `_FIN` file (`<base>_FIN.<ext>`): the person chose it from the drafts, so its
/// parent is not in the name (a draw row can record it). Only tried when the name has no marker.
fn parse_fin(file: &str) -> Option<Parsed> {
    let (stem, ext) = file.split_once('.').filter(|(_, e)| !e.is_empty())?;
    let base = stem.strip_suffix("_FIN")?;
    if base.is_empty() {
        return None;
    }
    Some(Parsed {
        base: base.to_string(),
        chain: Vec::new(),
        variants: vec!["FIN".to_string()],
        ext: ext.to_string(),
    })
}

#[derive(Debug, Clone)]
pub struct Version {
    pub chain: Vec<Seg>,
    pub variants: BTreeSet<String>,
    pub exts: BTreeSet<String>,
    /// The original file names (one per rendition), exactly as listed.
    pub files: BTreeSet<String>,
    pub parent: Option<String>,
    pub inferred: bool,
}

#[derive(Debug, Default)]
pub struct Instrument {
    pub versions: BTreeMap<String, Version>,
}

/// A draw row: (base, version label, drew_from version label in the same document).
pub type Draw = (String, String, String);

fn infer_parent(chain: &[Seg], have: &BTreeSet<String>) -> Option<String> {
    let last = chain.last()?;
    if last.n > 1 {
        let mut c = chain.to_vec();
        let n = c.len();
        c[n - 1].n = last.n - 1;
        let l = label_of(&c);
        if have.contains(&l) {
            return Some(l);
        }
    }
    if chain.len() > 1 {
        let l = label_of(&chain[..chain.len() - 1]);
        if have.contains(&l) {
            return Some(l);
        }
    }
    // the running number: one counter shared by all people, so the parent is the highest single
    // number below this one, whoever made it (JW25 -> MW26 -> JW27)
    if chain.len() == 1 {
        let mut best: Option<(u32, &String)> = None;
        for l in have {
            if l.contains('_') {
                continue;
            }
            let Some(at) = l.find(|c: char| c.is_ascii_digit()) else {
                continue;
            };
            let Ok(n) = l[at..].parse::<u32>() else {
                continue;
            };
            if n < last.n && best.is_none_or(|(b, _)| n > b) {
                best = Some((n, l));
            }
        }
        if let Some((_, l)) = best {
            return Some(l.clone());
        }
    }
    // a gap: the highest existing number below this one for the same person on the same prefix
    if last.n > 1 {
        let prefix = &chain[..chain.len() - 1];
        let mut best: Option<(u32, String)> = None;
        for k in (1..last.n).rev() {
            let mut c = prefix.to_vec();
            c.push(Seg {
                who: last.who.clone(),
                n: k,
            });
            let l = label_of(&c);
            if have.contains(&l) {
                best = Some((k, l));
                break;
            }
        }
        if let Some((_, l)) = best {
            return Some(l);
        }
    }
    None
}

/// Build the lineage of every instrument from file names, with optional draw rows.
/// Returns the instruments and the count of file names that carry no version chain.
pub fn build(
    files: &[String],
    initials: &[&str],
    draws: &[Draw],
) -> (BTreeMap<String, Instrument>, usize) {
    let mut inst: BTreeMap<String, Instrument> = BTreeMap::new();
    let mut unparsed = 0;
    for f in files {
        match parse_name(f, initials).or_else(|| parse_fin(f)) {
            None => unparsed += 1,
            Some(p) => {
                let v = inst
                    .entry(p.base.clone())
                    .or_default()
                    .versions
                    .entry(p.label())
                    .or_insert(Version {
                        chain: p.chain.clone(),
                        variants: BTreeSet::new(),
                        exts: BTreeSet::new(),
                        files: BTreeSet::new(),
                        parent: None,
                        inferred: true,
                    });
                v.variants.extend(p.variants.iter().cloned());
                v.exts.insert(p.ext.clone());
                v.files.insert(f.clone());
            }
        }
    }
    for (base, i) in inst.iter_mut() {
        let have: BTreeSet<String> = i.versions.keys().cloned().collect();
        let labels: Vec<String> = i.versions.keys().cloned().collect();
        for l in labels {
            let drawn = draws
                .iter()
                .find(|(b, v, _)| b == base && *v == l)
                .map(|(_, _, from)| from.clone());
            let v = i.versions.get_mut(&l).expect("label from keys");
            match drawn {
                Some(from) => {
                    v.parent = Some(from);
                    v.inferred = false;
                }
                None => v.parent = infer_parent(&v.chain, &have),
            }
        }
    }
    (inst, unparsed)
}

impl Instrument {
    /// Versions nothing derives from. One head is the normal case; two or more is a fork.
    pub fn heads(&self) -> Vec<&str> {
        let parents: BTreeSet<&str> = self
            .versions
            .values()
            .filter_map(|v| v.parent.as_deref())
            .collect();
        self.versions
            .keys()
            .map(String::as_str)
            .filter(|l| !parents.contains(l))
            .collect()
    }
    /// The path from a version back to its root, oldest first.
    pub fn spine(&self, head: &str) -> Vec<String> {
        let mut out = vec![head.to_string()];
        let mut cur = head.to_string();
        while let Some(p) = self.versions.get(&cur).and_then(|v| v.parent.clone()) {
            if out.contains(&p) {
                break;
            }
            out.push(p.clone());
            cur = p;
        }
        out.reverse();
        out
    }
    /// Versions with no parent (the first version, or one whose earlier versions are missing).
    pub fn roots(&self) -> Vec<&str> {
        self.versions
            .iter()
            .filter(|(_, v)| v.parent.is_none())
            .map(|(k, _)| k.as_str())
            .collect()
    }
}

/// Counts only: nothing here names an instrument or a person.
#[derive(Debug, Default)]
pub struct Report {
    pub files: usize,
    pub unparsed: usize,
    pub instruments: usize,
    pub versions: usize,
    pub chained_versions: usize,
    pub by_author: BTreeMap<String, usize>,
    pub variants: BTreeMap<String, usize>,
    pub exts: BTreeMap<String, usize>,
    /// number of heads -> number of instruments
    pub heads: BTreeMap<usize, usize>,
    pub forks: usize,
    pub instruments_with_gaps: usize,
    pub instruments_starting_above_one: usize,
    pub max_chain: usize,
    pub longest_spine: usize,
}

pub fn report(files: &[String], initials: &[&str], draws: &[Draw]) -> Report {
    let (inst, unparsed) = build(files, initials, draws);
    let mut r = Report {
        files: files.len(),
        unparsed,
        instruments: inst.len(),
        ..Default::default()
    };
    for i in inst.values() {
        r.versions += i.versions.len();
        for v in i.versions.values() {
            if v.chain.len() > 1 {
                r.chained_versions += 1;
            }
            r.max_chain = r.max_chain.max(v.chain.len());
            if let Some(last) = v.chain.last() {
                *r.by_author.entry(last.who.clone()).or_default() += 1;
            }
            for w in &v.variants {
                *r.variants.entry(w.clone()).or_default() += 1;
            }
            for e in &v.exts {
                *r.exts.entry(e.clone()).or_default() += 1;
            }
        }
        let heads = i.heads();
        *r.heads.entry(heads.len()).or_default() += 1;
        if heads.len() > 1 {
            r.forks += 1;
        }
        if let Some(h) = heads.first() {
            r.longest_spine = r.longest_spine.max(i.spine(h).len());
        }
        let roots = i.roots();
        if roots.iter().any(|l| {
            let c = &i.versions[*l].chain;
            c.len() == 1 && c[0].n > 1
        }) {
            r.instruments_starting_above_one += 1;
        }
        if i.versions.values().any(|v| {
            v.parent.as_ref().is_some_and(|p| {
                let Some(last) = v.chain.last() else {
                    return false;
                };
                let want = {
                    let mut c = v.chain.clone();
                    let n = c.len();
                    c[n - 1].n = last.n.saturating_sub(1);
                    label_of(&c)
                };
                last.n > 1 && *p != want && v.chain.len() == 1
            })
        }) {
            r.instruments_with_gaps += 1;
        }
    }
    r
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(f: &str) -> Option<Parsed> {
        parse_name(f, DEFAULT_INITIALS)
    }

    #[test]
    fn parses_plain_chained_and_variant_names() {
        let a = p("Memo_A_JW5.html").unwrap();
        assert_eq!(
            (a.base.as_str(), a.label().as_str(), a.author()),
            ("Memo_A", "JW5", "JW")
        );
        let b = p("Memo_A_JW3_MW1.docx").unwrap();
        assert_eq!((b.label().as_str(), b.author()), ("JW3_MW1", "MW"));
        let c = p("Memo_A_JW5_FIN.pdf").unwrap();
        assert_eq!(
            (c.label().as_str(), c.variants.clone()),
            ("JW5", vec!["FIN".to_string()])
        );
        let d = p("COMPLIANCE_ENT-01_2026_10_24_Facts_X_JW1.facts.toml").unwrap();
        assert_eq!(
            (d.base.as_str(), d.ext.as_str()),
            ("COMPLIANCE_ENT-01_2026_10_24_Facts_X", "facts.toml")
        );
    }

    #[test]
    fn rejects_names_without_a_registered_chain() {
        assert!(
            p("Report_YE2011.csv").is_none(),
            "year-end token is not initials"
        );
        assert!(p("notes.txt").is_none());
        assert!(p("Memo_ZZ3.html").is_none(), "unregistered initials");
        assert!(p("JW3.html").is_none(), "no base");
        assert!(
            parse_name("Memo_ZZ3.html", &["ZZ"]).is_some(),
            "a tool can register more"
        );
    }

    #[test]
    fn linear_chain_has_one_head_and_a_spine() {
        let files: Vec<String> = (1..=4).map(|n| format!("X_JW{n}.html")).collect();
        let (inst, unparsed) = build(&files, DEFAULT_INITIALS, &[]);
        let i = &inst["X"];
        assert_eq!(unparsed, 0);
        assert_eq!(i.heads(), vec!["JW4"]);
        assert_eq!(i.spine("JW4"), vec!["JW1", "JW2", "JW3", "JW4"]);
        assert_eq!(i.roots(), vec!["JW1"]);
    }

    #[test]
    fn renditions_collapse_into_one_version() {
        let files = vec![
            "X_JW1.html".to_string(),
            "X_JW1.pdf".to_string(),
            "X_JW1.docx".to_string(),
        ];
        let (inst, _) = build(&files, DEFAULT_INITIALS, &[]);
        assert_eq!(inst["X"].versions.len(), 1);
        assert_eq!(inst["X"].versions["JW1"].exts.len(), 3);
    }

    #[test]
    fn two_people_deriving_from_the_same_file_is_a_fork() {
        let files: Vec<String> = ["X_JW3.html", "X_JW4.html", "X_JW3_MW1.docx"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let (inst, _) = build(&files, DEFAULT_INITIALS, &[]);
        let i = &inst["X"];
        assert_eq!(i.versions["JW3_MW1"].parent.as_deref(), Some("JW3"));
        assert_eq!(i.heads(), vec!["JW3_MW1", "JW4"]);
    }

    #[test]
    fn same_person_continuing_a_chain_replaces_the_last_segment() {
        let files: Vec<String> = ["X_JW3.html", "X_JW3_MW1.docx", "X_JW3_MW2.docx"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let (inst, _) = build(&files, DEFAULT_INITIALS, &[]);
        let i = &inst["X"];
        assert_eq!(i.versions["JW3_MW2"].parent.as_deref(), Some("JW3_MW1"));
        assert_eq!(i.heads(), vec!["JW3_MW2"]);
    }

    #[test]
    fn a_gap_is_bridged_and_a_draw_row_overrides_the_name() {
        let files: Vec<String> = ["X_JW1.html", "X_JW2.html", "X_JW4.html"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let (inst, _) = build(&files, DEFAULT_INITIALS, &[]);
        assert_eq!(inst["X"].versions["JW4"].parent.as_deref(), Some("JW2"));
        let draws = vec![("X".to_string(), "JW4".to_string(), "JW1".to_string())];
        let (inst, _) = build(&files, DEFAULT_INITIALS, &draws);
        let v = &inst["X"].versions["JW4"];
        assert_eq!((v.parent.as_deref(), v.inferred), (Some("JW1"), false));
        // JW2 is now a dead end beside the spine: two heads, JW2 and JW4
        assert_eq!(inst["X"].heads(), vec!["JW2", "JW4"]);
    }

    #[test]
    fn report_counts_without_naming_anything() {
        let files: Vec<String> = [
            "X_JW1.html",
            "X_JW2.html",
            "X_JW2_FIN.pdf",
            "Y_JW1.html",
            "Y_JW1_MW1.docx",
            "readme.txt",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        let r = report(&files, DEFAULT_INITIALS, &[]);
        assert_eq!(
            (
                r.files,
                r.unparsed,
                r.instruments,
                r.versions,
                r.chained_versions
            ),
            (6, 1, 2, 4, 1)
        );
        assert_eq!(r.by_author.get("MW"), Some(&1));
        assert_eq!(r.variants.get("FIN"), Some(&1));
        assert_eq!(r.heads.get(&1), Some(&2));
        assert_eq!(r.forks, 0);
        assert_eq!(r.longest_spine, 2);
    }

    #[test]
    fn running_number_links_people_in_order_and_fin_has_no_marker() {
        let files: Vec<String> = [
            "X_2026_11_15_Memo_JW25.html",
            "X_2026_11_15_Memo_MW26.html",
            "X_2026_11_15_Memo_JWB27.html",
            "X_2026_11_20_Memo_FIN.pdf",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        let (inst, unparsed) = build(&files, &["JW", "MW", "JWB"], &[]);
        assert_eq!(unparsed, 0);
        let i = &inst["X_2026_11_15_Memo"];
        assert_eq!(i.versions["MW26"].parent.as_deref(), Some("JW25"));
        assert_eq!(i.versions["JWB27"].parent.as_deref(), Some("MW26"));
        assert_eq!(i.heads(), vec!["JWB27"]);
        let f = &inst["X_2026_11_20_Memo"].versions["FIN"];
        assert_eq!((f.chain.len(), f.parent.clone()), (0, None));
        let drawn = vec![(
            "X_2026_11_20_Memo".to_string(),
            "FIN".to_string(),
            "JW25".to_string(),
        )];
        let (inst, _) = build(&files, &["JW", "MW", "JWB"], &drawn);
        assert_eq!(
            inst["X_2026_11_20_Memo"].versions["FIN"].parent.as_deref(),
            Some("JW25")
        );
        assert!(
            parse_name("Memo_ABCDEFG3.html", &["ABCDEFG"]).is_none(),
            "7 letters is too long"
        );
        assert!(
            report(&files, &["JW", "MW", "JWB"], &[])
                .by_author
                .get("MW")
                == Some(&1)
        );
    }
}
