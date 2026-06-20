---
artifact: brief
schema: foundry-brief-v1
brief-id: project-design-app-privategit-design
title: "app-privategit-design — 2030 Design System Platform"
status: active
owner: project-design
created: 2026-06-06
updated: 2026-06-20
---

# BRIEF — app-privategit-design

## Context

**Current v0.1.0 state (verified against the repo, not memory):**

1. `app-privategit-design/Cargo.toml` declares **v0.1.0**; `src/main.rs` is a **single 293-line file** with `SECTIONS = ["elements"]` — one axum binary, one hard-coded artifact section, path-traversal guards inline (`slug.contains("..")`).
2. The token source `pointsav-design-system/tokens/dtcg-bundle.json` is **40 KB across four top groups** (`primitive` / `semantic` / `component` / `workplace`) and uses **only 6 of the 13 stable DTCG `$type` values** (`color`, `dimension`, `number`, `fontWeight`, `fontFamily`, `duration`, `cubicBezier`), plus an invalid `$type: "string"` (4×) and an invalid `$type: "boolean"` (in `dtcg-vault`). Composite types — `typography`, `shadow`, `border`, `transition`, `gradient`, `strokeStyle` — are absent.
3. Moonshot replacement stubs **`moonshot-index`** and **`moonshot-database`** exist as bare `lib.rs` files; **`moonshot-template`, `moonshot-highlight`, `moonshot-fs-watch`, `moonshot-markup` do not exist** (verified absent).

**v0.2.0 mission:** Rebuild app-privategit-design from a 293-line single-file viewer into the sovereign, self-hosted, DTCG-native design-system platform that occupies the unbuilt "design system as CMS" slot — one DTCG token graph that simultaneously drives UI components, token-browser documentation, and marketing pages, served from an axum SSR chassis with minimal JS, Carbon CSS-variable absorption, schema-aware rendering, a server-rendered + SSE-morph live sidebar, a server-rendered WYSIWYG edit overlay, and a local-inference-first AI bridge (OLMo via Doorman, SYS-ADR-07-safe). The architecture mirrors the ratified project-marketing chassis split and the project-bim module layout, and advances the "We Own It" discipline by making search and FS-watch sovereign in v0.2.0 itself.

## Scope

**This BRIEF covers:** the v0.2.0→v0.3.0 architecture for `app-privategit-design` — multi-file Rust module tree, schema-aware rendering (v0.3.0 adds Marketing + Bundle), WYSIWYG dual-mode editing, real-time sidebar, AI bridge SSE endpoint, Carbon CSS integration, mobile CSS, DTCG token-schema gaps (v0.3.0: composite groups added), the marketing-pages render pipeline (design.pointsav.com), the contributor workflow, the We-Own-It dependency map, and the 2030 platform vision.

**This BRIEF explicitly does NOT cover:**
- **DESIGN-BUNDLE ratified 2026-06-20.** `schema/bundle.rs` implemented in v0.3.0 — identity header, metadata panel, member list with role chips, body prose, ZIP download via `/elements/:slug/download`. **DTCG multi-file split and Style Dictionary resolver** remain deferred (v0.4.0).
- **Stage 6 promotion / canonical writes.** DTCG multi-file split, resolver, and any Style Dictionary build wiring stage as DESIGN drafts or commit in *this* archive; promotion to canonical is a Command-Session route via outbox (per `scope-discipline.md`).
- **Figma / MCP / Code Connect as a code source of truth.** Import-only, draft-tier, hand-verified — never generates committed code.

## Decisions locked

| # | Decision | Rationale (from research) |
|---|---|---|
| **D1** | **Multi-file Rust structure (chassis pattern).** Decompose the 293-line `main.rs` into a module tree (`config`/`state`/`vault`/`schema/`/`routes/`/`render/`/`templates/`/`static/`). One axum binary, ownership-clean modules. | Mirrors the ratified project-marketing `app-mediakit-shell` / `app-mediakit-marketing` chassis split and the project-bim `app-privategit-bim` `src/` layout (locked 2026-06-14). `vault.rs` owns the existing path-traversal guard and extends `SECTIONS` from `["elements"]` to the schema-aware set; `schema/mod.rs` is the dispatch point replacing the hard-coded elements-only assumption. |
| **D2** | **Schema-aware rendering.** Renderer dispatches on artifact schema via cheapest-signal-first detection (extension → `$schema` → frontmatter → Content-Type). Schemas: COMPONENT, TOKEN, RESEARCH, MARKETING confirmed; BUNDLE deferred. | track-wysiwyg §5 (detection layering); track-dtcg-schema (`$schema` = `designtokens.org/schemas/2025.10/format.json`). BUNDLE is reserved route only, pending ratification (D2 table below). |
| **D3** | **WYSIWYG dual-mode = server-rendered "edit overlay" from a shared AST** — NOT ProseMirror/TipTap/Lexical/CodeMirror as the primary surface. `contenteditable` for prose blocks; native `<input>`/`<select>` for DTCG token values; ~7 KB Alpine/vanilla for the view↔edit toggle + save POST. | track-wysiwyg §1–§6: minimizes JS footprint (~7 KB vs TipTap 56 KB vs Lexical 469 KB); the only two surfaces needed (prose + token forms) are native primitives; keeps the vault file canonical (git/Storybook model, not hosted-DB). **Fallback:** TipTap scoped to prose body only (~56 KB, lazy, behind `?edit=1`, never on the read path). Token editing stays plain form fields regardless. **Note divergence from project-bim, which chose CodeMirror 6 — see §8.** |
| **D4** | **Real-time sidebar = server-rendered `<nav>` in axum + SSE morph-patch reload + `content-visibility:auto`.** No virtualization library. SSE → HTML fragment → morph by `id` (idiomorph/Datastar), NOT SSE → JSON → manual diff. | track-sidebar §1–§6. `notify`/inotify watches vault → `tokio::sync::watch`/`broadcast` → axum `Sse` re-renders `<nav>` fragment → client morphs by id (preserves scroll + expand). Native `EventSource` reconnect/`Last-Event-ID` free. Rows: `content-visibility:auto; contain-intrinsic-size:auto 32px` (in DOM/a11y tree/Ctrl-F intact, ~7× render saving). Collapsed branches `hidden="until-found"`. Filter client-side; active via server-stamped `aria-current="page"`. |
| **D5** | **AI bridge = single normalized-SSE relay at `POST /ai/session`** with two upstream adapters behind one `UpstreamModel` trait: `DoormanOlmo` (local, `127.0.0.1:9080`, free, SYS-ADR-07-safe) and `ClaudeCloud`. Browser sends only the highlighted selection + structural context descriptor + schema — never the whole file, never a key. | track-ai-integration §1–§6. **Architectural correction (flagged, not silently built):** the claude.ai "MCP session relay" does NOT exist as a supported surface and cookie-replay is ToS-violating/blocked. Replaced with: relay to the Anthropic Messages API using the user's own credential held only in the per-user browser session. Selection-only context + prompt caching of the stable design-system prefix + diff-back (not full rewrite). **Structured/strict tool use, NOT computer use** (10–100× cheaper, one turn). SYS-ADR-07/10/19 maintained (human reviews diff, F12 approve). Verified: Doorman responds `ok` at `127.0.0.1:9080/healthz`. |
| **D6** | **Carbon CSS integration = absorb `--cds-*` custom properties directly; selectively load individual `@carbon/web-components` from CDN only where interactivity is real.** No npm, bundler, React, or build step. | track-carbon-v11 §1–§7. Serve one compiled `tokens.css` (theme via `data-carbon-theme`); portal CSS against `var(--cds-text-primary)` etc. Interactive widgets via `<script type="module" src="…/v2/…">` (the `/v2/` segment required for v11; latest 2.56.0). **Caveat:** `cds-side-nav` is ~2-level, unvirtualized — do NOT use for the deep token tree (server-render styled with `cds--side-nav__*`, per D4). `cds-ai-label` (ex-Slug, stable @carbon/react 1.66.0) marks AI content. Carbon does NOT yet export `.tokens.json` (issue #22437) — consume `--cds-*`, do not plan to ingest a Carbon DTCG file. |
| **D7** | **Mobile CSS = single hand-authored stylesheet on four Baseline primitives** — cascade layers (`@layer reset, tokens, base, components, utilities`), custom properties, container queries, sticky positioning — plus ~12 lines of drawer-toggle JS. No framework, no build. | track-mobile-css §1–§7. Sidebar = overlay drawer + scrim on phones (Carbon/M3/Spectrum all drop the persistent sidebar on phones), flips to sticky column at `≥64rem`. Token tables = sticky-first-column horizontal scroll (stacked-card only at 6+ columns). Code blocks rely on native iOS 13+ momentum (`overflow-x:auto` + `overscroll-behavior-x:contain`). 48px touch targets (Apple 44 + Material 48). `tokens` layer below `components` so DTCG values never need `!important`. |
| **D8** | **Search and FS-watch are sovereign in v0.2.0 itself** — build the in-memory inverted-index (`moonshot-index`, S effort) instead of adding tantivy; use raw `inotify` (Linux-only target) instead of adding `notify`. | track-rust-sovereign: tantivy is overkill at ~1,500 tokens; inotify is sufficient for a Linux-only deployment. Both are small enough to be sovereign day one, removing tantivy and notify from the dep tree before they are ever added. |

## We Own It — dependency map

Per `feedback-we-own-it-dependency-discipline.md`: every 3rd-party dep gets a `moonshot-*` replacement stub, EXCEPT substrate crates (last rows). Effort/timeline from track-rust-sovereign.

| 3rd party crate | Version | Purpose | moonshot-* target | Effort | Timeline |
|---|---|---|---|---|---|
| **minijinja** (+ minijinja-autoreload) | 2.x / 2.19.0 | HTML templates w/ hot-reload (chosen over askama: runtime + autoreload beats compile-time for designer iteration) | `moonshot-template` (create; builds on `moonshot-parser`) | **L** | 2027 (ship on minijinja behind facade) |
| **syntect** (`default-fancy`/fancy-regex) | 5.3.0 | Syntax highlighting for code samples / token values; fancy-regex = no C/Oniguruma | `moonshot-highlight` (create) | M–L | 2026 Q4–2027 |
| **tantivy** | 0.26.1 | Full-text search — **DO NOT ADD.** Overkill at ~1,500 tokens; build in-memory inverted `HashMap` | `moonshot-index` (exists; build S version now) | **S** | **v0.2.0** (sovereign day one; tantivy never enters dep tree) |
| **rusqlite** (`bundled`) + spawn_blocking | 0.32 | Token-cache + session store (chosen over sqlx: thinner, closer to moonshot-database, no unused multi-DB layer) | `moonshot-database` (exists) | **L** | 2027+ (last; SQLite is highest-trust C lib) |
| **notify** | 8.1.0 | FS watch for vault change → SSE. **Linux-only target → raw `inotify` sufficient**; skip the abstraction | `moonshot-fs-watch` (create) | **S** | **v0.2.0** (sovereign day one on inotify; notify never added) |
| **pulldown-cmark** | 0.11 → **0.13** (bump) | Markdown render (DTCG docs, RESEARCH prose); pull-parser composes with highlight code-block hook | `moonshot-markup` (create; pull-parser + AST + token-directive layer) | M | 2027 |
| **zip** | 2.x | DESIGN-BUNDLE ZIP packaging — **ADDED v0.3.0** (BUNDLE ratified 2026-06-20); replaces async_zip/s-zip consideration | `moonshot-archive` (future target; zip is simple enough to own if needed) | M | deferred past v0.3.0 |
| **axum** | 0.7 → 0.8 | HTTP framework — **WON'T REPLACE** | none | — | pinned/audited |
| **tokio** | 1.x | Async runtime — **WON'T REPLACE** | none | — | pinned/audited |
| **tower-http** | 0.5 | CompressionLayer (gzip/Brotli for sidebar payload) — **WON'T REPLACE** (tower ecosystem) | none | — | pinned/audited |
| **serde / serde_json** | 1.x | (De)serialization contract for DTCG + AI deltas — **WON'T REPLACE** | none | — | pinned/audited |

**Won't-replace rationale (track-rust-sovereign §"Won't replace"):** tokio/axum/tower-http/serde define the *substrate*, not a swappable component — replacing them rewrites the foundation with no proportional sovereignty payoff. They carry **no C dependency, no vendor key custody, no network egress, no licensing gate** — the exact risks that motivate moonshot replacements (rusqlite's C SQLite, syntect's potential C Oniguruma, tree-sitter's C core). Pure-Rust, MIT/Apache, community-governed; treated as pinned, audited foundation crates (version-locked in Cargo.lock, reviewed at upgrade) — like the Rust compiler itself. The OXC/VoidZero ecosystem makes the same distinction: rewrite the *tools*, build them *on* the standard substrate. **Two areas (search, fs-watch) are small enough to be sovereign in v0.2.0**, removing tantivy and notify before they are ever added.

## v0.2.0 file structure

The 293-line single-file `main.rs` decomposes into:

```
app-privategit-design/src/
├── main.rs        — bind, AppState assembly, Router wiring, FS-watch + SSE spawn
├── config.rs      — env vars (DESIGN_VAULT_DIR, DESIGN_BIND, Doorman addr), defaults
├── state.rs       — AppState { vault, nav: Arc<…>, watch_tx, schema_registry, Db };
│                    holds the tokio::sync::watch/broadcast sender fed by the FS watcher
├── vault.rs       — discover_nav / discover_tabs / path-traversal guard (lifted from main.rs);
│                    owns the existing slug.contains("..") guards; extends SECTIONS from
│                    ["elements"] to the full schema-aware set; schema-aware file classification
│                    (extension + $schema sniff, track-wysiwyg §5)
├── schema/
│   ├── mod.rs     — ArtifactSchema enum (Component | Token | Research | Marketing); detect();
│   │                dispatch point replacing the hard-coded elements-only assumption
│   ├── component.rs — COMPONENT render-context builder (4-section tabs)
│   ├── token.rs     — TOKEN render-context builder (token-browser table)
│   ├── research.rs  — RESEARCH render-context builder (prose)
│   └── marketing.rs — MARKETING render-context builder (block composition)
├── routes/
│   ├── mod.rs     — route table
│   ├── browse.rs  — index, element_redirect, element_tab (current handlers)
│   ├── edit.rs    — GET ?edit=1 overlay, PUT /vault/{path} save-back (track-wysiwyg §6);
│   │                writes via commit-on-approve path (project-marketing pending.rs pattern), not raw FS
│   ├── ai.rs      — POST /ai/session SSE relay (track-ai-integration; D5)
│   ├── sse.rs     — GET /sidebar/sse live-reload stream (track-sidebar §2; D4)
│   └── bundle.rs  — RESERVED, not built (pending Command ratification; §DESIGN-BUNDLE)
├── render/
│   ├── markdown.rs — render_markdown (pulldown-cmark, code-block hook for highlight)
│   ├── nav.rs      — render_nav / render_tab_bar (lifted from main.rs)
│   ├── shell.rs    — HTML shell, Carbon CSS-var injection (track-carbon-v11 §1,§5)
│   └── highlight.rs — syntect class-span emission (track-rust-sovereign §2)
├── templates/      — minijinja runtime templates + autoreload (track-rust-sovereign §1)
└── static/         — compiled tokens.css (--cds-* + DTCG), portal CSS, ~12-line drawer JS,
                      vendored Carbon web-component ESM (optional, CDN-first)
```

**Cross-archive reuse question (open):** the marketing surface may *reuse* `app-mediakit-shell` (lib crate: Section trait, `section_catalog()`, tokens.rs DTCG loader) rather than reimplement — see Decisions open.

## AI integration

**Endpoint:** `POST /ai/session` — a single normalized-SSE relay. Two upstream adapters behind one Rust trait `UpstreamModel`:
- **`DoormanOlmo`** — local OLMo 7B via Doorman at `127.0.0.1:9080`; free; SYS-ADR-07-safe; no data leaves the VM. Verified live: `127.0.0.1:9080/healthz` returns `ok`.
- **`ClaudeCloud`** — Anthropic **Messages API** using the user's own credential held **only in the per-user browser session** (pasted key in session storage, or OAuth bearer token). The app holds no key.

**Architectural correction (surfaced by track-ai-integration, flagged not silently built):** the previously-assumed claude.ai "MCP session relay" **does not exist as a supported surface**; the cookie-replay equivalent is ToS-violating and actively blocked by Anthropic. The replacement above preserves every stated goal: isolated per-user session, app holds no key, same SSE bridge, same OLMo/Doorman local leg.

**Wire contract:** the browser sends **only** the highlighted selection + a small structural context descriptor + desired schema — never the whole component file, never an API key. The relay returns a normalized `{type:"delta",text}` / `error` / `done` stream the browser consumes identically regardless of backend.

**Edit pattern:** selection-only context + **prompt caching of the stable design-system prefix** + **diff-back** (not full rewrite). **Structured/strict tool use, NOT computer use** (10–100× cheaper, one turn).

**Governance:** SYS-ADR-07/10/19 maintained — AI composes, human reviews the diff, human approves (F12). The save-back path adopts the project-marketing **commit-on-approve** pattern (git2 commit on approval, not raw FS mutation), and exposes a native MCP surface (JSON-RPC 2.0, no mcpkit dependency) consistent with project-marketing's 5-tool model. MCP tools are read-only (per project-bim precedent); writes are human-approved.

## Mobile + Carbon CSS

**Carbon (D6):** absorb Carbon's `--cds-*` CSS custom properties directly; selectively load individual `@carbon/web-components` from CDN only where real interactivity is needed. No npm, no bundler, no React, no build step. Serve one compiled `tokens.css` (theme via `data-carbon-theme`) from the axum static handler; static chrome/layout uses `cds--*` classes on server-rendered HTML (no JS); interactive widgets (dropdown, modal, ai-label) loaded per-component via `<script type="module" src="…/v2/…">` (the `/v2/` segment is required for v11; latest is 2.56.0). **Do NOT use `cds-side-nav` for the deep token tree** (≤2-level, unvirtualized) — server-render the tree styled with `cds--side-nav__*` classes (D4). `cds-ai-label` (ex-Slug, stable @carbon/react 1.66.0) marks AI-generated content. Carbon does not yet export `.tokens.json` (issue #22437) — consume `--cds-*` vars only.

**Mobile (D7):** a single hand-authored stylesheet on four native Baseline primitives — cascade layers (`@layer reset, tokens, base, components, utilities`), custom properties, container queries (`@container`), sticky positioning — plus ~12 lines of drawer-toggle JS. The sidebar is an **overlay drawer + scrim on phones** (matching Carbon/M3/Spectrum, which all drop the persistent sidebar on phones), flipping to a sticky persistent column only at `≥64rem` where 272px + content both fit. Token tables use **sticky-first-column horizontal scroll** (preserves comparison density; stacked-card only for 6+ column tables). Code blocks rely on native iOS 13+ momentum (`-webkit-overflow-scrolling` is a no-op since iOS 13) with `overflow-x:auto` + `overscroll-behavior-x:contain`. 48px touch targets satisfy both Apple 44px and Material 48dp. Cascade layers put `tokens` below `components` so DTCG values never need `!important`.

## Token schema gaps

Verified: only 6 of 13 stable `$type` values are in use. Two correctness fixes ship **with** v0.2.0:

- **Invalid types:** `$type: "string"` (4×, verified) and `$type: "boolean"` (in dtcg-vault) are **not valid DTCG types** — relocate to `$extensions` (icon names, font-feature settings have no native home).
- **Legacy string form:** `dimension` (`"16px"`), `duration` (`"150ms"`), `number` (`"0"`) use the legacy string form; 2025.10 canonical is the object form `{value:16,unit:"px"}`. Needed for strict-validator / SD v5 / Figma API round-trip. **Biggest migration item.**

**Token groups / `$type` values to add (highest impact first):**

| Add | `$type` | Serves | Why |
|---|---|---|---|
| `semantic.typography` | **typography** (composite) | viewer + marketing + editor | `prose.body`, `prose.h1`–`h6`, `prose.code`, `ui.label`, `ui.button`, `display.hero` |
| `semantic.elevation` | **shadow** (composite) | SaaS UI + marketing cards | `flat`/`raised`/`overlay`/`modal`/`popover`; add `primitive.color.shadow` |
| `semantic.border` | **border** (composite) + `primitive.stroke` (strokeStyle) | all surfaces | `default`/`subtle`/`focus`/`error` |
| `semantic.transition` | **transition** (composite) | UI + marketing hover | references existing `primitive.motion.duration`/`easing` |
| `primitive.font.size` | **dimension** scale + letter-spacing | viewer + editor | required before typography composites resolve (currently absent) |
| `semantic.z` | **number** | editor/token-browser overlays | promote out of `workplace`-only namespace |
| `semantic.gradient` | **gradient** (composite) | marketing | hero/CTA/brand-fade — only place gradients live |
| `semantic.opacity` | **number** | UI | disabled/muted/overlay-scrim |
| `semantic.print` | **dimension** group | viewer (print-aware) | page-margin, prose.measure max-line-length, print color overrides |
| `semantic.icon.size`/`.color` | **dimension** / **color** | all | icon identity by string ref in `$extensions` — do NOT invent `$type:"asset"` (no spec support) |

**Theming structural change:** split the single bundle into `primitive.json` + `theme/light.json` + `theme/dark.json` + `theme/dark-high-contrast.json` + a **Resolver Module** file (`theme` modifier: light/dark/lightHighContrast/darkHighContrast — high-contrast as a layered override). `$modes` is dead (issue #210 not adopted). **Scope note:** this is DESIGN-* work owned by this Totebox; the multi-file split + resolver + any Style Dictionary build wiring stage as DESIGN drafts / commit in this archive, NOT promoted from here.

## Marketing pages

**design.pointsav.com served from DTCG tokens (track-marketing-pages).** The design system *is* the CMS: one DTCG token graph drives UI components, token-browser docs, *and* marketing pages — the "design system as CMS" thesis articulated in the literature (Sanity, AEM Universal Editor, CMS.gov) but **not yet shipped as a unified product** by any incumbent. This is the leapfrog seam.

**Pipeline (Rust/axum, no external CMS — Payload/Sanity/Prismic all push token binding to the render layer anyway, so build only that render layer):**

1. **TOKEN leg** — `tokens/main-page/*.dtcg.json` + a DTCG 2025.10 **Resolver file** (`resolver.json`) declaring `sets`, `modifiers` (theme: light/dark/brand), `resolutionOrder`. There is no `$modes` key — the Resolver Module is the spec-blessed theming mechanism (track-dtcg-schema §5).
2. **Build step (Rust, no Node)** — a small resolver reads token sets + resolver, flattens, applies context, resolves aliases, emits `:root{--…}` custom properties per context. A Rust reimplementation of the Style Dictionary CSS transform (a `moonshot-*` sovereign reimplementation, not a Node dependency). This is the STYLESHEET leg.
3. **Content (TEMPLATE leg)** — `templates/<page>.md`: YAML front-matter (selects template + theme context: `theme: dark`, `brand: woodfine`) + a **block sequence** (fenced `:::hero`, `:::feature-grid`, `:::cta`, `:::pricing`, `:::logo-wall`). Body is markdown or a thin block DSL.
4. **Component grammar** — `components/*.html`: a closed set of block templates, each consuming **only CSS custom properties**, never literals.
5. **Render (axum)** — parse front-matter → select template + resolver context; parse block sequence → select component HTML per block; render via minijinja binding content into block templates; inject the per-context compiled CSS-variable block in `<head>`; serve static or SSR.

**Author surface principle (§6):** a non-coder (typesetter, UX writer) authors safely because the only exposed choices are **(a) which component, (b) which token-defined variation, (c) the content** — styling is structurally inaccessible. v1 author interface = block-fenced markdown; a visual token-picker editor is a later layer over the same bundle, NOT a prerequisite. Mirrors the project-marketing `app-mediakit-shell` typed section-manifest model (Section trait, `page.yaml`) — the design-system marketing surface should converge on the same chassis.

## Contributor workflow

Lightweight, proportionate for 2–3 people (track-contributor; 2–5 is the *normal* design-system team size — "small by design, not small by default"):

1. **Propose** — one `DESIGN-RESEARCH-*` draft in `.agent/drafts-outbound/` with `foundry-draft-v1` frontmatter + five research-trail fields. States intent, **named token list affected**, mockup, variation-vs-new, a11y target (WCAG 2.1 AA). Collapses Carbon stages 1–2 into one artifact — the "increase-scope-or-don't" gate.
2. **Token first, in Git** — define/edit tokens in `dtcg-bundle.json` (source of truth) **before any component code**; Style Dictionary (build-time transform, low lock-in; `moonshot-*` stub target) emits Rust/CSS. The token-file diff is the **intent preview** (the design analogue of "show diff in chat before applying").
3. **Build + document together** — implement component + fill the markdown doc template (usage / style / code / accessibility — Carbon's four sections) in the same PR. Non-engineers write usage/style prose routed through **drafts-outbound → project-design gateway** (the Foundry-native equivalent of Supernova's Contributor-role-without-publish — replicate the role boundary, not the SaaS). A11y is non-negotiable even when other ceremony is cut.
4. **Two-layer preview + co-sign** — visual regression via **Storybook 8 + Chromatic** (required UI Tests check; flag-every-affected-story blast-radius for token changes; batch-accept). Operator/Master reviews the token-diff (intent) + the Chromatic visual-diff (consequence), then **adds Master co-sign in frontmatter** (already mandated by this archive's CLAUDE.md: DESIGN-TOKEN-CHANGE requires Master co-sign). No merge without the required check + co-sign — the design instantiation of "preview before writing."
5. **Commit, version, monthly audit** — commit via `bin/commit-as-next.sh`; bump **SemVer** (minor = new component, patch = token/fix, major = breaking rename); Stage 6 promotion is a Command-Session route via outbox (per scope-discipline.md). **One monthly token audit** (NOT weekly — over-ritual at this scale; weekly only during rapid growth, quarterly too slow).

**Skip (premature at 2–3 people):** weekly audits, champion programs, multi-platform commitments, RFC committees. Figma Dev Mode / MCP / Code Connect are **import-only, draft-tier, hand-verified** — never a code source of truth (~80% responsiveness, blind generation, ToS posture per "preview before writing").

## DESIGN-BUNDLE — research reference (pending ratification)

> **⚠️ PENDING COMMAND RATIFICATION per operator decision 2026-06-14 — renderer design NOT started. Reference material only; `routes/bundle.rs` is a reserved name with no implementation.**

A DESIGN-BUNDLE = multi-file artifact (TOKEN + STYLESHEET + TEMPLATE), the same manifest pattern as a legal-document bundle (track-marketing-pages §7), differing only in that the design/marketing TEMPLATE leg gains a **composition grammar** and the TOKEN leg gains **resolver contexts**. UX patterns for the eventual bundle-detail page (track-bundle-ux):

- **Identity-first, then members** — one canonical bundle name/version up top; members addressed as *properties* of it (Figma component-set model §1, Storybook Meta/story §2), never an undifferentiated file dump.
- **Index over tree** — a flat, role-labeled member list (3–8 files: `component.html`, `tokens.json`, `README.md`) is far more scannable for non-engineers than a recursive file tree (jsr symbol-index §3, GitHub tree failure-mode §4). Reserve a true tree only at large scale, with search + collapse + accessible keyboard nav.
- **Four affordances:** (a) Download-all → ZIP with `Content-Disposition: attachment; filename="bundle-v.zip"` (sync `zip` crate into an in-memory buffer is lowest-risk for kilobyte bundles; `s-zip` only if large/concurrent, ~5 MB constant memory); (b) per-member preview/edit tabs (member-as-tab labeled by *role*, one consolidated switcher — Figma single-panel principle); (c) manifest metadata panel (id, version, member sizes, interdependencies, trust signals — npm/jsr rail); (d) rendered preview (PDF via WeasyPrint shell-out `weasyprint - -` with `--base-url` + a Rust-enforced wall-clock timeout, since `-t` only bounds HTTP fetches; + sandboxed `<iframe>` HTML preview).
- **Inheritance model** — bundle-level metadata with per-member overrides (Storybook Meta vs story) keeps the page DRY.

## Decisions open

- **Which editor library for the fallback rich-text layer?** D3 locks the server-rendered overlay as primary, with TipTap (prose-only, lazy) as the named fallback — but project-bim chose CodeMirror 6 for its dual-mode. Reconcile (shared `moonshot-code-editor`) or document the deliberate difference (BIM edits raw IFC/JSON; design edits prose + token-forms). Operator/cross-archive call needed.
- **Final moonshot-* timelines.** Search + fs-watch are v0.2.0; the rest (template 2027, highlight 2026 Q4–2027, markup 2027, database 2027+) need operator ratification and cross-archive deconfliction (project-bim proposes `moonshot-schema-validator`, `moonshot-registry`, `moonshot-code-editor` that overlap).
- **`master_cosign` for legal/regulated tokens.** Awaiting Master co-sign on `legal-subscription-agreement` + `prospectus-formatting` tokens (DESIGN-TOKEN-CHANGE rule).
- **Reuse `app-mediakit-shell` for the marketing surface vs reimplement?** Cross-archive decision with project-marketing.
- **DESIGN-BUNDLE ratification** — Command to ratify before bundle-renderer design starts.

## Work log
- 2026-06-13: Initial BRIEF created; 8 architecture decisions confirmed; Q&A grilling completed (7/8 questions)
- 2026-06-14: Major BRIEF rewrite from 11-track OPUS research; synthesis completed; DESIGN-BUNDLE ratification flag sent to Command
- 2026-06-15: v0.2.0 baseline (74527127) + Phase A (3dfa228d) + Phase B (51b4e009) + Phase C (92bd5f9e) + Phase D (d6d4b0e5) all committed. D1–D8 fully implemented: routes/ split, schema dispatch, inotify watcher, inverted-index search, SSE sidebar, edit overlay, AI bridge (DoormanOlmo + ClaudeCloud), static/ai.js. reqwest 0.12 rustls-tls added. Clippy clean on all crates. Stage 6 pending Command via outbox project-design-20260615-stage6-app-design-phase-d.

## Carry-forward
- Stage 6 for Phase A–D commits (74527127→d6d4b0e5) — Command scope; outbox sent
- Await Command ratification of DESIGN-BUNDLE before designing bundle renderer
- Await master_cosign on legal-subscription-agreement + prospectus-formatting tokens
- minijinja templates (Phase B plan item deferred — render/mod.rs hand-rolled shell; minijinja is next iteration)
- Route cross-archive moonshot-stub deconfliction (shared registry per OXC "one AST, many tools") to Command Session outbox
