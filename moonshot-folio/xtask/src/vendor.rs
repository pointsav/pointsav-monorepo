// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! `vendor-check` gate: each crate builds alone with only its family path dependencies.

use crate::util::{crate_names, read, walk, Res, Verdict};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Family crates (directory names) that `name` depends on, transitively, including itself.
fn closure(root: &Path, name: &str) -> Res<BTreeSet<String>> {
    let mut seen = BTreeSet::new();
    let mut stack = vec![name.to_string()];
    while let Some(n) = stack.pop() {
        if !seen.insert(n.clone()) {
            continue;
        }
        let text = read(&root.join("crates").join(&n).join("Cargo.toml"))?;
        for line in text.lines().filter(|l| l.contains("path = \"../")) {
            if let Some(dep) = line
                .split("path = \"../")
                .nth(1)
                .and_then(|r| r.split('"').next())
            {
                stack.push(dep.to_string());
            }
        }
    }
    Ok(seen)
}

/// Copy one crate directory into `dest/crates/<name>`.
fn copy_crate(root: &Path, dest: &Path, name: &str) -> Res<()> {
    let src = root.join("crates").join(name);
    for file in walk(&src)? {
        let rel = file.strip_prefix(&src).map_err(|e| e.to_string())?;
        let target = dest.join("crates").join(name).join(rel);
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        fs::copy(&file, &target).map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// Build one crate in an isolated directory; `Ok(None)` on success, `Ok(Some(msg))` on failure.
fn check_one(root: &Path, name: &str, tmp: &Path) -> Res<Option<String>> {
    let dir = tmp.join(name);
    let members = closure(root, name)?;
    for member in &members {
        copy_crate(root, &dir, member)?;
    }
    let list: Vec<String> = members.iter().map(|m| format!("\"crates/{m}\"")).collect();
    let manifest = format!(
        "[workspace]\nresolver = \"2\"\nmembers = [{}]\n",
        list.join(", ")
    );
    fs::write(dir.join("Cargo.toml"), manifest).map_err(|e| e.to_string())?;
    fs::copy(root.join("clippy.toml"), dir.join("clippy.toml")).map_err(|e| e.to_string())?;
    let out = Command::new("cargo")
        .args(["build", "--quiet", "-p"])
        .arg(format!("moonshot-folio-{name}"))
        .current_dir(&dir)
        .env("CARGO_TARGET_DIR", dir.join("target"))
        .output()
        .map_err(|e| e.to_string())?;
    if out.status.success() {
        return Ok(None);
    }
    Ok(Some(format!(
        "{name}: {}",
        String::from_utf8_lossy(&out.stderr)
    )))
}

/// Run the gate.
pub(crate) fn run(root: &Path) -> Res<Verdict> {
    let tmp: PathBuf = std::env::temp_dir().join(format!("folio-vendor-{}", std::process::id()));
    let mut failures = Vec::new();
    let names = crate_names(root)?;
    for name in &names {
        if let Some(msg) = check_one(root, name, &tmp)? {
            failures.push(msg);
        }
    }
    fs::remove_dir_all(&tmp).map_err(|e| e.to_string())?;
    if failures.is_empty() {
        return Ok(Verdict::Pass(format!(
            "{} crates build standalone",
            names.len()
        )));
    }
    Ok(Verdict::Fail(failures))
}
