---
artifact: brief
schema: foundry-brief-v1
brief-id: project-design-app-privategit-design
status: active
owner: project-design
cluster: project-design
created: 2026-06-06
updated: 2026-06-13
---

# BRIEF: design.pointsav.com — app-privategit-design

## Current state (2026-06-06)

Old v0.1.0 binary **restored** from GCP snapshot `20260605040103`.
- `sha256: 3f8b35b58deba1a7aa2673af73aa2556bd30ef737cdb6d679390dbfc45749509`
- `curl http://localhost:9094/healthz` → `{"status":"ok","service":"app-privategit-design","version":"0.1.0"}`
- design.pointsav.com is serving the full design system again

Recovered binary file saved at:
`/srv/foundry/clones/project-design/app-privategit-design-recovered`
(keep this — it is the only surviving copy of the old source binary; gitignored in archive)

Binary ledger still shows the June 5 deploy entry. Command needs to append
a recovery ledger entry to `data/binary-ledger/app-privategit-design.jsonl`.

---

## Old source status

Old binary source was **never committed to git** anywhere (confirmed by full
object-database scan of all 7 Totebox clones + all remotes). The only path
back was the GCP snapshot — now done.

The old scaffold pre-dating Option B was `app-privategit-design-system` — a 4-line
architectural stub (`src/lib.rs`), never a web server.

---

## MVP delivered (2026-06-08, commit e16545e8)

**design.pointsav.com now shows org-chart-tokens in the Elements sidebar with visual specimens.**

What shipped:
- `app-privategit-design/src/main.rs` — rewritten (250 lines): reads `DESIGN_VAULT_DIR`,
  discovers `elements/` subdirs dynamically (all 5 appear in nav), discovers tabs from `.md`
  files, renders markdown via pulldown-cmark (ENABLE_TABLES + HTML pass-through), Carbon light
  shell (sticky header, 256px sidebar, tab bar, page title)
- `app-privategit-design/Cargo.toml` — pulldown-cmark 0.11 added
- Vault: `deployments/vault-privategit-design-1/elements/org-chart-tokens/overview.md` updated
  with 13 inline HTML figure swatches (210×110px, border/bg per token, IBM Carbon family labels)
- Binary deployed: `local-design.service` active; smoke tested; all 5 elements in nav

Stage 6 pending (msg: project-design-20260608-stage6-app-privategit-design in outbox).

---

## Follow-up items (post-MVP)

- Mirror org-chart-tokens/overview.md swatch content to `pointsav-design-system` sub-clone
- Extend SECTIONS in main.rs to also serve `components/` (currently elements-only)
- Add remaining tabs where vault has usage/style/code/accessibility .md files
- Full Carbon redesign per §3 below (templates, nav-items.yaml, search, prev/next pager)

---

## Full redesign task (future work)

Build a proper Carbon-style replacement for `app-privategit-design` that serves
the full deployment vault. The new binary will SUPERSEDE the recovered v0.1.0
(not replace Option B — that prototype is too minimal).

### IBM Carbon architecture findings (Opus agent analysis, 2026-06-06)

**Route shape:**
```
GET /{section}/{slug}/{tab}   → vault/{section}/{slug}/{tab}.md
GET /{section}/{slug}         → first tab (overview.md or first .md file)
GET /{section}                → section landing page
GET /                         → home / nav index
GET /healthz                  → health check
GET /search                   → client-side search (serve MiniSearch/Lunr JSON index)
```
Trailing-slash normalise. Tabs derived automatically by listing sibling `.md` files —
no per-page config. One file = no tab bar.

**Nav from one YAML** (`nav-items.yaml` at vault root):
```yaml
- title: Elements
  pages:
    - title: Color
      path: /elements/color/overview
    - title: Motion
      path: /elements/motion/overview
    ...
- title: Components
  pages:
    - title: Button
      path: /components/button/usage
    ...
- title: About
  pages:
    - path: /about/what-is-pointsav-design
...
```
This single file drives: sidebar tree, active-item highlighting, prev/next pager.
Tab ordering also explicit in nav (first tab = nav entry path).

**Deployment vault sections to route** (21+ components, all sections):
```
elements/       → /elements/{slug}/{tab}
components/     → /components/{slug}/{tab}   (tab set: usage, style, code, accessibility)
about/          → /about/{page}
designing/      → /designing/{page}
developing/     → /developing/{page}
guidelines/     → /guidelines/{category}/{page}
help/           → /help/{page}
research/       → /research/{slug}
exports/        → /exports/{slug}  (or static file serve for .json/.css)
themes/         → /themes/{slug}
tokens/         → /tokens/{slug}
```

**Standard component tab set** (matches Carbon):
`usage.md`, `style.md`, `code.md`, `accessibility.md`

**Token/element pages** — rendered specimens, not tables:
- Color → swatch grid (pass-through HTML in markdown, or axum shortcode expander)
- Typography → rendered type specimen table
- Spacing → labelled measurement bars
- Motion → duration/easing with animated demo
- org-chart-tokens → token table + usage context

**Search:** build MiniSearch/Lunr JSON index from vault at startup; serve as
`/search-index.json`; search runs in-browser, no backend.

**Aesthetic (Carbon-style):**
- Light shell: white content area + light-gray 256px left sidebar + thin top header bar
- NOT dark sidebar — Carbon is predominantly light (white / Gray-10)
- IBM Plex-style sans for body, monospace for code
- Generous whitespace, wide line-length, large H1 page titles
- Copy-to-clipboard on code blocks
- "Edit on GitHub" link per page (configurable repo URL)
- Breadcrumbs implicit via persistent sidebar (not separate breadcrumb component)
- Prev/next pager at page bottom (from nav YAML ordering)

**Tech stack for new binary:**
- axum HTTP server (same as now)
- `pulldown-cmark` or `comrak` for markdown → HTML rendering
- Tera or minijinja for HTML templates
- `serde_yaml` for `nav-items.yaml` parsing
- Vault read at startup; watch for changes (optional inotify)
- Static assets (CSS, fonts, search JS) embedded via `include_str!` / `rust-embed`
- No JS framework — vanilla JS for search + copy buttons only

**Env vars for new binary:**
```
DESIGN_VAULT      = /srv/foundry/deployments/vault-privategit-design-1
DESIGN_BIND       = 127.0.0.1:9094
DESIGN_PUBLIC_URL = https://design.pointsav.com
DESIGN_REPO_URL   = https://github.com/pointsav/pointsav-design-system  (edit links)
```
(Remove `DESIGN_VAULT_DIR` from service — replaced by `DESIGN_VAULT` pointing at vault root)

### Key files to create/modify

| File | What |
|---|---|
| `app-privategit-design/src/main.rs` | Full rewrite (~500-800 lines) |
| `app-privategit-design/src/vault.rs` | Vault walker: nav YAML parser, section/slug/tab discovery |
| `app-privategit-design/src/render.rs` | Markdown → HTML, shortcode expansion |
| `app-privategit-design/src/search.rs` | MiniSearch index builder |
| `app-privategit-design/templates/` | Tera/minijinja templates: shell, page, component, element |
| `app-privategit-design/static/` | CSS (Carbon-style light shell), search.js, copy.js |
| `vault-privategit-design-1/nav-items.yaml` | New file: ordered nav for all sections |
| `/etc/systemd/system/local-design.service` | `DESIGN_VAULT_DIR` → `DESIGN_VAULT` (vault root) |

### Cargo dependencies to add

```toml
comrak = "0.21"          # CommonMark + GFM markdown renderer
tera = "1"               # or minijinja = "1" for templates
serde_yaml = "0.9"       # nav-items.yaml parsing
rust-embed = "8"         # embed static assets into binary
notify = "6"             # optional: vault hot-reload
```

---

## Pending signals to Command

- Binary ledger entry needed: recovery of sha256 `3f8b35b5...` from snapshot `20260605040103`
  (msg-id: `project-design-20260606-binary-recovery-v0.1.0` in outbox — pending pickup)
- Stage 6 still pending for monorepo `06c60c6d` (Option B code) + design-system `1b5ef66` (4 DESIGN-RESEARCH files)
- `DESIGN_VAULT_DIR` → `DESIGN_VAULT` service fix (low priority, resolved in redesign)
- master_cosign still needed for DESIGN-TOKEN-POINTSAV-icon-tab-steel
  (msg-id: `project-design-20260605-cosign-request-icon-tab-steel` in outbox — pending)

---

## v0.2.0 clean-sheet plan (2026-06-13 session)

**Trigger:** 12-point operator request for a full rewrite informed by deep research. The current
12KB single-file binary (v0.1.0) is a prototype. v0.2.0 is the institutional-grade platform.

### Confirmed architectural decisions (Q&A 2026-06-13)

| # | Decision | Confirmed |
|---|---|---|
| 1 | AI integration | Option B: isolated browser session using user's own Claude.ai login via SSE bridge. App never holds API key. Multi-model by design (same SSE interface for OLMo + Claude + others). |
| 2 | moonshot-* stubs | Create stub crates now: Cargo.toml + src/lib.rs + README.md per crate. Feature-flagged so 3rd party dep → sovereign swap is a single Cargo.toml edit. |
| 3 | WYSIWYG editor scope | Both content types: markdown (split-pane prose+preview) and DTCG token JSON (structured form fields). Schema detected from frontmatter/$type. |
| 4 | Workflow output | Research + BRIEFs + detailed rewrite spec. No code written yet. Spec detailed enough to start coding immediately in the next session. |
| 5 | project-marketing/bim BRIEFs | Read for knowledge mining only. All output goes into this consolidated project-design BRIEF. No cross-archive writes. |
| 6 | Model budget | All 10 research tracks on OPUS. Full depth everywhere. |
| 7 | DESIGN-BUNDLE handling | New schema type in v0.2.0: bundle renderer showing all 3 members (token + stylesheet + template) as cohesive unit with download-as-zip, preview, and per-member editing. |
| 8 | Q8 not answered | User called SHUTDOWN before Q8 (DESIGN-BUNDLE ratification). Carry forward. |

### "We Own It" dependency discipline

Every 3rd party crate used must have a named moonshot-* replacement target in /pointsav-monorepo.

| 3rd party | Purpose | moonshot-* target | Action |
|---|---|---|---|
| askama | Compile-time HTML templates | moonshot-template | stub needed |
| rusqlite | Token index cache + sessions | moonshot-database | wire to existing |
| syntect | Syntax highlighting | moonshot-highlight | stub needed |
| tantivy | Full-text token search | moonshot-index | wire to existing |
| serde_json | JSON parsing | moonshot-toolkit | wire to existing |
| notify | Filesystem watch | moonshot-fs-watch | stub needed |
| axum | HTTP framework | moonshot-http (long-term) | track in BRIEF |
| pulldown-cmark | Markdown | moonshot-markup (long-term) | track in BRIEF |

Stubs carry: Cargo.toml + src/lib.rs (empty pub mod) + README.md explaining sovereign mission.

### DESIGN-BUNDLE artifact type (discovered 2026-06-13 in project-documents)

A DESIGN-BUNDLE is a manifest grouping 3 interdependent pieces for document reproduction:
1. **TOKEN** — DTCG 2025.10 JSON (all numeric values: page geometry, typography, color)
2. **STYLESHEET** — DESIGN-COMPONENT CSS consuming token values
3. **TEMPLATE** — DESIGN-COMPONENT DOM skeleton with wording-agnostic placeholder system

**Two archetypes in the wild:**
- Prospectus Bundle (NI 41-101): external linked stylesheet, multi-section, WeasyPrint 61.1
- Subscription Agreement Bundle (accredited investor form): inlined CSS, single self-contained file

**Status:** 8 constituent artifacts in project-documents drafts-outbound/. Artifact type is
PROVISIONAL — pending Command ratification in artifact-registry. Many more bundles incoming
(legal, financial, corporate). design.pointsav.com must handle serving any bundle type.

**v0.2.0 bundle renderer requirements:**
- Detects bundle manifest (bundle.json in vault directory)
- Displays all 3 members with metadata, download-as-zip, render preview (PDF iframe)
- Per-member editing: token = structured form, stylesheet = CodeMirror, template = prose+preview
- Sidebar token browser surfaces tokens/legal/ namespace alongside existing namespaces
- Vault structure: bundles/<name>/bundle.json manifest + member files

### Pending Workflow (not yet executed)

A Workflow with 11 OPUS research tracks is planned but not started.
Plan captured at: /home/jennifer/.claude/plans/structured-squishing-hopcroft.md

Research tracks:
1. IBM Carbon v11 current state
2. WYSIWYG dual-mode editor patterns (2024-2025)
3. Real-time sidebar navigation for deep token trees
4. 2030 leapfrog Design System CMS vision
5. AI-in-browser component editing (Option B: browser session, not API)
6. Rust sovereign ownership + moonshot-* stack planning
7. Mobile compatibility for design system documentation
8. Marketing pages as design system output
9. DTCG 2025.10 schema standardization (complete token namespace spec)
10. Contributor workflow for design + code
11. DESIGN-BUNDLE serving: how to display multi-file artifact bundles on a portal

Output: consolidated project-design BRIEF + rewrite spec detailed enough to code from.
Reading: project-marketing BRIEFs + project-bim BRIEFs for knowledge mining (no cross-archive writes).
