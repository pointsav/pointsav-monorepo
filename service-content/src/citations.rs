// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Citation registry resolver — Phase 2 (P2-2.4) of
//! learning-loop-master-plan-2026-05-18.md.
//!
//! Loads `/srv/foundry/citations.yaml` at startup, indexes the registry
//! by canonical ID + aliases + URL, and exposes a fuzzy-lookup endpoint
//! so editorial Task sessions can resolve `[external: <url>]` and
//! aliased mentions ("NI 51-102", "Wikipedia article on …") into
//! canonical citation IDs without hallucinating.
//!
//! Hot-reload is best-effort: a `mtime` poll on the file path lets a
//! background task re-read when the YAML changes (operator edit). The
//! Doctrine claim #34 (citation substrate) and convention
//! `citation-substrate.md` are the authoritative specs.

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::RwLock;
use std::time::SystemTime;

/// Default location of the citation registry — overridable via env
/// `FOUNDRY_CITATIONS_PATH` so tests and alternative deployments can point
/// elsewhere.
const DEFAULT_CITATIONS_PATH: &str = "/srv/foundry/citations.yaml";

/// One entry from `citations.yaml` — the fields we care about for
/// resolution. The full schema has more (last_verified, content_hash,
/// jurisdiction); we only project the lookup-relevant subset and pass
/// through the rest as an opaque blob if needed by callers.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CitationEntry {
    /// Canonical ID — e.g. `ni-51-102`, `wikipedia-rust-programming-language`.
    /// This is the key in the YAML map; the resolver populates it after
    /// deserialisation.
    #[serde(skip)]
    pub id: String,
    /// Type tag — `regulatory-instrument` | `research-paper` | etc.
    #[serde(rename = "type", default)]
    pub kind: String,
    /// Authoritative title.
    #[serde(default)]
    pub title: String,
    /// Stable URL.
    #[serde(default)]
    pub url: String,
    /// Evidence class per Knowledge Provenance Pillar.
    #[serde(default)]
    pub evidence_class: String,
    /// Alternate forms that may appear in prose. The resolver builds a
    /// lookup index over `id`, `title`, `url`, and all `aliases`.
    #[serde(default)]
    pub aliases: Vec<String>,
}

/// Top-level YAML wire shape: a single `citations:` map. Other top-level
/// keys (`schema:`, `last_updated:`, etc.) are ignored.
#[derive(Debug, Deserialize)]
struct RegistryWire {
    #[serde(default)]
    citations: HashMap<String, CitationEntry>,
}

/// In-memory registry. The lookup index is a `HashMap<lowercase_key, id>`
/// where `key` covers id / title / url / aliases. Resolution is O(1)
/// expected; the registry is small (~hundred entries at most) so memory
/// is negligible.
pub struct CitationRegistry {
    path: PathBuf,
    inner: RwLock<RegistryInner>,
}

struct RegistryInner {
    entries: HashMap<String, CitationEntry>,
    /// Lowercased lookup keys → canonical ID.
    index: HashMap<String, String>,
    /// File mtime at last successful load. `reload_if_changed` polls this.
    last_loaded_at: Option<SystemTime>,
}

impl CitationRegistry {
    /// Open the registry at `path`. Returns an empty registry (no error)
    /// when the file doesn't exist — many deployment environments may
    /// not have citations.yaml seeded yet, and the resolver should
    /// degrade to "no matches" rather than failing service startup.
    pub fn open(path: impl Into<PathBuf>) -> Result<Self> {
        let path = path.into();
        let inner = if path.exists() {
            load_from_path(&path).with_context(|| {
                format!("failed to load citation registry at {}", path.display())
            })?
        } else {
            RegistryInner {
                entries: HashMap::new(),
                index: HashMap::new(),
                last_loaded_at: None,
            }
        };
        Ok(Self {
            path,
            inner: RwLock::new(inner),
        })
    }

    /// Open from `FOUNDRY_CITATIONS_PATH` env (falling back to
    /// [`DEFAULT_CITATIONS_PATH`]). The standard production entrypoint.
    pub fn from_env() -> Result<Self> {
        let path = std::env::var_os("FOUNDRY_CITATIONS_PATH")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from(DEFAULT_CITATIONS_PATH));
        Self::open(path)
    }

    /// Count registered citations.
    pub fn len(&self) -> usize {
        self.inner.read().expect("citations registry poisoned").entries.len()
    }

    /// Convenience: returns true when the registry is empty (no file, or
    /// empty `citations:` map).
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Resolve a query string to a canonical entry. Match precedence:
    /// 1. Exact case-insensitive match against id / title / url / alias.
    /// 2. URL prefix match (rare — registry URLs are full canonical URLs).
    /// Returns `None` when nothing matches; the caller decides whether
    /// to surface as a 404 or fall back to `[external: <url>]`.
    pub fn resolve(&self, query: &str) -> Option<CitationEntry> {
        let guard = self.inner.read().expect("citations registry poisoned");
        let q = query.trim().to_lowercase();
        if q.is_empty() {
            return None;
        }
        if let Some(id) = guard.index.get(&q) {
            return guard.entries.get(id).cloned();
        }
        // Secondary: URL prefix match — handles cases where the caller
        // passes a URL with an additional path fragment.
        for (key, id) in guard.index.iter() {
            if key.starts_with("http") && q.starts_with(key) {
                return guard.entries.get(id).cloned();
            }
        }
        None
    }

    /// Best-effort reload: if the file's mtime changed since the last
    /// successful load, re-read and swap the in-memory state. Errors
    /// during reload are non-fatal — the previous index stays in
    /// service and the error is returned for logging.
    pub fn reload_if_changed(&self) -> Result<bool> {
        let mtime = std::fs::metadata(&self.path)
            .and_then(|m| m.modified())
            .ok();
        {
            let guard = self.inner.read().expect("citations registry poisoned");
            if guard.last_loaded_at == mtime {
                return Ok(false);
            }
        }
        let fresh = load_from_path(&self.path).with_context(|| {
            format!("hot reload failed for {}", self.path.display())
        })?;
        let mut guard = self.inner.write().expect("citations registry poisoned");
        *guard = fresh;
        Ok(true)
    }

    /// Path to the underlying YAML file (diagnostic surface).
    pub fn path(&self) -> &Path {
        &self.path
    }
}

fn load_from_path(path: &Path) -> Result<RegistryInner> {
    let body = std::fs::read_to_string(path)
        .with_context(|| format!("read {}", path.display()))?;

    // Strip top-level YAML frontmatter if present. The citations.yaml
    // file uses `---` frontmatter delimiters around an initial header,
    // then a `citations:` map below. serde_yaml can't deserialise a
    // multi-document YAML into a single struct, so we slice off the
    // frontmatter manually.
    let body_without_fm = strip_yaml_frontmatter(&body);
    let wire: RegistryWire = serde_yaml::from_str(body_without_fm)
        .with_context(|| format!("parse {}", path.display()))?;

    let mut index = HashMap::new();
    let mut entries = HashMap::new();
    for (id, mut entry) in wire.citations.into_iter() {
        entry.id = id.clone();
        index.insert(id.to_lowercase(), id.clone());
        if !entry.title.is_empty() {
            index.insert(entry.title.to_lowercase(), id.clone());
        }
        if !entry.url.is_empty() {
            index.insert(entry.url.to_lowercase(), id.clone());
        }
        for alias in entry.aliases.iter() {
            index.insert(alias.to_lowercase(), id.clone());
        }
        entries.insert(id, entry);
    }
    let mtime = std::fs::metadata(path).and_then(|m| m.modified()).ok();
    Ok(RegistryInner {
        entries,
        index,
        last_loaded_at: mtime,
    })
}

/// Strip a leading `---\n...\n---\n` frontmatter block, if present.
/// citations.yaml begins with such a block; the body below is the
/// `citations:` map that serde_yaml consumes.
fn strip_yaml_frontmatter(body: &str) -> &str {
    let trimmed_start = body.trim_start();
    if !trimmed_start.starts_with("---") {
        return body;
    }
    // Skip the leading delimiter line (first `---\n`) and look for the
    // closing one.
    let after_first = match trimmed_start.find('\n') {
        Some(n) => &trimmed_start[n + 1..],
        None => return body,
    };
    // Find a `\n---\n` (the closing delimiter) — start searching from
    // index 0 since the leading `---\n` has been stripped.
    if let Some(end) = after_first.find("\n---") {
        let after_close_marker = &after_first[end + 4..];
        let after_close_marker = after_close_marker
            .strip_prefix('\n')
            .unwrap_or(after_close_marker);
        return after_close_marker;
    }
    body
}

/// HTTP wire shape for `GET /v1/citations/resolve?q=<query>`.
#[derive(Debug, Serialize)]
pub struct ResolveResponse {
    pub matched: bool,
    pub entry: Option<CitationEntry>,
    /// Total registered citations — useful for callers wanting to
    /// detect "registry not loaded" vs "no match for this query".
    pub registry_size: usize,
}

impl CitationRegistry {
    /// Build the HTTP response body for a `?q=<query>` lookup.
    pub fn resolve_response(&self, query: &str) -> ResolveResponse {
        let entry = self.resolve(query);
        ResolveResponse {
            matched: entry.is_some(),
            entry,
            registry_size: self.len(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write_fixture(body: &str) -> PathBuf {
        let path = std::env::temp_dir().join(format!(
            "citations-fixture-{}.yaml",
            chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0)
        ));
        std::fs::write(&path, body).unwrap();
        path
    }

    #[test]
    fn empty_when_file_missing() {
        let r = CitationRegistry::open("/does/not/exist/citations.yaml").unwrap();
        assert!(r.is_empty());
        assert!(r.resolve("anything").is_none());
    }

    #[test]
    fn loads_minimal_registry() {
        let body = "\
---
schema: foundry-citations-v1
---

citations:
  ni-51-102:
    type: regulatory-instrument
    title: National Instrument 51-102 — Continuous Disclosure Obligations
    url: https://example.com/ni-51-102
    aliases:
      - \"NI 51-102\"
      - \"51-102 Continuous Disclosure Obligations\"
  rfc-7519:
    type: open-standard
    title: \"RFC 7519: JSON Web Token (JWT)\"
    url: https://www.rfc-editor.org/rfc/rfc7519
    aliases:
      - JWT
";
        let path = write_fixture(body);
        let r = CitationRegistry::open(&path).unwrap();
        assert_eq!(r.len(), 2);
        // Resolves by id.
        let e = r.resolve("ni-51-102").unwrap();
        assert_eq!(e.id, "ni-51-102");
        // Resolves by alias.
        let e = r.resolve("NI 51-102").unwrap();
        assert_eq!(e.id, "ni-51-102");
        // Resolves by URL.
        let e = r.resolve("https://www.rfc-editor.org/rfc/rfc7519").unwrap();
        assert_eq!(e.id, "rfc-7519");
        // Resolves by lowercased alias.
        let e = r.resolve("jwt").unwrap();
        assert_eq!(e.id, "rfc-7519");
        // No match → None.
        assert!(r.resolve("unknown-id").is_none());
        std::fs::remove_file(&path).ok();
    }

    #[test]
    fn hot_reload_picks_up_changes() {
        let body_v1 = "citations:\n  alpha:\n    title: First\n    aliases: [a]\n";
        let path = write_fixture(body_v1);
        let r = CitationRegistry::open(&path).unwrap();
        assert!(r.resolve("alpha").is_some());
        assert!(r.resolve("beta").is_none());
        // Sleep briefly so mtime changes — file systems track mtime at
        // 1-second resolution on many setups.
        std::thread::sleep(std::time::Duration::from_millis(1100));
        let body_v2 = "citations:\n  alpha:\n    title: First\n  beta:\n    title: Second\n";
        std::fs::write(&path, body_v2).unwrap();
        let reloaded = r.reload_if_changed().unwrap();
        assert!(reloaded);
        assert!(r.resolve("beta").is_some());
        std::fs::remove_file(&path).ok();
    }

    #[test]
    fn resolve_response_shape() {
        let body = "citations:\n  alpha:\n    title: Alpha\n";
        let path = write_fixture(body);
        let r = CitationRegistry::open(&path).unwrap();
        let resp = r.resolve_response("alpha");
        assert!(resp.matched);
        assert_eq!(resp.registry_size, 1);
        let miss = r.resolve_response("does-not-exist");
        assert!(!miss.matched);
        std::fs::remove_file(&path).ok();
    }
}
