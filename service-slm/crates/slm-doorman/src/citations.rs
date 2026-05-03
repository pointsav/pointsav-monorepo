// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Best-effort citation resolver against `~/Foundry/citations.yaml`.
//!
//! Per `conventions/citation-substrate.md` and the apprenticeship
//! convention §3, a brief's `doctrine_citations` field carries citation
//! IDs that resolve here. The resolver is intentionally narrow — it
//! reads the registry's flat key-value shape with no full YAML parser
//! dependency, extracts each citation block's `title:` and `url:`
//! lines, and produces a small `ResolvedCitation` per ID. Unresolved
//! IDs come back with `title = None` and `url = None`; the caller
//! decides whether that's a hard error (it is not, in AS-2 — the
//! apprentice is told the ID and continues).
//!
//! Why not pull in serde_yaml: serde_yaml is unmaintained as of 2024,
//! the registry's schema is stable, and the read path is simple
//! enough to keep dependency surface small. If the registry ever
//! grows aliases we materially depend on, swap to a maintained
//! YAML crate.

use std::fs;
use std::path::Path;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResolvedCitation {
    pub id: String,
    pub title: Option<String>,
    pub url: Option<String>,
}

/// Resolve `ids` against the citations YAML at `path`. Reads the file
/// once and walks it line-by-line. Best-effort: a missing file or
/// unrecognised ID returns an unresolved entry rather than an error.
pub fn resolve(path: &Path, ids: &[String]) -> Vec<ResolvedCitation> {
    let body = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(_) => {
            // Registry missing — surface IDs only.
            return ids
                .iter()
                .map(|id| ResolvedCitation {
                    id: id.clone(),
                    title: None,
                    url: None,
                })
                .collect();
        }
    };

    ids.iter().map(|id| resolve_one(&body, id)).collect()
}

fn resolve_one(body: &str, id: &str) -> ResolvedCitation {
    let header = format!("  {id}:");
    let mut lines = body.lines();
    // Find the citation block by looking for "  <id>:".
    let mut found = false;
    for line in lines.by_ref() {
        if line == header {
            found = true;
            break;
        }
    }
    if !found {
        return ResolvedCitation {
            id: id.to_string(),
            title: None,
            url: None,
        };
    }

    let mut title = None;
    let mut url = None;
    for line in lines {
        // Block ends when indent drops back to 2 (next id) or 0
        // (top-level field) — anything not starting with at least four
        // spaces is the start of the next block / section.
        if !line.starts_with("    ") && !line.is_empty() {
            break;
        }
        let trimmed = line.trim_start();
        if let Some(rest) = trimmed.strip_prefix("title:") {
            title = Some(rest.trim().trim_matches(&['"', '\''][..]).to_string());
        } else if let Some(rest) = trimmed.strip_prefix("url:") {
            url = Some(rest.trim().trim_matches(&['"', '\''][..]).to_string());
        }
        if title.is_some() && url.is_some() {
            break;
        }
    }
    ResolvedCitation {
        id: id.to_string(),
        title,
        url,
    }
}

/// Render resolved citations as a markdown bullet list for inclusion in
/// the apprentice prompt. Each entry is `- id — title (url)`; missing
/// fields collapse gracefully.
pub fn render_for_prompt(citations: &[ResolvedCitation]) -> String {
    if citations.is_empty() {
        return "(no doctrine citations declared)".to_string();
    }
    let mut out = String::new();
    for c in citations {
        out.push_str("- ");
        out.push_str(&c.id);
        match (&c.title, &c.url) {
            (Some(t), Some(u)) => {
                out.push_str(" — ");
                out.push_str(t);
                out.push_str(" (");
                out.push_str(u);
                out.push(')');
            }
            (Some(t), None) => {
                out.push_str(" — ");
                out.push_str(t);
            }
            (None, Some(u)) => {
                out.push_str(" — ");
                out.push_str(u);
            }
            (None, None) => {
                out.push_str(" — (unresolved against citations.yaml)");
            }
        }
        out.push('\n');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn write_tmp(body: &str) -> std::path::PathBuf {
        let path = std::env::temp_dir().join(format!(
            "slm-doorman-citations-test-{}.yaml",
            chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0)
        ));
        let mut f = std::fs::File::create(&path).unwrap();
        f.write_all(body.as_bytes()).unwrap();
        path
    }

    #[test]
    fn resolves_known_id_with_title_and_url() {
        let path = write_tmp(
            "citations:\n  ni-51-102:\n    type: regulatory-instrument\n    \
             title: National Instrument 51-102 — Continuous Disclosure Obligations\n    \
             url: https://www.example.bcsc/51-102\n  other:\n    title: Other\n",
        );
        let r = resolve(&path, &["ni-51-102".to_string()]);
        assert_eq!(r.len(), 1);
        assert_eq!(r[0].id, "ni-51-102");
        assert_eq!(
            r[0].title.as_deref(),
            Some("National Instrument 51-102 — Continuous Disclosure Obligations")
        );
        assert_eq!(r[0].url.as_deref(), Some("https://www.example.bcsc/51-102"));
    }

    #[test]
    fn unresolved_id_returns_none_fields() {
        let path = write_tmp("citations:\n  ni-51-102:\n    title: t\n    url: u\n");
        let r = resolve(&path, &["nonsense-id".to_string()]);
        assert_eq!(r.len(), 1);
        assert_eq!(r[0].id, "nonsense-id");
        assert!(r[0].title.is_none());
        assert!(r[0].url.is_none());
    }

    #[test]
    fn missing_registry_surfaces_ids_only() {
        let r = resolve(
            std::path::Path::new("/nonexistent/path/citations.yaml"),
            &["foo".to_string(), "bar".to_string()],
        );
        assert_eq!(r.len(), 2);
        assert!(r[0].title.is_none() && r[0].url.is_none());
        assert!(r[1].title.is_none() && r[1].url.is_none());
    }

    // ---- Citations edge-case tests (PS.6 chunk #6 tail) ----

    /// A citation block that has `title` but no `url` resolves with
    /// `title = Some(...)` and `url = None` — the resolver must not
    /// require both fields to be present.
    #[test]
    fn partial_block_title_only_resolves_with_url_none() {
        let path = write_tmp(
            "citations:\n  title-only:\n    type: article\n    title: Just A Title\n  other:\n    title: Other\n    url: https://other\n",
        );
        let r = resolve(&path, &["title-only".to_string()]);
        assert_eq!(r.len(), 1);
        assert_eq!(r[0].title.as_deref(), Some("Just A Title"));
        assert!(
            r[0].url.is_none(),
            "url must be None for a block with only title"
        );
    }

    /// A citation block that has `url` but no `title` resolves with
    /// `url = Some(...)` and `title = None`.
    #[test]
    fn partial_block_url_only_resolves_with_title_none() {
        let path = write_tmp(
            "citations:\n  url-only:\n    type: article\n    url: https://example.com/spec\n  other:\n    title: Other\n",
        );
        let r = resolve(&path, &["url-only".to_string()]);
        assert_eq!(r.len(), 1);
        assert!(
            r[0].title.is_none(),
            "title must be None for a block with only url"
        );
        assert_eq!(r[0].url.as_deref(), Some("https://example.com/spec"));
    }

    /// Resolving an empty ID slice against any registry must return an
    /// empty `Vec` — no panic, no allocation for phantom entries.
    #[test]
    fn empty_ids_slice_returns_empty_vec() {
        let path = write_tmp("citations:\n  foo:\n    title: Foo\n    url: https://foo\n");
        let r = resolve(&path, &[]);
        assert!(r.is_empty(), "empty IDs must yield empty result vec");

        // Also confirm the missing-file path handles the empty slice.
        let r2 = resolve(std::path::Path::new("/nonexistent/citations.yaml"), &[]);
        assert!(r2.is_empty());
    }

    /// When the same ID appears multiple times in the registry file, the
    /// resolver must return the first match — later duplicates are ignored.
    #[test]
    fn duplicate_id_in_registry_resolves_first_occurrence() {
        // Two blocks with the same ID; the first has title "First" and the
        // second has title "Second".
        let yaml = "citations:\n  dupe-id:\n    title: First\n    url: https://first\n  dupe-id:\n    title: Second\n    url: https://second\n";
        let path = write_tmp(yaml);
        let r = resolve(&path, &["dupe-id".to_string()]);
        assert_eq!(r.len(), 1);
        assert_eq!(
            r[0].title.as_deref(),
            Some("First"),
            "first occurrence of a duplicate ID should win"
        );
    }

    #[test]
    fn renders_resolved_and_unresolved_entries() {
        let cs = vec![
            ResolvedCitation {
                id: "x".into(),
                title: Some("X paper".into()),
                url: Some("https://x".into()),
            },
            ResolvedCitation {
                id: "y".into(),
                title: None,
                url: None,
            },
        ];
        let s = render_for_prompt(&cs);
        assert!(s.contains("- x — X paper (https://x)"));
        assert!(s.contains("- y — (unresolved against citations.yaml)"));
    }
}
