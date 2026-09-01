// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! The claim layer — Phase 3 of `KNOWLEDGE-PLATFORM-PLAN.md`.
//!
//! Extracts `Claim` objects from claim-annotated markdown per
//! `conventions/claim-authoring-convention.md` (Doctrine claim #54). The
//! carrier is an HTML comment pair — `<!--claim id=… cites=[…]
//! confidence=…-->text<!--/claim-->` — which comrak passes through unchanged
//! (`render.rs`'s `opts.render.r#unsafe = true`), so annotated markdown
//! renders identically whether or not this module ever runs (graceful
//! degradation, convention §2.3).
//!
//! This module only *extracts*; `content_hash` here is a pure function of the
//! claim text. `published_at` / `revision_sha` (the git-derived half of the
//! two-clock model) are filled in by the caller from `history::file_history`,
//! since this module has no access to a git repository.

use serde::{Deserialize, Serialize};

/// The claim's epistemic grade (convention §4.4). Closed enum — an unrecognised
/// value makes the whole marker malformed (degrades to inert comment, §4.2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Confidence {
    Established,
    Reported,
    Projected,
    Contested,
    Structural,
}

impl Confidence {
    fn parse(s: &str) -> Option<Self> {
        match s {
            "established" => Some(Self::Established),
            "reported" => Some(Self::Reported),
            "projected" => Some(Self::Projected),
            "contested" => Some(Self::Contested),
            "structural" => Some(Self::Structural),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Established => "established",
            Self::Reported => "reported",
            Self::Projected => "projected",
            Self::Contested => "contested",
            Self::Structural => "structural",
        }
    }
}

/// One authored claim extracted from a topic's markdown body.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claim {
    /// Local id as authored. Global address is `<topic_slug>:<id>` (convention §7).
    pub id: String,
    pub topic_slug: String,
    /// `citations.yaml` registry IDs. Empty only permitted when `confidence`
    /// is `structural` — the convention's rule, not enforced here (linter's job).
    pub cites: Vec<String>,
    pub confidence: Confidence,
    /// When the asserted fact began applying. `None` ⇒ timeless.
    pub valid_at: Option<String>,
    /// Other claim ids this claim's truth rests on (`<id>` same-file or
    /// `<topic-slug>:<id>` cross-file).
    pub depends_on: Vec<String>,
    /// The rendered text between the markers, trimmed.
    pub text: String,
    /// blake3 hex of the normalised (trimmed) claim-span text.
    pub content_hash: String,
    /// Derived: the committer timestamp (ISO `YYYY-MM-DD`) of the Git commit
    /// that last touched this claim's span. Filled in by the caller from
    /// `history::file_history` — this module has no git access.
    pub published_at: Option<String>,
    /// Derived: the commit SHA the claim was extracted at.
    pub revision_sha: Option<String>,
}

impl Claim {
    /// The globally-addressable id: `<topic_slug>:<id>`.
    pub fn global_id(&self) -> String {
        format!("{}:{}", self.topic_slug, self.id)
    }
}

/// Extract every well-formed claim from a topic's raw markdown body.
///
/// Never fails on malformed input — a marker missing a required field, an
/// unrecognised `confidence`, a duplicate `id` within the file, unbalanced or
/// nested markers, are all silently skipped (degrade to inert comments per
/// convention §4.2/§9; the editorial linter's claim-validation pass is the
/// place that surfaces these as warnings, not this extractor).
pub fn extract_claims(topic_slug: &str, body_md: &str) -> Vec<Claim> {
    const OPEN: &str = "<!--claim";
    const CLOSE: &str = "<!--/claim-->";

    let mut out = Vec::new();
    let mut seen_ids = std::collections::HashSet::new();
    let mut cursor = 0usize;

    while let Some(rel_start) = body_md[cursor..].find(OPEN) {
        let start = cursor + rel_start;
        let Some(rel_marker_end) = body_md[start..].find("-->") else {
            // No `-->` at all for the rest of the file — nothing more to find.
            break;
        };
        let marker_end = start + rel_marker_end + 3;
        let marker_body = &body_md[start + OPEN.len()..start + rel_marker_end];

        let Some(rel_close) = body_md[marker_end..].find(CLOSE) else {
            // Unbalanced: no closing marker anywhere after this opener.
            // Advance past this opener and keep scanning the rest of the file.
            cursor = marker_end;
            continue;
        };
        let content_start = marker_end;
        let content_end = marker_end + rel_close;
        let close_end = content_end + CLOSE.len();

        // Nesting guard: another opener inside this span means the author
        // nested claims, which the convention forbids (§4.1). Depth-count
        // forward from marker_end to find the OUTER marker's true matching
        // closer (the naive `content_end` above is actually the INNER
        // claim's closer in this case) and skip the entire malformed region
        // — extracting nothing from it, inner claim included. A shallow
        // `cursor = marker_end` here would leave the true outer closer
        // un-skipped and, worse, let the inner claim be parsed as valid on
        // the next loop iteration.
        if body_md[content_start..content_end].contains(OPEN) {
            let mut depth = 1usize;
            let mut scan = marker_end;
            let true_close_end = loop {
                let next_open = body_md[scan..].find(OPEN).map(|p| scan + p);
                let next_close = body_md[scan..].find(CLOSE).map(|p| scan + p);
                match (next_open, next_close) {
                    (Some(o), Some(c)) if o < c => {
                        depth += 1;
                        scan = o + OPEN.len();
                    }
                    (_, Some(c)) => {
                        depth -= 1;
                        let end = c + CLOSE.len();
                        if depth == 0 {
                            break end;
                        }
                        scan = end;
                    }
                    _ => break body_md.len(), // unbalanced past this point — consume to EOF
                }
            };
            cursor = true_close_end;
            continue;
        }

        if let Some(fields) = parse_fields(marker_body) {
            if let (Some(id), Some(confidence)) = (fields.id, fields.confidence) {
                if seen_ids.insert(id.clone()) {
                    let text = body_md[content_start..content_end].trim().to_string();
                    let content_hash = blake3::hash(text.as_bytes()).to_hex().to_string();
                    out.push(Claim {
                        id,
                        topic_slug: topic_slug.to_string(),
                        cites: fields.cites,
                        confidence,
                        valid_at: fields.valid_at,
                        depends_on: fields.depends_on,
                        text,
                        content_hash,
                        published_at: None,
                        revision_sha: None,
                    });
                }
                // else: duplicate id within this file — first occurrence wins,
                // per §7 ("an id is a stable handle" — a repeat is malformed).
            }
        }
        cursor = close_end;
    }

    out
}

#[derive(Default)]
struct Fields {
    id: Option<String>,
    cites: Vec<String>,
    confidence: Option<Confidence>,
    valid_at: Option<String>,
    depends_on: Vec<String>,
}

/// Parse the space-separated `key=value` fields inside an opening marker's
/// body (the text between `<!--claim` and `-->`). Grammar (convention §4.2):
/// a value has no spaces, so a simple whitespace split is exact. An unknown
/// key is ignored (forward-compatible per §4.2).
fn parse_fields(marker_body: &str) -> Option<Fields> {
    let mut fields = Fields::default();
    for token in marker_body.split_whitespace() {
        let (key, value) = token.split_once('=')?;
        match key {
            "id" => fields.id = Some(value.to_string()),
            "confidence" => fields.confidence = Confidence::parse(value),
            "valid_at" => fields.valid_at = Some(value.to_string()),
            "cites" => fields.cites = parse_list(value),
            "depends_on" => fields.depends_on = parse_list(value),
            _ => {}
        }
    }
    Some(fields)
}

/// Parse a `[a,b,c]` list value; `[]` yields an empty vec; a bare (non-list)
/// value is treated as a single-element list for leniency.
fn parse_list(value: &str) -> Vec<String> {
    let Some(inner) = value.strip_prefix('[').and_then(|v| v.strip_suffix(']')) else {
        return vec![value.to_string()];
    };
    if inner.is_empty() {
        return Vec::new();
    }
    inner.split(',').map(|s| s.trim().to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // The four worked examples are taken verbatim from
    // conventions/claim-authoring-convention.md §8 — this is the Phase 3.1
    // Engine Verification Gate the convention names.

    #[test]
    fn extracts_structural_claim_no_citation() {
        let md = "<!--claim id=derived-state confidence=structural cites=[]-->\nThe search index and the link graph are derived state: deleting them and\nre-reading the Git tree reproduces them exactly.\n<!--/claim-->";
        let claims = extract_claims("test-topic", md);
        assert_eq!(claims.len(), 1);
        assert_eq!(claims[0].id, "derived-state");
        assert_eq!(claims[0].confidence, Confidence::Structural);
        assert!(claims[0].cites.is_empty());
        assert!(claims[0].text.contains("derived state"));
        assert_eq!(claims[0].global_id(), "test-topic:derived-state");
    }

    #[test]
    fn extracts_established_claim_with_citation_and_date() {
        let md = "<!--claim id=tile-format-c2sp cites=[c2sp-tlog-tiles] valid_at=2024 confidence=established-->\nThe on-disk tile format follows the C2SP tlog-tiles specification verbatim.\n<!--/claim-->";
        let claims = extract_claims("test-topic", md);
        assert_eq!(claims.len(), 1);
        assert_eq!(claims[0].cites, vec!["c2sp-tlog-tiles"]);
        assert_eq!(claims[0].valid_at.as_deref(), Some("2024"));
        assert_eq!(claims[0].confidence, Confidence::Established);
    }

    #[test]
    fn extracts_projected_claim() {
        let md = "<!--claim id=monthly-anchor confidence=projected valid_at=2026 cites=[c2sp-tlog-tiles]-->\nThe platform plans to publish tile checkpoints monthly to a public transparency\nlog, giving any third party an independent integrity oracle.\n<!--/claim-->";
        let claims = extract_claims("test-topic", md);
        assert_eq!(claims.len(), 1);
        assert_eq!(claims[0].confidence, Confidence::Projected);
    }

    #[test]
    fn extracts_inline_claim_with_depends_on() {
        let md = "Because checkpoints are externally anchored, <!--claim id=audit-without-operator confidence=established cites=[rfc-9162] depends_on=[monthly-anchor]-->an auditor can confirm a record's integrity without involving the platform operator<!--/claim-->.";
        let claims = extract_claims("test-topic", md);
        assert_eq!(claims.len(), 1);
        assert_eq!(claims[0].depends_on, vec!["monthly-anchor"]);
        assert!(claims[0].text.contains("an auditor can confirm"));
    }

    #[test]
    fn multiple_non_overlapping_claims_in_one_file() {
        let md = "<!--claim id=a confidence=structural cites=[]-->First.<!--/claim-->\n\nSome connective prose.\n\n<!--claim id=b confidence=structural cites=[]-->Second.<!--/claim-->";
        let claims = extract_claims("test-topic", md);
        assert_eq!(claims.len(), 2);
        assert_eq!(claims[0].id, "a");
        assert_eq!(claims[1].id, "b");
    }

    #[test]
    fn malformed_marker_missing_id_is_skipped() {
        let md = "<!--claim confidence=structural cites=[]-->No id here.<!--/claim-->";
        assert!(extract_claims("t", md).is_empty());
    }

    #[test]
    fn malformed_marker_unknown_confidence_is_skipped() {
        let md = "<!--claim id=x confidence=maybe cites=[]-->Bad confidence.<!--/claim-->";
        assert!(extract_claims("t", md).is_empty());
    }

    #[test]
    fn duplicate_id_within_file_keeps_first_only() {
        let md = "<!--claim id=dup confidence=structural cites=[]-->First.<!--/claim--><!--claim id=dup confidence=structural cites=[]-->Second.<!--/claim-->";
        let claims = extract_claims("t", md);
        assert_eq!(claims.len(), 1);
        assert!(claims[0].text.contains("First"));
    }

    #[test]
    fn unclosed_marker_is_skipped_not_panicking() {
        let md = "<!--claim id=x confidence=structural cites=[]-->Never closed.";
        assert!(extract_claims("t", md).is_empty());
    }

    #[test]
    fn nested_markers_both_outer_and_inner_are_skipped() {
        let md = "<!--claim id=outer confidence=structural cites=[]-->Text <!--claim id=inner confidence=structural cites=[]-->nested<!--/claim--> more<!--/claim-->";
        // The outer opener's span contains another opener before its own
        // closer — malformed per §4.1, the ENTIRE region (outer AND inner)
        // is skipped as a unit, not just the outer id.
        let claims = extract_claims("t", md);
        assert!(claims.iter().all(|c| c.id != "outer"));
        assert!(claims.iter().all(|c| c.id != "inner"));
    }

    #[test]
    fn valid_claim_after_a_nested_malformed_region_still_extracts() {
        let md = "<!--claim id=outer confidence=structural cites=[]-->Text <!--claim id=inner confidence=structural cites=[]-->nested<!--/claim--> more<!--/claim--><!--claim id=valid confidence=structural cites=[]-->Fine.<!--/claim-->";
        let claims = extract_claims("t", md);
        assert_eq!(claims.len(), 1);
        assert_eq!(claims[0].id, "valid");
    }

    #[test]
    fn content_hash_is_deterministic_and_blake3_shaped() {
        let md = "<!--claim id=a confidence=structural cites=[]-->Same text.<!--/claim-->";
        let c1 = extract_claims("t", md);
        let c2 = extract_claims("t", md);
        assert_eq!(c1[0].content_hash, c2[0].content_hash);
        assert_eq!(c1[0].content_hash.len(), 64);
    }

    #[test]
    fn no_claims_in_plain_markdown_is_empty_not_error() {
        assert!(extract_claims("t", "# Just a heading\n\nPlain prose, no claims.\n").is_empty());
    }
}
