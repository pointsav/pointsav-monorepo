// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.
//! Frontmatter split and typed access. Frontmatter is optional: a page is reported as Ok, Missing
//! (no `---` block) or Invalid (a block that is not valid YAML). The field set differs by kind of
//! page, so the fields are a map with accessors for the few that most pages carry.

use std::collections::BTreeMap;

use serde_yaml::Value;

/// A parsed `---`-delimited YAML frontmatter block.
#[derive(Debug, Clone, Default)]
pub struct Frontmatter {
    fields: BTreeMap<String, Value>,
}

/// What reading a page's frontmatter found.
#[derive(Debug, Clone)]
pub enum FrontmatterState {
    Ok(Frontmatter),
    /// No `---` block at the top of the file (a README, a page of plain notes).
    Missing,
    /// A block exists but is not valid YAML; the message is the parser's.
    Invalid(String),
}

impl FrontmatterState {
    pub fn is_ok(&self) -> bool {
        matches!(self, FrontmatterState::Ok(_))
    }
}

impl Frontmatter {
    /// Split `content` into (state, body). A leading byte-order mark is tolerated. For Missing,
    /// the body is the whole file; for Invalid, the body is what follows the block.
    pub fn parse(content: &str) -> (FrontmatterState, &str) {
        let content = content.strip_prefix('\u{feff}').unwrap_or(content);
        let rest = content
            .strip_prefix("---\n")
            .or_else(|| content.strip_prefix("---\r\n"));
        let Some(rest) = rest else {
            return (FrontmatterState::Missing, content);
        };
        let Some(end) = rest.find("\n---\n").or_else(|| rest.find("\n---\r\n")) else {
            return (FrontmatterState::Missing, content);
        };
        let yaml_str = &rest[..end];
        let after_marker = &rest[end + 1..];
        let body_start = after_marker
            .find('\n')
            .map(|i| i + 1)
            .unwrap_or(after_marker.len());
        let body = &after_marker[body_start..];
        match serde_yaml::from_str::<Value>(yaml_str) {
            Err(e) => (FrontmatterState::Invalid(e.to_string()), body),
            Ok(value) => {
                let mut fields = BTreeMap::new();
                if let Value::Mapping(map) = value {
                    for (k, v) in map {
                        if let Value::String(key) = k {
                            fields.insert(key, v);
                        }
                    }
                }
                (FrontmatterState::Ok(Frontmatter { fields }), body)
            }
        }
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

    /// Every field name present, for callers that check a schema.
    pub fn field_names(&self) -> impl Iterator<Item = &str> {
        self.fields.keys().map(String::as_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_a_block_and_returns_the_body() {
        let (st, body) =
            Frontmatter::parse("---\ntitle: \"Made Up\"\nstatus: draft\n---\n\nBody text.\n");
        let FrontmatterState::Ok(fm) = st else {
            panic!("expected Ok")
        };
        assert_eq!(fm.title(), Some("Made Up"));
        assert_eq!(fm.get_str("status"), Some("draft"));
        assert!(body.trim_start().starts_with("Body text."));
    }

    #[test]
    fn no_block_is_missing_and_keeps_the_whole_file_as_body() {
        let text = "# Just a heading\n\nNo frontmatter here.\n";
        let (st, body) = Frontmatter::parse(text);
        assert!(matches!(st, FrontmatterState::Missing));
        assert_eq!(body, text);
    }

    #[test]
    fn an_unclosed_block_is_missing() {
        let (st, _) = Frontmatter::parse("---\ntitle: x\nno closing line\n");
        assert!(matches!(st, FrontmatterState::Missing));
    }

    #[test]
    fn bad_yaml_is_invalid_not_a_panic() {
        let (st, body) = Frontmatter::parse("---\ntitle: [unclosed\n---\nBody.\n");
        assert!(matches!(st, FrontmatterState::Invalid(_)));
        assert_eq!(body, "Body.\n");
    }

    #[test]
    fn tolerates_a_byte_order_mark() {
        let (st, _) = Frontmatter::parse("\u{feff}---\ntitle: \"X\"\n---\nBody.\n");
        assert!(st.is_ok());
    }

    #[test]
    fn nested_lists_and_block_scalars_parse() {
        let text = "---\ncites:\n  - a\n  - b\nnote: |\n  line one\n  line two\n---\nBody.\n";
        let (st, _) = Frontmatter::parse(text);
        let FrontmatterState::Ok(fm) = st else {
            panic!("expected Ok")
        };
        assert!(fm.get("cites").is_some());
        assert!(fm.get_str("note").unwrap().contains("line two"));
    }
}
