// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Per-wiki data loaded from the content repo root at startup:
//! `categories.yaml` (the canonical category nav — id, display name, order) and
//! `redirects.yaml` (Hugo-style `from → to` 301s). Both are optional; missing or
//! malformed files degrade to empty, so the engine falls back to its config.

use std::collections::HashMap;
use std::path::Path;

use serde::Deserialize;

/// A category from `categories.yaml` — `id` is the dir/route, `name` is display.
/// `kind` is `"topic"` or `"guide"` — the sidebar's section-grouping field
/// (added 2026-08-04; see `naming-convention.md`'s categories.yaml decision
/// log). Defaults to `"topic"` when absent (older/other wikis' categories.yaml
/// files may not carry it yet) — never silently drops a category from the nav.
#[derive(Debug, Clone)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub order: i64,
}

#[derive(Deserialize)]
struct CategoriesFile {
    #[serde(default)]
    categories: Vec<CatEntry>,
}

#[derive(Deserialize)]
struct CatEntry {
    id: String,
    #[serde(default)]
    name: String,
    #[serde(default = "default_kind")]
    kind: String,
    #[serde(default)]
    order: i64,
}

fn default_kind() -> String {
    "topic".to_string()
}

/// Load `categories.yaml` from the mount root, sorted by `order`. Empty if absent.
pub fn load_categories(root: &Path) -> Vec<Category> {
    let Ok(text) = std::fs::read_to_string(root.join("categories.yaml")) else {
        return Vec::new();
    };
    let Ok(file) = serde_yaml::from_str::<CategoriesFile>(&text) else {
        return Vec::new();
    };
    let mut cats: Vec<Category> = file
        .categories
        .into_iter()
        .map(|c| Category {
            id: c.id,
            name: c.name,
            kind: c.kind,
            order: c.order,
        })
        .collect();
    cats.sort_by_key(|c| c.order);
    cats
}

#[derive(Deserialize)]
struct RedirectsFile {
    #[serde(default)]
    redirects: Vec<Redirect>,
}

#[derive(Deserialize)]
struct Redirect {
    from: String,
    to: String,
}

/// Load `redirects.yaml` from the mount root → `from → to` map. Empty if absent.
pub fn load_redirects(root: &Path) -> HashMap<String, String> {
    let Ok(text) = std::fs::read_to_string(root.join("redirects.yaml")) else {
        return HashMap::new();
    };
    match serde_yaml::from_str::<RedirectsFile>(&text) {
        Ok(file) => file.redirects.into_iter().map(|r| (r.from, r.to)).collect(),
        Err(_) => HashMap::new(),
    }
}

/// One `redactions.yaml` entry — the boundary and reason for hiding an
/// article's pre-correction revisions from `/history/{slug}` (2026-09-06
/// history-exposure decision, BRIEF-knowledge-ng-rewrite.md). Editor-
/// maintained, can be added or updated at any time (not tied to commit time —
/// the operator scenario this exists for is discovering a historical problem
/// *after* the correcting commit already landed).
#[derive(Debug, Clone)]
pub struct Redaction {
    /// Every revision at or before this commit (an ancestor of it, or itself)
    /// is hidden from `file_at_rev`/`file_diff` output. `file_history` still
    /// lists the row — the fact a correction happened stays visible, only the
    /// pre-fix content and diff are hidden — per convention/citation-substrate
    /// discipline: redaction hides content, never the fact of a correction.
    pub through: String,
    /// Editor-authored, shown in the "redacted" placeholder in place of the
    /// real commit message/diff. Never the real reason if that reason itself
    /// would leak the sensitive content — write a generic note instead.
    pub reason: Option<String>,
}

#[derive(Deserialize)]
struct RedactionsFile {
    #[serde(default)]
    redactions: HashMap<String, RedactionEntry>,
}

#[derive(Deserialize)]
struct RedactionEntry {
    through: String,
    #[serde(default)]
    reason: Option<String>,
}

/// Load `redactions.yaml` from the mount root → slug → `Redaction` map.
/// Empty if absent or malformed — a missing/broken file must never take the
/// whole wiki down, and (per the safe-default discipline this file already
/// carries) never fails open to "everything visible" vs. "everything hidden"
/// in a way that surprises an editor; absent just means no redactions apply.
pub fn load_redactions(root: &Path) -> HashMap<String, Redaction> {
    let Ok(text) = std::fs::read_to_string(root.join("redactions.yaml")) else {
        return HashMap::new();
    };
    match serde_yaml::from_str::<RedactionsFile>(&text) {
        Ok(file) => file
            .redactions
            .into_iter()
            .map(|(slug, e)| {
                (
                    slug,
                    Redaction {
                        through: e.through,
                        reason: e.reason,
                    },
                )
            })
            .collect(),
        Err(_) => HashMap::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn categories_sorted_by_order_and_absent_is_empty() {
        let dir = tempfile::tempdir().unwrap();
        // Deliberately out of order; loader must sort by `order`.
        std::fs::write(
            dir.path().join("categories.yaml"),
            "wiki: docs\ncategories:\n  - id: services\n    name: \"Platform Services\"\n    order: 5\n  - id: architecture\n    name: \"How It's Built\"\n    order: 1\n",
        )
        .unwrap();
        let cats = load_categories(dir.path());
        assert_eq!(cats.len(), 2);
        assert_eq!(cats[0].id, "architecture"); // order 1 first
        assert_eq!(cats[0].name, "How It's Built");
        assert_eq!(cats[1].id, "services");

        // Absent file → empty (graceful fallback).
        let empty = tempfile::tempdir().unwrap();
        assert!(load_categories(empty.path()).is_empty());
    }

    #[test]
    fn kind_defaults_to_topic_when_absent_and_reads_guide_when_present() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("categories.yaml"),
            "categories:\n  - id: architecture\n    name: \"How It's Built\"\n    order: 1\n  - id: how-to\n    name: \"How You Run It\"\n    kind: guide\n    order: 2\n",
        )
        .unwrap();
        let cats = load_categories(dir.path());
        assert_eq!(cats[0].kind, "topic"); // no kind: in the YAML — defaults
        assert_eq!(cats[1].kind, "guide");
    }

    #[test]
    fn redirects_map_from_to() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("redirects.yaml"),
            "redirects:\n  - from: /old-path\n    to: https://example.com/new\n",
        )
        .unwrap();
        let map = load_redirects(dir.path());
        assert_eq!(
            map.get("/old-path").map(String::as_str),
            Some("https://example.com/new")
        );
        assert!(load_redirects(tempfile::tempdir().unwrap().path()).is_empty());
    }

    #[test]
    fn redactions_load_by_slug_with_reason() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("redactions.yaml"),
            "redactions:\n  q3-figures:\n    through: abc1234\n    reason: \"Superseded by corrected figures.\"\n",
        )
        .unwrap();
        let map = load_redactions(dir.path());
        let r = map.get("q3-figures").unwrap();
        assert_eq!(r.through, "abc1234");
        assert_eq!(
            r.reason.as_deref(),
            Some("Superseded by corrected figures.")
        );
    }

    #[test]
    fn redactions_absent_file_is_empty() {
        assert!(load_redactions(tempfile::tempdir().unwrap().path()).is_empty());
    }

    #[test]
    fn redactions_malformed_yaml_is_empty_not_panic() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("redactions.yaml"), ":::not valid:::").unwrap();
        assert!(load_redactions(dir.path()).is_empty());
    }
}
