# app-mediakit-knowledge

Wikipedia-pattern HTTP knowledge wiki for `os-mediakit`. Serves the
`content-wiki-documentation` repository as a fully navigable wiki at
`documentation.pointsav.com`. Built in Rust. No database. No runtime
dependencies beyond the compiled binary.

| | |
|---|---|
| **Licence** | Apache 2.0 |
| **Platform layer** | Independent system — no Totebox or OrchestrationOS dependency |
| **Content source** | `github.com/pointsav/content-wiki-documentation` |
| **Infrastructure** | `fleet-infrastructure-cloud` (GCP e2-micro) |
| **Architecture reference** | `content-wiki-documentation/architecture/app-mediakit-knowledge.md` |

---

## For Claude Code — build order

The scaffold is complete. All types, module boundaries, and function
signatures are defined. Every incomplete function has a numbered `TODO`
list describing exactly what to implement. Work top to bottom.

### Step 1 — Renderer (pure functions, no I/O, easy to test first)

**`src/renderer/toc.rs` — complete `extract()`**

The `TODO` block contains the full algorithm as commented pseudocode.
Implement the heading extraction pass:

- Match `Event::Start(Tag::Heading)` for h2 and h3
- Collect text content between Start and End events
- Build anchor id via `slugify(&heading_text)`
- Inject `Event::Html(format!("<span id=\"{id}\"></span>").into())` before the End event
- Call `insert_entry(&mut toc, ...)` to build the tree
- The existing `insert_entry` and `slugify` functions are already implemented

Run: `cargo test renderer::toc`

**`src/renderer/footnotes.rs` — complete `process_inline()`**

Replace the passthrough loop. Match `Event::FootnoteReference(label)` and
emit a superscript anchor link. Record the inline anchor id in the
`InlineAnchors` map for the back-arrow bibliography. The `TODO` block has
the exact HTML structure required.

Run: `cargo test renderer::footnotes`

**`src/renderer/markdown.rs` — Stage 4 syntax highlighting**

Intercept `Event::Start(Tag::CodeBlock(kind))` and `Event::End(TagEnd::CodeBlock)`
events. Collect the code text between them, run it through `syntect`'s
`ClassedHTMLGenerator`, and emit the highlighted HTML as `Event::Html`.
Use `syntect::parsing::SyntaxSet::load_defaults_newlines()`.

Run: `cargo test renderer`

---

### Step 2 — Search

**`src/search/index.rs` — complete `autocomplete()`**

Replace the fallback `search()` call with a proper Tantivy prefix query on
the title field. Use `tantivy::query::RegexQuery` with pattern
`format!("{}.*", regex::escape(partial))` on the title field. This keeps
autocomplete fast and title-scoped.

Also implement excerpt generation in `search()`: after collecting hits,
open the stored body text, find the first occurrence of any query term,
and extract a 160-character window centred on that position.

Run: `cargo test search`

---

### Step 3 — HTTP handlers

**`src/server/handlers.rs` — implement all stubs**

Each handler has a numbered `TODO` list. Implement in this order:

1. `home` — render `index.md`; pattern is identical to `article` but the
   slug is always `"index"` and the path is `content_path/index.md`
2. `article` — the core read path. Cache lookup → render → cache insert → template
3. `category_index` — walk `<category>/` for all `.md` files to build the
   article list; render `_index.md` as the body
4. `search_results` — call `state.search.search()`, render `search.html`
5. `search_autocomplete` — call `state.search.autocomplete()`, return `Json`
6. `current_head` — one line: return `head_sha(&state.config)` as plain text
7. `preview` — read body bytes as UTF-8, call `renderer::render()` with an
   empty `PageIndex`, return `Html(article.body_html)`
8. `editor_load` / `editor_submit` — last, requires MBA auth integration

**Building the PageIndex for wikilink resolution:**

The handlers need a `PageIndex` to pass to the renderer. Add it to `AppState`
as `Arc<RwLock<wikilinks::PageIndex>>` and build it at startup in `main.rs`
by walking the content directory and reading each file's front matter `slug`,
`title`, `category` fields.

---

### Step 4 — Wire AppState PageIndex

In `src/main.rs`:

```rust
// After SearchIndex::build():
let page_index = Arc::new(tokio::sync::RwLock::new(
    build_page_index(&config.content_path)?
));
```

Add `build_page_index(path: &Path) -> Result<wikilinks::PageIndex>` in
a new `src/page_index.rs` module. Walk the content directory, parse front
matter from each `.md` file, insert into the `PageIndex` map.

Update `AppState` to include `page_index: Arc<RwLock<wikilinks::PageIndex>>`.
Update the sync daemon to rebuild the index after a git pull advances HEAD.

---

## Running locally

```bash
# Clone content repository alongside the monorepo
git clone https://github.com/pointsav/content-wiki-documentation \
          ../content-wiki-documentation

CONTENT_PATH=../content-wiki-documentation cargo run
# → http://localhost:3000
```

## Environment variables

| Variable | Default | Notes |
|---|---|---|
| `CONTENT_PATH` | *(required)* | Path to content-wiki-documentation |
| `GIT_REMOTE` | `origin` | Remote to pull from |
| `SYNC_INTERVAL` | `60` | Seconds between git pulls |
| `CACHE_SIZE` | `256` | LRU page cache entry limit |
| `EDITOR_ENABLED` | `false` | Enable browser editor |
| `EDITOR_AUTH` | *(required if editor on)* | MBA auth endpoint URL |
| `BIND_ADDR` | `0.0.0.0:3000` | HTTP bind address |
| `SITE_TITLE` | `PointSav Documentation` | Header wordmark and `<title>` |
| `BASE_URL` | `http://localhost:3000` | Canonical link generation |

## ADR compliance

| ADR | Rule | This component |
|---|---|---|
| SYS-ADR-07 | Structured data never routes through AI | Renderer, search, and sync are fully deterministic. AI is never invoked. |
| SYS-ADR-10 | F12 is the mandatory human checkpoint | Not applicable — this is a read path. The editor commit path requires verified human identity from MBA auth. |
| SYS-ADR-19 | Automated AI publishing to verified ledgers is prohibited | Every commit from `editor::commit::apply()` must carry a verified human author from the MBA auth layer. The handler enforces this before calling `apply()`. |

## File layout

```
app-mediakit-knowledge/
├── Cargo.toml
├── .gitignore
├── README.md
├── src/
│   ├── main.rs                  ✓ entry point, AppState, task spawn
│   ├── config.rs                ✓ environment config
│   ├── renderer/
│   │   ├── mod.rs               ✓ RenderedArticle, ArticleMeta, Cache
│   │   ├── markdown.rs          ✓ pipeline (stage 4 TODO: syntect)
│   │   ├── toc.rs               ← TODO: complete extract()
│   │   ├── footnotes.rs         ← TODO: complete process_inline()
│   │   └── wikilinks.rs         ✓ [[slug]] resolution
│   ├── search/
│   │   ├── mod.rs               ✓
│   │   └── index.rs             ← TODO: autocomplete(), excerpt
│   ├── sync/
│   │   ├── mod.rs               ✓ background task
│   │   └── git.rs               ✓ libgit2 fast-forward pull
│   ├── editor/
│   │   ├── mod.rs               ✓ EditSubmission, EditResult
│   │   └── commit.rs            ✓ git commit path
│   └── server/
│       ├── mod.rs               ✓ router assembly
│       ├── routes.rs            ✓ route table
│       ├── handlers.rs          ← TODO: implement all stubs
│       └── templates.rs         ✓ Tera render helpers
├── templates/
│   ├── article.html             ✓ Wikipedia-faithful article layout
│   ├── category.html            ✓ category index
│   ├── search.html              ✓ search results
│   └── editor.html              ✓ two-pane section editor
├── static/
│   ├── style.css                ✓ Vector 2022 stylesheet
│   └── wiki.js                  ✓ TOC tracking, search, editor JS
└── tests/
    └── fixtures/
        ├── index.md             ✓ wiki home fixture
        └── architecture/
            └── os-totebox.md    ✓ article fixture
```
