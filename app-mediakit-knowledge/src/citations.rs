// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! `citations.yaml` registry resolver (Phase 3.2 of `KNOWLEDGE-PLATFORM-PLAN.md`).
//!
//! Per `conventions/citation-substrate.md`, `citations.yaml` is the canonical
//! resolver every claim's `cites:` id set resolves against. The `[citations]`
//! path in `knowledge.toml` names the file; this module is the first thing in
//! this crate to actually load and parse it — before Phase 3 it was a config
//! field with nothing consuming it.

use std::collections::HashMap;
use std::path::Path;

use serde::Deserialize;

/// One `citations.yaml` entry.
#[derive(Debug, Clone, Deserialize)]
pub struct CitationEntry {
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(default)]
    pub jurisdiction: Option<String>,
    pub title: String,
    pub url: String,
    #[serde(default)]
    pub last_verified: Option<String>,
    #[serde(default)]
    pub content_hash: Option<String>,
    #[serde(default)]
    pub evidence_class: Option<String>,
    #[serde(default)]
    pub aliases: Vec<String>,
    // --- Bibliography fields (P5b Phase 2) — real registry entries already carry
    // these (see e.g. constitutional-ai-2212-08073); the JOURNAL References
    // generator (SPEC-journal-wiki-render-contract.md §1.4) is the first consumer.
    /// Author name(s) as written in the registry (free text, e.g. `["Bai et al."]`
    /// or `["Rose, Scott", "Borchert, Oliver"]`) — not structured per-author data.
    #[serde(default)]
    pub authors: Vec<String>,
    #[serde(default)]
    pub year: Option<i64>,
    #[serde(default)]
    pub venue: Option<String>,
    #[serde(default)]
    pub arxiv_id: Option<String>,
    #[serde(default)]
    pub doi: Option<String>,
}

impl CitationEntry {
    /// One References-list line (SPEC §1.4): authors, year, title, venue,
    /// then doi if present else arxiv_id. Fields absent from this entry are
    /// omitted, not rendered as empty placeholders.
    pub fn bibliography_line(&self) -> String {
        let mut parts = Vec::new();
        if !self.authors.is_empty() {
            parts.push(self.authors.join(", "));
        }
        if let Some(y) = self.year {
            parts.push(format!("({y})"));
        }
        parts.push(self.title.clone());
        if let Some(v) = self.venue.as_deref().filter(|v| !v.is_empty()) {
            parts.push(v.to_string());
        }
        if let Some(doi) = self.doi.as_deref().filter(|d| !d.is_empty()) {
            parts.push(format!("doi:{doi}"));
        } else if let Some(a) = self.arxiv_id.as_deref().filter(|a| !a.is_empty()) {
            parts.push(format!("arXiv:{a}"));
        }
        parts.join(". ")
    }
}

#[derive(Debug, Deserialize)]
struct RawRegistry {
    citations: HashMap<String, CitationEntry>,
}

/// The parsed registry — a plain lookup table, id → entry.
#[derive(Debug, Clone, Default)]
pub struct CitationRegistry {
    entries: HashMap<String, CitationEntry>,
}

impl CitationRegistry {
    /// Load and parse `citations.yaml` from `path`. Returns an empty registry
    /// (not an error) if the file is absent or malformed — a missing registry
    /// must never take the whole wiki down; callers that need to know whether
    /// resolution is actually possible check `is_empty()`.
    pub fn load(path: &Path) -> Self {
        let Ok(text) = std::fs::read_to_string(path) else {
            tracing::warn!(
                "citations.yaml not found at {}; citation resolution disabled",
                path.display()
            );
            return Self::default();
        };
        match serde_yaml::from_str::<RawRegistry>(&text) {
            Ok(raw) => Self {
                entries: raw.citations,
            },
            Err(e) => {
                tracing::warn!("citations.yaml at {} failed to parse: {e}", path.display());
                Self::default()
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn get(&self, id: &str) -> Option<&CitationEntry> {
        self.entries.get(id)
    }

    /// Resolve a set of citation ids to their URLs, skipping any id that
    /// doesn't resolve (an unresolvable citation is a linter concern — see
    /// convention §9 — not a reason to fail rendering).
    pub fn resolve_urls(&self, ids: &[String]) -> Vec<String> {
        ids.iter()
            .filter_map(|id| self.get(id))
            .map(|e| e.url.clone())
            .collect()
    }

    /// Every registered citation id — the re-verification scheduler's full
    /// candidate list is `all_ids() ∩ actually-cited-by-some-claim`, computed
    /// by the caller against `ClaimStore::all_cited_citation_ids`.
    pub fn all_ids(&self) -> impl Iterator<Item = &String> {
        self.entries.keys()
    }

    /// JOURNAL publish-gate check (SPEC-journal-wiki-render-contract.md §1.3):
    /// `in-text ids ⊆ frontmatter cites: ⊆ citations.yaml keys`. Returns every
    /// violation found — empty means the gate passes. This is an editorial
    /// checklist item (SPEC §7), not a render-blocking check: a paper with
    /// violations still renders (an unresolved in-text id still gets a
    /// visibly-flagged References entry — see `content::render::render_journal_doc`)
    /// so an editor can see exactly what's broken rather than a blank page.
    pub fn check_citation_gate(&self, in_text_ids: &[String], cites: &[String]) -> Vec<String> {
        let mut violations = Vec::new();
        for id in in_text_ids {
            if !cites.contains(id) {
                violations.push(format!(
                    "in-text citation [{id}] is not listed in frontmatter cites:"
                ));
            }
        }
        for id in cites {
            if self.get(id).is_none() {
                violations.push(format!(
                    "cites: id `{id}` does not resolve in citations.yaml"
                ));
            }
            if !in_text_ids.contains(id) {
                violations.push(format!(
                    "cites: id `{id}` is never used in-text (unused citation)"
                ));
            }
        }
        violations
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_and_resolves_real_registry_shape() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("citations.yaml");
        std::fs::write(
            &path,
            r#"
citations:
  ni-51-102:
    type: regulatory-instrument
    jurisdiction: ca-bcsc
    title: National Instrument 51-102
    url: https://example.com/ni-51-102
    last_verified: 2026-04-26
    evidence_class: regulatory-primary
    aliases:
      - "NI 51-102"
"#,
        )
        .unwrap();
        let registry = CitationRegistry::load(&path);
        assert!(!registry.is_empty());
        let entry = registry.get("ni-51-102").unwrap();
        assert_eq!(entry.url, "https://example.com/ni-51-102");
        assert_eq!(entry.aliases, vec!["NI 51-102".to_string()]);
    }

    #[test]
    fn missing_file_yields_empty_registry_not_panic() {
        let registry = CitationRegistry::load(Path::new("/nonexistent/citations.yaml"));
        assert!(registry.is_empty());
    }

    #[test]
    fn resolve_urls_skips_unknown_ids() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("citations.yaml");
        std::fs::write(
            &path,
            "citations:\n  known:\n    type: vendor-doc\n    title: Known\n    url: https://x/known\n",
        )
        .unwrap();
        let registry = CitationRegistry::load(&path);
        let urls = registry.resolve_urls(&["known".to_string(), "unknown".to_string()]);
        assert_eq!(urls, vec!["https://x/known".to_string()]);
    }

    #[test]
    fn malformed_yaml_yields_empty_registry_not_panic() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("citations.yaml");
        std::fs::write(&path, ":::not valid:::").unwrap();
        assert!(CitationRegistry::load(&path).is_empty());
    }

    #[test]
    fn loads_bibliography_fields_matching_real_registry_shape() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("citations.yaml");
        std::fs::write(
            &path,
            r#"
citations:
  constitutional-ai-2212-08073:
    type: research-paper
    title: "Constitutional AI: Harmlessness from AI Feedback"
    authors: ["Bai et al."]
    venue: arXiv
    arxiv_id: "2212.08073"
    url: https://arxiv.org/abs/2212.08073
    last_verified: 2026-04-26
    evidence_class: research-primary
"#,
        )
        .unwrap();
        let registry = CitationRegistry::load(&path);
        let entry = registry.get("constitutional-ai-2212-08073").unwrap();
        assert_eq!(entry.authors, vec!["Bai et al.".to_string()]);
        assert_eq!(entry.venue.as_deref(), Some("arXiv"));
        assert_eq!(entry.arxiv_id.as_deref(), Some("2212.08073"));
        assert_eq!(
            entry.bibliography_line(),
            "Bai et al.. Constitutional AI: Harmlessness from AI Feedback. arXiv. arXiv:2212.08073"
        );
    }

    #[test]
    fn bibliography_line_omits_absent_fields() {
        let entry = CitationEntry {
            kind: "vendor-doc".to_string(),
            jurisdiction: None,
            title: "Bare Title".to_string(),
            url: "https://x".to_string(),
            last_verified: None,
            content_hash: None,
            evidence_class: None,
            aliases: vec![],
            authors: vec![],
            year: None,
            venue: None,
            arxiv_id: None,
            doi: None,
        };
        assert_eq!(entry.bibliography_line(), "Bare Title");
    }

    #[test]
    fn bibliography_line_prefers_doi_over_arxiv_id() {
        let entry = CitationEntry {
            kind: "research-paper".to_string(),
            jurisdiction: None,
            title: "T".to_string(),
            url: "https://x".to_string(),
            last_verified: None,
            content_hash: None,
            evidence_class: None,
            aliases: vec![],
            authors: vec!["A. Author".to_string()],
            year: Some(2020),
            venue: Some("Venue".to_string()),
            arxiv_id: Some("1234.5678".to_string()),
            doi: Some("10.1/x".to_string()),
        };
        assert_eq!(
            entry.bibliography_line(),
            "A. Author. (2020). T. Venue. doi:10.1/x"
        );
    }

    #[test]
    fn citation_gate_passes_when_sets_agree() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("citations.yaml");
        std::fs::write(
            &path,
            "citations:\n  a:\n    type: vendor-doc\n    title: A\n    url: https://x/a\n  b:\n    type: vendor-doc\n    title: B\n    url: https://x/b\n",
        )
        .unwrap();
        let registry = CitationRegistry::load(&path);
        let ids = vec!["a".to_string(), "b".to_string()];
        assert!(registry.check_citation_gate(&ids, &ids).is_empty());
    }

    #[test]
    fn citation_gate_flags_unresolved_unused_and_missing_from_cites() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("citations.yaml");
        std::fs::write(
            &path,
            "citations:\n  a:\n    type: vendor-doc\n    title: A\n    url: https://x/a\n",
        )
        .unwrap();
        let registry = CitationRegistry::load(&path);
        // In-text cites "a" and "orphan" (not in citations.yaml at all);
        // frontmatter cites: lists "a" and "unused" (never cited in-text).
        let in_text = vec!["a".to_string(), "orphan".to_string()];
        let cites = vec!["a".to_string(), "unused".to_string()];
        let violations = registry.check_citation_gate(&in_text, &cites);
        assert!(violations.iter().any(|v| v.contains("[orphan]") && v.contains("not listed in frontmatter cites")));
        assert!(violations.iter().any(|v| v.contains("`unused`") && v.contains("never used in-text")));
        assert!(violations.iter().any(|v| v.contains("`unused`") && v.contains("does not resolve")));
    }
}
