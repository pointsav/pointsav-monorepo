---
schema: foundry-draft-v1
title: "Guide: Knowledge Wiki — Wikipedia Parity Sprint Roadmap"
slug: guide-knowledge-wiki-sprint-roadmap
language: en
status: draft-pending-language-pass
author: task@project-knowledge
created: 2026-05-07
target_path: woodfine/woodfine-fleet-deployment/gateway-knowledge-documentation-1/guide-knowledge-wiki-sprint-roadmap.md
bilingual_pair: ~
cites: []
bcsc_class: operational
forward_looking: true
research_trail:
  method: "Three concurrent research agents synthesised into implementation plan: codebase gap analysis + MediaWiki architecture research + Rust ecosystem research, 2026-05-07"
  depth: "comrak 0.29–0.52 API, similar crate InlineChange API, MediaWiki Vector 2022 layout, Wikipedia UX inventory"
  confidence: high
  limitations: "Implementation estimates are approximate; sprint sequencing assumes a single Task Claude session per sprint"
---

# Guide: Knowledge Wiki — Wikipedia Parity Sprint Roadmap

This guide records the implementation roadmap for closing the gap between `app-mediakit-knowledge`'s current ~78% Wikipedia muscle memory and a target of ~95%, followed by the Leapfrog 2030 additions that go beyond Wikipedia. It is an operational reference for the Task Claude session executing each sprint.

All code changes are in `pointsav-monorepo/app-mediakit-knowledge/`. All commits use `bin/commit-as-next.sh`. The engine is rebuilt and deployed after each sprint via `cargo build --release` + `sudo systemctl restart local-knowledge-documentation.service` (and the two sibling services).

---

## Prerequisites

- `app-mediakit-knowledge` is at Phase 5 (auth + edit review queue shipped)
- comrak 0.29 is the current version; upgrade to 0.52 is Sprint B item 1
- The `similar` crate is already in Cargo.toml with `features = ["inline"]`
- `regex` crate is already in Cargo.toml
- All three wiki services respond 200 OK at their public URLs

---

## Sprint A — Quick wins (est. 1 session)

Seven changes requiring no architectural additions. Each is 20–120 lines of CSS/JS/Rust.

### A1: Footnote CSS — Wikipedia `[1][2][3]` style

**File:** `static/style.css`

Add the following at the bottom of the file:

```css
/* Wikipedia-style footnote superscripts */
.footnote-ref { font-size: 0.75em; vertical-align: super; line-height: 0; }
.footnote-ref a { color: #3366cc; text-decoration: none; padding: 0 1px; }
.footnote-ref a::before { content: "["; }
.footnote-ref a::after  { content: "]"; }
.footnotes { font-size: 0.875em; border-top: 1px solid #a2a9b1; margin-top: 2em; padding-top: 1em; }
.footnotes ol { padding-left: 1.5em; }
.footnote-backref { color: #3366cc; text-decoration: none; margin-left: 0.25em; }
```

comrak already emits `<sup class="footnote-ref">` and `<section class="footnotes">`. This CSS makes them render as blue `[1]` superscripts matching Wikipedia's visual output exactly.

**Verification:** Add `[^1]` to any test article. The footnote superscript should render as a blue bracketed number. The reference list at the bottom should be visually separated.

---

### A2: Footnote hover tooltip

**File:** `static/wiki.js`

Add `initFootnoteTooltips()` function before the DOMContentLoaded boot block:

```javascript
function initFootnoteTooltips() {
  var tooltip = document.createElement('div');
  tooltip.className = 'wiki-footnote-tooltip';
  tooltip.style.display = 'none';
  document.body.appendChild(tooltip);

  document.querySelectorAll('.footnote-ref a').forEach(function(ref) {
    ref.addEventListener('mouseenter', function(e) {
      var target = document.querySelector(ref.getAttribute('href'));
      if (!target) return;
      var clone = target.cloneNode(true);
      clone.querySelectorAll('.footnote-backref').forEach(function(b) { b.remove(); });
      tooltip.innerHTML = clone.innerHTML;
      tooltip.style.display = 'block';
      var rect = ref.getBoundingClientRect();
      tooltip.style.left = Math.max(10, rect.left + window.scrollX) + 'px';
      tooltip.style.top  = (rect.top + window.scrollY - tooltip.offsetHeight - 8) + 'px';
    });
    ref.addEventListener('mouseleave', function() { tooltip.style.display = 'none'; });
  });
}
```

Add `initFootnoteTooltips();` in the DOMContentLoaded handler alongside the other `init*` calls.

**CSS (style.css):**
```css
.wiki-footnote-tooltip {
  position: absolute; z-index: 400;
  background: #fff; border: 1px solid #a2a9b1;
  padding: 0.5em 0.75em; max-width: 400px;
  font-size: 0.875em; line-height: 1.5;
  box-shadow: 0 2px 6px rgba(0,0,0,0.15);
  pointer-events: none;
}
```

---

### A3: Enable definition lists

**File:** `src/render.rs`

In the comrak `Options` construction (near the other `options.extension.*` assignments):

```rust
options.extension.definition_lists = true;
```

One line. Enables `Term\n: Definition` → `<dl><dt><dd>` output. Matches Wikipedia's use of description lists for terminology articles.

---

### A4: `/random` route

**File:** `src/server.rs`

Add handler:
```rust
async fn random_article(State(state): State<AppState>) -> impl IntoResponse {
    let files = collect_all_topic_files(
        &state.content_dir,
        &[state.guide_dir.as_deref(), state.guide_dir_2.as_deref()],
    );
    if files.is_empty() {
        return Redirect::to("/").into_response();
    }
    use std::time::{SystemTime, UNIX_EPOCH};
    let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().subsec_nanos() as usize;
    let idx = seed % files.len();
    let slug = &files[idx].0;
    Redirect::to(&format!("/wiki/{slug}")).into_response()
}
```

Add to router: `.route("/random", get(random_article))`

Add to nav/toolbox section in `wiki_chrome()`: a "Random article" link pointing to `/random`.

---

### A5: Redirects via frontmatter

**File:** `src/render.rs` — add field to `Frontmatter`:
```rust
#[serde(default)]
pub redirect_to: Option<String>,
```

**File:** `src/server.rs` — in `wiki_page()`, after parsing frontmatter and before rendering:
```rust
if let Some(ref target) = fm.redirect_to {
    let target_url = format!("/wiki/{}", target);
    return Redirect::permanent(&target_url).into_response();
}
```

---

### A6: Disambiguation page type

**File:** `src/render.rs` — add to `Frontmatter`:
```rust
#[serde(default)]
pub disambig: Option<bool>,
```

**File:** `src/server.rs` — in `wiki_chrome()`, after the hatnote block, add:
```rust
@if fm.disambig == Some(true) {
    div.hatnote.disambiguation {
        "This page is a disambiguation page. It lists articles with similar titles."
    }
}
```

Add CSS class `.disambiguation` with a light grey background and a small disambiguation icon.

---

### A7: Edit summary field

**File:** `src/server.rs` — in the `GET /edit/{slug}` form template, add:
```html
<div class="edit-summary-row">
  <label for="edit-summary">Edit summary (optional):</label>
  <input id="edit-summary" name="summary" type="text" maxlength="255"
         placeholder="Briefly describe your changes">
</div>
```

**File:** `src/edit.rs` — in the POST handler, read `summary` from the form body and prepend to the git commit message:
```rust
let summary_line = form.summary.as_deref().unwrap_or("").trim();
let commit_msg = if summary_line.is_empty() {
    format!("Edit: {slug}")
} else {
    format!("{summary_line}\n\nEdit: {slug}")
};
```

---

## Sprint B — Block types + comrak upgrade (est. 1–2 sessions)

### B1: Upgrade comrak 0.29 → 0.52

**File:** `Cargo.toml`

```toml
comrak = "0.52"
```

Run `cargo check` inside `app-mediakit-knowledge/`. The API is stable across these versions. Existing extension flags remain unchanged. This unlocks the `block_directive` extension for `:::infobox` / `:::navbox` syntax used in B2 and B3.

Enable in `render.rs` after upgrade:
```rust
options.extension.block_directive = true;
```

**Verification:** `cargo test --lib` should pass.

---

### B2: Infobox native block type

**File:** `src/render.rs`

After `parse_document()`, walk the AST to find `CodeBlock` nodes with `info = "infobox"` (pre-upgrade approach) or `BlockDirective` nodes with name `"infobox"` (post-upgrade approach). Parse the body as YAML and replace with rendered HTML:

```rust
pub fn render_infobox(fields: &serde_yaml::Mapping) -> String {
    let mut html = String::from(r#"<table class="infobox"><tbody>"#);
    for (k, v) in fields {
        let key = k.as_str().unwrap_or("");
        let val = v.as_str().unwrap_or("");
        html.push_str(&format!(
            "<tr><th scope=\"row\">{}</th><td>{}</td></tr>",
            html_escape(key), html_escape(val)
        ));
    }
    html.push_str("</tbody></table>");
    html
}
```

**CSS (style.css):**
```css
.infobox {
  float: right; clear: right;
  margin: 0 0 1em 1.5em;
  padding: 0.5em;
  border: 1px solid #a2a9b1;
  background: #f8f9fa;
  font-size: 0.875em;
  width: 22em;
  line-height: 1.5;
}
.infobox th { font-weight: bold; padding: 0.2em 0.5em; text-align: left; }
.infobox td { padding: 0.2em 0.5em; }
@media (max-width: 799px) {
  .infobox { float: none; width: 100%; margin: 0 0 1em 0; }
}
```

**Syntax for article authors:**
````
```infobox
Name: PointSav Digital Systems
Founded: 2024
Jurisdiction: British Columbia
```
````

---

### B3: Navbox native block type

**File:** `src/render.rs`

Match `CodeBlock` with `info = "navbox"` (or `BlockDirective` with name `"navbox"` post-upgrade). Parse YAML body:

```yaml
title: PointSav Platform
groups:
  Applications: "[[app-mediakit-knowledge]], [[app-mediakit-marketing]]"
  Services: "[[service-slm]], [[service-content]]"
```

Render as a collapsible `<table class="navbox">`. The existing wikilink resolver processes links inside navbox group text.

**JS (`wiki.js`):** Add `initNavboxes()` — if `document.querySelectorAll('.navbox').length >= 2`, collapse all navboxes by default. A `[show]` / `[hide]` toggle per navbox expands/collapses the body.

**CSS:** Horizontal table layout with dark header band, light body, `border-collapse: collapse`, matching Vector 2022 navbox appearance.

---

## Sprint C — Special pages + Talk namespace (est. 1 session)

### C1: `Special:RecentChanges`

**File:** `src/server.rs`

New handler `recent_changes()` — call `git log --pretty=format:"%H|%ae|%ad|%s" --date=short -n 100` across the content directory. Parse output into a `Vec<ChangeEntry>` struct and render an HTML table:

```
| Time | Article | Author | Summary |
```

Filter controls (by namespace, by author) can be added in Sprint D. Add route: `.route("/special/recent-changes", get(recent_changes))`.

---

### C2: `Special:AllPages`

New handler `all_pages()` — collect all topic files via `collect_all_topic_files()`, sort alphabetically by slug, group by first letter (A–Z), render as a multi-column alphabetical directory matching Wikipedia's `Special:AllPages` layout.

Add route: `.route("/special/all-pages", get(all_pages))`.

---

### C3: `Special:Statistics`

New handler `site_statistics()` — compute:
- Total article count (from `collect_all_topic_files()`)
- Category count (from `bucket_topics_by_category()`)
- Most recently edited article (from `last_edited:` frontmatter scan)
- Redlink count (walk all articles, count `wiki-redlink` hrefs — existing machinery in `/wanted` handler)

Render as a single-page stat display.

---

### C4: Talk namespace

**File:** `src/server.rs`

Add two routes:
- `GET /talk/{*slug}` — serve `<content_dir>/talk/<slug>.md` if it exists, or render an empty talk page stub with a "Start discussion" button
- `POST /talk/{*slug}` — append a new section to the talk page file using the existing edit machinery

Update `wiki_chrome()` Article/Talk tab rendering: the Talk tab should be fully linked (not disabled). Add an edit link on the Talk page. The talk page renders with the same chrome but with a `talk-namespace` CSS class on the body.

Talk pages are stored as standard Markdown files in `<content_dir>/talk/`. They are indexed by Tantivy but not shown in category listings.

---

## Sprint D — Visual diff + search improvements (est. 1 session)

### D1: Two-column word-level diff

**File:** `src/server.rs` (the `/diff/{slug}` handler)

Replace the current unified diff output with a two-column HTML table using `similar::TextDiff`:

```rust
use similar::{ChangeTag, TextDiff};

let diff = TextDiff::from_lines(old_text, new_text);
let mut rows = String::new();
for op in diff.ops() {
    for change in diff.iter_inline_changes(op) {
        let (old_cell, new_cell) = match change.tag() {
            ChangeTag::Equal  => { /* render equal rows */ }
            ChangeTag::Delete => { /* render deleted rows in red */ }
            ChangeTag::Insert => { /* render inserted rows in green */ }
        };
        rows.push_str(&format!("<tr>{old_cell}{new_cell}</tr>"));
    }
}
```

For word-level emphasis within changed lines: `change.values()` returns `&[(bool, &str)]` where the bool marks the changed word segment. Wrap emphasized segments in `<del>` (red background) or `<ins>` (green background).

**CSS:**
```css
.diff-table { width: 100%; border-collapse: collapse; font-family: monospace; font-size: 0.875em; }
.diff-table td { padding: 0.1em 0.5em; vertical-align: top; width: 50%; white-space: pre-wrap; }
.diff-del { background: #ffd7d5; }
.diff-ins { background: #ccffd8; }
del { background: #ff9999; text-decoration: none; }
ins { background: #66ff66; text-decoration: none; }
```

---

### D2: Search autocomplete dropdown

**File:** `src/server.rs`

Add a new lightweight endpoint:
```
GET /api/complete?q={prefix}
→ JSON array of {slug, title} for articles whose title starts with or contains q
→ max 8 results, ranked by title match quality
```

Implementation: filter the in-memory Tantivy index results using a prefix query on the title field. Return JSON.

**File:** `static/wiki.js`

Add `initSearchAutocomplete()` — attach to the search `<input>`, debounce 150ms, fetch `/api/complete?q={value}`, render a dropdown `<ul>` below the input, keyboard navigate with Up/Down/Enter.

---

### D3: `Special:Statistics` (moved here if deferred from C)

See Sprint C item C3 above.

---

## Sprint E — Leapfrog 2030 layer (est. 2–3 sessions)

### E1: Citation Authority Ribbon

Per-article IVC status rendered as a coloured ribbon at the top of the article body. Requires:
1. Resolve all `cites: [list]` frontmatter values against `citations.yaml`
2. For each citation, check for a `verified_date:` and `status: verified|unverified|contested` field in the YAML
3. Render a small ribbon (`green` / `amber` / `red`) below the title indicating overall verification state
4. Phase 7 will extend this to per-claim inline ribbons (citation-authority ribbon DESIGN-COMPONENT is already drafted in `drafts-outbound/`)

### E2: Research Trail Footer

Per-article provenance block rendered as a collapsible `<details>` at the end of the article body. Reads five frontmatter fields already present in `foundry-draft-v1` schema: `research_trail.method`, `research_trail.depth`, `research_trail.confidence`, `research_trail.limitations`, `research_trail.date`. Renders as a structured section below "External links" and above the navboxes.

### E3: Doorman editor integration

Wire the existing 501-stub endpoints `POST /api/doorman/complete` and `POST /api/doorman/instruct` to the `app-orchestration-command` Doorman proxy at `http://localhost:9100/v1/chat` (or the production Doorman URL from config). Gated on a new `--doorman-url` CLI flag / `WIKI_DOORMAN_URL` env var. When not set, stubs return 501 as today.

### E4: Freshness Ribbon

Per-article temporal accuracy indicator. Reads `last_edited:` frontmatter date. Computes age bucket (< 30 days = current, 30–180 days = recent, > 180 days = aging, > 365 days = stale). Renders a small coloured badge in the article footer alongside the "Last edited" date.

---

## Deployment sequence after each sprint

```bash
# From inside app-mediakit-knowledge/
cargo build --release
sudo systemctl stop local-knowledge-documentation.service \
                    local-knowledge-projects.service \
                    local-knowledge-corporate.service
sudo install -m 755 target/release/app-mediakit-knowledge /usr/local/bin/app-mediakit-knowledge
sudo systemctl start local-knowledge-documentation.service \
                     local-knowledge-projects.service \
                     local-knowledge-corporate.service
# Verify:
curl -s -o /dev/null -w "%{http_code}" https://documentation.pointsav.com/
curl -s -o /dev/null -w "%{http_code}" https://projects.woodfinegroup.com/
curl -s -o /dev/null -w "%{http_code}" https://corporate.woodfinegroup.com/
```

---

## Summary: Estimated sprint sessions to full parity + Leapfrog

| Sprint | Items | Est. sessions | Outcome |
|---|---|---|---|
| A | 7 quick wins | 1 | ~83% Wikipedia parity |
| B | comrak upgrade + infobox + navbox | 1–2 | ~90% Wikipedia parity |
| C | Special pages + Talk namespace | 1 | ~93% Wikipedia parity |
| D | Two-column diff + search autocomplete | 1 | ~95% Wikipedia parity |
| E | Citation ribbon + research trail + Doorman + freshness | 2–3 | Leapfrog 2030 complete |

Total: 6–8 Task Claude sessions.
