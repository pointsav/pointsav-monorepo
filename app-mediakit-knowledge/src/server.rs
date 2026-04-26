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
//! The existing `chrome()` function is retained for the index page.

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
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs;

use crate::assets::StaticAsset;
use crate::error::WikiError;
use crate::jsonld::jsonld_for_topic;
use crate::render::{extract_headings, inject_edit_pencils, parse_page, render_html_raw, Frontmatter};
use crate::search::{search as run_search, SearchIndex};

#[derive(Clone)]
pub struct AppState {
    pub content_dir: PathBuf,
    /// Path to the workspace citation registry YAML file.
    /// Defaults to `/srv/foundry/citations.yaml`; overridable via
    /// `--citations-yaml` / `WIKI_CITATIONS_YAML`.
    pub citations_yaml: PathBuf,
    /// Phase 3 Step 3.2: tantivy full-text search index. Built on
    /// startup from a tree walk of `content_dir`; reindexed on every
    /// successful edit / create. Clone-cheap (Arc-wrapped internals).
    pub search: Arc<SearchIndex>,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/wiki/{slug}", get(wiki_page))
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
        // Phase 3 Step 3.2 — full-text search HTML page over the tantivy index
        .route("/search", get(search_page))
        // Phase 3 Step 3.3 — Atom + JSON Feed syndication
        .route("/feed.atom", get(crate::feeds::get_atom))
        .route("/feed.json", get(crate::feeds::get_json_feed))
        // Phase 3 Step 3.4 — crawler discovery + raw Markdown source
        .route("/sitemap.xml", get(sitemap_xml))
        .route("/robots.txt", get(robots_txt))
        .route("/llms.txt", get(llms_txt))
        // axum 0.8 doesn't allow a literal `.md` suffix after a dynamic
        // segment, so the route captures `{slug}` as a single segment and
        // the handler strips an optional trailing `.md` for the
        // git-clone-style UX (`/git/topic-foo.md` or `/git/topic-foo`).
        .route("/git/{slug}", get(git_markdown))
        .with_state(Arc::new(state))
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

async fn index(State(state): State<Arc<AppState>>) -> Result<Markup, WikiError> {
    let mut entries = fs::read_dir(&state.content_dir).await?;
    let mut pages: Vec<String> = Vec::new();
    while let Some(entry) = entries.next_entry().await? {
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if let Some(slug) = name.strip_suffix(".md") {
            // Skip the bilingual sibling files (`*.es.md`) for the index;
            // they're addressable directly by full slug if needed.
            if !slug.ends_with(".es") {
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
    ))
}

async fn wiki_page(
    State(state): State<Arc<AppState>>,
    Path(slug): Path<String>,
) -> Result<Markup, WikiError> {
    // Phase 1 slug safety: reject path traversal and nested paths.
    // Phase 6 introduces full slug normalisation rules.
    if slug.contains("..") || slug.contains('/') || slug.is_empty() {
        return Err(WikiError::NotFound(slug));
    }
    let path = state.content_dir.join(format!("{slug}.md"));
    let text = match fs::read_to_string(&path).await {
        Ok(t) => t,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            return Err(WikiError::NotFound(slug));
        }
        Err(e) => return Err(e.into()),
    };
    let parsed = parse_page(&text)?;
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
    Ok(wiki_chrome(&title, &slug, parsed.frontmatter, &body_html, headings))
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
/// - Reader density toggle (Off / Exceptions only / All; localStorage)
/// - Per-section [edit] pencils (injected into rendered HTML by render module)
/// - Footer block: categories → license → about/contact links
fn wiki_chrome(
    title: &str,
    slug: &str,
    fm: Frontmatter,
    body_html: &str,
    headings: Vec<(String, String, u8)>,
) -> Markup {
    let talk_slug = format!("{slug}.talk");

    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { (title) " — PointSav Knowledge" }
                link rel="stylesheet" href="/static/style.css";
                // JSON-LD baseline (Phase 2 Step 1) — schema.org TechArticle /
                // DefinedTerm. Cumulative across phases; AEO crawlers + downstream
                // consumers ingest the structured data.
                (PreEscaped(jsonld_for_topic(&fm, slug)))
            }
            body {
                // Top site header — unchanged from Phase 1
                header.site-header {
                    a.site-title href="/" { "PointSav Knowledge" }
                    nav.site-nav {
                        a href="/" { "Index" }
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

                        // Title row: tabs (top-left) + title + language switcher + action tabs (top-right)
                        div.wiki-title-row {
                            // Article / Talk tabs — top-left
                            nav.wiki-page-tabs aria-label="Page tabs" {
                                a.wiki-tab.wiki-tab-active
                                    href={ "/wiki/" (slug) }
                                    aria-current="page"
                                { "Article" }
                                a.wiki-tab
                                    href={ "/wiki/" (talk_slug) }
                                { "Talk" }
                            }

                            // Page title + tagline + language switcher (centre)
                            div.wiki-title-block {
                                div.wiki-title-inner {
                                    h1.page-title { (title) }
                                    // Language switcher — next to title (item 14)
                                    @if let Some(translations) = &fm.translations {
                                        @if !translations.is_empty() {
                                            div.wiki-lang-switcher {
                                                @for (lang, lang_slug) in translations {
                                                    a.wiki-lang-btn
                                                        href={ "/wiki/" (lang_slug) }
                                                        lang=(lang)
                                                    { (lang) }
                                                }
                                            }
                                        }
                                    }
                                }
                                p.wiki-tagline { "From PointSav Knowledge" }
                            }

                            // Read / Edit / View history tabs — top-right (item 2)
                            nav.wiki-action-tabs aria-label="Page actions" {
                                a.wiki-tab.wiki-tab-active
                                    href={ "/wiki/" (slug) }
                                    aria-current="page"
                                { "Read" }
                                a.wiki-tab href="#" { "Edit" }
                                a.wiki-tab href="#" { "View history" }
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
                            // Categories (item 15 / item 5 — last section before page footer)
                            @if let Some(cats) = &fm.categories {
                                @if !cats.is_empty() {
                                    div.wiki-categories {
                                        span.cats-label { "Categories:" }
                                        ul.cats-list {
                                            @for cat in cats {
                                                li { a href="#" { (cat) } }
                                            }
                                        }
                                    }
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
                        "PointSav Knowledge — "
                        a href="/" { "Index" }
                        " · Engine: app-mediakit-knowledge Phase 1.1 — see "
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
/// Walks `content_dir`, emits one `<url>` per TOPIC (excluding `*.es.md`
/// bilingual siblings). Content-Type: `application/xml; charset=utf-8`.
async fn sitemap_xml(State(state): State<Arc<AppState>>) -> Result<Response, WikiError> {
    let mut entries = fs::read_dir(&state.content_dir).await?;
    let mut slugs: Vec<String> = Vec::new();

    while let Some(entry) = entries.next_entry().await? {
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if let Some(slug) = name.strip_suffix(".md") {
            if !slug.ends_with(".es") {
                slugs.push(slug.to_string());
            }
        }
    }
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
    let mut entries = fs::read_dir(&state.content_dir).await?;
    let mut slugs: Vec<String> = Vec::new();

    while let Some(entry) = entries.next_entry().await? {
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if let Some(slug) = name.strip_suffix(".md") {
            if !slug.ends_with(".es") {
                slugs.push(slug.to_string());
            }
        }
    }
    slugs.sort();

    // Read each TOPIC to extract a one-line title + snippet directly from the
    // parsed body — avoids a second directory traversal compared to calling
    // `collect_recent_items`.
    let mut topic_lines: Vec<String> = Vec::new();
    for slug in &slugs {
        let path = state.content_dir.join(format!("{slug}.md"));
        let text = match fs::read_to_string(&path).await {
            Ok(t) => t,
            Err(_) => continue,
        };
        let parsed = match crate::render::parse_page(&text) {
            Ok(p) => p,
            Err(_) => continue,
        };
        let title = parsed.frontmatter.title.unwrap_or_else(|| slug.clone());

        // Build a ~120-character snippet from the first non-heading body line.
        let body_snippet = llms_txt_snippet(&parsed.body_md, 120);

        topic_lines.push(format!("- [{title}](/wiki/{slug}): {body_snippet}"));
    }

    let topics_section = topic_lines.join("\n");

    let body = format!(
        "# PointSav Knowledge\n\
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
         - Sitemap: `/sitemap.xml`\n"
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

/// Shared shell for non-article pages (index, errors).
/// Retained exactly from Phase 1 — no changes.
fn chrome(title: &str, body: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { (title) " — PointSav Knowledge" }
                link rel="stylesheet" href="/static/style.css";
            }
            body {
                header.site-header {
                    a.site-title href="/" { "PointSav Knowledge" }
                    nav.site-nav {
                        a href="/" { "Index" }
                    }
                }
                main.site-main {
                    (body)
                }
                footer.site-footer {
                    p { "Engine: app-mediakit-knowledge — Phase 1 — see ARCHITECTURE.md" }
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
        (
            AppState {
                content_dir: dir.path().to_path_buf(),
                // Use a path that does not exist; citation tests live in
                // tests/citations_test.rs where they control this path.
                // Server tests do not exercise /api/citations so the missing
                // file never triggers a load.
                citations_yaml: PathBuf::from("/nonexistent/citations.yaml"),
                search: Arc::new(index),
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
            html.contains("From PointSav Knowledge"),
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
        let state = AppState {
            content_dir: dir.path().to_path_buf(),
            citations_yaml: PathBuf::from("/nonexistent/citations.yaml"),
            search: Arc::new(index),
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
        let state = AppState {
            content_dir: dir.path().to_path_buf(),
            citations_yaml: PathBuf::from("/nonexistent/citations.yaml"),
            search: Arc::new(index),
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
        let state = AppState {
            content_dir: dir.path().to_path_buf(),
            citations_yaml: PathBuf::from("/nonexistent/citations.yaml"),
            search: Arc::new(index),
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
}
