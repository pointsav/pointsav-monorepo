// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.
//! Walk every file under a directory inside a [`Root`]. Zero dependencies, read-only.
//!
//! The one shared walk for tool-precedent-* readers: pages, code, shards. It skips hidden
//! folders (except names the caller keeps, such as `.archive`), build folders (`target`,
//! `node_modules`) and vendored code (`vendor-*`). It never follows a symlink. It uses an explicit
//! stack with a depth limit, so there is no recursion and no unbounded loop. What it skipped is
//! returned beside the files, so a caller can count it and nothing disappears silently.

use crate::error::Result;
use crate::tree::Root;
use std::fs;
use std::path::Path;

/// Directories deeper than this are not entered; they are listed as skipped.
pub const MAX_DEPTH: usize = 64;

const SKIP_NAMES: [&str; 2] = ["target", "node_modules"];

/// What to walk.
#[derive(Debug, Clone, Copy, Default)]
pub struct Opts<'a> {
    /// Keep only files with one of these extensions (without the dot). Empty keeps every file.
    pub exts: &'a [&'a str],
    /// Hidden folders to enter anyway, for example `.archive`.
    pub keep: &'a [&'a str],
}

/// The result of a walk: root-relative paths with `/` separators, sorted.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Walked {
    pub files: Vec<String>,
    /// Folders skipped by rule (with a trailing `/`), symlinks, over-deep folders and entries that
    /// could not be read.
    pub skipped: Vec<String>,
}

fn skip_dir(name: &str, keep: &[&str]) -> bool {
    if keep.contains(&name) {
        return false;
    }
    name.starts_with('.') || name.starts_with("vendor-") || SKIP_NAMES.contains(&name)
}

fn wanted(name: &str, exts: &[&str]) -> bool {
    exts.is_empty()
        || Path::new(name)
            .extension()
            .and_then(|e| e.to_str())
            .is_some_and(|e| exts.contains(&e))
}

/// Walk `dir` (a path relative to the root; `""` or `"."` means the root itself).
pub fn walk(root: &Root, dir: &str, opts: &Opts) -> Result<Walked> {
    let start = if dir.is_empty() || dir == "." {
        root.base().to_path_buf()
    } else {
        root.resolve(dir)?
    };
    let base = root.base().to_path_buf();
    let rel = |p: &Path| -> String {
        p.strip_prefix(&base)
            .unwrap_or(p)
            .components()
            .map(|c| c.as_os_str().to_string_lossy().into_owned())
            .collect::<Vec<_>>()
            .join("/")
    };
    let mut out = Walked::default();
    let mut stack = vec![(start, 0usize)];
    while let Some((d, depth)) = stack.pop() {
        let entries = match fs::read_dir(&d) {
            Ok(e) => e,
            Err(_) => {
                out.skipped.push(format!("{}/", rel(&d)));
                continue;
            }
        };
        for e in entries {
            let Ok(e) = e else {
                out.skipped.push(format!("{}/?", rel(&d)));
                continue;
            };
            let path = e.path();
            let Some(name) = e.file_name().to_str().map(str::to_string) else {
                out.skipped.push(rel(&path));
                continue;
            };
            let Ok(kind) = e.file_type() else {
                out.skipped.push(rel(&path));
                continue;
            };
            if kind.is_symlink() {
                out.skipped.push(rel(&path));
            } else if kind.is_dir() {
                if skip_dir(&name, opts.keep) || depth + 1 > MAX_DEPTH {
                    out.skipped.push(format!("{}/", rel(&path)));
                } else {
                    stack.push((path, depth + 1));
                }
            } else if kind.is_file() && wanted(&name, opts.exts) {
                out.files.push(rel(&path));
            }
        }
    }
    out.files.sort();
    out.skipped.sort();
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::CoreError;
    use std::sync::atomic::{AtomicUsize, Ordering};

    fn scratch() -> std::path::PathBuf {
        static N: AtomicUsize = AtomicUsize::new(0);
        let d = std::env::temp_dir().join(format!(
            "tpc-walk-{}-{}",
            std::process::id(),
            N.fetch_add(1, Ordering::SeqCst)
        ));
        fs::create_dir_all(&d).unwrap();
        d
    }

    fn put(d: &Path, rel: &str) {
        let p = d.join(rel);
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::write(p, "x").unwrap();
    }

    fn tree(d: &Path) {
        for f in [
            "a.md",
            "sub/b.md",
            "sub/deeper/h.rs",
            ".hidden/c.md",
            ".archive/d.md",
            "target/e.md",
            "vendor-lib/f.md",
            "node_modules/g.md",
        ] {
            put(d, f);
        }
    }

    #[test]
    fn skips_hidden_build_and_vendored_and_reports_them() {
        let d = scratch();
        tree(&d);
        let r = Root::new(&d).unwrap();
        let w = walk(&r, "", &Opts::default()).unwrap();
        assert_eq!(w.files, ["a.md", "sub/b.md", "sub/deeper/h.rs"]);
        assert_eq!(
            w.skipped,
            [
                ".archive/",
                ".hidden/",
                "node_modules/",
                "target/",
                "vendor-lib/"
            ]
        );
        let _ = fs::remove_dir_all(d);
    }

    #[test]
    fn extension_filter_and_kept_hidden_folder() {
        let d = scratch();
        tree(&d);
        let r = Root::new(&d).unwrap();
        let o = Opts {
            exts: &["md"],
            keep: &[".archive"],
        };
        let w = walk(&r, ".", &o).unwrap();
        assert_eq!(w.files, [".archive/d.md", "a.md", "sub/b.md"]);
        assert!(!w.skipped.contains(&".archive/".to_string()));
        let _ = fs::remove_dir_all(d);
    }

    #[test]
    fn walks_a_subfolder_with_root_relative_paths() {
        let d = scratch();
        tree(&d);
        let r = Root::new(&d).unwrap();
        let w = walk(&r, "sub", &Opts::default()).unwrap();
        assert_eq!(w.files, ["sub/b.md", "sub/deeper/h.rs"]);
        let _ = fs::remove_dir_all(d);
    }

    #[test]
    fn refuses_to_leave_the_root() {
        let d = scratch();
        let r = Root::new(&d).unwrap();
        assert!(matches!(
            walk(&r, "../x", &Opts::default()),
            Err(CoreError::OutsideRoot(_))
        ));
        let _ = fs::remove_dir_all(d);
    }

    #[cfg(unix)]
    #[test]
    fn never_follows_a_symlink() {
        let d = scratch();
        let outside = scratch();
        put(&outside, "secret.md");
        put(&d, "a.md");
        std::os::unix::fs::symlink(&outside, d.join("link")).unwrap();
        let r = Root::new(&d).unwrap();
        let w = walk(&r, "", &Opts::default()).unwrap();
        assert_eq!(w.files, ["a.md"]);
        assert_eq!(w.skipped, ["link"]);
        let _ = fs::remove_dir_all(d);
        let _ = fs::remove_dir_all(outside);
    }

    #[test]
    fn stops_at_the_depth_limit_and_says_so() {
        let d = scratch();
        let mut deep = String::new();
        for _ in 0..(MAX_DEPTH + 2) {
            deep.push_str("d/");
        }
        put(&d, &format!("{deep}x.md"));
        put(&d, "top.md");
        let r = Root::new(&d).unwrap();
        let w = walk(&r, "", &Opts::default()).unwrap();
        assert_eq!(w.files, ["top.md"]);
        assert_eq!(w.skipped.len(), 1);
        let _ = fs::remove_dir_all(d);
    }
}
