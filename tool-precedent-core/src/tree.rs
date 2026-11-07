// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.
//! Root-confined file tree: every path is relative to a root and cannot escape it (no absolute
//! paths, no `..`, no symlink that resolves outside). Writes are atomic (temp file in the same
//! directory, then rename) and optionally guarded by the stamp (mtime + length) from the read.

use crate::error::{CoreError, Result};
use std::fs;
use std::path::{Component, Path, PathBuf};
use std::time::SystemTime;

/// Identity of a file's content as of a read: modification time and length.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Stamp {
    pub mtime: SystemTime,
    pub len: u64,
}

#[derive(Debug, Clone)]
pub struct Root {
    base: PathBuf,
}

impl Root {
    /// `base` must exist; it is canonicalized once so later containment checks are exact.
    pub fn new(base: impl AsRef<Path>) -> Result<Self> {
        Ok(Self {
            base: fs::canonicalize(base.as_ref())?,
        })
    }

    pub fn base(&self) -> &Path {
        &self.base
    }

    /// Resolve `rel` to an absolute path inside the root. The file need not exist, but its
    /// deepest existing ancestor must resolve (through symlinks) to somewhere inside the root.
    pub fn resolve(&self, rel: &str) -> Result<PathBuf> {
        let out = || CoreError::OutsideRoot(rel.to_string());
        let mut parts = Vec::new();
        for c in Path::new(rel).components() {
            match c {
                Component::Normal(p) => parts.push(p.to_os_string()),
                Component::CurDir => {}
                _ => return Err(out()),
            }
        }
        if parts.is_empty() {
            return Err(out());
        }
        let full = parts.iter().fold(self.base.clone(), |p, c| p.join(c));
        let mut existing = full.as_path();
        while !existing.exists() {
            existing = existing.parent().ok_or_else(out)?;
        }
        if !fs::canonicalize(existing)?.starts_with(&self.base) {
            return Err(out());
        }
        Ok(full)
    }

    pub fn stamp(&self, rel: &str) -> Result<Stamp> {
        let m = fs::metadata(self.resolve(rel)?)?;
        Ok(Stamp {
            mtime: m.modified()?,
            len: m.len(),
        })
    }

    /// Read a UTF-8 file and the stamp it had when read.
    pub fn read(&self, rel: &str) -> Result<(String, Stamp)> {
        let p = self.resolve(rel)?;
        let stamp = self.stamp(rel)?;
        Ok((fs::read_to_string(p)?, stamp))
    }

    /// Atomically replace (or create) `rel`. If `expected` is given, the file must still carry
    /// that stamp, otherwise `Stale`. Missing parent directories are created inside the root.
    pub fn write_atomic(&self, rel: &str, content: &str, expected: Option<Stamp>) -> Result<Stamp> {
        let p = self.resolve(rel)?;
        if let Some(want) = expected {
            if self.stamp(rel).ok() != Some(want) {
                return Err(CoreError::Stale(rel.to_string()));
            }
        }
        let dir = p
            .parent()
            .ok_or_else(|| CoreError::OutsideRoot(rel.to_string()))?;
        fs::create_dir_all(dir)?;
        if !fs::canonicalize(dir)?.starts_with(&self.base) {
            return Err(CoreError::OutsideRoot(rel.to_string()));
        }
        let name = p
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        let tmp = dir.join(format!(".{name}.tmp.{}", std::process::id()));
        let res = fs::write(&tmp, content).and_then(|_| fs::rename(&tmp, &p));
        if res.is_err() {
            let _ = fs::remove_file(&tmp);
        }
        res?;
        self.stamp(rel)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    fn scratch() -> PathBuf {
        static N: AtomicUsize = AtomicUsize::new(0);
        let d = std::env::temp_dir().join(format!(
            "tpc-tree-{}-{}",
            std::process::id(),
            N.fetch_add(1, Ordering::SeqCst)
        ));
        fs::create_dir_all(&d).unwrap();
        d
    }

    #[test]
    fn rejects_escapes() {
        let d = scratch();
        let r = Root::new(&d).unwrap();
        for bad in ["", "..", "../x", "a/../../x", "/etc/passwd"] {
            assert!(
                matches!(r.resolve(bad), Err(CoreError::OutsideRoot(_))),
                "{bad}"
            );
        }
        assert!(r.resolve("a/b.txt").is_ok());
        let _ = fs::remove_dir_all(d);
    }

    #[cfg(unix)]
    #[test]
    fn rejects_symlink_escape() {
        let d = scratch();
        let outside = scratch();
        std::os::unix::fs::symlink(&outside, d.join("link")).unwrap();
        let r = Root::new(&d).unwrap();
        assert!(matches!(
            r.resolve("link/x.txt"),
            Err(CoreError::OutsideRoot(_))
        ));
        assert!(matches!(
            r.write_atomic("link/x.txt", "no", None),
            Err(CoreError::OutsideRoot(_))
        ));
        assert!(!outside.join("x.txt").exists());
        let _ = fs::remove_dir_all(d);
        let _ = fs::remove_dir_all(outside);
    }

    #[test]
    fn atomic_write_read_and_stale_guard() {
        let d = scratch();
        let r = Root::new(&d).unwrap();
        let s1 = r.write_atomic("sub/a.txt", "one", None).unwrap();
        let (text, got) = r.read("sub/a.txt").unwrap();
        assert_eq!((text.as_str(), got), ("one", s1));
        let s2 = r.write_atomic("sub/a.txt", "twelve", Some(s1)).unwrap();
        assert_eq!(r.read("sub/a.txt").unwrap().0, "twelve");
        // writing against the old stamp is refused and leaves the file alone
        assert_eq!(
            r.write_atomic("sub/a.txt", "x", Some(s1)),
            Err(CoreError::Stale("sub/a.txt".into()))
        );
        assert_eq!(r.read("sub/a.txt").unwrap().0, "twelve");
        assert!(r.write_atomic("sub/a.txt", "ok", Some(s2)).is_ok());
        // no temp files left behind
        assert_eq!(fs::read_dir(d.join("sub")).unwrap().count(), 1);
        let _ = fs::remove_dir_all(d);
    }
}
