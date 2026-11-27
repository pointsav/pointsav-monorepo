// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.
//! tool-precedent-core: the generic shard. Zero dependencies.
//! Commands: changes-add, changes, versions, terms, terms-add, draws, draw-add, apply, catalog, registry, shared-hash.
#![forbid(unsafe_code)]
pub mod adr07;
pub mod catalog;
pub mod changes;
pub mod commands;
pub mod csv;
pub mod draws;
pub mod error;
pub mod fence;
pub mod find;
pub mod fuzz;
pub mod language;
pub mod lineage;
pub mod prose;
pub mod registry;
pub mod span;
pub mod store;
pub mod survey;
pub mod terms;
pub mod tree;
pub mod versions;
pub mod walk;

use std::path::{Path, PathBuf};
use std::process::ExitCode;

const VALUE_FLAGS: &[&str] = &[
    "--root",
    "--file",
    "--date",
    "--instrument",
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
    "--context",
    "--from",
    "--from-version",
    "--note",
    "--dir",
    "--out",
    "--initials",
    "--min-words",
    "--entity",
    "--text",
    "--min-docs",
    "--entities",
    "--parties-file",
    "--near",
];

/// True when `t` is one of this tool's own flags (so it cannot be the value of another flag).
fn is_known_flag(t: &str) -> bool {
    t == "--root"
        || VALUE_FLAGS.contains(&t)
        || commands::COMMANDS
            .iter()
            .any(|c| c.flags.iter().any(|f| f.name == t))
}
/// The value after `--name`. A following token that is itself a known flag means the value is
/// missing, so `--term --status retired` has no term.
fn flag(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .filter(|v| !is_known_flag(v))
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
/// A path a command writes to, given as `rel` (relative to `--root`): it must stay inside the root.
fn confine(root: &Path, rel: &str) -> std::result::Result<PathBuf, String> {
    tree::Root::new(root)
        .and_then(|r| r.resolve(rel))
        .map_err(|e| e.to_string())
}
/// The `--file` target of a command that writes: the default under the root, or the named path confined to the root.
fn write_file(args: &[String], root: &Path, default: &str) -> std::result::Result<PathBuf, String> {
    match flag(args, "--file") {
        Some(f) => confine(root, &f),
        None if root.exists() => confine(root, default),
        None => Ok(root.join(default)),
    }
}

/// `help`, `--help`, `describe` and `man`: they need no root. `None` when `cmd` is another command.
fn meta_command(cmd: &str, rest: &[String]) -> Option<ExitCode> {
    if matches!(cmd, "help" | "--help" | "-h") {
        return Some(match rest.first().filter(|a| !a.starts_with('-')) {
            Some(c) => match commands::help_one(c) {
                Some(t) => {
                    print!("{t}");
                    ExitCode::SUCCESS
                }
                None => fail(&format!("no command named {c}; try: help")),
            },
            None => {
                print!("{}", commands::help_all());
                ExitCode::SUCCESS
            }
        });
    }
    if rest.iter().any(|a| a == "--help" || a == "-h") {
        if let Some(t) = commands::help_one(cmd) {
            print!("{t}");
            return Some(ExitCode::SUCCESS);
        }
    }
    if cmd == "describe" {
        print!(
            "{}",
            if rest.iter().any(|a| a == "--markdown") {
                commands::describe_markdown()
            } else {
                commands::describe_json()
            }
        );
        return Some(ExitCode::SUCCESS);
    }
    if cmd == "man" {
        print!("{}", commands::man_page());
        return Some(ExitCode::SUCCESS);
    }
    None
}

/// The value of `--name`, or an empty string.
fn arg(rest: &[String], n: &str) -> String {
    flag(rest, n).unwrap_or_default()
}

pub fn run(args: Vec<String>, default_root: &str) -> ExitCode {
    let Some(cmd) = args.first().cloned() else {
        eprint!("{}", commands::help_all());
        return ExitCode::FAILURE;
    };
    let rest = &args[1..];
    if let Some(code) = meta_command(&cmd, rest) {
        return code;
    }
    let root = PathBuf::from(flag(rest, "--root").unwrap_or_else(|| default_root.into()));
    match cmd.as_str() {
        "changes-add" => cmd_changes_add(rest, &root),
        "changes" => cmd_changes(rest, &root),
        "versions" => cmd_versions(rest, &root),
        "terms" => cmd_terms(rest, &root),
        "terms-add" => cmd_terms_add(rest, &root),
        "draws" => cmd_draws(rest, &root),
        "draw-add" => cmd_draw_add(rest, &root),
        "apply" => match positionals(rest).first() {
            Some(f) => match confine(&root, f) {
                Ok(target) => fence::apply(&target.to_string_lossy(), &root, rest),
                Err(e) => fail(&e),
            },
            None => fail(&commands::usage()),
        },
        "catalog" => cmd_catalog(rest, &root),
        "registry" => ExitCode::from(registry::run(rest, &root) as u8),
        "lineage-report" | "language-report" => cmd_report(&cmd, rest, &root),
        "find" => cmd_find(rest, &root),
        "shared-hash" => {
            for (n, h) in shared_hashes() {
                println!("{h}  {n}");
            }
            ExitCode::SUCCESS
        }
        _ => fail(&commands::usage()),
    }
}

/// Print "appended to <path>" for an append that worked, or the error.
fn appended(path: &Path, res: std::result::Result<(), String>) -> ExitCode {
    match res {
        Ok(()) => {
            println!("appended to {}", path.display());
            ExitCode::SUCCESS
        }
        Err(e) => fail(&e),
    }
}

fn cmd_changes_add(rest: &[String], root: &Path) -> ExitCode {
    let g = |n: &str| arg(rest, n);
    let row = changes::ChangeRow {
        instrument: g("--instrument"),
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
        &row.instrument,
        &row.version,
        &row.baseline,
        &row.section,
        &row.tag,
        &row.decision_ref,
    ]
    .iter()
    .any(|v| v.is_empty())
    {
        return fail("changes-add requires --instrument --version --baseline --section --old --new --tag --decision");
    }
    let f = match write_file(rest, root, "precedent/changes.csv") {
        Ok(f) => f,
        Err(e) => return fail(&e),
    };
    appended(&f, changes::append_change(&f, &row))
}

fn cmd_changes(rest: &[String], root: &Path) -> ExitCode {
    match changes::read_changes(&file(rest, root, "precedent/changes.csv")) {
        Ok(rows) => {
            for r in rows.iter().filter(|r| {
                flag(rest, "--instrument")
                    .map(|d| r.instrument == d)
                    .unwrap_or(true)
                    && flag(rest, "--version")
                        .map(|v| r.version == v)
                        .unwrap_or(true)
            }) {
                println!(
                    "{} {}  {}  [{}]  {}  ->  {}",
                    r.instrument, r.version, r.section, r.tag, r.baseline_text, r.new_text
                );
            }
            ExitCode::SUCCESS
        }
        Err(e) => fail(&e),
    }
}

fn cmd_versions(rest: &[String], root: &Path) -> ExitCode {
    let out = match flag(rest, "--out") {
        Some(o) => confine(root, &o),
        None => write_file(rest, root, "precedent/versions.csv"),
    };
    let out = match out {
        Ok(o) => o,
        Err(e) => return fail(&e),
    };
    let scan_dir = flag(rest, "--dir").unwrap_or_else(|| "outputs".into());
    if let Err(e) = confine(root, &scan_dir) {
        return fail(&e);
    }
    if !root.join(&scan_dir).is_dir() {
        return fail(&format!(
            "{scan_dir}: not a folder under the root; the index was not changed"
        ));
    }
    let rows = versions::scan_versions(root, &scan_dir);
    match versions::write_versions(&rows, &out) {
        Ok(()) => {
            println!("wrote {} version rows to {}", rows.len(), out.display());
            ExitCode::SUCCESS
        }
        Err(e) => fail(&format!("{}: {e}", out.display())),
    }
}

fn cmd_terms(rest: &[String], root: &Path) -> ExitCode {
    match terms::read_terms(&file(rest, root, "precedent/terms.csv")) {
        Ok(rows) => {
            for r in terms::select(
                &rows,
                flag(rest, "--instrument").as_deref(),
                flag(rest, "--term").as_deref(),
            ) {
                println!(
                    "{} {}  {}  [{}]{}",
                    r.instrument,
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
    }
}

fn cmd_terms_add(rest: &[String], root: &Path) -> ExitCode {
    let g = |n: &str| arg(rest, n);
    let row = terms::TermRow {
        instrument: g("--instrument"),
        version: g("--version"),
        term: g("--term"),
        replaces: g("--replaces"),
        status: flag(rest, "--status").unwrap_or_else(|| "live".into()),
        date: g("--date"),
        context: g("--context"),
    };
    let f = match write_file(rest, root, "precedent/terms.csv") {
        Ok(f) => f,
        Err(e) => return fail(&e),
    };
    appended(&f, terms::append_term(&f, &row))
}

fn cmd_draws(rest: &[String], root: &Path) -> ExitCode {
    match draws::read_draws(&file(rest, root, "precedent/draws.csv")) {
        Ok(rows) => {
            for r in draws::select(&rows, flag(rest, "--instrument").as_deref()) {
                println!(
                    "{} {} drew from {} {}  {}{}",
                    r.instrument,
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
    }
}

fn cmd_draw_add(rest: &[String], root: &Path) -> ExitCode {
    let g = |n: &str| arg(rest, n);
    let row = draws::DrawRow {
        instrument: g("--instrument"),
        version: g("--version"),
        drew_from: g("--from"),
        drew_from_version: g("--from-version"),
        date: g("--date"),
        note: g("--note"),
    };
    let f = match write_file(rest, root, "precedent/draws.csv") {
        Ok(f) => f,
        Err(e) => return fail(&e),
    };
    appended(&f, draws::append_draw(&f, &row))
}

fn cmd_catalog(rest: &[String], root: &Path) -> ExitCode {
    let dir = flag(rest, "--dir").unwrap_or_else(|| catalog::DEFAULT_DIR.into());
    match tree::Root::new(root).and_then(|r| catalog::scan(&r, &dir)) {
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

fn cmd_report(cmd: &str, rest: &[String], root: &Path) -> ExitCode {
    let dirs = flag(rest, "--dir").unwrap_or_else(|| "service-wip".into());
    let ini = flag(rest, "--initials");
    let names = rest.iter().any(|a| a == "--names");
    let out = tree::Root::new(root).and_then(|r| {
        if cmd == "lineage-report" {
            survey::lineage_report(&r, &dirs, ini, names)
        } else {
            let mw = flag(rest, "--min-words")
                .and_then(|v| v.parse().ok())
                .unwrap_or(12);
            let nr = flag(rest, "--near")
                .and_then(|v| v.parse().ok())
                .unwrap_or(0.5);
            survey::language_report(&r, &dirs, ini, mw, nr, names)
        }
    });
    match out {
        Ok(text) => {
            print!("{text}");
            ExitCode::SUCCESS
        }
        Err(e) => fail(&e.to_string()),
    }
}

fn cmd_find(rest: &[String], root: &Path) -> ExitCode {
    let has = |f: &str| rest.iter().any(|a| a == f);
    let q = if let Some(t) = flag(rest, "--term") {
        survey::Query::Term(t)
    } else if let Some(e) = flag(rest, "--entity") {
        survey::Query::Entity(e)
    } else if let Some(t) = flag(rest, "--text") {
        survey::Query::Text(t)
    } else if has("--parties") {
        survey::Query::Parties
    } else {
        survey::Query::Glossary
    };
    let o = survey::FindOpts {
        counts_only: has("--counts"),
        show: has("--show"),
        show_text: has("--show-text"),
        entities: flag(rest, "--entities"),
        parties_file: flag(rest, "--parties-file"),
        all_versions: has("--all-versions"),
        near: flag(rest, "--near")
            .and_then(|v| v.parse().ok())
            .unwrap_or(0.5),
        min_docs: flag(rest, "--min-docs")
            .and_then(|v| v.parse().ok())
            .unwrap_or(2),
        show_names: has("--names"),
    };
    let dirs = flag(rest, "--dir").unwrap_or_else(|| "service-wip".into());
    match tree::Root::new(root)
        .and_then(|r| survey::find_report(&r, &dirs, flag(rest, "--initials"), q, &o))
    {
        Ok(text) => {
            print!("{text}");
            ExitCode::SUCCESS
        }
        Err(e) => fail(&e.to_string()),
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
