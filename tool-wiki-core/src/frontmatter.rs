//! Frontmatter split + typed access for the real, heterogeneous field set used across
//! wiki article types (TOPIC, GUIDE, JOURNAL, `_index` MOC pages). Fields vary by type
//! (e.g. `index_type`/`index_scope` exist only on `_index.md`; `paper_class` only on
//! JOURNAL), so the schema is a map, not a fixed struct — with typed accessors for the
//! handful of fields every article carries.

use std::collections::BTreeMap;

use serde_yaml::Value;

/// A parsed `---`-delimited YAML frontmatter block, plus the raw body text that followed it.
#[derive(Debug, Clone, Default)]
pub struct Frontmatter {
    fields: BTreeMap<String, Value>,
}

#[derive(Debug)]
pub enum FrontmatterError {
    /// The file has no `---`-delimited block at all (not necessarily an error for every
    /// caller — some non-article files legitimately lack frontmatter).
    Missing,
    /// The block exists but isn't valid YAML.
    Invalid(serde_yaml::Error),
}

impl std::fmt::Display for FrontmatterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FrontmatterError::Missing => write!(f, "no frontmatter block found"),
            FrontmatterError::Invalid(e) => write!(f, "invalid frontmatter YAML: {e}"),
        }
    }
}

impl std::error::Error for FrontmatterError {}

impl Frontmatter {
    /// Split `content` into (frontmatter, body). `content` is expected to start with a
    /// `---` line, followed by YAML, followed by a closing `---` line — the shape every
    /// real article in the corpus uses (confirmed: zero exceptions found across three
    /// wikis during the 2026-09 title-drift sweep).
    pub fn parse(content: &str) -> Result<(Frontmatter, &str), FrontmatterError> {
        let content = content.strip_prefix('\u{feff}').unwrap_or(content); // tolerate a BOM
        let rest = content.strip_prefix("---\n").or_else(|| content.strip_prefix("---\r\n"));
        let Some(rest) = rest else {
            return Err(FrontmatterError::Missing);
        };
        let end = rest
            .find("\n---\n")
            .or_else(|| rest.find("\n---\r\n"))
            .ok_or(FrontmatterError::Missing)?;
        let yaml_str = &rest[..end];
        let after_marker = &rest[end + 1..]; // skip the leading '\n' before "---"
        let body_start = after_marker
            .find('\n')
            .map(|i| i + 1)
            .unwrap_or(after_marker.len());
        let body = &after_marker[body_start..];

        let value: Value = serde_yaml::from_str(yaml_str).map_err(FrontmatterError::Invalid)?;
        let mut fields = BTreeMap::new();
        if let Value::Mapping(map) = value {
            for (k, v) in map {
                if let Value::String(key) = k {
                    fields.insert(key, v);
                }
            }
        }
        Ok((Frontmatter { fields }, body))
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.fields.get(key)
    }

    pub fn get_str(&self, key: &str) -> Option<&str> {
        self.fields.get(key).and_then(Value::as_str)
    }

    pub fn title(&self) -> Option<&str> {
        self.get_str("title")
    }

    pub fn slug(&self) -> Option<&str> {
        self.get_str("slug")
    }

    pub fn category(&self) -> Option<&str> {
        self.get_str("category")
    }

    pub fn content_type(&self) -> Option<&str> {
        self.get_str("content_type")
    }

    pub fn status(&self) -> Option<&str> {
        self.get_str("status")
    }

    pub fn paired_with(&self) -> Option<&str> {
        self.get_str("paired_with")
    }

    /// Every field name present, for callers that need to check schema conformance
    /// (e.g. "does this article carry `short_description`?") without a dedicated accessor.
    pub fn field_names(&self) -> impl Iterator<Item = &str> {
        self.fields.keys().map(String::as_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_real_shape() {
        let content = "---\nschema: foundry-doc-v1\ntitle: \"How It's Built\"\nslug: architecture-index\ncategory: architecture\nstatus: active\n---\n\nBody text starts here.\n";
        let (fm, body) = Frontmatter::parse(content).expect("should parse");
        assert_eq!(fm.title(), Some("How It's Built"));
        assert_eq!(fm.slug(), Some("architecture-index"));
        assert_eq!(fm.category(), Some("architecture"));
        assert_eq!(fm.status(), Some("active"));
        assert!(body.trim_start().starts_with("Body text starts here."));
    }

    #[test]
    fn missing_frontmatter_is_reported_not_panicked() {
        let content = "# Just a heading\n\nNo frontmatter here.\n";
        let err = Frontmatter::parse(content).unwrap_err();
        assert!(matches!(err, FrontmatterError::Missing));
    }

    #[test]
    fn invalid_yaml_is_reported_not_panicked() {
        let content = "---\ntitle: [unclosed\n---\nBody.\n";
        let err = Frontmatter::parse(content).unwrap_err();
        assert!(matches!(err, FrontmatterError::Invalid(_)));
    }

    #[test]
    fn tolerates_leading_bom() {
        let content = "\u{feff}---\ntitle: \"X\"\n---\nBody.\n";
        let (fm, _) = Frontmatter::parse(content).expect("should parse past BOM");
        assert_eq!(fm.title(), Some("X"));
    }
}
