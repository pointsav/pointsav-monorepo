// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.
//! Scan a folder: walk it with the shared `walk` of tool-precedent-core, read every `.md` page,
//! and count what was found. Read-only. The report holds counts, never page names or text.

use std::fmt;
use std::path::Path;

use tool_precedent_core::error::Result;
use tool_precedent_core::tree::Root;
use tool_precedent_core::walk::{walk, Opts};

use crate::frontmatter::FrontmatterState;
use crate::page::Page;

/// Counts from one scan.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Report {
    /// `.md` files found by the walk.
    pub files_found: usize,
    /// Files that could not be read as UTF-8 text.
    pub unreadable: usize,
    /// Files read (found minus unreadable).
    pub pages_read: usize,
    pub frontmatter_ok: usize,
    pub frontmatter_missing: usize,
    pub frontmatter_invalid: usize,
    /// Relative links whose target was looked for (not external, not `#anchor`, not site-absolute).
    pub links_checked: usize,
    pub links_broken: usize,
    /// Links not looked for: a scheme (`https:`, `mailto:`), a bare `#anchor`, or a `/site/path`.
    pub links_not_checked: usize,
    /// Folders and entries the walk skipped by rule.
    pub skipped: usize,
}

impl fmt::Display for Report {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "pages found:          {}", self.files_found)?;
        writeln!(f, "pages read:           {}", self.pages_read)?;
        writeln!(f, "unreadable:           {}", self.unreadable)?;
        writeln!(f, "frontmatter ok:       {}", self.frontmatter_ok)?;
        writeln!(f, "no frontmatter:       {}", self.frontmatter_missing)?;
        writeln!(f, "bad frontmatter:      {}", self.frontmatter_invalid)?;
        writeln!(f, "links checked:        {}", self.links_checked)?;
        writeln!(f, "broken links:         {}", self.links_broken)?;
        writeln!(f, "links not checked:    {}", self.links_not_checked)?;
        writeln!(f, "skipped by rule:      {}", self.skipped)
    }
}

enum Kind {
    NotChecked,
    Relative(String),
}

/// Sort a link target: external and site-absolute links and bare anchors are not looked for; a
/// relative target is returned without its `#fragment` or `?query`.
fn classify(url: &str) -> Kind {
    let url = url.trim();
    let has_scheme = url.split_once(':').is_some_and(|(s, _)| {
        !s.is_empty()
            && s.chars()
                .all(|c| c.is_ascii_alphanumeric() || "+.-".contains(c))
    });
    if url.is_empty() || url.starts_with('#') || url.starts_with('/') || has_scheme {
        return Kind::NotChecked;
    }
    let target = url.split(['#', '?']).next().unwrap_or("");
    if target.is_empty() {
        Kind::NotChecked
    } else {
        Kind::Relative(target.to_string())
    }
}

/// Resolve `target` against the page's own folder. `None` when it climbs above the root.
fn resolve(page_path: &str, target: &str) -> Option<String> {
    let mut parts: Vec<&str> = page_path.split('/').collect();
    parts.pop(); // the page's own file name
    for seg in target.split('/') {
        match seg {
            "" | "." => {}
            ".." => {
                parts.pop()?;
            }
            s => parts.push(s),
        }
    }
    if parts.is_empty() {
        None
    } else {
        Some(parts.join("/"))
    }
}

/// Scan `dir` (relative to `root`; `""` means the root). `keep` names hidden folders to enter.
pub fn scan(root: &Root, dir: &str, keep: &[&str]) -> Result<(Vec<Page>, Report)> {
    let walked = walk(
        root,
        dir,
        &Opts {
            exts: &["md"],
            keep,
        },
    )?;
    let mut report = Report {
        files_found: walked.files.len(),
        skipped: walked.skipped.len(),
        ..Report::default()
    };
    let mut pages = Vec::new();
    for rel in &walked.files {
        let Ok((text, _)) = root.read(rel) else {
            report.unreadable += 1;
            continue;
        };
        report.pages_read += 1;
        let page = Page::parse(rel, &text);
        match page.frontmatter {
            FrontmatterState::Ok(_) => report.frontmatter_ok += 1,
            FrontmatterState::Missing => report.frontmatter_missing += 1,
            FrontmatterState::Invalid(_) => report.frontmatter_invalid += 1,
        }
        for link in &page.links {
            match classify(&link.url) {
                Kind::NotChecked => report.links_not_checked += 1,
                Kind::Relative(target) => {
                    report.links_checked += 1;
                    let found = resolve(rel, &target)
                        .and_then(|r| root.resolve(&r).ok())
                        .is_some_and(|p| p.exists());
                    if !found {
                        report.links_broken += 1;
                    }
                }
            }
        }
        pages.push(page);
    }
    Ok((pages, report))
}

/// Scan a folder on disk: the folder itself is the root.
pub fn scan_dir(dir: &Path, keep: &[&str]) -> Result<(Vec<Page>, Report)> {
    let root = Root::new(dir)?;
    scan(&root, "", keep)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_links() {
        assert!(matches!(
            classify("https://example.org/x"),
            Kind::NotChecked
        ));
        assert!(matches!(classify("mailto:a@example.org"), Kind::NotChecked));
        assert!(matches!(classify("#top"), Kind::NotChecked));
        assert!(matches!(classify("/site/page"), Kind::NotChecked));
        assert!(matches!(classify("a.md#part"), Kind::Relative(t) if t == "a.md"));
    }

    #[test]
    fn resolves_against_the_page_folder() {
        assert_eq!(resolve("a/b.md", "c.md").as_deref(), Some("a/c.md"));
        assert_eq!(resolve("a/b.md", "../c.md").as_deref(), Some("c.md"));
        assert_eq!(resolve("a/b.md", "./d/e.md").as_deref(), Some("a/d/e.md"));
        assert_eq!(resolve("b.md", "../c.md"), None);
    }
}
