// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.
//! Read-only surveys over a work folder: `lineage-report` and `language-report`. They list
//! files in place under a root-confined directory and print COUNTS ONLY: no file name, entity
//! code, term or text is ever printed, so a report is safe to share.

use crate::draws;
use crate::error::{CoreError, Result};
use crate::find::{self, Doc};
use crate::language::{self, classify_base, TermClass};
use crate::lineage::{self, Draw, DEFAULT_INITIALS};
use crate::tree::Root;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

fn list(root: &Root, dirs: &str) -> Result<Vec<(String, PathBuf)>> {
    let mut out = Vec::new();
    for d in dirs.split(',').map(str::trim).filter(|d| !d.is_empty()) {
        let dir = root.resolve(d)?;
        if !dir.is_dir() {
            continue;
        }
        for e in std::fs::read_dir(&dir)? {
            let e = e?;
            if e.path().is_file() {
                if let Some(n) = e.file_name().to_str() {
                    out.push((n.to_string(), e.path()));
                }
            }
        }
    }
    out.sort();
    Ok(out)
}

fn read_draws(root: &Root) -> Vec<Draw> {
    match root.resolve("precedent/draws.csv") {
        Ok(p) if p.is_file() => draws::read_draws(Path::new(&p))
            .unwrap_or_default()
            .into_iter()
            .filter(|r| r.drew_from == r.instrument)
            .map(|r| (r.instrument, r.version, r.drew_from_version))
            .collect(),
        _ => vec![],
    }
}

fn initials_arg(arg: Option<String>) -> Vec<String> {
    match arg {
        Some(s) => s
            .split(',')
            .map(|x| x.trim().to_string())
            .filter(|x| !x.is_empty())
            .collect(),
        None => DEFAULT_INITIALS.iter().map(|s| s.to_string()).collect(),
    }
}

fn pct(n: usize, of: usize) -> String {
    (100 * n + of / 2)
        .checked_div(of)
        .map_or_else(|| "0%".to_string(), |v| format!("{v}%"))
}

pub fn lineage_report(
    root: &Root,
    dirs: &str,
    initials: Option<String>,
    show_names: bool,
) -> Result<String> {
    let files: Vec<String> = list(root, dirs)?.into_iter().map(|(n, _)| n).collect();
    let ini = initials_arg(initials);
    let ini: Vec<&str> = ini.iter().map(String::as_str).collect();
    let draws = read_draws(root);
    let r = lineage::report(&files, &ini, &draws);
    let mut o = String::new();
    o.push_str(&format!(
        "files listed: {}   without a version chain: {}\n",
        r.files, r.unparsed
    ));
    o.push_str(&format!(
        "instruments: {}   versions: {}   chained versions (2+ people): {}   longest chain: {}\n",
        r.instruments, r.versions, r.chained_versions, r.max_chain
    ));
    o.push_str(&format!(
        "draw rows read (same-document): {}\n",
        draws.len()
    ));
    let heads: Vec<String> = r
        .heads
        .iter()
        .map(|(h, n)| format!("{h} head: {n}"))
        .collect();
    o.push_str(&format!(
        "heads per instrument: {}   forks (2+ heads): {}\n",
        heads.join(", "),
        r.forks
    ));
    o.push_str(&format!("instruments starting above 1 (earlier versions absent): {}   with a bridged number gap: {}\n", r.instruments_starting_above_one, r.instruments_with_gaps));
    o.push_str(&format!(
        "longest spine (versions on the path to a head): {}\n",
        r.longest_spine
    ));
    // Words taken from file names (initials, variant words, extensions) are shown only on request.
    if show_names {
        o.push_str(&format!("versions by author initials: {:?}\n", r.by_author));
        o.push_str(&format!(
            "variant words after the chain: {:?}\n",
            r.variants
        ));
        o.push_str(&format!("renditions (extensions) seen: {:?}\n", r.exts));
    } else {
        o.push_str(&format!(
            "author initials: {} distinct   variant words: {} distinct   renditions: {} distinct   (add --names to list them)\n",
            r.by_author.len(),
            r.variants.len(),
            r.exts.len()
        ));
    }
    Ok(o)
}

pub fn language_report(
    root: &Root,
    dirs: &str,
    initials: Option<String>,
    min_words: usize,
    near: f64,
    show_names: bool,
) -> Result<String> {
    let listed = list(root, dirs)?;
    let names: Vec<String> = listed.iter().map(|(n, _)| n.clone()).collect();
    let ini = initials_arg(initials);
    let ini: Vec<&str> = ini.iter().map(String::as_str).collect();
    let draws = read_draws(root);
    let (inst, _) = lineage::build(&names, &ini, &draws);
    let path_of: BTreeMap<&str, &PathBuf> = listed.iter().map(|(n, p)| (n.as_str(), p)).collect();
    // the current version(s) of every instrument: its heads, HTML rendition only
    let mut docs: Vec<(String, String)> = Vec::new();
    for (base, i) in &inst {
        for h in i.heads() {
            if let Some(f) = i.versions[h].files.iter().find(|f| f.ends_with(".html")) {
                if let Some(p) = path_of.get(f.as_str()) {
                    let html =
                        std::fs::read_to_string(p).map_err(|e| CoreError::Io(e.to_string()))?;
                    docs.push((base.clone(), html));
                }
            }
        }
    }
    let r = language::report(&docs, min_words, near);
    let mut o = String::new();
    o.push_str(&format!(
        "current HTML documents read (one per head): {}\n",
        r.documents
    ));
    o.push_str(&format!(
        "units: headings {}, paragraphs {}, table cells {}\n",
        r.headings, r.paragraphs, r.cells
    ));
    o.push_str(&format!(
        "paragraphs of {min_words}+ words considered: {}\n",
        r.considered
    ));
    o.push_str(&format!(
        "  exact twin in another document: {} ({})\n",
        r.exact_twin,
        pct(r.exact_twin, r.considered)
    ));
    o.push_str(&format!(
        "  near twin ({:.0}-99% similar) in another document: {} ({})\n",
        near * 100.0,
        r.near_twin,
        pct(r.near_twin, r.considered)
    ));
    o.push_str(&format!(
        "  no counterpart in any other document: {} ({})\n",
        r.unique,
        pct(r.unique, r.considered)
    ));
    o.push_str(&format!(
        "documents sharing at least one exact paragraph with another: {} of {}\n",
        r.docs_sharing_exact, r.documents
    ));
    o.push_str(&format!(
        "defined terms found: {}   used in 2+ documents: {}\n",
        r.defined_terms, r.terms_in_2plus_docs
    ));
    o.push_str(&format!(
        "file names that carry AREA_ENTITY_DATE_Type: {} of {}\n",
        r.names_classified, r.documents
    ));
    o.push_str(&format!(
        "entity codes: {}   in 2+ documents: {}   most documents under one code: {}\n",
        r.entities, r.entities_in_2plus_docs, r.max_docs_one_entity
    ));
    if show_names {
        o.push_str(&format!("document types: {:?}\n", r.kinds));
    } else {
        o.push_str(&format!(
            "document types: {} distinct   (add --names to list them)\n",
            r.kinds.len()
        ));
    }
    Ok(o)
}

/// What to look for.
pub enum Query {
    Glossary,
    Parties,
    Term(String),
    Entity(String),
    Text(String),
}

pub struct FindOpts {
    pub counts_only: bool,
    pub show: bool,
    pub show_text: bool,
    /// Entity register(s) to read, comma-separated paths (absolute, or relative to the root).
    pub entities: Option<String>,
    /// The residue list (default: precedent/parties.txt under the root, if it exists).
    pub parties_file: Option<String>,
    pub all_versions: bool,
    pub near: f64,
    pub min_docs: usize,
}

fn load_docs(
    root: &Root,
    dirs: &str,
    initials: Option<String>,
    all_versions: bool,
) -> Result<Vec<Doc>> {
    let listed = list(root, dirs)?;
    let names: Vec<String> = listed.iter().map(|(n, _)| n.clone()).collect();
    let ini = initials_arg(initials);
    let ini: Vec<&str> = ini.iter().map(String::as_str).collect();
    let (inst, _) = lineage::build(&names, &ini, &read_draws(root));
    let path_of: BTreeMap<&str, &PathBuf> = listed.iter().map(|(n, p)| (n.as_str(), p)).collect();
    let mut docs = Vec::new();
    for (base, i) in &inst {
        let labels: Vec<&str> = if all_versions {
            i.versions.keys().map(String::as_str).collect()
        } else {
            i.heads()
        };
        for l in labels {
            if let Some(f) = i.versions[l].files.iter().find(|f| f.ends_with(".html")) {
                if let Some(p) = path_of.get(f.as_str()) {
                    let html =
                        std::fs::read_to_string(p).map_err(|e| CoreError::Io(e.to_string()))?;
                    docs.push(Doc {
                        name: f.clone(),
                        base: base.clone(),
                        units: language::html_units(&html),
                    });
                }
            }
        }
    }
    Ok(docs)
}

fn load_names(root: &Root, o: &FindOpts) -> Result<find::Names> {
    let mut names = find::Names::default();
    let abs = |p: &str| {
        let p = Path::new(p.trim());
        if p.is_absolute() {
            p.to_path_buf()
        } else {
            root.base().join(p)
        }
    };
    if let Some(list) = &o.entities {
        for p in list.split(',').filter(|p| !p.trim().is_empty()) {
            let text =
                std::fs::read_to_string(abs(p)).map_err(|e| CoreError::Io(format!("{p}: {e}")))?;
            find::read_register(&text, &mut names);
        }
    }
    let residue = match &o.parties_file {
        Some(f) => Some(abs(f)),
        None => root
            .resolve("precedent/parties.txt")
            .ok()
            .filter(|p| p.is_file()),
    };
    if let Some(p) = residue {
        let text = std::fs::read_to_string(&p)
            .map_err(|e| CoreError::Io(format!("{}: {e}", p.display())))?;
        find::read_residue(&text, &mut names);
    }
    Ok(names)
}

fn source_tag(e: &find::Entry) -> String {
    match &e.source {
        find::Source::Rule if e.class == TermClass::Party => {
            "  [suggested by the rules]".to_string()
        }
        find::Source::Rule => String::new(),
        find::Source::List => "  [your list]".to_string(),
        find::Source::Register(c) if c.is_empty() => "  [entity register]".to_string(),
        find::Source::Register(c) => format!("  [entity register: {c}]"),
    }
}

pub fn find_report(
    root: &Root,
    dirs: &str,
    initials: Option<String>,
    q: Query,
    o: &FindOpts,
) -> Result<String> {
    let docs = load_docs(root, dirs, initials, o.all_versions)?;
    let mut out = format!(
        "documents read: {}{}\n",
        docs.len(),
        if o.all_versions {
            " (every version)"
        } else {
            " (current versions)"
        }
    );
    match q {
        Query::Glossary | Query::Parties | Query::Term(_) => {
            let names = load_names(root, o)?;
            let mut g = find::glossary_with(&docs, o.near, &names);
            let (terms_n, parties_n) = (
                g.iter()
                    .filter(|e| e.defining.len() >= o.min_docs && e.class == TermClass::Term)
                    .count(),
                g.iter()
                    .filter(|e| e.defining.len() >= o.min_docs && e.class == TermClass::Party)
                    .count(),
            );
            if let Query::Term(t) = &q {
                let t = t.to_lowercase();
                g.retain(|e| e.term.contains(&t));
            } else {
                let want = if matches!(q, Query::Parties) {
                    TermClass::Party
                } else {
                    TermClass::Term
                };
                g.retain(|e| e.defining.len() >= o.min_docs && e.class == want);
            }
            if o.counts_only {
                out.push_str(&format!("defined in {}+ documents: {} defined terms and {} parties or entities (the list below is one of the two)\n", o.min_docs, terms_n, parties_n));
            }
            g.sort_by(|a, b| {
                b.defining
                    .len()
                    .cmp(&a.defining.len())
                    .then(a.term.cmp(&b.term))
            });
            let with_variants = g.iter().filter(|e| e.wordings.len() > 1).count();
            if o.counts_only && g.iter().any(|e| e.class == TermClass::Party) {
                let n = |f: fn(&find::Source) -> bool| {
                    g.iter()
                        .filter(|e| e.class == TermClass::Party && f(&e.source))
                        .count()
                };
                out.push_str(&format!(
                    "parties by source: entity register {}, your list {}, suggested by the rules {}\n",
                    n(|s| matches!(s, find::Source::Register(_))),
                    n(|s| matches!(s, find::Source::List)),
                    n(|s| matches!(s, find::Source::Rule))
                ));
            }
            if o.counts_only {
                out.push_str(&format!("terms listed: {}   with more than one wording: {}   most documents defining one term: {}\n", g.len(), with_variants, g.first().map_or(0, |e| e.defining.len())));
                out.push_str(&format!(
                    "documents that use a listed term without defining it (sum over terms): {}\n",
                    g.iter().map(|e| e.using.len()).sum::<usize>()
                ));
            } else {
                for e in &g {
                    out.push_str(&format!(
                        "\"{}\"  defined in {}   used in {} more   wordings: {}{}\n",
                        e.term,
                        e.defining.len(),
                        e.using.len(),
                        e.wordings.len(),
                        source_tag(e)
                    ));
                    if o.show || o.show_text {
                        for (k, w) in e.wordings.iter().enumerate() {
                            let names = w
                                .iter()
                                .map(|d| docs[*d].name.as_str())
                                .collect::<Vec<_>>()
                                .join(", ");
                            if o.show_text {
                                out.push_str(&format!(
                                    "   wording {} ({} document{}): {}\n      in: {}\n",
                                    k + 1,
                                    w.len(),
                                    if w.len() == 1 { "" } else { "s" },
                                    e.texts.get(k).map_or("", String::as_str),
                                    names
                                ));
                                if k > 0 {
                                    if let (Some(first), Some(this)) =
                                        (e.texts.first(), e.texts.get(k))
                                    {
                                        out.push_str(&format!(
                                            "      differs from wording 1: {}\n",
                                            find::word_diff(first, this)
                                        ));
                                    }
                                }
                            } else {
                                out.push_str(&format!("   wording {}: {}\n", k + 1, names));
                            }
                        }
                    }
                }
            }
        }
        Query::Entity(e) => {
            let hit = find::by_entity(&docs, &e);
            if o.counts_only {
                let mut kinds: BTreeMap<String, usize> = BTreeMap::new();
                for d in &hit {
                    if let Some(c) = classify_base(&docs[*d].base) {
                        *kinds.entry(c.kind).or_default() += 1;
                    }
                }
                out.push_str(&format!(
                    "documents under that entity code: {}   by type: {:?}\n",
                    hit.len(),
                    kinds
                ));
            } else {
                for d in hit {
                    out.push_str(&format!("{}\n", docs[d].name));
                }
            }
        }
        Query::Text(p) => {
            let hit = find::by_text(&docs, &p);
            if o.counts_only {
                out.push_str(&format!(
                    "documents containing the phrase: {}   units: {}   occurrences: {}\n",
                    hit.len(),
                    hit.iter().map(|h| h.1).sum::<usize>(),
                    hit.iter().map(|h| h.2).sum::<usize>()
                ));
            } else {
                for (d, units, n) in hit {
                    out.push_str(&format!(
                        "{}   {} unit(s), {} occurrence(s)\n",
                        docs[d].name, units, n
                    ));
                }
            }
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn scratch() -> PathBuf {
        use std::sync::atomic::{AtomicUsize, Ordering};
        static N: AtomicUsize = AtomicUsize::new(0);
        let d = std::env::temp_dir().join(format!(
            "tpc-survey-{}-{}",
            std::process::id(),
            N.fetch_add(1, Ordering::SeqCst)
        ));
        let _ = fs::remove_dir_all(&d);
        fs::create_dir_all(d.join("wip")).unwrap();
        d
    }

    #[test]
    fn reports_counts_only_and_writes_nothing() {
        let d = scratch();
        let p = "this is a long enough paragraph of words so that the probe counts it as a unit for the test run okay";
        fs::write(
            d.join("wip/COMPLIANCE_ENT-01_2026_01_02_Memo_A_JW1.html"),
            format!("<p>{p}</p>"),
        )
        .unwrap();
        fs::write(
            d.join("wip/COMPLIANCE_ENT-01_2026_01_02_Memo_A_JW2.html"),
            format!("<p>{p}</p>"),
        )
        .unwrap();
        fs::write(
            d.join("wip/COMPLIANCE_ENT-01_2026_01_02_Memo_A_JW2.pdf"),
            "x",
        )
        .unwrap();
        fs::write(
            d.join("wip/COMPLIANCE_ENT-01_2026_01_03_Memo_B_JW1.html"),
            format!("<p>{p}</p>"),
        )
        .unwrap();
        let before = fs::read_dir(d.join("wip")).unwrap().count();
        let root = Root::new(&d).unwrap();
        let l = lineage_report(&root, "wip", None, false).unwrap();
        assert!(l.contains("instruments: 2   versions: 3"));
        assert!(
            !l.contains("COMPLIANCE") && !l.contains("ENT-01"),
            "no names in the output"
        );
        let g = language_report(&root, "wip", None, 12, 0.5, false).unwrap();
        assert!(g.contains("current HTML documents read (one per head): 2"));
        assert!(g.contains("exact twin in another document: 2"));
        assert!(!g.contains("COMPLIANCE") && !g.contains("ENT-01") && !g.contains("long enough"));
        assert_eq!(
            fs::read_dir(d.join("wip")).unwrap().count(),
            before,
            "nothing was written"
        );
        assert!(lineage_report(&root, "../x", None, false).is_err());
        let _ = fs::remove_dir_all(d);
    }

    #[test]
    fn find_reports_terms_entities_and_text() {
        let d = scratch();
        let def = "<p>\u{201c}Fund\u{201d} means the limited partnership established by this agreement and its successors in title at law.</p>";
        fs::write(
            d.join("wip/COMPLIANCE_ENT-01_2026_01_02_Agreement_A_JW1.html"),
            def,
        )
        .unwrap();
        fs::write(
            d.join("wip/COMPLIANCE_ENT-01_2026_01_03_Memo_B_JW1.html"),
            def,
        )
        .unwrap();
        fs::write(
            d.join("wip/COMPLIANCE_ENT-02_2026_01_04_Letter_C_JW1.html"),
            "<p>The fund will pay its own costs out of the money it holds.</p>",
        )
        .unwrap();
        let root = Root::new(&d).unwrap();
        let o = FindOpts {
            counts_only: true,
            show: false,
            show_text: false,
            entities: None,
            parties_file: None,
            all_versions: false,
            near: 0.5,
            min_docs: 2,
        };
        let g = find_report(&root, "wip", None, Query::Glossary, &o).unwrap();
        assert!(g.contains("terms listed: 1   with more than one wording: 0   most documents defining one term: 2"), "{g}");
        assert!(g.contains("without defining it (sum over terms): 1"), "{g}");
        let e = find_report(&root, "wip", None, Query::Entity("ent-01".into()), &o).unwrap();
        assert!(e.contains("documents under that entity code: 2"), "{e}");
        let t = find_report(&root, "wip", None, Query::Text("the fund".into()), &o).unwrap();
        assert!(t.contains("documents containing the phrase: 1"), "{t}");
        assert!(
            !g.contains("COMPLIANCE") && !e.contains("COMPLIANCE") && !t.contains("COMPLIANCE"),
            "counts-only prints no names"
        );
        let named = FindOpts {
            counts_only: false,
            show: true,
            ..o
        };
        let g2 = find_report(&root, "wip", None, Query::Glossary, &named).unwrap();
        assert!(g2.contains("\"fund\"") && g2.contains("wording 1"));
        let _ = fs::remove_dir_all(d);
    }
}
