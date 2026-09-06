// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Git-backed article history (git2 0.20).
//!
//! The content mount is itself a git repository (canonical per Doctrine §IV.e), so
//! an article's revision history is the git log of its file. We walk HEAD newest-
//! first and keep the commits that touched the file (compared against the first
//! parent, i.e. `git log` default semantics). Read-only; opened per request.

use std::path::Path;

use git2::{DiffOptions, Repository, Sort};

/// One revision of an article.
pub struct Revision {
    pub sha: String, // full oid, for diff links
    pub short_sha: String,
    pub author: String,
    pub date_iso: String, // YYYY-MM-DD
    pub message: String,  // subject line only
    /// `true` if a `redactions.yaml` entry covers this revision (see
    /// `sitedata::Redaction`). The row still renders (date/sha, the fact a
    /// correction exists) — callers must never render `message` as-is for a
    /// redacted revision, and must never call `file_at_rev`/`file_diff` for
    /// its `sha`; render a generic redacted-notice instead. Content hiding
    /// happens at the call site (`is_redacted` gate before those two
    /// functions), not by blanking data here — this field is presentation
    /// guidance, not itself a security boundary.
    pub redacted: bool,
}

/// One line of a unified diff.
pub struct DiffLine {
    pub origin: char, // ' ' context, '+' add, '-' del, 'H' hunk header
    pub content: String,
}

/// The diff a single commit made to one file (vs its first parent).
pub struct FileDiff {
    pub short_sha: String,
    pub author: String,
    pub date_iso: String,
    pub message: String,
    pub lines: Vec<DiffLine>,
}

/// History of `rel` (path relative to `repo_root`), newest first, up to `limit`.
/// `redacted_through`, if present, is a `redactions.yaml` boundary sha for
/// this article (see `sitedata::Redaction`) — every revision at or before it
/// is marked `redacted: true`. Returns an empty vec on any git error (no
/// repo, detached, etc.).
pub fn file_history(
    repo_root: &Path,
    rel: &Path,
    limit: usize,
    redacted_through: Option<&str>,
) -> Vec<Revision> {
    let Ok(repo) = Repository::open(repo_root) else {
        return Vec::new();
    };
    let Ok(mut walk) = repo.revwalk() else {
        return Vec::new();
    };
    let _ = walk.set_sorting(Sort::TIME);
    if walk.push_head().is_err() {
        return Vec::new();
    }
    let boundary = redacted_through
        .and_then(|s| repo.revparse_single(s).ok())
        .map(|o| o.id());

    let mut out = Vec::new();
    for oid in walk.flatten() {
        if out.len() >= limit {
            break;
        }
        let Ok(commit) = repo.find_commit(oid) else {
            continue;
        };
        if !touches(&repo, &commit, rel) {
            continue;
        }
        let author = commit.author();
        let subject = commit.summary().unwrap_or("").to_string();
        let redacted =
            boundary.is_some_and(|b| oid == b || repo.graph_descendant_of(b, oid).unwrap_or(false));
        out.push(Revision {
            sha: oid.to_string(),
            short_sha: oid.to_string().chars().take(8).collect(),
            author: author.name().unwrap_or("unknown").to_string(),
            date_iso: iso_date(commit.time().seconds()),
            message: subject,
            redacted,
        });
    }
    out
}

/// `true` iff `sha` is at or before `redacted_through` — i.e. `sha` is
/// `redacted_through` itself, or an ancestor of it. Callers gate
/// `file_at_rev`/`file_diff` on this *before* reading any blob/diff content —
/// unlike `file_history`'s row-level `redacted` flag (presentation-only),
/// this function is the actual content-hiding boundary. `false` (not
/// redacted) on any git error or unresolvable sha — a boundary that fails to
/// resolve must never silently widen what's hidden or, more dangerously,
/// silently widen what's *shown*; see the call sites, which treat "can't
/// resolve the boundary" as "can't resolve the target either" and 404 either
/// way rather than exposing content past a broken redaction config.
pub fn is_redacted(repo_root: &Path, sha: &str, redacted_through: &str) -> bool {
    let Ok(repo) = Repository::open(repo_root) else {
        return false;
    };
    let Some(target) = repo.revparse_single(sha).ok().map(|o| o.id()) else {
        return false;
    };
    let Some(boundary) = repo.revparse_single(redacted_through).ok().map(|o| o.id()) else {
        return false;
    };
    target == boundary || repo.graph_descendant_of(boundary, target).unwrap_or(false)
}

/// `true` iff `s` is a plausible (possibly abbreviated) commit SHA — 7 to 40
/// hex digits, nothing else. The engine only ever *generates* full-oid links
/// (`Revision.sha`), so this is the entire legitimate input space; anything
/// else (a git revspec like `HEAD~3`, `main@{upstream}`, or the `:/<regex>`
/// commit-message-search syntax) is unintended surface for a public
/// `?rev=`/`?rev=` query param and is rejected before it ever reaches git2's
/// `revparse_single`, which accepts the full revspec grammar.
fn looks_like_sha(s: &str) -> bool {
    (7..=40).contains(&s.len()) && s.bytes().all(|b| b.is_ascii_hexdigit())
}

/// The diff commit `sha` made to `rel` (vs its first parent; whole file for a
/// root commit). Returns None on any git error, or if `sha` isn't a
/// plausible commit SHA.
pub fn file_diff(repo_root: &Path, rel: &Path, sha: &str) -> Option<FileDiff> {
    if !looks_like_sha(sha) {
        return None;
    }
    let repo = Repository::open(repo_root).ok()?;
    let commit = repo.revparse_single(sha).ok()?.peel_to_commit().ok()?;
    let tree = commit.tree().ok()?;
    let parent_tree = commit.parent(0).ok().and_then(|p| p.tree().ok());
    let mut opts = DiffOptions::new();
    opts.pathspec(rel);
    opts.context_lines(3);
    let diff = repo
        .diff_tree_to_tree(parent_tree.as_ref(), Some(&tree), Some(&mut opts))
        .ok()?;
    let mut lines = Vec::new();
    diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| {
        // Skip the 'F' file-header lines (diff --git / index / --- / +++).
        if line.origin() != 'F' {
            let content = String::from_utf8_lossy(line.content())
                .trim_end_matches('\n')
                .to_string();
            lines.push(DiffLine {
                origin: line.origin(),
                content,
            });
        }
        true
    })
    .ok()?;
    let author = commit.author();
    Some(FileDiff {
        short_sha: commit.id().to_string().chars().take(8).collect(),
        author: author.name().unwrap_or("unknown").to_string(),
        date_iso: iso_date(commit.time().seconds()),
        message: commit.summary().unwrap_or("").to_string(),
        lines,
    })
}

/// The content of `rel` as it stood at commit `sha`, plus that commit's date
/// (`YYYY-MM-DD`). Used by the point-in-time "as-of" article view. None on any
/// git error, if `sha` isn't a plausible commit SHA, or if the file did not
/// exist at that revision.
pub fn file_at_rev(repo_root: &Path, rel: &Path, sha: &str) -> Option<(String, String)> {
    if !looks_like_sha(sha) {
        return None;
    }
    let repo = Repository::open(repo_root).ok()?;
    let commit = repo.revparse_single(sha).ok()?.peel_to_commit().ok()?;
    let tree = commit.tree().ok()?;
    let entry = tree.get_path(rel).ok()?;
    let obj = entry.to_object(&repo).ok()?;
    let blob = obj.as_blob()?;
    let content = String::from_utf8_lossy(blob.content()).to_string();
    Some((content, iso_date(commit.time().seconds())))
}

/// Did `commit` change `rel` relative to its first parent (or, for a root commit,
/// is the file present)?
fn touches(repo: &Repository, commit: &git2::Commit, rel: &Path) -> bool {
    let Ok(tree) = commit.tree() else {
        return false;
    };
    let mut opts = DiffOptions::new();
    opts.pathspec(rel);
    if commit.parent_count() == 0 {
        return repo
            .diff_tree_to_tree(None, Some(&tree), Some(&mut opts))
            .map(|d| d.deltas().len() > 0)
            .unwrap_or(false);
    }
    let Ok(parent) = commit.parent(0) else {
        return false;
    };
    let Ok(ptree) = parent.tree() else {
        return false;
    };
    repo.diff_tree_to_tree(Some(&ptree), Some(&tree), Some(&mut opts))
        .map(|d| d.deltas().len() > 0)
        .unwrap_or(false)
}

/// Format a Unix timestamp (UTC) as `YYYY-MM-DD` without pulling a date crate.
/// Uses Howard Hinnant's civil-from-days algorithm.
fn iso_date(secs: i64) -> String {
    let days = secs.div_euclid(86_400);
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    format!("{:04}-{:02}-{:02}", y, m, d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn looks_like_sha_accepts_full_and_abbreviated_hex() {
        assert!(looks_like_sha("1234567")); // 7 chars — the floor
        assert!(looks_like_sha("0123456789abcdef0123456789abcdef01234567")); // 40 chars — a full oid
        assert!(looks_like_sha("deadbeef"));
    }

    #[test]
    fn looks_like_sha_rejects_git_revspec_syntax() {
        assert!(!looks_like_sha("HEAD~3"));
        assert!(!looks_like_sha("main@{upstream}"));
        assert!(!looks_like_sha(":/some pattern"));
        assert!(!looks_like_sha("HEAD"));
    }

    #[test]
    fn looks_like_sha_rejects_too_short_or_too_long() {
        assert!(!looks_like_sha("abc123")); // 6 chars, below the 7 floor
        assert!(!looks_like_sha(&"a".repeat(41))); // above the 40 ceiling
        assert!(!looks_like_sha(""));
    }

    #[test]
    fn looks_like_sha_rejects_non_hex_chars() {
        assert!(!looks_like_sha("123456g")); // 'g' is not hex
        assert!(!looks_like_sha("HEAD-abc"));
    }

    #[test]
    fn file_diff_and_file_at_rev_reject_non_sha_input_before_touching_git() {
        // repo_root doesn't even need to exist — rejection happens before
        // Repository::open, proving the guard runs first.
        let bogus_repo = Path::new("/nonexistent-repo-path-for-test");
        let rel = Path::new("file.md");
        assert!(file_diff(bogus_repo, rel, "HEAD~3").is_none());
        assert!(file_at_rev(bogus_repo, rel, ":/pattern").is_none());
    }
}
