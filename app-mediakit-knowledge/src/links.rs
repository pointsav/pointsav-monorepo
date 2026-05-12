//! Phase 4 Steps 4.4+4.5 — redb-backed wikilink graph + blake3 content hashes.
//!
//! Two tables in a single redb database at `<state_dir>/links.redb`:
//!
//! - `OUTLINKS`: composite key `"from_slug\x00to_slug"` → u8 sentinel.
//!   Supports two query patterns:
//!   - Outlinks for a slug: prefix scan `"slug\x00" ..`.
//!   - Backlinks to a slug: full scan, filter by `"\x00target"` suffix.
//!
//! - `HASHES`: composite key `"slug\x00revision_sha"` → 32-byte blake3 digest.
//!   Federation-seam baseline (Phase 7 lights up an efficient reverse index).

use redb::{Database, ReadableDatabase, ReadableTable, TableDefinition};
use regex::Regex;
use std::{path::Path, sync::Arc};
use crate::error::WikiError;

const OUTLINKS: TableDefinition<&str, u8> = TableDefinition::new("outlinks");
const HASHES: TableDefinition<&str, &[u8]> = TableDefinition::new("hashes");

pub struct LinkGraph {
    db: Arc<Database>,
}

impl LinkGraph {
    /// Open an existing database or create a new one at `path`.
    pub fn open_or_create(path: &Path) -> Result<Self, WikiError> {
        let db = if path.exists() {
            Database::open(path)
        } else {
            Database::create(path)
        }
        .map_err(|e| WikiError::LinkGraph(e.to_string()))?;

        // Ensure both tables exist on first open.
        let wtx = db.begin_write().map_err(|e| WikiError::LinkGraph(e.to_string()))?;
        let _ = wtx.open_table(OUTLINKS).map_err(|e| WikiError::LinkGraph(e.to_string()))?;
        let _ = wtx.open_table(HASHES).map_err(|e| WikiError::LinkGraph(e.to_string()))?;
        wtx.commit().map_err(|e| WikiError::LinkGraph(e.to_string()))?;

        Ok(Self { db: Arc::new(db) })
    }

    /// Create a temporary database for use in tests. Each call returns a
    /// distinct database so parallel tests do not conflict.
    pub fn for_testing() -> Arc<Self> {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let n = COUNTER.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir()
            .join(format!("wiki-links-{}-{}.redb", std::process::id(), n));
        Arc::new(Self::open_or_create(&path).expect("link graph test init failed"))
    }

    /// Rebuild all wikilink edges for `slug`. Deletes existing outlinks, then
    /// inserts one row per `[[target]]` found in `body`.
    pub fn rebuild_for_slug(&self, slug: &str, body: &str) -> Result<(), WikiError> {
        let targets = parse_wikilinks(body);
        let prefix = format!("{}\x00", slug);

        let wtx = self.db.begin_write().map_err(|e| WikiError::LinkGraph(e.to_string()))?;
        {
            let mut table = wtx.open_table(OUTLINKS).map_err(|e| WikiError::LinkGraph(e.to_string()))?;

            // Collect existing keys for this slug (avoids borrow-while-mutate).
            let to_remove: Vec<String> = table
                .iter()
                .map_err(|e| WikiError::LinkGraph(e.to_string()))?
                .filter_map(|r| r.ok())
                .filter_map(|(k, _)| {
                    let key = k.value();
                    if key.starts_with(prefix.as_str()) {
                        Some(key.to_owned())
                    } else {
                        None
                    }
                })
                .collect();

            for key in &to_remove {
                table.remove(key.as_str()).map_err(|e| WikiError::LinkGraph(e.to_string()))?;
            }

            for target in &targets {
                let key = format!("{}\x00{}", slug, target);
                table.insert(key.as_str(), 0u8).map_err(|e| WikiError::LinkGraph(e.to_string()))?;
            }
        }
        wtx.commit().map_err(|e| WikiError::LinkGraph(e.to_string()))?;
        Ok(())
    }

    /// Returns the slugs of all articles that contain a wikilink to `target`.
    ///
    /// Full-scan O(n) — corpus is small. Phase 7 adds a reverse-index table.
    pub fn backlinks(&self, target: &str) -> Result<Vec<String>, WikiError> {
        let suffix = format!("\x00{}", target);

        let rtx = self.db.begin_read().map_err(|e| WikiError::LinkGraph(e.to_string()))?;
        let table = rtx.open_table(OUTLINKS).map_err(|e| WikiError::LinkGraph(e.to_string()))?;

        let results = table
            .iter()
            .map_err(|e| WikiError::LinkGraph(e.to_string()))?
            .filter_map(|r| r.ok())
            .filter_map(|(k, _)| {
                let key = k.value();
                if key.ends_with(suffix.as_str()) {
                    key.find('\x00').map(|pos| key[..pos].to_owned())
                } else {
                    None
                }
            })
            .collect();

        Ok(results)
    }

    /// Store the blake3 hash of `body` keyed by `(slug, revision_sha)`.
    pub fn record_hash(&self, slug: &str, revision_sha: &str, body: &[u8]) -> Result<(), WikiError> {
        let hash = blake3::hash(body);
        let hash_bytes: &[u8] = hash.as_bytes();
        let key = format!("{}\x00{}", slug, revision_sha);

        let wtx = self.db.begin_write().map_err(|e| WikiError::LinkGraph(e.to_string()))?;
        {
            let mut table = wtx.open_table(HASHES).map_err(|e| WikiError::LinkGraph(e.to_string()))?;
            table.insert(key.as_str(), hash_bytes).map_err(|e| WikiError::LinkGraph(e.to_string()))?;
        }
        wtx.commit().map_err(|e| WikiError::LinkGraph(e.to_string()))?;
        Ok(())
    }

    /// Look up a `(slug, revision_sha)` pair by its blake3 hash.
    ///
    /// Linear scan — Phase 7 lights up an efficient reverse index path.
    pub fn lookup_by_hash(&self, hash: &[u8; 32]) -> Result<Option<(String, String)>, WikiError> {
        let rtx = self.db.begin_read().map_err(|e| WikiError::LinkGraph(e.to_string()))?;
        let table = rtx.open_table(HASHES).map_err(|e| WikiError::LinkGraph(e.to_string()))?;

        let result = table
            .iter()
            .map_err(|e| WikiError::LinkGraph(e.to_string()))?
            .filter_map(|r| r.ok())
            .find_map(|(k, v)| {
                if v.value() == hash.as_slice() {
                    let key = k.value();
                    key.find('\x00')
                        .map(|pos| (key[..pos].to_owned(), key[pos + 1..].to_owned()))
                } else {
                    None
                }
            });

        Ok(result)
    }
}

/// Parse `[[target]]` wikilinks from Markdown body. Returns the slug form of
/// each target: lowercased, spaces replaced with hyphens, anchor/alias stripped.
fn parse_wikilinks(body: &str) -> Vec<String> {
    static RE: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();
    let re = RE.get_or_init(|| Regex::new(r"\[\[([^\]|#\[]+)").unwrap());
    re.captures_iter(body)
        .map(|cap| cap[1].trim().to_lowercase().replace(' ', "-"))
        .filter(|s| !s.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn graph() -> Arc<LinkGraph> {
        LinkGraph::for_testing()
    }

    #[test]
    fn parse_simple_wikilink() {
        let links = parse_wikilinks("See [[foo-bar]] and [[baz]].");
        assert_eq!(links, vec!["foo-bar", "baz"]);
    }

    #[test]
    fn parse_wikilink_with_alias() {
        let links = parse_wikilinks("See [[foo-bar|display text]].");
        assert_eq!(links, vec!["foo-bar"]);
    }

    #[test]
    fn parse_wikilink_with_anchor() {
        let links = parse_wikilinks("See [[foo-bar#section]].");
        assert_eq!(links, vec!["foo-bar"]);
    }

    #[test]
    fn backlinks_empty_when_no_links() {
        let g = graph();
        let bl = g.backlinks("target").unwrap();
        assert!(bl.is_empty());
    }

    #[test]
    fn backlinks_found_after_rebuild() {
        let g = graph();
        g.rebuild_for_slug("article-a", "See [[target]].").unwrap();
        let bl = g.backlinks("target").unwrap();
        assert_eq!(bl, vec!["article-a"]);
    }

    #[test]
    fn backlinks_cleared_after_rebuild_removes_link() {
        let g = graph();
        g.rebuild_for_slug("article-a", "See [[target]].").unwrap();
        g.rebuild_for_slug("article-a", "No links here.").unwrap();
        let bl = g.backlinks("target").unwrap();
        assert!(bl.is_empty());
    }

    #[test]
    fn hash_round_trip() {
        let g = graph();
        let body = b"Hello, world!";
        g.record_hash("my-slug", "abc123", body).unwrap();
        let expected = blake3::hash(body);
        let result = g.lookup_by_hash(expected.as_bytes()).unwrap();
        assert_eq!(result, Some(("my-slug".to_owned(), "abc123".to_owned())));
    }
}
