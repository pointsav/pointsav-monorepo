//! Parsing and validation for the two wiki-wide config files every content root carries
//! alongside its articles: `categories.yaml` (the category taxonomy) and `redirects.yaml`
//! (301 redirect rules). Part 4 items 1–2 of the BRIEF-tool-wiki plan: a consistency
//! validator (every category id resolves to real content; every redirect target is a real
//! page) and a redirect-chain flattener (the engine follows a `from` straight to its `to`
//! in one hop — it does not chase a `to` that is itself another rule's `from`).

use std::collections::{HashMap, HashSet};

use serde::Deserialize;

use crate::document::Document;

#[derive(Debug, Clone, Deserialize)]
pub struct RedirectRule {
    pub from: String,
    pub to: String,
}

#[derive(Debug, Clone, Deserialize)]
struct RedirectsFile {
    redirects: Vec<RedirectRule>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CategoryEntry {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
struct CategoriesFile {
    categories: Vec<CategoryEntry>,
}

#[derive(Debug)]
pub struct ParseError(pub String);

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::error::Error for ParseError {}

pub fn parse_redirects(yaml: &str) -> Result<Vec<RedirectRule>, ParseError> {
    let file: RedirectsFile =
        serde_yaml::from_str(yaml).map_err(|e| ParseError(e.to_string()))?;
    Ok(file.redirects)
}

pub fn parse_categories(yaml: &str) -> Result<Vec<CategoryEntry>, ParseError> {
    let file: CategoriesFile =
        serde_yaml::from_str(yaml).map_err(|e| ParseError(e.to_string()))?;
    Ok(file.categories)
}

/// A redirect whose `to` target is itself another rule's `from` — the engine only ever
/// follows one hop, so a visitor to `chain.from` gets a 301 to `chain.to`, and then has
/// to make a *second* request that redirects again. `final_to` is what `from` should point
/// at directly to collapse the chain to a single hop.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedirectChain {
    pub from: String,
    pub via: Vec<String>,
    pub final_to: String,
}

/// Find every redirect chain longer than one hop. Follows each rule's `to` through the
/// `from -> to` map until it lands on a target that isn't itself a redirect source, or
/// until a cycle is detected (reported with the cycle members in `via`, `final_to` equal
/// to `from` -- a real authoring bug, not a chain to flatten).
pub fn find_redirect_chains(redirects: &[RedirectRule]) -> Vec<RedirectChain> {
    let by_from: HashMap<&str, &str> = redirects
        .iter()
        .map(|r| (r.from.as_str(), r.to.as_str()))
        .collect();

    let mut chains = Vec::new();
    for rule in redirects {
        let mut via = Vec::new();
        let mut current = rule.to.as_str();
        let mut seen: HashSet<&str> = HashSet::new();
        seen.insert(rule.from.as_str());
        let mut cycle = false;
        while let Some(&next) = by_from.get(current) {
            if !seen.insert(current) {
                // Cycle: current has already been visited in this walk.
                via.push(current.to_string());
                cycle = true;
                break;
            }
            via.push(current.to_string());
            current = next;
        }
        if via.is_empty() {
            continue;
        }
        let final_to = if cycle { rule.from.clone() } else { current.to_string() };
        chains.push(RedirectChain { from: rule.from.clone(), via, final_to });
    }
    chains
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeadRedirect {
    pub from: String,
    pub to: String,
    pub reason: String,
}

/// Find every redirect whose `to` target doesn't resolve to real content: a `/wiki/<slug>`
/// target whose slug isn't among `known_slugs`, or a `/category/<id>` target whose id isn't
/// among `known_category_ids`. Anything else (an external URL, a bare `/`) is out of scope
/// and not reported.
pub fn find_dead_redirects(
    redirects: &[RedirectRule],
    known_slugs: &HashSet<String>,
    known_category_ids: &HashSet<String>,
) -> Vec<DeadRedirect> {
    let mut dead = Vec::new();
    for r in redirects {
        if let Some(slug) = r.to.strip_prefix("/wiki/") {
            if !known_slugs.contains(slug) {
                dead.push(DeadRedirect {
                    from: r.from.clone(),
                    to: r.to.clone(),
                    reason: format!("no article with slug '{slug}'"),
                });
            }
        } else if let Some(id) = r.to.strip_prefix("/category/") {
            if !known_category_ids.contains(id) {
                dead.push(DeadRedirect {
                    from: r.from.clone(),
                    to: r.to.clone(),
                    reason: format!("no category with id '{id}'"),
                });
            }
        }
    }
    dead
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrphanCategory {
    pub id: String,
    pub reason: String,
}

/// Find every `categories.yaml` entry with no real content: no document anywhere in the
/// crawl carries `category: <id>` in its frontmatter. A category that exists only in the
/// taxonomy file renders as an empty shelf on the live site.
pub fn find_orphan_categories(
    categories: &[CategoryEntry],
    document_categories: &HashSet<String>,
) -> Vec<OrphanCategory> {
    categories
        .iter()
        .filter(|c| !document_categories.contains(&c.id))
        .map(|c| OrphanCategory {
            id: c.id.clone(),
            reason: "no document's frontmatter `category:` references this id".to_string(),
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnknownCategory {
    /// Path (relative to the wiki content root) of the document carrying the unknown
    /// category value.
    pub path: String,
    pub category: String,
}

/// Reserved `category:` values the render engine deliberately excludes from category
/// counts and breadcrumbs -- they are sentinels meaning "this page hangs off the site
/// root", not taxonomy ids, and no wiki's `categories.yaml` declares them (nor should it).
///
/// Verified against the live engine source, which applies exactly these two exclusions in
/// both places a category id is consumed:
///   * `app-mediakit-knowledge/src/content/walk.rs:80` -- `category_counts()`:
///     `if !cat.is_empty() && cat != "root"`
///   * `app-mediakit-knowledge/src/app.rs:721` -- breadcrumb trail:
///     `.filter(|c| !c.is_empty() && *c != "root")`
///
/// Used identically across all three wikis for site-root pages (`index.md`,
/// `page-privacy.md`, `page-disclaimer.md`, `about.md`, `contact.md`,
/// `important-information.md`, `CONTRIBUTING.md`, `SECURITY.md`). Treating `root` as an
/// unknown category produced 24 false positives across the three wikis (10 documentation,
/// 14 corporate) -- every one a correctly-authored site-root page.
pub const RESERVED_CATEGORY_IDS: &[&str] = &["", "root"];

/// True when `category` is a reserved engine sentinel rather than a taxonomy id.
pub fn is_reserved_category(category: &str) -> bool {
    RESERVED_CATEGORY_IDS.contains(&category)
}

/// The inverse of [`find_orphan_categories`]: find every document whose frontmatter
/// `category:` value does NOT exist as an `id:` in `categories.yaml` -- a category that was
/// renamed or retired in the taxonomy file without updating (or retiring) the articles that
/// still reference the old id, which renders as a 404 category link or an uncategorised
/// article on the live site rather than an empty shelf.
///
/// [`RESERVED_CATEGORY_IDS`] are excluded: they are engine sentinels, not taxonomy ids, and
/// a `categories.yaml` entry for one would itself be the defect.
pub fn find_unknown_categories(
    documents: &[Document],
    known_category_ids: &HashSet<String>,
) -> Vec<UnknownCategory> {
    documents
        .iter()
        .filter_map(|d| {
            let category = d.frontmatter.category()?;
            if is_reserved_category(category) || known_category_ids.contains(category) {
                None
            } else {
                Some(UnknownCategory {
                    path: d.path.display().to_string(),
                    category: category.to_string(),
                })
            }
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CategoryTitleDrift {
    pub category_id: String,
    pub categories_yaml_name: String,
    /// `None` when no English `_index.md` was found for this category at all -- a
    /// different, more serious defect than a title mismatch, reported the same way for
    /// visibility.
    pub index_title: Option<String>,
    pub index_path: Option<String>,
}

/// Compare every `categories.yaml` entry's `name:` field against its English `_index.md`'s
/// `title:` frontmatter. This is the exact defect class found by hand twice in one session
/// (BRIEF-tool-wiki-core.md) -- a category's display name changed in one file but not the
/// other. Only the bare (English) `_index.md` is matched here, never `_index.es.md` -- the
/// Spanish pair's own EN/ES-identity check is a separate, cheaper comparison a caller can
/// do directly against two `Document`s without needing a dedicated crate function.
pub fn find_category_title_drift(
    categories: &[CategoryEntry],
    documents: &[Document],
) -> Vec<CategoryTitleDrift> {
    categories
        .iter()
        .filter_map(|c| {
            let index_doc = documents.iter().find(|d| {
                d.is_index_page()
                    && d.frontmatter.category() == Some(c.id.as_str())
                    && !d.path.to_string_lossy().ends_with(".es.md")
            });
            match index_doc {
                Some(doc) => {
                    let title = doc.frontmatter.title();
                    if title == Some(c.name.as_str()) {
                        None
                    } else {
                        Some(CategoryTitleDrift {
                            category_id: c.id.clone(),
                            categories_yaml_name: c.name.clone(),
                            index_title: title.map(str::to_string),
                            index_path: Some(doc.path.display().to_string()),
                        })
                    }
                }
                None => Some(CategoryTitleDrift {
                    category_id: c.id.clone(),
                    categories_yaml_name: c.name.clone(),
                    index_title: None,
                    index_path: None,
                }),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn parses_real_redirects_shape() {
        let yaml = "redirects:\n  - from: /co-location/power-centres\n    to: /wiki/power-centres\n  - from: /old/gis-page\n    to: /category/gis\n";
        let rules = parse_redirects(yaml).unwrap();
        assert_eq!(rules.len(), 2);
        assert_eq!(rules[0].from, "/co-location/power-centres");
        assert_eq!(rules[1].to, "/category/gis");
    }

    #[test]
    fn parses_real_categories_shape() {
        let yaml = "categories:\n  - id: buildings\n    name: \"Development Classes\"\n    scope: >-\n      What we build.\n";
        let cats = parse_categories(yaml).unwrap();
        assert_eq!(cats.len(), 1);
        assert_eq!(cats[0].id, "buildings");
        assert_eq!(cats[0].name, "Development Classes");
    }

    #[test]
    fn detects_a_two_hop_chain() {
        let rules = vec![
            RedirectRule { from: "/a".into(), to: "/b".into() },
            RedirectRule { from: "/b".into(), to: "/wiki/final".into() },
        ];
        let chains = find_redirect_chains(&rules);
        // /a -> /b -> /wiki/final is a real 2-hop chain; /b -> /wiki/final is a single hop.
        assert_eq!(chains.len(), 1);
        assert_eq!(chains[0].from, "/a");
        assert_eq!(chains[0].via, vec!["/b".to_string()]);
        assert_eq!(chains[0].final_to, "/wiki/final");
    }

    #[test]
    fn single_hop_redirects_are_not_reported_as_chains() {
        let rules = vec![RedirectRule { from: "/a".into(), to: "/wiki/final".into() }];
        assert!(find_redirect_chains(&rules).is_empty());
    }

    #[test]
    fn detects_a_redirect_cycle_without_infinite_looping() {
        let rules = vec![
            RedirectRule { from: "/a".into(), to: "/b".into() },
            RedirectRule { from: "/b".into(), to: "/a".into() },
        ];
        let chains = find_redirect_chains(&rules);
        // Must terminate (the real bug this test guards against is an infinite loop) and
        // report something -- exact shape less important than "does not hang".
        assert!(!chains.is_empty());
    }

    #[test]
    fn finds_dead_wiki_and_category_redirects() {
        let rules = vec![
            RedirectRule { from: "/a".into(), to: "/wiki/real-slug".into() },
            RedirectRule { from: "/b".into(), to: "/wiki/ghost-slug".into() },
            RedirectRule { from: "/c".into(), to: "/category/ghost-category".into() },
        ];
        let mut slugs = HashSet::new();
        slugs.insert("real-slug".to_string());
        let mut cats = HashSet::new();
        cats.insert("buildings".to_string());

        let dead = find_dead_redirects(&rules, &slugs, &cats);
        assert_eq!(dead.len(), 2);
        assert!(dead.iter().any(|d| d.to == "/wiki/ghost-slug"));
        assert!(dead.iter().any(|d| d.to == "/category/ghost-category"));
    }

    #[test]
    fn finds_orphan_categories() {
        let categories = vec![
            CategoryEntry { id: "buildings".into(), name: "Development Classes".into() },
            CategoryEntry { id: "ghost".into(), name: "Nothing Here".into() },
        ];
        let mut doc_cats = HashSet::new();
        doc_cats.insert("buildings".to_string());

        let orphans = find_orphan_categories(&categories, &doc_cats);
        assert_eq!(orphans.len(), 1);
        assert_eq!(orphans[0].id, "ghost");
    }

    fn doc(path: &str, content: &str) -> Document {
        Document::parse(PathBuf::from(path), content).unwrap()
    }

    #[test]
    fn finds_unknown_categories() {
        let docs = vec![
            doc(
                "buildings/a.md",
                "---\nslug: a\ncategory: buildings\n---\n\nBody.\n",
            ),
            doc(
                "buildings/b.md",
                "---\nslug: b\ncategory: retired-category\n---\n\nBody.\n",
            ),
        ];
        let mut known = HashSet::new();
        known.insert("buildings".to_string());

        let unknown = find_unknown_categories(&docs, &known);
        assert_eq!(unknown.len(), 1);
        assert_eq!(unknown[0].category, "retired-category");
        assert_eq!(unknown[0].path, "buildings/b.md");
    }

    #[test]
    fn reserved_root_sentinel_is_not_an_unknown_category() {
        // Real shape: every site-root page across all three wikis carries `category: root`,
        // which no categories.yaml declares. The engine excludes it by design
        // (walk.rs:80, app.rs:721) -- so must this check.
        let docs = vec![
            doc("index.md", "---\nslug: index\ncategory: root\n---\n\nBody.\n"),
            doc("page-privacy.md", "---\nslug: page-privacy\ncategory: root\n---\n\nBody.\n"),
            doc("CONTRIBUTING.md", "---\nslug: contributing\ncategory: root\n---\n\nBody.\n"),
        ];
        let mut known = HashSet::new();
        known.insert("architecture".to_string());
        assert!(find_unknown_categories(&docs, &known).is_empty());
    }

    #[test]
    fn empty_category_is_also_reserved() {
        let docs = vec![doc("x.md", "---\nslug: x\ncategory: \"\"\n---\n\nBody.\n")];
        let known = HashSet::new();
        assert!(find_unknown_categories(&docs, &known).is_empty());
    }

    #[test]
    fn reserved_sentinels_do_not_mask_a_real_unknown_in_the_same_crawl() {
        let docs = vec![
            doc("index.md", "---\nslug: index\ncategory: root\n---\n\nBody.\n"),
            doc("a/b.md", "---\nslug: b\ncategory: retired-category\n---\n\nBody.\n"),
        ];
        let mut known = HashSet::new();
        known.insert("a".to_string());
        let unknown = find_unknown_categories(&docs, &known);
        assert_eq!(unknown.len(), 1);
        assert_eq!(unknown[0].category, "retired-category");
    }

    #[test]
    fn reserved_set_matches_the_engines_two_exclusions() {
        assert_eq!(RESERVED_CATEGORY_IDS, &["", "root"]);
        assert!(is_reserved_category("root"));
        assert!(is_reserved_category(""));
        assert!(!is_reserved_category("architecture"));
    }

    #[test]
    fn known_category_reports_no_unknowns() {
        let docs = vec![doc(
            "buildings/a.md",
            "---\nslug: a\ncategory: buildings\n---\n\nBody.\n",
        )];
        let mut known = HashSet::new();
        known.insert("buildings".to_string());
        assert!(find_unknown_categories(&docs, &known).is_empty());
    }

    #[test]
    fn detects_category_title_drift() {
        let categories = vec![CategoryEntry {
            id: "buildings".into(),
            name: "Development Classes".into(),
        }];
        let docs = vec![doc(
            "buildings/_index.md",
            "---\ntitle: \"Stale Title\"\nslug: buildings-index\ncategory: buildings\ncontent_type: topic\n---\n\nBody.\n",
        )];
        let drift = find_category_title_drift(&categories, &docs);
        assert_eq!(drift.len(), 1);
        assert_eq!(drift[0].category_id, "buildings");
        assert_eq!(drift[0].index_title.as_deref(), Some("Stale Title"));
    }

    #[test]
    fn matching_index_title_reports_no_drift() {
        let categories = vec![CategoryEntry {
            id: "buildings".into(),
            name: "Development Classes".into(),
        }];
        let docs = vec![doc(
            "buildings/_index.md",
            "---\ntitle: \"Development Classes\"\nslug: buildings-index\ncategory: buildings\ncontent_type: topic\n---\n\nBody.\n",
        )];
        assert!(find_category_title_drift(&categories, &docs).is_empty());
    }

    #[test]
    fn missing_index_page_is_reported() {
        let categories = vec![CategoryEntry {
            id: "ghost".into(),
            name: "Nothing Here".into(),
        }];
        let drift = find_category_title_drift(&categories, &[]);
        assert_eq!(drift.len(), 1);
        assert_eq!(drift[0].index_title, None);
    }

    #[test]
    fn spanish_index_is_never_matched_as_the_english_one() {
        let categories = vec![CategoryEntry {
            id: "buildings".into(),
            name: "Development Classes".into(),
        }];
        let docs = vec![doc(
            "buildings/_index.es.md",
            "---\ntitle: \"Clases de Desarrollo\"\nslug: buildings-index\ncategory: buildings\ncontent_type: topic\n---\n\nBody.\n",
        )];
        // No English _index.md exists -- must report "missing", not match the Spanish one.
        let drift = find_category_title_drift(&categories, &docs);
        assert_eq!(drift.len(), 1);
        assert_eq!(drift[0].index_title, None);
    }
}
