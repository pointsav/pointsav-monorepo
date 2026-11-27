// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! `rules` gate: zero third-party crates and the code rules that grep can check.

use crate::util::{crate_names, family_rust_files, read, walk, Res, Verdict};
use std::path::{Path, PathBuf};

/// (token, reason). Lines starting with `//` are ignored.
const BANNED: [(&str, &str); 7] = [
    (
        "macro_rules!",
        "macro_rules outside a const table generator",
    ),
    ("SystemTime", "wall clock read (determinism)"),
    ("Instant::now", "clock read (determinism)"),
    ("std::env::", "environment read (determinism)"),
    ("HashMap", "unordered map (determinism)"),
    ("HashSet", "unordered set (determinism)"),
    ("unsafe ", "unsafe code"),
];

/// Manifests whose dependencies must all be family path dependencies.
fn manifests(root: &Path) -> Res<Vec<PathBuf>> {
    let mut out = vec![root.join("Cargo.toml"), root.join("xtask/Cargo.toml")];
    for name in crate_names(root)? {
        out.push(root.join("crates").join(name).join("Cargo.toml"));
    }
    Ok(out)
}

/// Dependency lines in one manifest that are not `moonshot-folio-* = { path = ... }`.
fn bad_dependencies(text: &str) -> Vec<String> {
    let mut in_deps = false;
    let mut bad = Vec::new();
    for line in text.lines().map(str::trim) {
        if line.starts_with('[') {
            in_deps = line.ends_with("dependencies]");
        } else if in_deps && !line.is_empty() && !line.starts_with('#') {
            let family = line.starts_with("moonshot-folio");
            if !(family && line.contains("path =")) {
                bad.push(line.to_string());
            }
        }
    }
    bad
}

/// Problems in the manifests and the lockfile.
fn manifest_problems(root: &Path) -> Res<Vec<String>> {
    let mut problems = Vec::new();
    for manifest in manifests(root)? {
        let text = read(&manifest)?;
        for line in bad_dependencies(&text) {
            problems.push(format!(
                "{}: non-family dependency: {line}",
                manifest.display()
            ));
        }
        if text.contains("proc-macro") {
            problems.push(format!("{}: proc-macro crate", manifest.display()));
        }
    }
    let lock = read(&root.join("Cargo.lock"))?;
    for line in lock.lines().filter(|l| l.starts_with("name = ")) {
        if !line.starts_with("name = \"moonshot-folio") {
            problems.push(format!("Cargo.lock: third-party package {line}"));
        }
    }
    Ok(problems)
}

/// Problems in source files and the presence of build scripts.
fn source_problems(root: &Path) -> Res<Vec<String>> {
    let mut problems = Vec::new();
    for file in walk(root)? {
        if file.file_name().is_some_and(|n| n == "build.rs")
            && !file.starts_with(root.join("xtask"))
        {
            problems.push(format!("{}: build script", file.display()));
        }
    }
    for file in family_rust_files(root)? {
        let text = read(&file)?;
        for (n, line) in text.lines().enumerate() {
            if line.trim_start().starts_with("//") {
                continue;
            }
            for (token, why) in BANNED {
                if line.contains(token) {
                    let at = n.saturating_add(1);
                    problems.push(format!("{}:{at}: {why}", file.display()));
                }
            }
        }
    }
    Ok(problems)
}

/// Run the gate.
pub(crate) fn run(root: &Path) -> Res<Verdict> {
    let mut problems = manifest_problems(root)?;
    problems.extend(source_problems(root)?);
    if problems.is_empty() {
        return Ok(Verdict::Pass(
            "no third-party crates; code rules hold".to_string(),
        ));
    }
    Ok(Verdict::Fail(problems))
}
