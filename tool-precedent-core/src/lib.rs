// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.
//! tool-precedent-core: the generic shard. Zero dependencies.
//! Commands: changes-add, changes, versions, terms, terms-add, draws, draw-add, apply, catalog, registry, shared-hash.
pub mod adr07;
pub mod catalog;
pub mod changes;
pub mod csv;
pub mod draws;
pub mod error;
pub mod fence;
pub mod registry;
pub mod span;
pub mod store;
pub mod terms;
pub mod tree;
pub mod versions;

use std::path::{Path, PathBuf};
use std::process::ExitCode;

const VALUE_FLAGS: &[&str] = &[
    "--root",
    "--file",
    "--date",
    "--doc",
    "--version",
    "--baseline",
    "--section",
    "--old",
    "--new",
    "--tag",
    "--decision",
    "--term",
    "--replaces",
    "--status",
    "--project",
    "--from",
    "--from-version",
    "--note",
    "--dir",
    "--out",
];
const USAGE: &str = "usage: changes-add | changes | versions [--dir outputs] | terms | terms-add | draws | draw-add | apply <file.html> [--check] | catalog [--dir precedent] | registry check|classes|css|dtcg|lint | shared-hash   (all take --root DIR)";

fn flag(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}
fn positionals(args: &[String]) -> Vec<String> {
    let mut o = vec![];
    let mut skip = false;
    for a in args {
        if skip {
            skip = false;
            continue;
        }
        if VALUE_FLAGS.contains(&a.as_str()) {
            skip = true;
            continue;
        }
        if a.starts_with("--") {
            continue;
        }
        o.push(a.clone());
    }
    o
}
fn fail(m: &str) -> ExitCode {
    eprintln!("error: {m}");
    ExitCode::FAILURE
}
fn file(args: &[String], root: &Path, default: &str) -> PathBuf {
    match flag(args, "--file") {
        Some(f) => PathBuf::from(f),
        None => root.join(default),
    }
}

pub fn run(args: Vec<String>, default_root: &str) -> ExitCode {
    let Some(cmd) = args.first().cloned() else {
        return fail(USAGE);
    };
    let rest = &args[1..];
    let root = PathBuf::from(flag(rest, "--root").unwrap_or_else(|| default_root.into()));
    let g = |n: &str| flag(rest, n).unwrap_or_default();
    match cmd.as_str() {
        "changes-add" => {
            let row = changes::ChangeRow {
                doc_id: g("--doc"),
                version: g("--version"),
                baseline: g("--baseline"),
                section: g("--section"),
                baseline_text: g("--old"),
                new_text: g("--new"),
                tag: g("--tag"),
                decision_ref: g("--decision"),
                date: g("--date"),
            };
            if [
                &row.doc_id,
                &row.version,
                &row.baseline,
                &row.section,
                &row.tag,
                &row.decision_ref,
            ]
            .iter()
            .any(|v| v.is_empty())
            {
                return fail("changes-add requires --doc --version --baseline --section --old --new --tag --decision");
            }
            let f = file(rest, &root, "precedent/changes.csv");
            match changes::append_change(&f, &row) {
                Ok(()) => {
                    println!("appended to {}", f.display());
                    ExitCode::SUCCESS
                }
                Err(e) => fail(&e),
            }
        }
        "changes" => match changes::read_changes(&file(rest, &root, "precedent/changes.csv")) {
            Ok(rows) => {
                for r in rows.iter().filter(|r| {
                    flag(rest, "--doc").map(|d| r.doc_id == d).unwrap_or(true)
                        && flag(rest, "--version")
                            .map(|v| r.version == v)
                            .unwrap_or(true)
                }) {
                    println!(
                        "{} {}  {}  [{}]  {}  ->  {}",
                        r.doc_id, r.version, r.section, r.tag, r.baseline_text, r.new_text
                    );
                }
                ExitCode::SUCCESS
            }
            Err(e) => fail(&e),
        },
        "versions" => {
            let out = flag(rest, "--out")
                .map(PathBuf::from)
                .unwrap_or_else(|| file(rest, &root, "precedent/versions.csv"));
            let rows = versions::scan_versions(
                &root,
                &flag(rest, "--dir").unwrap_or_else(|| "outputs".into()),
            );
            match versions::write_versions(&rows, &out) {
                Ok(()) => {
                    println!("wrote {} version rows to {}", rows.len(), out.display());
                    ExitCode::SUCCESS
                }
                Err(e) => fail(&format!("{}: {e}", out.display())),
            }
        }
        "terms" => match terms::read_terms(&file(rest, &root, "precedent/terms.csv")) {
            Ok(rows) => {
                for r in terms::select(
                    &rows,
                    flag(rest, "--doc").as_deref(),
                    flag(rest, "--term").as_deref(),
                ) {
                    println!(
                        "{} {}  {}  [{}]{}",
                        r.doc_id,
                        r.version,
                        r.term,
                        r.status,
                        if r.replaces.is_empty() {
                            String::new()
                        } else {
                            format!("  replaces: {}", r.replaces)
                        }
                    );
                }
                ExitCode::SUCCESS
            }
            Err(e) => fail(&e),
        },
        "terms-add" => {
            let row = terms::TermRow {
                doc_id: g("--doc"),
                version: g("--version"),
                term: g("--term"),
                replaces: g("--replaces"),
                status: flag(rest, "--status").unwrap_or_else(|| "live".into()),
                date: g("--date"),
            };
            let f = file(rest, &root, "precedent/terms.csv");
            match terms::append_term(&f, &row) {
                Ok(()) => {
                    println!("appended to {}", f.display());
                    ExitCode::SUCCESS
                }
                Err(e) => fail(&e),
            }
        }
        "draws" => match draws::read_draws(&file(rest, &root, "precedent/draws.csv")) {
            Ok(rows) => {
                for r in draws::select(
                    &rows,
                    flag(rest, "--project").as_deref(),
                    flag(rest, "--doc").as_deref(),
                ) {
                    println!(
                        "{}: {} {} drew from {} {}  {}{}",
                        r.project,
                        r.doc_id,
                        r.version,
                        r.drew_from,
                        r.drew_from_version,
                        r.date,
                        if r.note.is_empty() {
                            String::new()
                        } else {
                            format!("  ({})", r.note)
                        }
                    );
                }
                ExitCode::SUCCESS
            }
            Err(e) => fail(&e),
        },
        "draw-add" => {
            let row = draws::DrawRow {
                project: g("--project"),
                doc_id: g("--doc"),
                version: g("--version"),
                drew_from: g("--from"),
                drew_from_version: g("--from-version"),
                date: g("--date"),
                note: g("--note"),
            };
            let f = file(rest, &root, "precedent/draws.csv");
            match draws::append_draw(&f, &row) {
                Ok(()) => {
                    println!("appended to {}", f.display());
                    ExitCode::SUCCESS
                }
                Err(e) => fail(&e),
            }
        }
        "apply" => match positionals(rest).first() {
            Some(f) => fence::apply(f, &root, rest),
            None => fail(USAGE),
        },
        "catalog" => {
            let dir = flag(rest, "--dir").unwrap_or_else(|| catalog::DEFAULT_DIR.into());
            match tree::Root::new(&root).and_then(|r| catalog::scan(&r, &dir)) {
                Ok(shards) => {
                    for s in shards {
                        println!(
                            "{}  {} rows{}  [{}]",
                            s.path,
                            s.rows,
                            if s.ragged > 0 {
                                format!(", {} ragged", s.ragged)
                            } else {
                                String::new()
                            },
                            s.columns.join(",")
                        );
                    }
                    ExitCode::SUCCESS
                }
                Err(e) => fail(&e.to_string()),
            }
        }
        "registry" => ExitCode::from(registry::run(rest, &root) as u8),
        "shared-hash" => {
            for (n, h) in shared_hashes() {
                println!("{h}  {n}");
            }
            ExitCode::SUCCESS
        }
        _ => fail(USAGE),
    }
}

/// FNV-1a 64 of each module that is copied between tool-precedent-core and tool-forecasting-core. The two lists must match line for line.
pub fn shared_hashes() -> Vec<(&'static str, String)> {
    vec![
        ("changes.rs", fence::hash(include_str!("changes.rs"))),
        ("csv.rs", fence::hash(include_str!("csv.rs"))),
        ("draws.rs", fence::hash(include_str!("draws.rs"))),
        ("fence.rs", fence::hash(include_str!("fence.rs"))),
        ("terms.rs", fence::hash(include_str!("terms.rs"))),
        ("versions.rs", fence::hash(include_str!("versions.rs"))),
    ]
}

#[cfg(test)]
mod tests {
    #[test]
    fn hash_is_fnv() {
        assert_eq!(crate::fence::hash(""), "cbf29ce484222325");
    }
    #[test]
    fn six_shared_modules() {
        assert_eq!(crate::shared_hashes().len(), 6);
    }
}
