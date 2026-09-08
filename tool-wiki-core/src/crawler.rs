//! Walks a wiki content directory tree and parses every `.md` file it finds into a
//! [`Document`](crate::document::Document). Deliberately tolerant of non-article `.md`
//! files (`README.md`, `TRADEMARK.md`, `page-*.md` static pages) — a file with no
//! frontmatter is recorded as a skip, not a fatal crawl error, since a real content tree
//! always has a few of these mixed in (confirmed: `media-knowledge-*` all carry several).

use std::fs;
use std::path::{Path, PathBuf};

use crate::document::{Document, DocumentError};

#[derive(Debug)]
pub enum CrawlError {
    Io(PathBuf, std::io::Error),
}

impl std::fmt::Display for CrawlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CrawlError::Io(p, e) => write!(f, "{}: {e}", p.display()),
        }
    }
}

impl std::error::Error for CrawlError {}

#[derive(Debug, Default)]
pub struct CrawlResult {
    /// Every `.md` file that parsed successfully (had a valid frontmatter block).
    pub documents: Vec<Document>,
    /// Every `.md` file that failed to parse, with why — most commonly
    /// `DocumentError::Frontmatter(FrontmatterError::Missing)` for a non-article page,
    /// which callers filtering for real content should treat as an expected skip.
    pub skipped: Vec<(PathBuf, DocumentError)>,
}

/// Crawl every `.md` file under `root`, recursively. Paths recorded on each `Document`
/// (and in `skipped`) are relative to `root`, so the result is portable across machines.
pub fn crawl(root: &Path) -> Result<CrawlResult, CrawlError> {
    let mut result = CrawlResult::default();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let entries = fs::read_dir(&dir).map_err(|e| CrawlError::Io(dir.clone(), e))?;
        for entry in entries {
            let entry = entry.map_err(|e| CrawlError::Io(dir.clone(), e))?;
            let path = entry.path();
            let file_name = entry.file_name();
            let file_name = file_name.to_string_lossy();

            // Skip dotdirs (.git, .archive is a real exception below) and build/vendor noise.
            if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                if file_name.starts_with('.') && file_name != ".archive" {
                    continue;
                }
                stack.push(path);
                continue;
            }

            if !file_name.ends_with(".md") {
                continue;
            }

            let content = match fs::read_to_string(&path) {
                Ok(c) => c,
                Err(e) => return Err(CrawlError::Io(path, e)),
            };
            let rel_path = path.strip_prefix(root).unwrap_or(&path).to_path_buf();
            match Document::parse(rel_path.clone(), &content) {
                Ok(doc) => result.documents.push(doc),
                Err(err) => result.skipped.push((rel_path, err)),
            }
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn write(dir: &Path, rel: &str, content: &str) {
        let path = dir.join(rel);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(path, content).unwrap();
    }

    #[test]
    fn crawls_a_realistic_small_tree() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();

        write(
            root,
            "urban/_index.md",
            "---\ntitle: \"Demand and Demographics\"\nslug: urban\n---\n\nBody.\n",
        );
        write(
            root,
            "urban/commuter.md",
            "---\ntitle: \"Commuter\"\nslug: commuter\n---\n\n## Overview\n\nBody with [[urban-fringe]].\n",
        );
        write(root, "README.md", "# Not an article\n\nNo frontmatter.\n");
        write(root, ".git/config", "should never be walked into");

        let result = crawl(root).unwrap();
        assert_eq!(result.documents.len(), 2);
        assert_eq!(result.skipped.len(), 1);
        assert_eq!(result.skipped[0].0, PathBuf::from("README.md"));

        let slugs: Vec<&str> = result
            .documents
            .iter()
            .filter_map(|d| d.frontmatter.slug())
            .collect();
        assert!(slugs.contains(&"urban"));
        assert!(slugs.contains(&"commuter"));
    }

    #[test]
    fn skips_dotdirs_but_walks_archive() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        write(root, ".archive/old.md", "---\ntitle: \"Old\"\n---\n\nBody.\n");
        write(root, ".hidden/skip-me.md", "---\ntitle: \"Skip\"\n---\n\nBody.\n");

        let result = crawl(root).unwrap();
        let titles: Vec<&str> = result
            .documents
            .iter()
            .filter_map(|d| d.frontmatter.title())
            .collect();
        assert!(titles.contains(&"Old"));
        assert!(!titles.contains(&"Skip"));
    }

    #[test]
    fn nonexistent_root_is_a_reported_error_not_a_panic() {
        let err = crawl(Path::new("/does/not/exist/anywhere")).unwrap_err();
        assert!(matches!(err, CrawlError::Io(_, _)));
    }
}
