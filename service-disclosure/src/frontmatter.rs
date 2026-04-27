// SPDX-License-Identifier: Apache-2.0
//
// Document frontmatter — the YAML preamble every doctrine clause,
// convention, and public-facing document carries per CLAUDE.md §16
// citation discipline + the BCSC posture in §6.

use serde::{Deserialize, Serialize};

use crate::request::Register;

/// Frontmatter block carried at the top of doctrine clauses, conventions,
/// and public-facing documents. Required fields per CLAUDE.md §16 are
/// `schema` and `cites`; the rest are required-or-optional per the host
/// document genre. Per-genre rules are enforced by `validate_frontmatter`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Frontmatter {
    /// Schema discriminator (e.g., `"foundry-doc-v1"`,
    /// `"foundry-convention-v1"`). Always required.
    pub schema: String,

    /// Citation IDs resolved against `~/Foundry/citations.yaml`. Always
    /// required (an empty list is permitted for documents that genuinely
    /// cite nothing — most do not).
    #[serde(default)]
    pub cites: Vec<String>,

    /// SPDX-License-Identifier (e.g., `"Apache-2.0"`, `"CC-BY-4.0"`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,

    /// Copyright statement (e.g., `"© 2026 PointSav Digital Systems"`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copyright: Option<String>,

    /// True if the document contains forward-looking statements per
    /// OSC SN 51-721. When true, validators require accompanying cautionary
    /// language elsewhere in the document (out-of-band check; this field
    /// only declares intent).
    #[serde(default)]
    pub forward_looking: bool,

    /// Editorial register (Bloomberg-grade default for public-facing prose).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub register: Option<Register>,

    /// IETF BCP 47 language tag of the document body. Required for any
    /// document with a bilingual sibling (`README.md` ↔ `README.es.md`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    /// Tenant slug (e.g., `"pointsav"`, `"woodfine"`). Optional at the
    /// engineering tier; required at the customer tier so propagation can
    /// route per-tenant adapters correctly.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant: Option<String>,
}

impl Frontmatter {
    /// Construct a minimally-populated frontmatter for a foundry-doc-v1
    /// document. Convenience for tests and template authors.
    pub fn foundry_doc(cites: impl IntoIterator<Item = String>) -> Self {
        Self {
            schema: "foundry-doc-v1".to_string(),
            cites: cites.into_iter().collect(),
            license: None,
            copyright: None,
            forward_looking: false,
            register: None,
            language: None,
            tenant: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_through_yaml_preserves_all_fields() {
        let fm = Frontmatter {
            schema: "foundry-convention-v1".into(),
            cites: vec!["ni-51-102".into(), "osc-sn-51-721".into()],
            license: Some("Apache-2.0".into()),
            copyright: Some("© 2026 PointSav Digital Systems".into()),
            forward_looking: true,
            register: Some(Register::Bloomberg),
            language: Some("en".into()),
            tenant: Some("pointsav".into()),
        };
        let y = serde_yaml::to_string(&fm).expect("serialise");
        let back: Frontmatter = serde_yaml::from_str(&y).expect("deserialise");
        assert_eq!(fm, back);
    }

    #[test]
    fn optional_fields_omitted_from_wire_when_none() {
        let fm = Frontmatter::foundry_doc([]);
        let y = serde_yaml::to_string(&fm).expect("serialise");
        for absent_key in ["license:", "copyright:", "register:", "language:", "tenant:"] {
            assert!(!y.contains(absent_key), "unexpected key {absent_key} in: {y}");
        }
        // schema, cites, forward_looking are always present in serde output.
        assert!(y.contains("schema:"));
        assert!(y.contains("cites:"));
        assert!(y.contains("forward_looking:"));
    }

    #[test]
    fn deserialises_minimal_frontmatter() {
        let y = "schema: foundry-doc-v1\ncites: []\n";
        let fm: Frontmatter = serde_yaml::from_str(y).expect("deserialise");
        assert_eq!(fm.schema, "foundry-doc-v1");
        assert!(fm.cites.is_empty());
        assert!(!fm.forward_looking);
    }
}
