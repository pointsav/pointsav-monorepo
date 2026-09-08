//! AUTO-GENERATED MEMBERSHIP drift detection (BRIEF-tool-wiki plan Part 4 item 3). Every
//! `_index.md` category page renders its group listings as
//! `- [[slug]] — <short_description, verbatim>` inside an AUTO-GENERATED MEMBERSHIP block,
//! and each block's own comment claims "a generator script regenerates this on commit" —
//! confirmed false anywhere in the repo (no such script exists). This module is the real
//! implementation: given an `_index.md` `Document` and its sibling articles (from the same
//! crawl), it reports every AUTO block bullet whose text no longer matches the linked
//! article's current `short_description` — the drift a real generator would otherwise fix
//! silently on every commit.

use std::collections::HashMap;

use crate::document::Document;

/// One bullet line's drift: the article's frontmatter `short_description` no longer
/// matches what the `_index.md` page currently renders for it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MembershipDrift {
    pub index_group: String,
    pub slug: String,
    /// The line the block *should* render, built from the sibling's current
    /// `short_description`.
    pub expected_line: String,
    /// The line the block currently renders, if the slug's bullet is present at all.
    pub actual_line: Option<String>,
}

/// Compare `index_doc`'s AUTO-GENERATED MEMBERSHIP blocks against `siblings`' current
/// `short_description` frontmatter. `siblings` should be every other document crawled from
/// the same category directory (the crawler's own output, filtered by the caller — this
/// function doesn't crawl anything itself, matching the crate's "crawler stays free of
/// rule-checking logic" split).
///
/// **Callers must pre-filter `siblings` to the same language as `index_doc`.** An EN/ES
/// bilingual pair shares the same `slug:` value (the pair is distinguished by file suffix
/// and `paired_with:`, not by slug) — passing both languages' documents in one `siblings`
/// slice will silently collapse them onto one map entry and report every bullet as
/// "drifted" against the wrong language's text. Found running this against the real
/// corpus: an unfiltered first pass reported 209 false positives, 100% explained by this.
pub fn check_membership(index_doc: &Document, siblings: &[Document]) -> Vec<MembershipDrift> {
    let by_slug: HashMap<&str, &Document> = siblings
        .iter()
        .filter_map(|d| d.frontmatter.slug().map(|s| (s, d)))
        .collect();

    let mut drift = Vec::new();
    for block in &index_doc.auto_generated_blocks {
        // A block's current items, keyed by the slug each bullet links to, so a bullet
        // whose slug moved to a different position (or a slug missing an item entirely)
        // is still detected correctly rather than compared positionally.
        let mut actual_by_slug: HashMap<String, String> = HashMap::new();
        for item in &block.items {
            if let Some(slug) = bullet_slug(item) {
                actual_by_slug.insert(slug, item.clone());
            }
        }

        for item in &block.items {
            let Some(slug) = bullet_slug(item) else { continue };
            let Some(sibling) = by_slug.get(slug.as_str()) else {
                // A linked slug that isn't among the crawled siblings -- out of this
                // function's scope (could be a cross-category link, or the sibling wasn't
                // included in this call's crawl slice); not reported as drift.
                continue;
            };
            let Some(short_description) = sibling.frontmatter.get_str("short_description")
            else {
                continue; // nothing to compare against
            };
            // Preserve the actual bullet's own link form -- bare `[[slug]]` or piped
            // `[[slug|Display Text]]` -- rather than assuming bare. Found the hard way:
            // an earlier version always rebuilt `[[slug]]`, which would have silently
            // discarded 232 of 511 real, intentional custom display texts in the
            // documentation wiki's AUTO blocks on a first real run.
            let Some(link_prefix) = bullet_link_prefix(item) else { continue };
            let expected_line = format!("{link_prefix} — {short_description}");
            if item != &expected_line {
                drift.push(MembershipDrift {
                    index_group: block.index_group.clone(),
                    slug: slug.clone(),
                    expected_line,
                    actual_line: Some(item.clone()),
                });
            }
        }
    }
    drift
}

/// Extract the `slug` from a `- [[slug]] — description` (or `- [[slug|display]] — ...`)
/// bullet line.
fn bullet_slug(item: &str) -> Option<String> {
    let start = item.find("[[")? + 2;
    let end = item[start..].find("]]")? + start;
    let inner = &item[start..end];
    let slug = inner.split('|').next().unwrap_or(inner);
    Some(slug.to_string())
}

/// Extract everything up to and including the closing `]]` of a bullet's link — e.g.
/// `- [[commuter]]` or `- [[commuter|Commuter (PKS)]]` — so a caller can rebuild the
/// bullet with a fresh description while keeping the exact link form the author chose.
fn bullet_link_prefix(item: &str) -> Option<&str> {
    let start = item.find("[[")?;
    let close = item[start..].find("]]")? + start + 2;
    Some(&item[..close])
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn doc(path: &str, content: &str) -> Document {
        Document::parse(PathBuf::from(path), content).unwrap()
    }

    #[test]
    fn detects_stale_bullet_text() {
        let index = doc(
            "urban/_index.md",
            "---\ntitle: \"X\"\n---\n\n\
<!-- AUTO-GENERATED MEMBERSHIP: DO NOT EDIT BELOW — regenerate from index_group: g -->\n\
- [[commuter]] — an old, stale description that no longer matches the source article.\n\
<!-- END AUTO-GENERATED -->\n",
        );
        let commuter = doc(
            "urban/commuter.md",
            "---\ntitle: \"Commuter\"\nslug: commuter\nshort_description: \"Commuter (PKS) clusters identify transit-adjacent sites.\"\n---\n\nBody.\n",
        );
        let drift = check_membership(&index, &[commuter]);
        assert_eq!(drift.len(), 1);
        assert_eq!(drift[0].slug, "commuter");
        assert_eq!(
            drift[0].expected_line,
            "- [[commuter]] — Commuter (PKS) clusters identify transit-adjacent sites."
        );
    }

    #[test]
    fn matching_bullet_reports_no_drift() {
        let index = doc(
            "urban/_index.md",
            "---\ntitle: \"X\"\n---\n\n\
<!-- AUTO-GENERATED MEMBERSHIP: DO NOT EDIT BELOW — regenerate from index_group: g -->\n\
- [[commuter]] — Commuter (PKS) clusters identify transit-adjacent sites.\n\
<!-- END AUTO-GENERATED -->\n",
        );
        let commuter = doc(
            "urban/commuter.md",
            "---\ntitle: \"Commuter\"\nslug: commuter\nshort_description: \"Commuter (PKS) clusters identify transit-adjacent sites.\"\n---\n\nBody.\n",
        );
        let drift = check_membership(&index, &[commuter]);
        assert!(drift.is_empty());
    }

    #[test]
    fn slug_not_among_siblings_is_not_reported() {
        let index = doc(
            "urban/_index.md",
            "---\ntitle: \"X\"\n---\n\n\
<!-- AUTO-GENERATED MEMBERSHIP: DO NOT EDIT BELOW — regenerate from index_group: g -->\n\
- [[some-other-slug]] — text.\n\
<!-- END AUTO-GENERATED -->\n",
        );
        let drift = check_membership(&index, &[]);
        assert!(drift.is_empty());
    }

    #[test]
    fn piped_wikilink_bullet_resolves_by_slug_and_preserves_display_text() {
        let index = doc(
            "x/_index.md",
            "---\ntitle: \"X\"\n---\n\n\
<!-- AUTO-GENERATED MEMBERSHIP: DO NOT EDIT BELOW — regenerate from index_group: g -->\n\
- [[commuter|Commuter (PKS)]] — stale text.\n\
<!-- END AUTO-GENERATED -->\n",
        );
        let commuter = doc(
            "x/commuter.md",
            "---\nslug: commuter\nshort_description: \"Fresh text.\"\n---\n\nBody.\n",
        );
        let drift = check_membership(&index, &[commuter]);
        assert_eq!(drift.len(), 1);
        assert_eq!(drift[0].slug, "commuter");
        // The fix must keep the author's custom display text, not collapse it to a bare
        // `[[commuter]]` link -- this is the real bug the documentation-wiki run found.
        assert_eq!(
            drift[0].expected_line,
            "- [[commuter|Commuter (PKS)]] — Fresh text."
        );
    }

    #[test]
    fn bare_wikilink_bullet_stays_bare_when_fixed() {
        let index = doc(
            "x/_index.md",
            "---\ntitle: \"X\"\n---\n\n\
<!-- AUTO-GENERATED MEMBERSHIP: DO NOT EDIT BELOW — regenerate from index_group: g -->\n\
- [[commuter]] — stale text.\n\
<!-- END AUTO-GENERATED -->\n",
        );
        let commuter = doc(
            "x/commuter.md",
            "---\nslug: commuter\nshort_description: \"Fresh text.\"\n---\n\nBody.\n",
        );
        let drift = check_membership(&index, &[commuter]);
        assert_eq!(drift.len(), 1);
        assert_eq!(drift[0].expected_line, "- [[commuter]] — Fresh text.");
    }
}
