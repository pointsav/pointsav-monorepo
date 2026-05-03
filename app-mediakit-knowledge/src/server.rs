//! HTTP server and route handlers.
//!
//! Phase 1.1 additions (all additive — no existing routes or responses changed):
//! - `wiki_chrome()` — full article-page shell with Wikipedia muscle-memory chrome
//! - Article / Talk tabs (top-left)
//! - Read / Edit / View history tabs (top-right; Edit + View-history are href="#" placeholders)
//! - IVC masthead band placeholder ("verification not yet available — Phase 7")
//! - Collapsible left-rail TOC (pure CSS + minimal JS; JS loaded from /static/wiki.js)
//! - Language-switcher button (populated from frontmatter `translations:`)
//! - Hatnote (italic, indented; rendered when frontmatter has a `hatnote:` field)
//! - "From PointSav Knowledge" tagline below the page title
//! - Reader density toggle (Off / Exceptions only / All; persisted to localStorage)
//! - Per-section [edit] pencils (injected by render::inject_edit_pencils)
//! - Footer convention (categories → license → about/contact links)
//! - The existing `chrome()` function is retained for the index page.
//!
//! Iteration-2 additions (all additive — no existing behaviour changed):
//! - Recursive content-directory walk: `collect_topic_files()` descends into
//!   category subdirectories (`architecture/`, `services/`, etc.) so that all
//!   130+ TOPICs are visible to the bucketing, featured-pin, and slug-resolution
//!   logic. Slugs for subdirectory TOPICs take the form `<category>/<stem>`.
//! - `short_description` subtitle: rendered as `<p class="topic-short-description">
//!   <em>…</em></p>` immediately below the article H1.
//! - Leapfrog 2030 facts panel: reads `leapfrog-facts.yaml` from `content_dir`;
//!   renders a "Leapfrog 2030" bullet panel on the home page right column.
//! - Breadcrumb navigation: `category:` frontmatter → "Documentation > Category > Title"
//!   breadcrumb rendered above the article TOC rail.
//! - Language toggle auto-detection (Item 11): `wiki_page()` checks for a `.es.md`
//!   sibling on disk and auto-injects the EN↔ES toggle without requiring explicit
//!   `translations:` frontmatter. EN articles get an "es" link; ES articles (`*.es`)
//!   get an "en" link back to the base slug. Explicit `translations:` frontmatter
//!   takes precedence over auto-detection when present and non-empty.

use axum::{
    extract::{Path, Query, State},
    http::{header, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use maud::{html, Markup, PreEscaped, DOCTYPE};
use serde::Deserialize;
use serde_json::json;
use std::collections::BTreeMap;
use std::path::{Path as FsPath, PathBuf};
use std::sync::{Arc, Mutex};
use tokio::fs;

use crate::assets::StaticAsset;
use crate::collab::CollabRooms;
use crate::error::WikiError;
use crate::jsonld::jsonld_for_topic;
use crate::render::{extract_headings, inject_edit_pencils, parse_page, render_html_raw, Frontmatter, TranslationMap};
use crate::search::{search as run_search, SearchIndex};

#[derive(Clone)]
pub struct AppState {
    pub content_dir: PathBuf,
    /// Optional extra directory of GUIDE-* Markdown files (e.g. a fleet-deployment
    /// repo). When set, the engine walks this dir alongside `content_dir` and
    /// serves files at `/wiki/<slug>` just like TOPICs.
    /// Set via `--guide-dir` / `WIKI_GUIDE_DIR`.
    pub guide_dir: Option<PathBuf>,
    /// Second optional guide directory. Allows a documentation wiki to serve
    /// guides from two separate fleet-deployment repos simultaneously.
    /// Set via `--guide-dir-2` / `WIKI_GUIDE_DIR_2`.
    pub guide_dir_2: Option<PathBuf>,
    /// Path to the workspace citation registry YAML file.
    /// Defaults to `/srv/foundry/citations.yaml`; overridable via
    /// `--citations-yaml` / `WIKI_CITATIONS_YAML`.
    pub citations_yaml: PathBuf,
    /// Display name for this wiki instance, shown in the browser tab, site
    /// header, and home-page H1 fallback. Set via `--site-title` /
    /// `WIKI_SITE_TITLE`. Default: `"PointSav Documentation Wiki"`.
    pub site_title: String,
    /// Phase 3 Step 3.2: tantivy full-text search index. Built on
    /// startup from a tree walk of `content_dir`; reindexed on every
    /// successful edit / create. Clone-cheap (Arc-wrapped internals).
    pub search: Arc<SearchIndex>,
    /// Phase 4 Step 4.1: git2 repository for content versioning. Mutex-wrapped
    /// because Repository is not thread-safe for mutating operations.
    pub git: Arc<Mutex<git2::Repository>>,
    /// Phase 2 Step 7: per-slug collab WebSocket rooms. Always present
    /// (cheap empty default); only the `/ws/collab/{slug}` route uses
    /// it, and that route is only mounted when `enable_collab` is true.
    pub collab: Arc<CollabRooms>,
    /// Phase 2 Step 7: when true, mount the `/ws/collab/{slug}` route
    /// and template `window.WIKI_COLLAB_ENABLED = true` into the editor
    /// page so `saa-init.js` lazy-loads the collab JS bundle.
    pub enable_collab: bool,
}

pub fn router(state: AppState) -> Router {
    let collab_enabled = state.enable_collab;
    let mut r = Router::new()
        .route("/", get(index))
        // Wildcard capture allows category-scoped slugs: `/wiki/architecture/compounding-substrate`
        .route("/wiki/{*slug}", get(wiki_page))
        .route("/static/{*path}", get(static_asset))
        .route("/healthz", get(healthz))
        // Phase 2 Step 2 — edit endpoint
        .route(
            "/edit/{slug}",
            get(crate::edit::get_edit).post(crate::edit::post_edit),
        )
        .route("/create", post(crate::edit::post_create))
        // Phase 2 Step 4 — SAA squiggle rules (deterministic; Phase 9 CCA
        // adds dynamic per-jurisdiction packs)
        .route(
            "/api/squiggle-rules",
            get(crate::squiggle::get_squiggle_rules),
        )
        // Phase 2 Step 5 — citation registry for autocomplete
        .route("/api/citations", get(crate::citations::get_citations))
        // Phase 2 Step 6 — three-keystroke ladder stubs (501 until Phase 4
        // wires the Doorman MCP server)
        .route("/api/doorman/complete", post(doorman_stub))
        .route("/api/doorman/instruct", post(doorman_stub))
        // Wave 5B — category listing pages
        .route("/category/{name}", get(category_page))
        // Phase 3 Step 3.2 — full-text search HTML page over the tantivy index
        .route("/search", get(search_page))
        // Phase 3 Step 3.3 — Atom + JSON Feed syndication
        .route("/feed.atom", get(crate::feeds::get_atom))
        .route("/feed.json", get(crate::feeds::get_json_feed))
        // Phase 3 Step 3.4 — crawler discovery + raw Markdown source
        .route("/sitemap.xml", get(sitemap_xml))
        .route("/robots.txt", get(robots_txt))
        .route("/llms.txt", get(llms_txt))
        // Phase 4 Step 4.2 — history and blame
        .route("/history/{*slug}", get(history_page))
        .route("/blame/{*slug}", get(blame_page))
        // axum 0.8 doesn't allow a literal `.md` suffix after a dynamic
        // segment, so the route captures `{slug}` as a single segment and
        // the handler strips an optional trailing `.md` for the
        // git-clone-style UX (`/git/topic-foo.md` or `/git/topic-foo`).
        .route("/git/{slug}", get(git_markdown));
    // Phase 2 Step 7 — collab WebSocket relay; only mounted when the CLI
    // flag is set (default off — production deploys without --enable-collab
    // never expose the route).
    if collab_enabled {
        r = r.route("/ws/collab/{slug}", get(crate::collab::ws_collab));
    }
    r.with_state(Arc::new(state))
}

async fn healthz() -> &'static str {
    "ok"
}

#[derive(Deserialize)]
struct SearchQueryParams {
    #[serde(default)]
    q: String,
}

/// Phase 3 Step 3.2 — `GET /search?q=...` HTML results page.
///
/// Empty query → empty results + the search form. Renders within the
/// existing `chrome()` shell for layout consistency with the index page.
/// Phase 3.x may upgrade to autocomplete + image previews per UX-DESIGN.md
/// §1 item 13; Phase 3.2 ships the basic form + ranked result list.
async fn search_page(
    State(state): State<Arc<AppState>>,
    Query(params): Query<SearchQueryParams>,
) -> Result<Markup, WikiError> {
    let query = params.q.trim().to_string();
    let hits = if query.is_empty() {
        Vec::new()
    } else {
        run_search(&state.search, &query, 25)?
    };

    Ok(chrome(
        if query.is_empty() {
            "Search".to_string()
        } else {
            format!("Search: {query}")
        }
        .as_str(),
        html! {
            h1 { "Search" }
            form.search-form action="/search" method="get" {
                input
                    type="search"
                    name="q"
                    value=(query)
                    placeholder="Search TOPICs"
                    autocomplete="off"
                    autofocus?[query.is_empty()];
                button type="submit" { "Search" }
            }
            @if !query.is_empty() {
                @if hits.is_empty() {
                    p.search-empty {
                        "No results for "
                        em { (query) }
                        "."
                    }
                } @else {
                    p.search-summary {
                        (hits.len())
                        " result" @if hits.len() != 1 { "s" }
                        " for "
                        em { (query) }
                        "."
                    }
                    ol.search-results {
                        @for hit in &hits {
                            li.search-hit {
                                a.search-hit-title href={ "/wiki/" (hit.slug) } {
                                    (hit.title)
                                }
                                span.search-hit-slug { (hit.slug) }
                                @if !hit.snippet.is_empty() {
                                    p.search-hit-snippet { (hit.snippet) }
                                }
                            }
                        }
                    }
                }
            }
        },
        &state.site_title,
    ))
}

/// Stub handler for Phase 2 Step 6 Doorman endpoints.
///
/// Returns `501 Not Implemented` with a JSON body indicating that the full
/// Doorman MCP integration lands in Phase 4. The client-side handlers in
/// `saa-init.js` check for this status and display a one-time toast rather
/// than treating it as an error.
async fn doorman_stub() -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(json!({
            "phase": 4,
            "reason": "Doorman MCP integration deferred to Phase 4"
        })),
    )
}

// ─── Home-page data types ───────────────────────────────────────────────────

/// Summary of a single TOPIC used in the home-page category panels and
/// recent-additions feed.
#[derive(Debug, Clone)]
pub struct TopicSummary {
    /// Slug (filename without `.md`; category-scoped for subdirectory files).
    pub slug: String,
    /// Title from frontmatter, or the slug when absent.
    pub title: String,
    /// `last_edited:` frontmatter value; may be None if not set.
    pub last_edited: Option<String>,
    /// `short_description` from frontmatter; may be None if not set.
    pub short_description: Option<String>,
    /// First non-blank, non-heading line of the body Markdown.
    pub lede_first_line: String,
    /// Absolute path to the source file on disk (used for git fallback).
    pub file_path: PathBuf,
}

/// Wikipedia-style stats banner shown immediately under the welcome lede.
/// Renders as: "N articles across N categories — last updated YYYY-MM-DD."
/// Per `content-wiki-documentation/index.md` ENGINE comment + iteration-2
/// home-page leapfrog primitive (Wikipedia welcome-banner pattern preserved).
#[derive(Debug, Clone)]
pub struct HomeStats {
    pub article_count: usize,
    pub category_count: usize,
    pub last_updated: Option<String>,
}

/// Category buckets: `BTreeMap<category_name, Vec<TopicSummary>>`.
pub type CategoryBuckets = BTreeMap<String, Vec<TopicSummary>>;

/// Ratified category set in render order.
/// Per naming-convention.md §10 Q5-A operator ratification 2026-04-28.
const RATIFIED_CATEGORIES: &[&str] = &[
    "architecture",
    "services",
    "systems",
    "applications",
    "governance",
    "infrastructure",
    "company",
    "reference",
    "help",
];

// ─── Home-page helpers ──────────────────────────────────────────────────────

/// A single topic file discovered during a recursive walk of `content_dir`.
///
/// `slug` is the routing slug used in `/wiki/<slug>` URLs. For files
/// directly in `content_dir` the slug equals the filename stem (e.g.,
/// `topic-hello`). For files in a subdirectory the slug is
/// `<subdir>/<stem>` (e.g., `architecture/compounding-substrate`).
struct TopicFile {
    slug: String,
    path: PathBuf,
}

/// Repo-management files that are not wiki content. Filtered out at the
/// root level of any content directory so they never appear in article
/// listings or the "All articles" catch-all section.
const SYSTEM_FILE_STEMS: &[&str] = &[
    "README",
    "CHANGELOG",
    "MANIFEST",
    "CLAUDE",
    "NEXT",
    "NOTAM",
    "TRADEMARK",
    "CODE_OF_CONDUCT",
    "BUDGET",
    "DOCTRINE",
    "LICENSE",
    "CONTRIBUTING",
    "SECURITY",
];

/// Recursively collect all TOPIC `.md` files under `content_dir`.
///
/// Skips:
/// - `*.es.md` bilingual siblings
/// - `index.md` and `_index.md` at any level
/// - Files whose stem starts with `_`
/// - Repo-management files listed in `SYSTEM_FILE_STEMS`
/// - Non-`.md` files
///
/// Descends one level into subdirectories (category folders). Does not
/// recurse further — the content tree is `<content_dir>/<category>/<slug>.md`.
async fn collect_topic_files(content_dir: &FsPath) -> std::io::Result<Vec<TopicFile>> {
    let mut out = Vec::new();
    let mut entries = fs::read_dir(content_dir).await?;

    while let Some(entry) = entries.next_entry().await? {
        let file_type = entry.file_type().await?;
        let name = entry.file_name();
        let name_str = name.to_string_lossy().to_string();

        if file_type.is_dir() {
            // Descend into category subdirectory.
            let subdir_name = name_str.clone();
            let mut sub_entries = match fs::read_dir(entry.path()).await {
                Ok(e) => e,
                Err(_) => continue,
            };
            while let Some(sub_entry) = sub_entries.next_entry().await? {
                let sub_name = sub_entry.file_name();
                let sub_str = sub_name.to_string_lossy().to_string();
                let stem = match sub_str.strip_suffix(".md") {
                    Some(s) => s.to_string(),
                    None => continue,
                };
                if stem.ends_with(".es")
                    || stem == "index"
                    || stem == "_index"
                    || stem.starts_with('_')
                    || SYSTEM_FILE_STEMS.contains(&stem.as_str())
                {
                    continue;
                }
                out.push(TopicFile {
                    slug: format!("{subdir_name}/{stem}"),
                    path: sub_entry.path(),
                });
            }
        } else {
            // File at root level.
            let stem = match name_str.strip_suffix(".md") {
                Some(s) => s.to_string(),
                None => continue,
            };
            if stem.ends_with(".es")
                || stem == "index"
                || stem == "_index"
                || stem.starts_with('_')
                || SYSTEM_FILE_STEMS.contains(&stem.as_str())
            {
                continue;
            }
            out.push(TopicFile {
                slug: stem,
                path: entry.path(),
            });
        }
    }

    Ok(out)
}

/// Collect topic files from `content_dir` and zero or more `guide_dirs`.
/// Slugs are unique within each dir; guide slugs are prefixed by their subdir
/// name so they don't collide with content slugs.
async fn collect_all_topic_files(
    content_dir: &FsPath,
    guide_dirs: &[Option<&FsPath>],
) -> std::io::Result<Vec<TopicFile>> {
    let mut files = collect_topic_files(content_dir).await?;
    for gd_opt in guide_dirs {
        if let Some(gd) = gd_opt {
            if gd.is_dir() {
                if let Ok(guide_files) = collect_topic_files(gd).await {
                    files.extend(guide_files);
                }
            }
        }
    }
    Ok(files)
}

/// Walk `content_dir` (and optional `guide_dir`) recursively, parse every `.md`
/// file, and group them into a `BTreeMap<category, Vec<TopicSummary>>`.
///
/// Descends into category subdirectories (`architecture/`, `services/`, etc.).
/// Slugs for subdirectory TOPICs take the form `<category>/<stem>` so they
/// resolve correctly in `/wiki/<slug>` routes.
///
/// Files with no `category:` frontmatter, or whose category is `root`, are
/// bucketed under `"uncategorised"`.
async fn bucket_topics_by_category(
    content_dir: &FsPath,
    guide_dir: Option<&FsPath>,
    guide_dir_2: Option<&FsPath>,
) -> std::io::Result<CategoryBuckets> {
    let topic_files = collect_all_topic_files(content_dir, &[guide_dir, guide_dir_2]).await?;
    let mut buckets: CategoryBuckets = BTreeMap::new();

    for tf in topic_files {
        let text = match fs::read_to_string(&tf.path).await {
            Ok(t) => t,
            Err(_) => continue,
        };

        let parsed = match crate::render::parse_page(&text) {
            Ok(p) => p,
            Err(_) => continue,
        };

        let title = parsed
            .frontmatter
            .title
            .clone()
            .unwrap_or_else(|| tf.slug.clone());

        // Category: prefer frontmatter `category:`, fall back to the
        // subdirectory name extracted from the slug.
        let category = match parsed.frontmatter.category.as_deref() {
            None | Some("root") | Some("") => {
                // Infer from slug prefix if file is in a subdirectory.
                if let Some(slash) = tf.slug.find('/') {
                    tf.slug[..slash].to_string()
                } else {
                    "uncategorised".to_string()
                }
            }
            Some(c) => c.to_string(),
        };

        let lede_first_line = first_body_line(&parsed.body_md);
        let last_edited = parsed.frontmatter.last_edited.clone();
        let short_description = parsed.frontmatter.short_description.clone();

        let summary = TopicSummary {
            slug: tf.slug,
            title,
            last_edited,
            short_description,
            lede_first_line,
            file_path: tf.path,
        };

        buckets.entry(category).or_default().push(summary);
    }

    // Sort each bucket by slug for deterministic output.
    for topics in buckets.values_mut() {
        topics.sort_by(|a, b| a.slug.cmp(&b.slug));
    }

    Ok(buckets)
}

/// Extract a lede from the first non-blank, non-heading Markdown line.
fn first_body_line(body_md: &str) -> String {
    body_md
        .lines()
        .find(|l| {
            let t = l.trim();
            !t.is_empty() && !t.starts_with('#') && !t.starts_with("---")
        })
        .map(|l| l.trim().to_string())
        .unwrap_or_default()
}

/// Flatten all buckets, sort by `last_edited` descending (filename ascending
/// as tiebreaker), and return the top `n` entries.
///
/// Topics with `last_edited: None` fall back to git-commit-date via
/// `git log -1 --format=%cI -- <path>`. If that fails, falls back to
/// filesystem mtime. Topics that cannot produce any date sort last.
fn recent_topics_by_last_edited(buckets: &CategoryBuckets, n: usize) -> Vec<TopicSummary> {
    let mut all: Vec<TopicSummary> = buckets.values().flatten().cloned().collect();

    // Resolve a sort key for each entry: prefer `last_edited`, then git, then mtime.
    // We use a String key so ISO-8601 lexicographic order == chronological order.
    let key_for = |t: &TopicSummary| -> String {
        if let Some(ref d) = t.last_edited {
            return d.clone();
        }
        // Try git commit date.
        if let Ok(output) = std::process::Command::new("git")
            .args(["log", "-1", "--format=%cI", "--", t.file_path.to_str().unwrap_or("")])
            .output()
        {
            let s = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !s.is_empty() {
                return s;
            }
        }
        // Fall back to filesystem mtime.
        if let Ok(meta) = std::fs::metadata(&t.file_path) {
            if let Ok(modified) = meta.modified() {
                // Convert to a rough ISO string for comparison.
                let dur = modified
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default();
                return format!("{}", dur.as_secs());
            }
        }
        String::new()
    };

    all.sort_by(|a, b| {
        let ka = key_for(a);
        let kb = key_for(b);
        // Descending by date, ascending by slug as tiebreaker.
        kb.cmp(&ka).then_with(|| a.slug.cmp(&b.slug))
    });

    all.truncate(n);
    all
}

/// Read and validate `<content_dir>/featured-topic.yaml`.
///
/// Returns `None` silently if the file is absent. Logs a warning via
/// `tracing::warn!` if the file is present but unparseable or if the slug
/// cannot be found in `buckets`.
/// Compute home-page stats banner contents.
///
/// `article_count` is the total number of bucketed topics across all
/// categories (excludes `index.md`, `_index.md`, and `*.es.md` siblings,
/// matching `bucket_topics_by_category()` discipline).
///
/// `category_count` is `RATIFIED_CATEGORIES.len()` — always 9, signalling
/// the platform's intended scope rather than only categories with
/// articles.
///
/// `last_updated` is the maximum `last_edited:` ISO-8601 string across
/// all bucketed topics. Returns `None` if no topic carries the field
/// (the banner suppresses the date in that case rather than rendering an
/// empty value).
fn compute_home_stats(buckets: &CategoryBuckets) -> HomeStats {
    let article_count: usize = buckets.values().map(|v| v.len()).sum();
    let last_updated = buckets
        .values()
        .flatten()
        .filter_map(|t| t.last_edited.as_deref())
        .max()
        .map(|s| s.to_string());
    HomeStats {
        article_count,
        category_count: RATIFIED_CATEGORIES.len(),
        last_updated,
    }
}

// ─── Home-page chrome ───────────────────────────────────────────────────────

/// Render the home-page shell.
///
/// Structure:
/// - Site header (reuses `chrome()` pattern)
/// - Lede (rendered body Markdown from `index.md`)
/// - Stats banner ("N articles across N categories — last updated YYYY-MM-DD.")
/// - Two-column main panel:
///   - Left: Optional featured TOPIC panel
///   - Right: Optional Leapfrog 2030 inventions bullet panel
/// - By-category 3×3 grid (all 9 ratified categories; empty ones show
///   "0 articles — in preparation")
/// - Recent additions feed (top 5, sorted by `last_edited` descending)
/// - Site footer with bilingual notice
/// How many articles to preview per category before showing "All N →".
/// Categories with ≤ PREVIEW_LIMIT articles always show the full list.
const PREVIEW_LIMIT: usize = 8;

// KEY_GUIDES removed — guides now served in-wiki via guide_dir and appear in
// the Help category grid alongside TOPICs. No external GitHub links on home page.
// Retained as dead code until woodfine-fleet-deployment is added to the cluster.
#[allow(dead_code)]
const KEY_GUIDES: &[(&str, &str, &str)] = &[
    // AI & SLM
    ("Operating the service-slm Doorman", "vault-privategit-source", "https://github.com/woodfine/woodfine-fleet-deployment/blob/main/vault-privategit-source/guide-doorman.md"),
    ("Operating the Yo-Yo Tier B Deployment", "vault-privategit-source", "https://github.com/woodfine/woodfine-fleet-deployment/blob/main/vault-privategit-source/guide-operating-yoyo.md"),
    ("Operating the Tier A Sysadmin TUI", "vault-privategit-source", "https://github.com/woodfine/woodfine-fleet-deployment/blob/main/vault-privategit-source/guide-tier-a-sysadmin-tui.md"),
    // Personnel cluster
    ("SLM Execution — Personnel Cluster", "cluster-totebox-personnel", "https://github.com/woodfine/woodfine-fleet-deployment/blob/main/cluster-totebox-personnel/guide-slm-execution.md"),
    ("Sovereign Search Operations", "cluster-totebox-personnel", "https://github.com/woodfine/woodfine-fleet-deployment/blob/main/cluster-totebox-personnel/guide-sovereign-search.md"),
    ("Microsoft Entra ID Sovereignty", "cluster-totebox-personnel", "https://github.com/woodfine/woodfine-fleet-deployment/blob/main/cluster-totebox-personnel/guide-msft-entra-id.md"),
    ("Ingress Operations & Self-Healing Loop", "cluster-totebox-personnel", "https://github.com/woodfine/woodfine-fleet-deployment/blob/main/cluster-totebox-personnel/guide-ingress-operations.md"),
    ("Totebox Orchestration & Autonomous Synthesis", "cluster-totebox-personnel", "https://github.com/woodfine/woodfine-fleet-deployment/blob/main/cluster-totebox-personnel/guide-totebox-orchestration.md"),
    ("LinkedIn Adapter (service-message-courier)", "cluster-totebox-personnel", "https://github.com/woodfine/woodfine-fleet-deployment/blob/main/cluster-totebox-personnel/guide-linkedin-adapter.md"),
    ("Physical Egress — Cold Storage Backup", "cluster-totebox-personnel", "https://github.com/woodfine/woodfine-fleet-deployment/blob/main/cluster-totebox-personnel/guide-cold-storage-sync.md"),
    ("Personnel Ledger Operations", "cluster-totebox-personnel", "https://github.com/woodfine/woodfine-fleet-deployment/blob/main/cluster-totebox-personnel/guide-personnel-ledger.md"),
    // Network & command
    ("PointSav Private Network Orchestration", "route-network-admin", "https://github.com/woodfine/woodfine-fleet-deployment/blob/main/route-network-admin/guide-mesh-orchestration.md"),
    ("Unified Command Ledger Operations", "node-console-operator", "https://github.com/woodfine/woodfine-fleet-deployment/blob/main/node-console-operator/guide-command-ledger.md"),
    // Infrastructure
    ("Bare-Metal Provisioning & Mesh Fusion", "fleet-infrastructure-onprem", "https://github.com/woodfine/woodfine-fleet-deployment/blob/main/fleet-infrastructure-onprem/guide-provision-onprem.md"),
    ("LXC Network Ledger Provisioning", "fleet-infrastructure-onprem", "https://github.com/woodfine/woodfine-fleet-deployment/blob/main/fleet-infrastructure-onprem/guide-lxc-network-admin.md"),
    ("PPN Cloud Anchor Ignition", "fleet-infrastructure-cloud", "https://github.com/woodfine/woodfine-fleet-deployment/blob/main/fleet-infrastructure-cloud/guide-provision-relay.md"),
    ("Sovereign VPN Deployment", "fleet-infrastructure-leased", "https://github.com/woodfine/woodfine-fleet-deployment/blob/main/fleet-infrastructure-leased/guide-deploy-vpn.md"),
    ("Standalone Bare-Metal Provisioning", "fleet-infrastructure-leased", "https://github.com/woodfine/woodfine-fleet-deployment/blob/main/fleet-infrastructure-leased/guide-provision-standalone.md"),
    // Fleet-wide operations
    ("Physical Egress — Regulatory Printing", "woodfine-fleet-deployment", "https://github.com/woodfine/woodfine-fleet-deployment/blob/main/guide-physical-egress.md"),
    ("Woodfine Telemetry Operations", "woodfine-fleet-deployment", "https://github.com/woodfine/woodfine-fleet-deployment/blob/main/guide-telemetry-operations.md"),
    ("Operating the Knowledge Wiki", "media-knowledge-documentation", "https://github.com/woodfine/woodfine-fleet-deployment/blob/main/media-knowledge-documentation/guide-operate-knowledge-wiki.md"),
    ("Telemetry Engine Integration", "media-marketing-landing", "https://github.com/pointsav/pointsav-fleet-deployment/blob/main/media-marketing-landing/guide-telemetry-integration.md"),
];

fn home_chrome(
    home_fm: &crate::render::Frontmatter,
    home_html: &str,
    buckets: &CategoryBuckets,
    recent: &[TopicSummary],
    stats: &HomeStats,
    guides: &[TopicSummary],
    site_title: &str,
) -> Markup {
    let title = home_fm.title.as_deref().unwrap_or(site_title);

    // Articles in non-ratified buckets (not already shown as guides) so that
    // every TOPIC and GUIDE is reachable from the home page.
    let guide_slug_set: std::collections::HashSet<&str> =
        guides.iter().map(|g| g.slug.as_str()).collect();
    let mut uncategorised: Vec<&TopicSummary> = buckets
        .iter()
        .filter(|(cat, _)| {
            !RATIFIED_CATEGORIES.contains(&cat.as_str()) && cat.as_str() != "root"
        })
        .flat_map(|(_, topics)| topics.iter())
        .filter(|t| !guide_slug_set.contains(t.slug.as_str()))
        .collect();
    uncategorised.sort_by(|a, b| a.title.cmp(&b.title));

    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { (site_title) }
                link rel="stylesheet" href="/static/style.css";
            }
            body {
                header.site-header {
                    a.site-title href="/" { (site_title) }
                    form.header-search action="/search" method="get" {
                        input type="search" name="q" placeholder="Search articles…" autocomplete="off";
                        button type="submit" { "Search" }
                    }
                    nav.site-nav {
                        a href="/" { "Home" }
                    }
                }
                main.site-main {
                    // Lede from index.md
                    @if !home_html.is_empty() {
                        div.wiki-home-lede { (PreEscaped(home_html)) }
                    }

                    // Stats banner
                    @if stats.article_count > 0 {
                        p.wiki-home-stats aria-label="Knowledge wiki scale" {
                            (stats.article_count) " article"
                            @if stats.article_count != 1 { "s" }
                            " across " (stats.category_count) " categories"
                            @if let Some(ref d) = stats.last_updated {
                                " — last updated " time datetime=(d) { (d) }
                            }
                            "."
                        }
                    }

                    // Category sections — show all articles for small categories,
                    // PREVIEW_LIMIT + "All N →" for larger ones.
                    div.wiki-home-categories {
                        @for cat in RATIFIED_CATEGORIES {
                            @let topics = buckets.get(*cat).map(|v| v.as_slice()).unwrap_or(&[]);
                            @let count = topics.len();
                            div.wiki-home-cat-section {
                                div.wiki-home-cat-section-head {
                                    h2 {
                                        a href={ "/category/" (cat) } { (capitalise(cat)) }
                                    }
                                    @if count > 0 {
                                        span.wiki-home-cat-section-count {
                                            (count) " article" @if count != 1 { "s" }
                                        }
                                    }
                                    @if count > PREVIEW_LIMIT {
                                        a.wiki-home-cat-section-all href={ "/category/" (cat) } {
                                            "All " (count) " →"
                                        }
                                    }
                                }
                                @if count == 0 {
                                    p.wiki-home-cat-in-prep { "In preparation." }
                                } @else {
                                    ul.wiki-home-cat-articles {
                                        @for t in topics.iter().take(PREVIEW_LIMIT) {
                                            li.wiki-home-cat-article {
                                                a href={ "/wiki/" (t.slug) } { (t.title) }
                                                @if let Some(ref desc) = t.short_description {
                                                    p.wiki-home-cat-article-desc { (desc) }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Operational guides — served locally via guide_dir
                    @if !guides.is_empty() {
                        div.wiki-home-guides {
                            div.wiki-home-guides-head {
                                h2 { "Operational guides" }
                            }
                            ul.wiki-home-guides-list {
                                @for g in guides {
                                    li.wiki-home-guides-item {
                                        a href={ "/wiki/" (g.slug) } { (g.title) }
                                        @if let Some(ref desc) = g.short_description {
                                            span.wiki-home-guides-desc { " — " (desc) }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Catch-all: every TOPIC/GUIDE not in a ratified category
                    @if !uncategorised.is_empty() {
                        div.wiki-home-uncategorised {
                            h2 { "All articles" }
                            p.wiki-home-uncategorised-note {
                                "Articles not yet sorted into a category."
                            }
                            ul.wiki-home-uncategorised-list {
                                @for t in &uncategorised {
                                    li {
                                        a href={ "/wiki/" (t.slug) } { (t.title) }
                                    }
                                }
                            }
                        }
                    }

                    // Other areas — GitHub and related resources
                    div.wiki-home-other {
                        h2 { "Other areas" }
                        ul {
                            li { a href="https://github.com/pointsav" { "PointSav on GitHub" } " — canonical vendor-tier source" }
                            li { a href="https://github.com/woodfine" { "Woodfine Management Corp. on GitHub" } " — customer-tier mirror" }
                            li { a href="https://github.com/pointsav/pointsav-design-system" { "Design system" } " — visual tokens, component recipes, brand conventions" }
                            li { a href="https://github.com/pointsav/factory-release-engineering" { "factory-release-engineering" } " — licensing matrix, contributor agreements, governance" }
                        }
                    }

                    // Recent additions — top 10 by last_edited
                    @if !recent.is_empty() {
                        h2.wiki-home-section-title { "Recent additions" }
                        ul.wiki-home-recent {
                            @for t in recent {
                                li.wiki-home-recent-item {
                                    @if let Some(ref d) = t.last_edited {
                                        span.wiki-home-recent-date { (d) }
                                    }
                                    a href={ "/wiki/" (t.slug) } { (t.title) }
                                }
                            }
                        }
                    }
                }
                footer.site-footer {
                    p.wiki-home-bilingual-notice {
                        em {
                            "Available in English and Español. "
                            "Spanish articles are strategic-adaptation overviews, not translations."
                        }
                        " "
                        a href="/wiki/index.es" { "Leer en Español →" }
                    }
                    p { (site_title) " — "
                        a href="/" { "Home" }
                        " · Engine: app-mediakit-knowledge — see "
                        a href="https://github.com/pointsav/pointsav-monorepo" { "ARCHITECTURE.md" }
                    }
                }
            }
        }
    }
}

/// Capitalise the first character of a category name for display.
fn capitalise(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().to_string() + c.as_str(),
    }
}

// ─── Placeholder index (index.md absent) ───────────────────────────────────

/// Current flat-listing index behaviour, preserved for the absent-`index.md`
/// case. Extracted verbatim from the pre-iteration-1 `index()` handler.
async fn placeholder_index(state: &AppState) -> Result<Markup, WikiError> {
    let mut entries = fs::read_dir(&state.content_dir).await?;
    let mut pages: Vec<String> = Vec::new();
    while let Some(entry) = entries.next_entry().await? {
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if let Some(slug) = name.strip_suffix(".md") {
            // Skip bilingual siblings, system/repo files.
            if !slug.ends_with(".es") && !SYSTEM_FILE_STEMS.contains(&slug) {
                pages.push(slug.to_string());
            }
        }
    }
    pages.sort();

    Ok(chrome(
        "Index",
        html! {
            h1 { "PointSav Knowledge" }
            p.lede {
                "Flat-file Markdown source-of-truth, single-binary engine, AI-optional. "
                "Phase 1 — render."
            }
            h2 { "Pages" }
            @if pages.is_empty() {
                p.empty { "No pages in content directory yet." }
            } @else {
                ul.page-list {
                    @for slug in &pages {
                        li {
                            a href=(format!("/wiki/{slug}")) { (slug) }
                        }
                    }
                }
            }
        },
        &state.site_title,
    ))
}

// ─── Category listing handler (Wave 5B) ─────────────────────────────────────

async fn category_page(
    State(state): State<Arc<AppState>>,
    Path(name): Path<String>,
) -> Result<Markup, WikiError> {
    let buckets = bucket_topics_by_category(&state.content_dir, state.guide_dir.as_deref(), state.guide_dir_2.as_deref()).await?;
    let empty: Vec<TopicSummary> = Vec::new();
    let topics = buckets.get(&name).unwrap_or(&empty);
    let display = capitalise(&name);
    let count = topics.len();

    Ok(chrome(
        &format!("{display} — {}", state.site_title),
        html! {
            h1.wiki-cat-page-title { (display) }
            @if count == 0 {
                p.wiki-cat-page-empty { "No articles in this category yet." }
            } @else {
                p.wiki-cat-page-count {
                    (count) " article" @if count != 1 { "s" }
                }
                ul.wiki-cat-page-list {
                    @for t in topics {
                        li.wiki-cat-page-item {
                            a.wiki-cat-page-item-title href={ "/wiki/" (t.slug) } { (t.title) }
                            @if let Some(ref d) = t.last_edited {
                                span.wiki-cat-page-item-date { (d) }
                            }
                            @if let Some(ref desc) = t.short_description {
                                p.wiki-cat-page-item-desc { (desc) }
                            }
                        }
                    }
                }
            }
        },
        &state.site_title,
    ))
}

// ─── index handler ──────────────────────────────────────────────────────────

async fn index(State(state): State<Arc<AppState>>) -> Result<Markup, WikiError> {
    let home_path = state.content_dir.join("index.md");
    if !home_path.exists() {
        return placeholder_index(&state).await;
    }

    let home_text = fs::read_to_string(&home_path).await?;
    let home_parsed = crate::render::parse_page(&home_text)?;
    let buckets = bucket_topics_by_category(&state.content_dir, state.guide_dir.as_deref(), state.guide_dir_2.as_deref()).await?;
    let recent = recent_topics_by_last_edited(&buckets, 10);
    let stats = compute_home_stats(&buckets);
    let home_html = crate::render::render_html_raw(&home_parsed.body_md);

    // Collect guide summaries for the dedicated guides section.
    // A guide is any entry whose filename stem starts with "guide-".
    let mut guide_summaries: Vec<TopicSummary> = buckets
        .values()
        .flatten()
        .filter(|t| {
            t.slug
                .split('/')
                .last()
                .map(|s| s.starts_with("guide-"))
                .unwrap_or(false)
        })
        .cloned()
        .collect();
    guide_summaries.sort_by(|a, b| a.title.cmp(&b.title));

    Ok(home_chrome(
        &home_parsed.frontmatter,
        &home_html,
        &buckets,
        &recent,
        &stats,
        &guide_summaries,
        &state.site_title,
    ))
}

async fn wiki_page(
    State(state): State<Arc<AppState>>,
    Path(slug): Path<String>,
) -> Result<Markup, WikiError> {
    // Slug safety: reject path traversal. Allow at most one `/` separator
    // for category-scoped slugs (`architecture/compounding-substrate`).
    if slug.contains("..") || slug.is_empty() {
        return Err(WikiError::NotFound(slug));
    }
    // Validate component parts for safety.
    let parts: Vec<&str> = slug.splitn(3, '/').collect();
    if parts.len() > 2 {
        // More than one directory level — reject.
        return Err(WikiError::NotFound(slug));
    }
    for part in &parts {
        if part.is_empty() || part.starts_with('.') {
            return Err(WikiError::NotFound(slug.clone()));
        }
    }

    // Try content_dir first; if not found, try guide_dir then guide_dir_2.
    let primary_path = state.content_dir.join(format!("{slug}.md"));
    let text = match fs::read_to_string(&primary_path).await {
        Ok(t) => t,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            let guide_dirs: &[Option<&PathBuf>] = &[state.guide_dir.as_ref(), state.guide_dir_2.as_ref()];
            let mut found: Option<String> = None;
            for gd_opt in guide_dirs {
                if let Some(gd) = gd_opt {
                    let gp = gd.join(format!("{slug}.md"));
                    if let Ok(t) = fs::read_to_string(&gp).await {
                        found = Some(t);
                        break;
                    }
                }
            }
            match found {
                Some(t) => t,
                None => return Err(WikiError::NotFound(slug)),
            }
        }
        Err(e) => return Err(e.into()),
    };
    let mut parsed = parse_page(&text)?;

    // ── Item 11: Language toggle auto-detection ───────────────────────────
    //
    // If `translations:` frontmatter is absent or empty, check whether a
    // bilingual sibling exists on disk and inject the toggle automatically.
    //
    // Two cases:
    //   (A) Viewing the EN article (slug does NOT end in `.es`):
    //       Look for `<slug>.es.md`; if present, inject { "es" → "<slug>.es" }.
    //   (B) Viewing the ES article (slug ends in `.es`):
    //       Derive the base slug by stripping `.es`; if `<base>.md` exists,
    //       inject { "en" → "<base>" }.
    //
    // This means every article that has a sibling gets the language toggle
    // without requiring the content author to maintain `translations:` by hand.
    if parsed.frontmatter.translations.as_ref().map(|t| t.is_empty()).unwrap_or(true) {
        let is_es = slug.ends_with(".es");
        if is_es {
            // Case B: we're on an ES article; offer EN link.
            let base_slug = slug.trim_end_matches(".es");
            let base_path = state.content_dir.join(format!("{base_slug}.md"));
            if base_path.exists() {
                let mut map = TranslationMap::new();
                map.insert("en".to_string(), base_slug.to_string());
                parsed.frontmatter.translations = Some(map);
            }
        } else {
            // Case A: we're on an EN article; offer ES link if sibling exists.
            let es_slug = format!("{slug}.es");
            let es_path = state.content_dir.join(format!("{es_slug}.md"));
            if es_path.exists() {
                let mut map = TranslationMap::new();
                map.insert("es".to_string(), es_slug);
                parsed.frontmatter.translations = Some(map);
            }
        }
    }

    // Two-step render: extract headings from clean comrak output (no edit pencils),
    // then inject pencils for the final body HTML. This keeps TOC text clean.
    let raw_html = render_html_raw(&parsed.body_md);
    let headings = extract_headings(&raw_html);
    let body_html = inject_edit_pencils(&raw_html);
    let title = parsed
        .frontmatter
        .title
        .clone()
        .unwrap_or_else(|| slug.clone());
    Ok(wiki_chrome(&title, &slug, parsed.frontmatter, &body_html, headings, &state.site_title))
}

async fn static_asset(Path(path): Path<String>) -> Response {
    match StaticAsset::get(&path) {
        Some(asset) => {
            let mime = mime_guess::from_path(&path).first_or_octet_stream();
            let mut resp = asset.data.into_owned().into_response();
            if let Ok(value) = HeaderValue::from_str(mime.as_ref()) {
                resp.headers_mut().insert(header::CONTENT_TYPE, value);
            }
            resp
        }
        None => (StatusCode::NOT_FOUND, "not found").into_response(),
    }
}

/// Full article-page shell with Phase 1.1 Wikipedia muscle-memory chrome.
///
/// Additive over Phase 1's `chrome()`: the existing chrome function is
/// untouched and continues to serve the index page. This function is used
/// only by `wiki_page`.
///
/// Elements added (all additive; no existing behaviour changed):
/// - Article / Talk tab pair (top-left of title row)
/// - Read / Edit / View history tabs (top-right; Edit and View-history are
///   `href="#"` placeholders — Phase 2 wires the routes)
/// - IVC masthead band placeholder (horizontal strip below title row)
/// - Collapsible left-rail TOC with sticky scroll (Vector 2022 pattern)
/// - Language-switcher button (populated from frontmatter `translations:`)
/// - Hatnote (italic, indented; only when `hatnote:` frontmatter is present)
/// - "From PointSav Knowledge" tagline below the title
/// - `short_description` subtitle (italic, below H1; iteration-2 addition)
/// - Breadcrumb navigation (Documentation > Category > Title; iteration-2)
/// - Reader density toggle (Off / Exceptions only / All; localStorage)
/// - Per-section [edit] pencils (injected into rendered HTML by render module)
/// - Footer block: categories → license → about/contact links
fn wiki_chrome(
    title: &str,
    slug: &str,
    fm: Frontmatter,
    body_html: &str,
    headings: Vec<(String, String, u8)>,
    site_title: &str,
) -> Markup {
    let talk_slug = format!("{slug}.talk");

    // Breadcrumb: derive category from frontmatter `category:` field,
    // or from the slug prefix when the TOPIC is in a subdirectory.
    let breadcrumb_category: Option<String> = fm.category.clone().or_else(|| {
        slug.find('/').map(|pos| capitalise(&slug[..pos]))
    });

    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { (site_title) }
                link rel="stylesheet" href="/static/style.css";
                // JSON-LD baseline (Phase 2 Step 1) — schema.org TechArticle /
                // DefinedTerm. Cumulative across phases; AEO crawlers + downstream
                // consumers ingest the structured data.
                (PreEscaped(jsonld_for_topic(&fm, slug)))
            }
            body {
                header.site-header {
                    a.site-title href="/" { (site_title) }
                    form.header-search action="/search" method="get" {
                        input type="search" name="q" placeholder="Search articles…" autocomplete="off";
                        button type="submit" { "Search" }
                    }
                    nav.site-nav {
                        a href="/" { "Home" }
                    }
                }

                // Article-page two-column layout: left TOC rail + article body
                div.wiki-layout {

                    // --- Left rail: collapsible TOC (Vector 2022 sticky pattern) ---
                    nav.wiki-toc #wiki-toc {
                        div.toc-header {
                            span.toc-title { "Contents" }
                            button.toc-toggle #toc-toggle
                                aria-controls="toc-list"
                                aria-expanded="true"
                                title="Toggle table of contents"
                            { "[hide]" }
                        }
                        @if !headings.is_empty() {
                            ol.toc-list #toc-list {
                                @for (id, text, level) in &headings {
                                    li class={ "toc-level-" (level) } {
                                        a href={ "#" (id) } { (text) }
                                    }
                                }
                            }
                        }
                    }

                    // --- Main article column ---
                    main.wiki-main {

                        // Breadcrumb navigation — "Documentation > Category > Title"
                        @if let Some(ref cat) = breadcrumb_category {
                            nav.wiki-breadcrumb aria-label="Breadcrumb" {
                                a href="/" { "Documentation" }
                                span.wiki-breadcrumb-sep { " › " }
                                a href={ "/category/" (cat.to_lowercase()) } { (cat) }
                                span.wiki-breadcrumb-sep { " › " }
                                span.wiki-breadcrumb-current { (title) }
                            }
                        }

                        // Title row: tabs (top-left) + title + language switcher + action tabs (top-right)
                        div.wiki-title-row {
                            // Article / Talk tabs — top-left
                            nav.wiki-page-tabs aria-label="Page tabs" {
                                a.wiki-tab.wiki-tab-active
                                    href={ "/wiki/" (slug) }
                                    aria-current="page"
                                { "Article" }
                                span.wiki-tab.wiki-tab-disabled
                                    aria-disabled="true"
                                    title="Talk pages coming soon"
                                { "Talk" }
                            }

                            // Page title + language switcher + tagline (centre)
                            // Language button sits BELOW the H1, left-aligned —
                            // matching MediaWiki Vector 2022 (.mw-portlet-lang placement).
                            div.wiki-title-block {
                                h1.page-title { (title) }
                                @if let Some(translations) = &fm.translations {
                                    @if !translations.is_empty() {
                                        div.wiki-lang-switcher {
                                            span.wiki-lang-globe aria-hidden="true" { "🌐" }
                                            @for (lang, lang_slug) in translations {
                                                @let lang_label = match lang.as_str() {
                                                    "es" => "Español",
                                                    "en" => "English",
                                                    "fr" => "Français",
                                                    "de" => "Deutsch",
                                                    "pt" => "Português",
                                                    "zh" => "中文",
                                                    "ja" => "日本語",
                                                    "ar" => "العربية",
                                                    _ => lang.as_str(),
                                                };
                                                a.wiki-lang-btn
                                                    href={ "/wiki/" (lang_slug) }
                                                    lang=(lang)
                                                    hreflang=(lang)
                                                    title={ "Read in " (lang_label) }
                                                { (lang_label) }
                                            }
                                        }
                                    }
                                }
                                p.wiki-tagline { "From PointSav Documentation" }
                                @if let Some(ref desc) = fm.short_description {
                                    p.topic-short-description { em { (desc) } }
                                }
                            }

                            // Read / Edit / View history tabs — top-right (item 2)
                            nav.wiki-action-tabs aria-label="Page actions" {
                                a.wiki-tab.wiki-tab-active
                                    href={ "/wiki/" (slug) }
                                    aria-current="page"
                                { "Read" }
                                a.wiki-tab
                                    href={ "/edit/" (slug) }
                                { "Edit" }
                                a.wiki-tab
                                    href={ "/history/" (slug) }
                                { "View history" }
                            }
                        }

                        // IVC masthead band placeholder (UX-DESIGN.md §4.5)
                        div.wiki-ivc-band role="status" aria-label="Verification status" {
                            span.ivc-band-text {
                                "Verification not yet available — Phase 7"
                            }
                            // Reader density toggle (UX-DESIGN.md §4.6)
                            // Preference persists to localStorage; no machinery honours it
                            // until Phase 7. Default: Exceptions only.
                            div.wiki-density-toggle {
                                span.density-label { "Citation marks:" }
                                button.density-btn #density-off { "Off" }
                                button.density-btn #density-exceptions.density-btn-active
                                    { "Exceptions only" }
                                button.density-btn #density-all { "All" }
                            }
                        }

                        // Forward-looking-information notice (unchanged from Phase 1)
                        @if fm.forward_looking {
                            aside.fli-notice {
                                strong { "Forward-looking information." }
                                " Statements herein are subject to material assumptions and risks. "
                                "Per NI 51-102 / OSC SN 51-721 disclosure posture."
                            }
                        }

                        // Hatnote (item 6): italic, indented, top of article body
                        @if let Some(hatnote) = &fm.hatnote {
                            div.wiki-hatnote {
                                (hatnote)
                            }
                        }

                        // Article body
                        article.wiki-article {
                            div.page-body {
                                (PreEscaped(body_html))
                            }
                        }

                        // End-of-article footer block (item 5 + item 15)
                        footer.wiki-article-footer {
                            // Categories list (from `categories:` array — item 15)
                            @if let Some(cats) = &fm.categories {
                                @if !cats.is_empty() {
                                    div.wiki-categories {
                                        span.cats-label { "Categories:" }
                                        ul.cats-list {
                                            @for cat in cats {
                                                li { a href={ "/category/" (cat.to_lowercase()) } { (cat) } }
                                            }
                                        }
                                    }
                                }
                            }
                            // Singular category tag from `category:` field when `categories:` absent
                            @else if let Some(ref cat) = fm.category {
                                @if cat != "root" {
                                    div.wiki-categories {
                                        span.cats-label { "Category:" }
                                        span.wiki-category-single-tag {
                                            a href={ "/category/" (cat) } { (capitalise(cat)) }
                                        }
                                    }
                                }
                            }

                            // Last-edited date — Wikipedia footer convention
                            @if let Some(ref date) = fm.last_edited {
                                div.wiki-article-last-edited {
                                    "Last edited: "
                                    time datetime=(date) { (date) }
                                }
                            }

                            // License + about/contact links
                            div.wiki-footer-meta {
                                p.wiki-license {
                                    "Content is available under "
                                    a href="https://creativecommons.org/licenses/by/4.0/" {
                                        "CC BY 4.0"
                                    }
                                    " unless otherwise stated."
                                }
                                nav.wiki-footer-links {
                                    a href="/wiki/about" { "About" }
                                    " · "
                                    a href="/wiki/contact" { "Contact" }
                                    " · "
                                    a href="/wiki/disclaimers" { "Disclaimers" }
                                }
                            }
                        }
                    }
                }

                // Bottom page footer — unchanged structure, updated copy
                footer.site-footer {
                    p {
                        (site_title) " — "
                        a href="/" { "Index" }
                        " · Engine: app-mediakit-knowledge — see "
                        a href="https://github.com/pointsav/pointsav-monorepo" {
                            "ARCHITECTURE.md"
                        }
                    }
                }

                // Minimal JS: TOC collapse toggle + density preference persistence.
                // Loaded last so HTML renders without it.
                script src="/static/wiki.js" defer="true" {}
            }
        }
    }
}

// ─── Phase 3 Step 3.4 handlers ─────────────────────────────────────────────

/// `GET /sitemap.xml` — sitemaps.org standard XML sitemap.
///
/// Walks `content_dir` recursively, emits one `<url>` per TOPIC (excluding
/// `*.es.md` bilingual siblings). Content-Type: `application/xml; charset=utf-8`.
async fn sitemap_xml(State(state): State<Arc<AppState>>) -> Result<Response, WikiError> {
    let topic_files = collect_all_topic_files(&state.content_dir, &[state.guide_dir.as_deref(), state.guide_dir_2.as_deref()]).await?;
    let mut slugs: Vec<String> = topic_files.into_iter().map(|tf| tf.slug).collect();
    slugs.sort();

    let mut xml = String::from(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
         <urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n",
    );
    for slug in &slugs {
        xml.push_str(&format!(
            "  <url><loc>/wiki/{slug}</loc></url>\n"
        ));
    }
    xml.push_str("</urlset>\n");

    let mut resp = xml.into_response();
    resp.headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/xml; charset=utf-8"),
    );
    Ok(resp)
}

/// `GET /robots.txt` — static crawl-permission declaration.
///
/// Allows all crawlers and declares the sitemap location.
/// Content-Type: `text/plain; charset=utf-8`.
async fn robots_txt() -> Response {
    let body = "User-agent: *\nAllow: /\nSitemap: /sitemap.xml\n";
    let mut resp = body.into_response();
    resp.headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("text/plain; charset=utf-8"),
    );
    resp
}

/// `GET /llms.txt` — emerging LLM-readable site manifest convention.
///
/// Per the llmstxt.org convention (informal, 2025–2026). Lists all TOPICs
/// with a one-line snippet, and points crawlers at the structured data
/// surfaces (JSON-LD, Atom, JSON Feed, sitemap). Content-Type:
/// `text/markdown; charset=utf-8`.
async fn llms_txt(State(state): State<Arc<AppState>>) -> Result<Response, WikiError> {
    let topic_files = collect_all_topic_files(&state.content_dir, &[state.guide_dir.as_deref(), state.guide_dir_2.as_deref()]).await?;
    let mut tf_list: Vec<(String, PathBuf)> = topic_files.into_iter()
        .map(|tf| (tf.slug, tf.path))
        .collect();
    tf_list.sort_by(|a, b| a.0.cmp(&b.0));

    // Read each TOPIC to extract a one-line title + snippet directly from the
    // parsed body — avoids a second directory traversal compared to calling
    // `collect_recent_items`.
    let mut topic_lines: Vec<String> = Vec::new();
    for (slug, path) in &tf_list {
        let text = match fs::read_to_string(path).await {
            Ok(t) => t,
            Err(_) => continue,
        };
        let slug_str = slug.as_str();
        let parsed = match crate::render::parse_page(&text) {
            Ok(p) => p,
            Err(_) => continue,
        };
        let title = parsed.frontmatter.title.unwrap_or_else(|| slug.clone());
        let slug = slug_str;

        // Build a ~120-character snippet from the first non-heading body line.
        let body_snippet = llms_txt_snippet(&parsed.body_md, 120);

        topic_lines.push(format!("- [{title}](/wiki/{slug}): {body_snippet}"));
    }

    let topics_section = topic_lines.join("\n");

    let body = format!(
        "# {site_title}\n\
         \n\
         > Single-binary Markdown wiki engine; flat-file source-of-truth, \
         AI-optional, Wikipedia-shaped UX. Substrate substitution per \
         DOCTRINE claim #29.\n\
         \n\
         ## TOPICs\n\
         \n\
         {topics_section}\n\
         \n\
         ## Structured data\n\
         \n\
         - JSON-LD: every TOPIC `<head>` carries schema.org `TechArticle` / `DefinedTerm`\n\
         - Atom feed: `/feed.atom`\n\
         - JSON Feed: `/feed.json`\n\
         - Sitemap: `/sitemap.xml`\n",
        site_title = state.site_title,
        topics_section = topics_section,
    );

    let mut resp = body.into_response();
    resp.headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("text/markdown; charset=utf-8"),
    );
    Ok(resp)
}

/// Extract a plain-text snippet for llms.txt, capped at `max_chars`.
/// Skips heading, blank, and HR lines; strips crude Markdown punctuation.
fn llms_txt_snippet(body_md: &str, max_chars: usize) -> String {
    let first = body_md
        .lines()
        .filter(|l| {
            let t = l.trim();
            !t.is_empty() && !t.starts_with('#') && !t.starts_with("---")
        })
        .next()
        .unwrap_or("");
    let clean: String = first
        .trim_start_matches(|c| matches!(c, '-' | '*' | '+' | '>' | ' '))
        .chars()
        .filter(|&c| c != '`' && c != '*' && c != '_')
        .collect();
    let clean = clean.trim();
    if clean.len() <= max_chars {
        clean.to_string()
    } else {
        let boundary = clean[..max_chars].rfind(' ').unwrap_or(max_chars);
        format!("{}…", &clean[..boundary])
    }
}

/// `GET /git/{slug}.md` — raw Markdown source for `git clone`-style ingestion.
///
/// Validates the slug via `crate::edit::validate_slug`, reads
/// `<content_dir>/<slug>.md` from disk, and returns the raw bytes with
/// Content-Type `text/markdown; charset=utf-8`. Phase 4 upgrades this to a
/// full read-only Git remote.
///
/// Axum 0.8 captures the `{slug}` parameter **without** the `.md` suffix
/// when the route pattern is `/git/{slug}.md` — the literal `.md` in the
/// pattern is consumed by the router and not included in the extract.
async fn git_markdown(
    State(state): State<Arc<AppState>>,
    Path(raw): Path<String>,
) -> Result<Response, WikiError> {
    // Accept both `/git/topic-foo` and `/git/topic-foo.md` — strip an
    // optional `.md` suffix before slug validation. The `.md` extension
    // surfaces in the URL for consumer convenience (looks like a static
    // file under `git clone` mirror semantics) but is not part of the slug.
    let slug = raw.strip_suffix(".md").unwrap_or(&raw).to_string();

    // Slug validation rejects path traversal, uppercase, and other illegal forms.
    crate::edit::validate_slug(&slug)?;

    let path = state.content_dir.join(format!("{slug}.md"));
    let bytes = match fs::read(&path).await {
        Ok(b) => b,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            return Err(WikiError::NotFound(slug));
        }
        Err(e) => return Err(e.into()),
    };

    let mut resp = bytes.into_response();
    resp.headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("text/markdown; charset=utf-8"),
    );
    Ok(resp)
}

async fn history_page(
    State(state): State<Arc<AppState>>,
    Path(slug): Path<String>,
) -> Result<Markup, WikiError> {
    crate::edit::validate_slug(&slug)?;
    let path = state.content_dir.join(format!("{slug}.md"));
    if !path.is_file() {
        return Err(WikiError::NotFound(slug));
    }
    let history = crate::history::topic_history(&state.content_dir, &slug, 50)?;

    let body = html! {
        h1 { "History: " (slug) }
        @if history.is_empty() {
            p { "No revision history yet." }
        } @else {
            table.history-table style="width: 100%; border-collapse: collapse; margin-top: 1em;" {
                thead {
                    tr style="border-bottom: 2px solid #eee; text-align: left;" {
                        th style="padding: 8px;" { "SHA" }
                        th style="padding: 8px;" { "Author" }
                        th style="padding: 8px;" { "Date" }
                        th style="padding: 8px;" { "Message" }
                    }
                }
                tbody {
                    @for entry in history {
                        tr style="border-bottom: 1px solid #eee;" {
                            td style="padding: 8px; font-family: monospace;" {
                                a href=(format!("/diff/{}?b={}&a={}~", slug, entry.sha, entry.sha)) {
                                    @if entry.sha.len() >= 7 {
                                        (entry.sha[..7].to_string())
                                    } @else {
                                        (entry.sha)
                                    }
                                }
                            }
                            td style="padding: 8px;" { (entry.author) }
                            td style="padding: 8px; color: #666; font-size: 0.9em;" { (entry.timestamp_iso) }
                            td style="padding: 8px;" { (entry.message) }
                        }
                    }
                }
            }
        }
    };

    Ok(chrome(&format!("History: {}", slug), body, &state.site_title))
}

async fn blame_page(
    State(state): State<Arc<AppState>>,
    Path(slug): Path<String>,
) -> Result<Markup, WikiError> {
    crate::edit::validate_slug(&slug)?;
    let path = state.content_dir.join(format!("{slug}.md"));
    if !path.is_file() {
        return Err(WikiError::NotFound(slug));
    }
    let blame = crate::history::topic_blame(&state.content_dir, &slug)?;

    let body = html! {
        h1 { "Blame: " (slug) }
        div.blame-container style="background: #f9f9f9; padding: 1em; border-radius: 4px; overflow-x: auto;" {
            pre style="margin: 0; font-family: monospace; font-size: 0.9em; line-height: 1.5;" {
                @for line in blame {
                    div.blame-line style="display: flex;" {
                        span.blame-meta style="color: #999; width: 200px; flex-shrink: 0; user-select: none; border-right: 1px solid #ddd; margin-right: 1em;" {
                            @if line.sha.len() >= 7 {
                                (line.sha[..7].to_string())
                            } @else {
                                (line.sha)
                            }
                            " " (line.author)
                        }
                        span.blame-text { (line.line_text) }
                    }
                }
            }
        }
    };

    Ok(chrome(&format!("Blame: {}", slug), body, &state.site_title))
}

/// Shared shell for non-article pages (search, category, errors).
fn chrome(title: &str, body: Markup, site_title: &str) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { (site_title) }
                link rel="stylesheet" href="/static/style.css";
            }
            body {
                header.site-header {
                    a.site-title href="/" { (site_title) }
                    form.header-search action="/search" method="get" {
                        input type="search" name="q" placeholder="Search articles…" autocomplete="off";
                        button type="submit" { "Search" }
                    }
                    nav.site-nav {
                        a href="/" { "Home" }
                    }
                }
                main.site-main {
                    (body)
                }
                footer.site-footer {
                    p { (site_title) " — " a href="/" { "Home" } " · Engine: app-mediakit-knowledge" }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    async fn fixture_state() -> (AppState, tempfile::TempDir, tempfile::TempDir) {
        let dir = tempfile::tempdir().unwrap();
        let state_dir = tempfile::tempdir().unwrap();
        tokio::fs::write(
            dir.path().join("topic-test.md"),
            "---\ntitle: Test Topic\n---\n# Heading\n\nbody with [[Other]] link.\n",
        )
        .await
        .unwrap();
        let index = crate::search::build_index(dir.path(), state_dir.path())
            .await
            .unwrap();
        let repo = crate::git::open_or_init(dir.path()).unwrap();
        (
            AppState {
                content_dir: dir.path().to_path_buf(),
                guide_dir: None,
                guide_dir_2: None,
                // Use a path that does not exist; citation tests live in
                // tests/citations_test.rs where they control this path.
                // Server tests do not exercise /api/citations so the missing
                // file never triggers a load.
                citations_yaml: PathBuf::from("/nonexistent/citations.yaml"),
                search: Arc::new(index),
                git: Arc::new(Mutex::new(repo)),
                collab: Arc::new(crate::collab::CollabRooms::new()),
                enable_collab: false,
                site_title: "PointSav Documentation Wiki".to_string(),
            },
            dir,
            state_dir,
        )
    }

    #[tokio::test]
    async fn healthz_responds_ok() {
        let (state, _dir, _state_dir) = fixture_state().await;
        let app = router(state);
        let resp = app
            .oneshot(Request::builder().uri("/healthz").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn renders_known_page() {
        let (state, _dir, _state_dir) = fixture_state().await;
        let app = router(state);
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/wiki/topic-test")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = resp.into_body().collect().await.unwrap().to_bytes();
        let html = std::str::from_utf8(&body).unwrap();
        assert!(html.contains("Test Topic"), "title should appear: {html}");
        assert!(html.contains("Heading"), "body heading should appear: {html}");
    }

    #[tokio::test]
    async fn returns_404_for_unknown_page() {
        let (state, _dir, _state_dir) = fixture_state().await;
        let app = router(state);
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/wiki/does-not-exist")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn rejects_path_traversal() {
        let (state, _dir, _state_dir) = fixture_state().await;
        let app = router(state);
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/wiki/..%2Fetc%2Fpasswd")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    // Phase 1.1 chrome tests — additive; all existing tests remain unchanged.

    /// Verify that the wiki page renders the Article / Talk tab pair and the
    /// Read / Edit / View history tabs (items 1 and 2 in the UX inventory).
    #[tokio::test]
    async fn wiki_page_has_navigation_tabs() {
        let (state, _dir, _state_dir) = fixture_state().await;
        let app = router(state);
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/wiki/topic-test")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = resp.into_body().collect().await.unwrap().to_bytes();
        let html = std::str::from_utf8(&body).unwrap();
        assert!(html.contains("Article"), "Article tab should appear: {html}");
        assert!(html.contains("Talk"), "Talk tab should appear: {html}");
        assert!(html.contains("Read"), "Read tab should appear: {html}");
        assert!(html.contains("Edit"), "Edit tab should appear: {html}");
        assert!(html.contains("View history"), "View history tab should appear: {html}");
    }

    /// Verify that the tagline appears below the page title (item 9).
    #[tokio::test]
    async fn wiki_page_has_tagline() {
        let (state, _dir, _state_dir) = fixture_state().await;
        let app = router(state);
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/wiki/topic-test")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let body = resp.into_body().collect().await.unwrap().to_bytes();
        let html = std::str::from_utf8(&body).unwrap();
        assert!(
            html.contains("From PointSav Documentation"),
            "tagline should appear: {html}"
        );
    }

    /// Verify that the IVC masthead band placeholder renders on every TOPIC.
    #[tokio::test]
    async fn wiki_page_has_ivc_masthead_band() {
        let (state, _dir, _state_dir) = fixture_state().await;
        let app = router(state);
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/wiki/topic-test")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let body = resp.into_body().collect().await.unwrap().to_bytes();
        let html = std::str::from_utf8(&body).unwrap();
        assert!(
            html.contains("wiki-ivc-band"),
            "IVC masthead band container should appear: {html}"
        );
    }

    /// Verify that the hatnote renders when the frontmatter field is present.
    #[tokio::test]
    async fn wiki_page_renders_hatnote() {
        let dir = tempfile::tempdir().unwrap();
        let state_dir = tempfile::tempdir().unwrap();
        tokio::fs::write(
            dir.path().join("with-hatnote.md"),
            "---\ntitle: Hatnote Test\nhatnote: \"See also the companion page.\"\n---\n# Body\n",
        )
        .await
        .unwrap();
        let index = crate::search::build_index(dir.path(), state_dir.path())
            .await
            .unwrap();
        let repo = crate::git::open_or_init(dir.path()).unwrap();
        let state = AppState {
            content_dir: dir.path().to_path_buf(),
            guide_dir: None,
            guide_dir_2: None,
            citations_yaml: PathBuf::from("/nonexistent/citations.yaml"),
            search: Arc::new(index),
            git: Arc::new(Mutex::new(repo)),
            collab: Arc::new(crate::collab::CollabRooms::new()),
            enable_collab: false,
            site_title: "PointSav Documentation Wiki".to_string(),
        };
        let app = router(state);
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/wiki/with-hatnote")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = resp.into_body().collect().await.unwrap().to_bytes();
        let html = std::str::from_utf8(&body).unwrap();
        assert!(
            html.contains("wiki-hatnote"),
            "hatnote block should appear: {html}"
        );
        assert!(
            html.contains("See also the companion page."),
            "hatnote text should appear: {html}"
        );
    }

    /// Verify that the reader density toggle buttons render (UX-DESIGN.md §4.6).
    #[tokio::test]
    async fn wiki_page_has_density_toggle() {
        let (state, _dir, _state_dir) = fixture_state().await;
        let app = router(state);
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/wiki/topic-test")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let body = resp.into_body().collect().await.unwrap().to_bytes();
        let html = std::str::from_utf8(&body).unwrap();
        assert!(html.contains("Exceptions only"), "density toggle should appear: {html}");
        assert!(html.contains("density-off"), "Off button should appear: {html}");
        assert!(html.contains("density-all"), "All button should appear: {html}");
    }

    /// Verify that per-section [edit] pencils appear on headings.
    #[tokio::test]
    async fn wiki_page_has_edit_pencils() {
        let dir = tempfile::tempdir().unwrap();
        let state_dir = tempfile::tempdir().unwrap();
        tokio::fs::write(
            dir.path().join("sections.md"),
            "---\ntitle: Sections\n---\n## First section\n\nText.\n\n## Second section\n\nMore.\n",
        )
        .await
        .unwrap();
        let index = crate::search::build_index(dir.path(), state_dir.path())
            .await
            .unwrap();
        let repo = crate::git::open_or_init(dir.path()).unwrap();
        let state = AppState {
            content_dir: dir.path().to_path_buf(),
            guide_dir: None,
            guide_dir_2: None,
            citations_yaml: PathBuf::from("/nonexistent/citations.yaml"),
            search: Arc::new(index),
            git: Arc::new(Mutex::new(repo)),
            collab: Arc::new(crate::collab::CollabRooms::new()),
            enable_collab: false,
            site_title: "PointSav Documentation Wiki".to_string(),
        };
        let app = router(state);
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/wiki/sections")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let body = resp.into_body().collect().await.unwrap().to_bytes();
        let html = std::str::from_utf8(&body).unwrap();
        assert!(
            html.contains("edit-pencil"),
            "edit pencil class should appear on headings: {html}"
        );
        assert!(
            html.contains("Edit this section"),
            "edit pencil title should appear: {html}"
        );
    }

    /// Verify categories render in the article footer when present.
    #[tokio::test]
    async fn wiki_page_renders_categories() {
        let dir = tempfile::tempdir().unwrap();
        let state_dir = tempfile::tempdir().unwrap();
        tokio::fs::write(
            dir.path().join("cats.md"),
            "---\ntitle: Cats\ncategories:\n  - Alpha\n  - Beta\n---\n# Body\n",
        )
        .await
        .unwrap();
        let index = crate::search::build_index(dir.path(), state_dir.path())
            .await
            .unwrap();
        let repo = crate::git::open_or_init(dir.path()).unwrap();
        let state = AppState {
            content_dir: dir.path().to_path_buf(),
            guide_dir: None,
            guide_dir_2: None,
            citations_yaml: PathBuf::from("/nonexistent/citations.yaml"),
            search: Arc::new(index),
            git: Arc::new(Mutex::new(repo)),
            collab: Arc::new(crate::collab::CollabRooms::new()),
            enable_collab: false,
            site_title: "PointSav Documentation Wiki".to_string(),
        };
        let app = router(state);
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/wiki/cats")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let body = resp.into_body().collect().await.unwrap().to_bytes();
        let html = std::str::from_utf8(&body).unwrap();
        assert!(html.contains("Alpha"), "category Alpha should appear: {html}");
        assert!(html.contains("Beta"), "category Beta should appear: {html}");
        assert!(html.contains("wiki-categories"), "categories block should appear: {html}");
    }

    // Iteration-2 tests — additive; all existing tests remain unchanged.

    /// Verify that `short_description` renders as italic subtitle below the H1.
    #[tokio::test]
    async fn wiki_page_renders_short_description() {
        let dir = tempfile::tempdir().unwrap();
        let state_dir = tempfile::tempdir().unwrap();
        tokio::fs::write(
            dir.path().join("described.md"),
            "---\ntitle: Described Topic\nshort_description: \"One-sentence summary here.\"\n---\nBody content.\n",
        )
        .await
        .unwrap();
        let index = crate::search::build_index(dir.path(), state_dir.path())
            .await
            .unwrap();
        let repo = crate::git::open_or_init(dir.path()).unwrap();
        let state = AppState {
            content_dir: dir.path().to_path_buf(),
            guide_dir: None,
            guide_dir_2: None,
            citations_yaml: PathBuf::from("/nonexistent/citations.yaml"),
            search: Arc::new(index),
            git: Arc::new(Mutex::new(repo)),
            collab: Arc::new(crate::collab::CollabRooms::new()),
            enable_collab: false,
            site_title: "PointSav Documentation Wiki".to_string(),
        };
        let app = router(state);
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/wiki/described")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = resp.into_body().collect().await.unwrap().to_bytes();
        let html = std::str::from_utf8(&body).unwrap();
        assert!(
            html.contains("topic-short-description"),
            "short_description container class should appear: {html}"
        );
        assert!(
            html.contains("One-sentence summary here."),
            "short_description text should appear: {html}"
        );
    }

    /// Verify that the breadcrumb renders when `category:` frontmatter is present.
    #[tokio::test]
    async fn wiki_page_renders_breadcrumb_from_category() {
        let dir = tempfile::tempdir().unwrap();
        let state_dir = tempfile::tempdir().unwrap();
        tokio::fs::write(
            dir.path().join("breadcrumb-test.md"),
            "---\ntitle: Breadcrumb Test\ncategory: architecture\n---\nBody.\n",
        )
        .await
        .unwrap();
        let index = crate::search::build_index(dir.path(), state_dir.path())
            .await
            .unwrap();
        let repo = crate::git::open_or_init(dir.path()).unwrap();
        let state = AppState {
            content_dir: dir.path().to_path_buf(),
            guide_dir: None,
            guide_dir_2: None,
            citations_yaml: PathBuf::from("/nonexistent/citations.yaml"),
            search: Arc::new(index),
            git: Arc::new(Mutex::new(repo)),
            collab: Arc::new(crate::collab::CollabRooms::new()),
            enable_collab: false,
            site_title: "PointSav Documentation Wiki".to_string(),
        };
        let app = router(state);
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/wiki/breadcrumb-test")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = resp.into_body().collect().await.unwrap().to_bytes();
        let html = std::str::from_utf8(&body).unwrap();
        assert!(
            html.contains("wiki-breadcrumb"),
            "breadcrumb nav should appear: {html}"
        );
        assert!(
            html.contains("Documentation"),
            "Documentation root link should appear in breadcrumb: {html}"
        );
    }

    /// Verify that a TOPIC in a subdirectory is reachable via the `/wiki/<cat>/<slug>` path.
    #[tokio::test]
    async fn wiki_page_resolves_subdirectory_slug() {
        let dir = tempfile::tempdir().unwrap();
        let state_dir = tempfile::tempdir().unwrap();
        // Create architecture/ subdirectory with one TOPIC.
        tokio::fs::create_dir_all(dir.path().join("architecture")).await.unwrap();
        tokio::fs::write(
            dir.path().join("architecture/compounding-substrate.md"),
            "---\ntitle: The Compounding Substrate\ncategory: architecture\n---\nSubstrate body.\n",
        )
        .await
        .unwrap();
        let index = crate::search::build_index(dir.path(), state_dir.path())
            .await
            .unwrap();
        let repo = crate::git::open_or_init(dir.path()).unwrap();
        let state = AppState {
            content_dir: dir.path().to_path_buf(),
            guide_dir: None,
            guide_dir_2: None,
            citations_yaml: PathBuf::from("/nonexistent/citations.yaml"),
            search: Arc::new(index),
            git: Arc::new(Mutex::new(repo)),
            collab: Arc::new(crate::collab::CollabRooms::new()),
            enable_collab: false,
            site_title: "PointSav Documentation Wiki".to_string(),
        };
        let app = router(state);
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/wiki/architecture/compounding-substrate")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK, "subdirectory TOPIC should resolve");
        let body = resp.into_body().collect().await.unwrap().to_bytes();
        let html = std::str::from_utf8(&body).unwrap();
        assert!(
            html.contains("The Compounding Substrate"),
            "title from frontmatter should appear: {html}"
        );
    }

    /// Verify that subdirectory TOPICs appear in the home-page category grid.
    #[tokio::test]
    async fn home_page_buckets_subdirectory_topics() {
        let dir = tempfile::tempdir().unwrap();
        let state_dir = tempfile::tempdir().unwrap();
        // index.md required for home_chrome path.
        tokio::fs::write(
            dir.path().join("index.md"),
            "---\ntitle: Home\ncategory: root\n---\nWelcome.\n",
        )
        .await
        .unwrap();
        // Architecture subdirectory with one TOPIC.
        tokio::fs::create_dir_all(dir.path().join("architecture")).await.unwrap();
        tokio::fs::write(
            dir.path().join("architecture/my-article.md"),
            "---\ntitle: My Article\ncategory: architecture\n---\nContent here.\n",
        )
        .await
        .unwrap();
        let index = crate::search::build_index(dir.path(), state_dir.path())
            .await
            .unwrap();
        let repo = crate::git::open_or_init(dir.path()).unwrap();
        let state = AppState {
            content_dir: dir.path().to_path_buf(),
            guide_dir: None,
            guide_dir_2: None,
            citations_yaml: PathBuf::from("/nonexistent/citations.yaml"),
            search: Arc::new(index),
            git: Arc::new(Mutex::new(repo)),
            collab: Arc::new(crate::collab::CollabRooms::new()),
            enable_collab: false,
            site_title: "PointSav Documentation Wiki".to_string(),
        };
        let app = router(state);
        let resp = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = resp.into_body().collect().await.unwrap().to_bytes();
        let html = std::str::from_utf8(&body).unwrap();
        // The article title should appear in the category grid.
        assert!(
            html.contains("My Article"),
            "subdirectory TOPIC title should appear in category grid: {html}"
        );
        // The Architecture category should show at least 1 article.
        assert!(
            html.contains("Architecture"),
            "Architecture category header should appear: {html}"
        );
    }

    // Iteration-2 Item 11 tests — language toggle auto-detection.

    /// EN article with a `.es.md` sibling gets an ES toggle auto-injected.
    #[tokio::test]
    async fn wiki_page_auto_detects_es_sibling() {
        let dir = tempfile::tempdir().unwrap();
        let state_dir = tempfile::tempdir().unwrap();
        // EN article
        tokio::fs::write(
            dir.path().join("my-topic.md"),
            "---\ntitle: My Topic\ncategory: architecture\n---\nEN content.\n",
        )
        .await
        .unwrap();
        // ES sibling
        tokio::fs::write(
            dir.path().join("my-topic.es.md"),
            "---\ntitle: Mi Tema\ncategory: architecture\n---\nContenido ES.\n",
        )
        .await
        .unwrap();
        let index = crate::search::build_index(dir.path(), state_dir.path())
            .await
            .unwrap();
        let repo = crate::git::open_or_init(dir.path()).unwrap();
        let state = AppState {
            content_dir: dir.path().to_path_buf(),
            guide_dir: None,
            guide_dir_2: None,
            citations_yaml: PathBuf::from("/nonexistent/citations.yaml"),
            search: Arc::new(index),
            git: Arc::new(Mutex::new(repo)),
            collab: Arc::new(crate::collab::CollabRooms::new()),
            enable_collab: false,
            site_title: "PointSav Documentation Wiki".to_string(),
        };
        let app = router(state);
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/wiki/my-topic")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = resp.into_body().collect().await.unwrap().to_bytes();
        let html = std::str::from_utf8(&body).unwrap();
        // Should show ES toggle
        assert!(
            html.contains("wiki-lang-switcher"),
            "language switcher should appear when .es.md sibling exists: {html}"
        );
        assert!(
            html.contains("/wiki/my-topic.es"),
            "ES sibling link should appear in language switcher: {html}"
        );
    }

    /// ES article auto-gets an EN link back to the base slug.
    #[tokio::test]
    async fn wiki_page_es_article_gets_en_toggle() {
        let dir = tempfile::tempdir().unwrap();
        let state_dir = tempfile::tempdir().unwrap();
        // EN base article
        tokio::fs::write(
            dir.path().join("my-topic.md"),
            "---\ntitle: My Topic\ncategory: architecture\n---\nEN content.\n",
        )
        .await
        .unwrap();
        // ES sibling
        tokio::fs::write(
            dir.path().join("my-topic.es.md"),
            "---\ntitle: Mi Tema\ncategory: architecture\n---\nContenido ES.\n",
        )
        .await
        .unwrap();
        let index = crate::search::build_index(dir.path(), state_dir.path())
            .await
            .unwrap();
        let repo = crate::git::open_or_init(dir.path()).unwrap();
        let state = AppState {
            content_dir: dir.path().to_path_buf(),
            guide_dir: None,
            guide_dir_2: None,
            citations_yaml: PathBuf::from("/nonexistent/citations.yaml"),
            search: Arc::new(index),
            git: Arc::new(Mutex::new(repo)),
            collab: Arc::new(crate::collab::CollabRooms::new()),
            enable_collab: false,
            site_title: "PointSav Documentation Wiki".to_string(),
        };
        let app = router(state);
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/wiki/my-topic.es")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = resp.into_body().collect().await.unwrap().to_bytes();
        let html = std::str::from_utf8(&body).unwrap();
        // ES article should show EN toggle back to base
        assert!(
            html.contains("wiki-lang-switcher"),
            "language switcher should appear on ES article: {html}"
        );
        assert!(
            html.contains("/wiki/my-topic\""),
            "EN base link should appear in language switcher on ES article: {html}"
        );
    }

    /// EN article WITHOUT an ES sibling should NOT show the language switcher.
    #[tokio::test]
    async fn wiki_page_no_toggle_when_sibling_absent() {
        let dir = tempfile::tempdir().unwrap();
        let state_dir = tempfile::tempdir().unwrap();
        tokio::fs::write(
            dir.path().join("solo-topic.md"),
            "---\ntitle: Solo Topic\ncategory: architecture\n---\nBody only.\n",
        )
        .await
        .unwrap();
        // No .es.md sibling written.
        let index = crate::search::build_index(dir.path(), state_dir.path())
            .await
            .unwrap();
        let repo = crate::git::open_or_init(dir.path()).unwrap();
        let state = AppState {
            content_dir: dir.path().to_path_buf(),
            guide_dir: None,
            guide_dir_2: None,
            citations_yaml: PathBuf::from("/nonexistent/citations.yaml"),
            search: Arc::new(index),
            git: Arc::new(Mutex::new(repo)),
            collab: Arc::new(crate::collab::CollabRooms::new()),
            enable_collab: false,
            site_title: "PointSav Documentation Wiki".to_string(),
        };
        let app = router(state);
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/wiki/solo-topic")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = resp.into_body().collect().await.unwrap().to_bytes();
        let html = std::str::from_utf8(&body).unwrap();
        assert!(
            !html.contains("wiki-lang-switcher"),
            "language switcher should NOT appear when no sibling exists: {html}"
        );
    }
}
