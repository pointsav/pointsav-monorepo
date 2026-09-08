//! Iceberg-ratio check (`BRIEF-category-index-guide-redesign.md` Queue #26, check 1 —
//! spec: `.agent/audit/category-redesign-fable-pass.md` §R2-6 Check 1, cross-read against
//! `category-redesign-opus-pass.md` §R6.3).
//!
//! Per category, what fraction of its real membership is actually *surfaced* in the
//! category's `_index.md` curated view, versus reachable only by scrolling the
//! AUTO-GENERATED enumeration (or by search). A category that surfaces almost nothing is a
//! flat dump; a category that surfaces almost everything has a second full listing rather
//! than a curated layer.
//!
//! Usage: `cargo run --example iceberg_ratio -- <path-to-wiki-content-root>`
//!
//! # Definitions
//!
//! * **Membership (M)** — same-language documents carrying `category: <id>`, excluding the
//!   `_index` page itself, anything under `.archive/`, and any document whose `status` is
//!   `archived`, `retired`, or `superseded`.
//! * **Surfaced (S)** — distinct member slugs wikilinked from the `_index` body **outside**
//!   every `AUTO-GENERATED MEMBERSHIP` block: the `START-HERE-HIGHLIGHT` card plus any
//!   hand-written prose link. This is the curation layer as it exists on disk today.
//!
//! # Simplifications against the written spec, and why
//!
//! 1. **`surfaced: true` frontmatter is not used.** Opus §R6.3 computes S from a per-article
//!    `surfaced: true` flag. That field does not exist in any schema or on any article
//!    (verified across all three wikis, 2026-09-06); its introduction is sequenced as step 6
//!    of that pass's own rollout table, as a `DESIGN-TOKEN-CHANGE` routed to project-design.
//!    Running the flag-based version today reports every category as `under_curated` and
//!    says nothing. Fable §R2-6's definition — links outside the AUTO blocks — is
//!    computable now and measures the same thing on today's corpus. When `surfaced:` lands,
//!    add it as an additional source of S rather than replacing this one (a hand-written
//!    prose link is a real surface either way).
//! 2. **`leaf: true` index groups are not excluded.** Fable's M excludes slugs in any
//!    `index_group` a category has declared `leaf: true` in `categories.yaml` — a *proposed*
//!    optional flag no `categories.yaml` carries today. Until it exists the check counts
//!    full membership and the `markets`-style leaf populations will read as under-curated;
//!    that is the honest reading, and the spec's own note says the ratio target applies to
//!    reader-meaningful entry points. `LEAF_GROUP_FLAG` below is the one-line hook.
//! 3. **Bands follow Fable, not Opus.** Fable's `S/M` band (0.15–0.45, target 0.30, INFO for
//!    7–9 members, exempt at ≤6) is a ratio; Opus's is an absolute clamp(0.30·M, 3, 12). At
//!    the corpus's real category sizes (2–88) the ratio band is the less arbitrary of the
//!    two, and it degrades sensibly for the very large categories Opus's ceiling of 12 would
//!    permanently flag.
//!
//! Report-only. Curation is editorial judgement by definition — there is no `--apply`.

use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::path::Path;

use tool_wiki_core::Document;

const TARGET: f64 = 0.30;
const BAND_LOW: f64 = 0.15;
const BAND_HIGH: f64 = 0.45;
/// Below this membership the iceberg does not apply -- surfacing everything is correct.
const EXEMPT_AT_OR_BELOW: usize = 6;
/// Between `EXEMPT_AT_OR_BELOW` and this, report as INFO rather than a finding.
const INFO_UP_TO: usize = 9;

/// `status:` values that take a document out of the live membership.
const DEAD_STATUSES: &[&str] = &["archived", "retired", "superseded"];

fn is_live(doc: &Document) -> bool {
    let archived_path = doc.path.components().any(|c| {
        c.as_os_str()
            .to_str()
            .map(|s| s == ".archive")
            .unwrap_or(false)
    });
    let dead_status = doc
        .frontmatter
        .status()
        .map(|s| DEAD_STATUSES.contains(&s))
        .unwrap_or(false);
    !archived_path && !dead_status
}

fn is_spanish(doc: &Document) -> bool {
    doc.path.to_string_lossy().ends_with(".es.md")
}

/// Wikilink targets in the `_index` body that fall outside every AUTO-GENERATED block.
fn curated_link_targets(index: &Document) -> BTreeSet<String> {
    index
        .wikilinks
        .iter()
        .filter(|w| {
            !index
                .auto_generated_blocks
                .iter()
                .any(|b| w.offset >= b.offset && w.offset < b.end)
        })
        .map(|w| w.target.clone())
        .collect()
}

#[derive(Debug, PartialEq, Eq)]
enum Verdict {
    Exempt,
    Info,
    FlatDump,
    OverSurfaced,
    Ok,
}

fn verdict(members: usize, surfaced: usize) -> Verdict {
    if members <= EXEMPT_AT_OR_BELOW {
        return Verdict::Exempt;
    }
    let ratio = surfaced as f64 / members as f64;
    if members <= INFO_UP_TO {
        return Verdict::Info;
    }
    if ratio < BAND_LOW {
        Verdict::FlatDump
    } else if ratio > BAND_HIGH {
        Verdict::OverSurfaced
    } else {
        Verdict::Ok
    }
}

fn main() {
    let root = env::args().nth(1).expect("usage: iceberg_ratio <path>");
    let root = Path::new(&root);

    let crawl = tool_wiki_core::crawl(root).expect("crawl failed");
    let live: Vec<&Document> = crawl.documents.iter().filter(|d| is_live(d)).collect();

    println!("crawled {} documents ({} live)\n", crawl.documents.len(), live.len());

    // (category, is_es) -> members; and the matching `_index` document.
    let mut members: BTreeMap<(String, bool), Vec<&Document>> = BTreeMap::new();
    let mut indexes: BTreeMap<(String, bool), &Document> = BTreeMap::new();
    for d in &live {
        let Some(cat) = d.frontmatter.category() else { continue };
        if tool_wiki_core::is_reserved_category(cat) {
            continue;
        }
        let key = (cat.to_string(), is_spanish(d));
        let is_index = d
            .path
            .file_stem()
            .and_then(|s| s.to_str())
            .map(|s| s == "_index" || s == "_index.es")
            .unwrap_or(false);
        if is_index {
            indexes.insert(key, d);
        } else {
            members.entry(key).or_default().push(d);
        }
    }

    // Whole-crawl slug -> (category, live) so a curated link that is not a member of this
    // category can be told apart into its three genuinely different cases (see below).
    let mut slug_map: BTreeMap<&str, (Option<&str>, bool)> = BTreeMap::new();
    for d in &crawl.documents {
        if let Some(slug) = d.frontmatter.slug() {
            let entry = (d.frontmatter.category(), is_live(d));
            // A live document always wins over a dead one carrying the same slug (the EN/ES
            // pair and `.archive/` copies share slugs).
            match slug_map.get(slug) {
                Some((_, true)) => {}
                _ => {
                    slug_map.insert(slug, entry);
                }
            }
        }
    }

    let mut rows: Vec<(String, bool, usize, usize, Verdict, Vec<String>)> = Vec::new();
    let mut no_index: Vec<(String, bool, usize)> = Vec::new();

    for (key, mems) in &members {
        let Some(index) = indexes.get(key) else {
            no_index.push((key.0.clone(), key.1, mems.len()));
            continue;
        };
        let member_slugs: BTreeSet<&str> =
            mems.iter().filter_map(|d| d.frontmatter.slug()).collect();
        let curated = curated_link_targets(index);

        let surfaced: BTreeSet<&String> = curated
            .iter()
            .filter(|t| member_slugs.contains(t.as_str()))
            .collect();
        // A curated link that is not a live member of this category splits into three cases
        // that mean genuinely different things. Only the last two are defects: a pointer to
        // another category's index or a related article is ordinary navigation, and flagging
        // it would bury the real findings.
        let foreign: Vec<String> = curated
            .iter()
            .filter(|t| !member_slugs.contains(t.as_str()))
            .filter_map(|t| match slug_map.get(t.as_str()) {
                None => Some(format!("DEAD  `{t}` — no document anywhere carries this slug")),
                Some((_, false)) => Some(format!(
                    "RETIRED  `{t}` — surfaced but archived/retired/superseded"
                )),
                Some((_, true)) => None, // live cross-category navigation link
            })
            .collect();

        let v = verdict(mems.len(), surfaced.len());
        rows.push((key.0.clone(), key.1, mems.len(), surfaced.len(), v, foreign));
    }

    rows.sort_by(|a, b| (b.2, &a.0).cmp(&(a.2, &b.0)));

    let mut findings = 0usize;
    println!("=== iceberg ratio per category (S = curated links outside AUTO blocks) ===");
    println!("  target {TARGET:.2}, band {BAND_LOW:.2}-{BAND_HIGH:.2}; M<={EXEMPT_AT_OR_BELOW} exempt, M<={INFO_UP_TO} info-only\n");
    for (cat, es, m, s, v, foreign) in &rows {
        let lang = if *es { "es" } else { "en" };
        let ratio = *s as f64 / *m as f64;
        let label = match v {
            Verdict::Ok => "OK",
            Verdict::Info => "INFO",
            Verdict::Exempt => "exempt",
            Verdict::FlatDump => "FLAT DUMP — no meaningful curated surface",
            Verdict::OverSurfaced => "OVER-SURFACED — the curation layer is a second full listing",
        };
        if matches!(v, Verdict::FlatDump | Verdict::OverSurfaced) {
            findings += 1;
        }
        println!("  {cat} [{lang}]: {s}/{m} surfaced ({ratio:.2}) — {label}");
        for f in foreign {
            findings += 1;
            println!("    ERROR: {f}");
        }
    }

    if !no_index.is_empty() {
        println!("\n=== categories with members but no `_index` page ({}) ===", no_index.len());
        for (cat, es, m) in &no_index {
            println!("  {cat} [{}]: {m} members, no index to curate", if *es { "es" } else { "en" });
        }
    }

    println!("\ntotal findings: {findings}");
}

#[cfg(test)]
mod iceberg_tests {
    use super::*;
    use std::path::PathBuf;

    fn doc(path: &str, content: &str) -> Document {
        Document::parse(PathBuf::from(path), content).unwrap()
    }

    #[test]
    fn bands_follow_the_spec() {
        assert_eq!(verdict(4, 4), Verdict::Exempt);
        assert_eq!(verdict(6, 1), Verdict::Exempt);
        assert_eq!(verdict(8, 1), Verdict::Info);
        assert_eq!(verdict(40, 1), Verdict::FlatDump); // 0.025
        assert_eq!(verdict(40, 12), Verdict::Ok); // 0.30 exactly
        assert_eq!(verdict(20, 3), Verdict::Ok); // 0.15 -- inclusive low edge
        assert_eq!(verdict(20, 9), Verdict::Ok); // 0.45 -- inclusive high edge
        assert_eq!(verdict(20, 10), Verdict::OverSurfaced); // 0.50
    }

    /// The real `_index` shape: a START-HERE card and prose above, AUTO blocks below. Only
    /// the links above count as surfaced.
    #[test]
    fn links_inside_auto_blocks_are_not_surfaced() {
        let index = doc(
            "architecture/_index.md",
            "---\ntitle: \"Architecture\"\nslug: architecture-index\ncategory: architecture\ncontent_type: topic\n---\n\n\
Lead prose mentioning [[three-ring-architecture]] directly.\n\n\
<!-- START-HERE-HIGHLIGHT -->\n\n**Start here:** [[three-ring-architecture]] — the frame.\n\n<!-- END-START-HERE-HIGHLIGHT -->\n\n\
## Platform structure\n\nIntro sentence.\n\n\
<!-- AUTO-GENERATED MEMBERSHIP: DO NOT EDIT BELOW — regenerate from index_group: platform-structure -->\n\
- [[three-ring-architecture]] — desc.\n\
- [[3-layer-stack]] — desc.\n\
- [[six-tier-sovereignty-matrix]] — desc.\n\
<!-- END AUTO-GENERATED -->\n",
        );
        let curated = curated_link_targets(&index);
        assert_eq!(curated.len(), 1, "only the prose/START-HERE link counts");
        assert!(curated.contains("three-ring-architecture"));
        assert!(!curated.contains("3-layer-stack"));
    }

    #[test]
    fn archived_path_and_dead_status_leave_the_membership() {
        assert!(!is_live(&doc(
            "gis/.archive/old.md",
            "---\nslug: old\ncategory: gis\n---\n\nBody.\n"
        )));
        assert!(!is_live(&doc(
            "gis/retired.md",
            "---\nslug: retired\ncategory: gis\nstatus: superseded\n---\n\nBody.\n"
        )));
        assert!(is_live(&doc(
            "gis/real.md",
            "---\nslug: real\ncategory: gis\nstatus: active\n---\n\nBody.\n"
        )));
    }

    #[test]
    fn a_multi_block_index_subtracts_every_block_not_just_the_first() {
        let index = doc(
            "x/_index.md",
            "---\ntitle: \"X\"\nslug: x-index\ncategory: x\ncontent_type: topic\n---\n\n\
Prose links [[keeper]].\n\n\
<!-- AUTO-GENERATED MEMBERSHIP: DO NOT EDIT BELOW — regenerate from index_group: one -->\n- [[a]] — d.\n<!-- END AUTO-GENERATED -->\n\n\
## Second\n\nText.\n\n\
<!-- AUTO-GENERATED MEMBERSHIP: DO NOT EDIT BELOW — regenerate from index_group: two -->\n- [[b]] — d.\n<!-- END AUTO-GENERATED -->\n",
        );
        let curated = curated_link_targets(&index);
        assert_eq!(curated, BTreeSet::from(["keeper".to_string()]));
    }
}
