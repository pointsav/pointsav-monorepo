// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Shared helpers for the gates.

use std::fs;
use std::path::{Path, PathBuf};

/// Result type used by every gate.
pub(crate) type Res<T> = Result<T, String>;

/// Outcome of one gate.
pub(crate) enum Verdict {
    /// The gate ran and found nothing wrong.
    Pass(String),
    /// The gate has nothing to check yet.
    Na(String),
    /// The gate found problems.
    Fail(Vec<String>),
}

/// Read a text file, naming the path on error.
pub(crate) fn read(path: &Path) -> Res<String> {
    fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))
}

/// Read a binary file, naming the path on error.
pub(crate) fn read_bytes(path: &Path) -> Res<Vec<u8>> {
    fs::read(path).map_err(|e| format!("{}: {e}", path.display()))
}

/// Family root: the parent of this crate's manifest directory.
pub(crate) fn family_root() -> Res<PathBuf> {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map(Path::to_path_buf)
        .ok_or_else(|| "manifest dir has no parent".to_string())
}

/// All files under `dir` (sorted), skipping `target`. A missing directory yields nothing.
pub(crate) fn walk(dir: &Path) -> Res<Vec<PathBuf>> {
    let mut out = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        let Ok(entries) = fs::read_dir(&d) else {
            continue;
        };
        for entry in entries {
            let path = entry.map_err(|e| e.to_string())?.path();
            if path.is_dir() {
                if path.file_name().is_some_and(|n| n != "target") {
                    stack.push(path);
                }
            } else {
                out.push(path);
            }
        }
    }
    out.sort();
    Ok(out)
}

/// Names of the directories directly under `root/crates`, sorted.
pub(crate) fn crate_names(root: &Path) -> Res<Vec<String>> {
    let mut names = Vec::new();
    let entries = fs::read_dir(root.join("crates")).map_err(|e| e.to_string())?;
    for entry in entries {
        let path = entry.map_err(|e| e.to_string())?.path();
        if let (true, Some(n)) = (path.is_dir(), path.file_name().and_then(|n| n.to_str())) {
            names.push(n.to_string());
        }
    }
    names.sort();
    Ok(names)
}

/// Source and test files (`.rs`) of the family, excluding the xtask itself.
pub(crate) fn family_rust_files(root: &Path) -> Res<Vec<PathBuf>> {
    let mut files = walk(&root.join("crates"))?;
    files.extend(walk(&root.join("src"))?);
    files.retain(|p| p.extension().is_some_and(|e| e == "rs"));
    Ok(files)
}
