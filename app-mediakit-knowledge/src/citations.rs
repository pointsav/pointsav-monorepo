//! Phase 2 Step 5 — citation registry loader and HTTP handler.
//!
//! Parses `~/Foundry/citations.yaml` (or the path supplied via `--citations-yaml`)
//! into a `Vec<CitationEntry>` and exposes the result via `GET /api/citations`.
//!
//! The YAML is read fresh on every request in Phase 2 for simplicity. Caching
//! (a single Arc<RwLock<Vec<CitationEntry>>> refreshed on a timed interval)
//! is a Phase 4 optimisation once the registry is confirmed stable.
//!
//! YAML structure observed in `~/Foundry/citations.yaml`:
//!   The file has a top-level `citations:` map whose values are records keyed
//!   by citation ID.  Example:
//!
//!   ```yaml
//!   citations:
//!     ni-51-102:
//!       type: regulatory-instrument
//!       title: "National Instrument 51-102 ..."
//!       url: "https://..."
//!       jurisdiction: ca-bcsc
//!       last_verified: 2026-04-26
//!       evidence_class: regulatory-primary
//!   ```
//!
//!   This module flattens each map entry into a `CitationEntry` by promoting
//!   the map key to the `id` field.

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;
use std::sync::Arc;

use crate::error::WikiError;
use crate::server::AppState;

/// A single entry from the citation registry.
///
/// Fields match the schema documented in `~/Foundry/citations.yaml`.  All
/// fields beyond `id` and `title` are optional so that missing or future
/// fields do not fail deserialization. The `extra` catch-all captures any
/// fields not explicitly listed — forward-compatible as the registry schema
/// evolves.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CitationEntry {
    /// Citation identifier — the YAML map key, e.g. `ni-51-102`.
    pub id: String,
    /// Authoritative title of the cited work.
    pub title: String,
    /// Stable URL for the cited work.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Publisher or issuing body.  Not always present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    /// Regulatory or standards jurisdiction (`ca-bcsc`, `eu`, `ietf`, …).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jurisdiction: Option<String>,
    /// Registry type string (`regulatory-instrument`, `research-paper`, …).
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub entry_type: Option<String>,
    /// Evidence classification (`regulatory-primary`, `research-primary`, …).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evidence_class: Option<String>,
    /// Date of most recent content-hash verification (ISO 8601 string).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_verified: Option<String>,
    /// Other known aliases for this citation — used by squiggle hygiene.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub aliases: Vec<String>,
    /// Forward-compatible catch-all for any field not listed above.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_yaml::Value>,
}

/// Intermediate deserialization target for the raw YAML structure.
///
/// The file has a single top-level key `citations:` whose value is a map
/// from citation-id strings to per-citation records.  The records themselves
/// do not include their own `id` field; that is the map key.
#[derive(Debug, Deserialize)]
struct CitationFile {
    citations: BTreeMap<String, RawCitationRecord>,
}

/// A single citation record as it appears under the `citations:` key — no
/// `id` field yet; the id is the map key.
#[derive(Debug, Deserialize)]
struct RawCitationRecord {
    #[serde(default)]
    title: String,
    #[serde(default)]
    url: Option<String>,
    #[serde(default)]
    publisher: Option<String>,
    #[serde(default)]
    jurisdiction: Option<String>,
    #[serde(rename = "type", default)]
    entry_type: Option<String>,
    #[serde(default)]
    evidence_class: Option<String>,
    #[serde(default)]
    last_verified: Option<serde_yaml::Value>,  // may be a date or a string
    #[serde(default)]
    aliases: Vec<String>,
    #[serde(flatten)]
    extra: BTreeMap<String, serde_yaml::Value>,
}

/// Load and parse a citation registry YAML file.
///
/// Returns a `Vec<CitationEntry>` sorted by ID for deterministic output.
/// On any I/O or parse error, returns `WikiError::CitationLoadFailed`.
pub async fn load_registry(path: &Path) -> Result<Vec<CitationEntry>, WikiError> {
    let raw = tokio::fs::read_to_string(path).await.map_err(|e| {
        WikiError::CitationLoadFailed(format!(
            "cannot read {}: {e}",
            path.display()
        ))
    })?;

    // The workspace `citations.yaml` opens with an optional YAML frontmatter
    // block (workspace metadata: schema, last_updated, maintainer, etc.)
    // before the actual `citations:` document. Strip it if present so the
    // parser sees only the citation records. Files without frontmatter pass
    // through untouched.
    let body: &str = if let Some(rest) = raw.strip_prefix("---\n") {
        if let Some(end) = rest.find("\n---\n") {
            &rest[end + "\n---\n".len()..]
        } else {
            raw.as_str()
        }
    } else {
        raw.as_str()
    };

    let file: CitationFile = serde_yaml::from_str(body).map_err(|e| {
        WikiError::CitationLoadFailed(format!(
            "cannot parse {}: {e}",
            path.display()
        ))
    })?;

    let mut entries: Vec<CitationEntry> = file
        .citations
        .into_iter()
        .map(|(id, rec)| {
            // Normalise `last_verified`: the YAML may emit a date node (not a
            // plain string) for YYYY-MM-DD values.  Serialize it back to a
            // string for the JSON response so the client sees a consistent type.
            let last_verified = rec.last_verified.map(|v| match &v {
                serde_yaml::Value::String(s) => s.clone(),
                other => serde_yaml::to_string(other)
                    .unwrap_or_default()
                    .trim()
                    .to_string(),
            });

            CitationEntry {
                id,
                title: rec.title,
                url: rec.url,
                publisher: rec.publisher,
                jurisdiction: rec.jurisdiction,
                entry_type: rec.entry_type,
                evidence_class: rec.evidence_class,
                last_verified,
                aliases: rec.aliases,
                extra: rec.extra,
            }
        })
        .collect();

    entries.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(entries)
}

/// HTTP handler for `GET /api/citations`.
///
/// Reads the registry fresh on every call in Phase 2.  If the file is absent
/// or unparseable, returns a 500 with the error message in the body (the
/// server started without one error stopping citation autocomplete; all other
/// surfaces continue working normally).
pub async fn get_citations(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<CitationEntry>>, WikiError> {
    let entries = load_registry(&state.citations_yaml).await?;
    Ok(Json(entries))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn parses_minimal_yaml() {
        let yaml = r#"
citations:
  test-id:
    type: regulatory-instrument
    title: "Test Entry"
    url: "https://example.com"
    jurisdiction: test-jurisdiction
    evidence_class: regulatory-primary
"#;
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("citations.yaml");
        tokio::fs::write(&path, yaml).await.unwrap();
        let entries = load_registry(&path).await.unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].id, "test-id");
        assert_eq!(entries[0].title, "Test Entry");
        assert_eq!(entries[0].url.as_deref(), Some("https://example.com"));
        assert_eq!(entries[0].jurisdiction.as_deref(), Some("test-jurisdiction"));
    }

    #[tokio::test]
    async fn returns_error_on_missing_file() {
        let result = load_registry(std::path::Path::new("/nonexistent/citations.yaml")).await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, WikiError::CitationLoadFailed(_)));
    }

    #[tokio::test]
    async fn entries_are_sorted_by_id() {
        let yaml = r#"
citations:
  zzz-last:
    title: "Last"
  aaa-first:
    title: "First"
  mmm-middle:
    title: "Middle"
"#;
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("citations.yaml");
        tokio::fs::write(&path, yaml).await.unwrap();
        let entries = load_registry(&path).await.unwrap();
        assert_eq!(entries[0].id, "aaa-first");
        assert_eq!(entries[1].id, "mmm-middle");
        assert_eq!(entries[2].id, "zzz-last");
    }
}
