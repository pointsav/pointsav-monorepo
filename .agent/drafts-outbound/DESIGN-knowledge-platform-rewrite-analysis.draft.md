---
schema: foundry-draft-v1
artifact_type: DESIGN-RESEARCH
language_protocol: DESIGN-RESEARCH
state: draft
title: "Knowledge Platform Fresh-Slate Analysis: UX Audit and Competing Rewrite Proposals"
target_repo: pointsav/pointsav-design-system
target_path: dtcg-vault/research/
target_filename: knowledge-platform-rewrite-analysis.md
created: 2026-06-04
author: project-knowledge@claude-code
research_done_count: 9
research_suggested_count: 3
open_questions_count: 5
research_provenance: "Live site audit (3 sites via WebFetch); content repo cross-check (3 repos, 24 articles sampled); three competing architect proposals with live web benchmarking (9 external sites benchmarked); codebase analysis (13,240 lines Rust, 26 source files)"
research_inline: true
bcsc_class: no-disclosure-implication
---

# Knowledge Platform Fresh-Slate Analysis

## 1. Executive Summary

The three deployed instances of `app-mediakit-knowledge` — documentation.woodfinegroup.com, projects.woodfinegroup.com, and corporate.woodfinegroup.com — are functional in their core rendering path but share three structural defects that impose real UX cost at current corpus scale: the absence of font preload tags causes a measurable first-paint latency gap on every cold load; the Spanish (`/es/`) homepage renders navigation chrome, section headings, and featured article titles in English rather than Spanish; and the engine's two complete federation modules (`mounts.rs`, `blueprints.rs`) are tested and correct but wired to nothing in the running binary. Content quality is uneven: the documentation and projects repos are publication-ready in structure but carry live red links and stub articles that readers encounter, while the corporate repo is blocked on two stub articles linked directly from the home page lede. Three architect proposals were evaluated — Architect A (Wikipedia-Faithful refactor), Architect B (Stripe/Vercel Docs pattern), and Architect C (Minimal Sovereign static pipeline). The recommendation is a hybrid: adopt Architect A's mechanical server.rs decomposition and mounts/blueprints wiring as the primary track, adopt Architect B's scroll-spy TOC and Cmd+K command palette as the two highest-value UX additions, and defer Architect C's static pipeline unless the edit workflow is formally retired.

---

## 2. UX Health by Site

### projects.woodfinegroup.com

| Dimension | Rating | Key Evidence |
|---|---|---|
| Navigation — tabs, breadcrumbs, prev/next | Green | Article/Talk/Edit/History tabs present; breadcrumbs correct; prev/next arrows functional |
| Navigation — language switcher correctness | Amber | Present but active-language indicator broken on `/es/` — shows `[EN]` with no current-language cue |
| Navigation — mobile bottom bar | Red | Not observed on any page; no thumb-reach fallback confirmed |
| Typography — font loading | Amber | Font classes present; `<link rel="preload">` tags not confirmable from rendered output |
| Typography — heading hierarchy | Green | Multi-level headings present; TOC correctly numbered; no heading-skip observed |
| Colour / Tokens | Amber | `--wf-*` / `--ps-*` tokens not surfaceable from rendered HTML; no confirmed dark-mode signal |
| Content Rendering — wikilinks and tables | Green | All internal links functional; tables populated; no TODO placeholders in sampled articles |
| Content Rendering — recent-changes list | Red | Title and date concatenated without separator (`"Contact Topic2026-06-03"`) |
| Mobile UX | Amber | Skip-to-content anchor present; viewport meta and safe-area-inset not confirmable externally |
| Performance — font preload | Amber | Not confirmable from rendered output; FOIT risk unverified |
| Broken Elements | Red | 4 dead wikilinks confirmed: `[[about]]`, `[[topic-catchment-ranking-methodology]]` (unversioned slug), `[[topic-co-location-cluster-formation]]`, `[[DATA-MANIFEST]]` |
| Brand Differentiation | Amber | Domain content and `WOODFINE CAPITAL PROJECTS` masthead distinguish it; no confirmed distinct accent colour |

**Key issues (projects):** The `/es/` homepage is the most critical failure — navigation labels, section headings, footer links, and featured article titles all render in English on a page with Spanish body text, making it unusable for a Spanish-primary reader. The recently-changed list formatting defect (title and date concatenated) is visible on the homepage. The single `governance` category absorbs 34 non-BIM articles including methodology, market profiles, and legal governance pages — at current scale this is navigable but will degrade as the corpus grows.

---

### corporate.woodfinegroup.com

| Dimension | Rating | Key Evidence |
|---|---|---|
| Navigation — tabs, breadcrumbs | Green | Article/Talk/Edit/History tabs; `Home › Reference › Article` breadcrumb confirmed |
| Navigation — TOC sidebar | Green | 4-entry TOC on sampled article; rendered correctly |
| Navigation — language switcher | Green | EN/ES toggle present on homepage and article; mirror URLs correct |
| Navigation — mobile bottom bar | Amber | Footer bar confirmed; dedicated thumb-reach sticky bar unconfirmed |
| Typography — fonts | Amber | Fonts committed in engine; preload and `font-display: swap` not verifiable via text fetch |
| Typography — heading hierarchy | Green | H1/H2 clean; no hierarchy violations detected |
| Colour / Tokens | Amber | Cannot confirm from text layer; stylesheet access required |
| Content Rendering — wikilinks | Green | Links render; slug-form display names are a minor content issue (slugs exposed as anchor text) |
| Content Rendering — hatnote / infobox | Green | Definition box renders correctly on article page |
| Content Rendering — placeholders | Amber | Two stub articles (`topic-perpetual-equity-model`, `topic-investment-units`) linked from home page lede |
| Mobile UX | Amber | Cannot confirm viewport meta or safe-area-inset without stylesheet access |
| Performance — font preload | Amber | Preload links and `font-display: swap` unconfirmed |
| Broken Elements | Green | No 404 references detected; `reference-invariants.yaml` slug prefix mismatch is a structural risk, not a confirmed broken link |
| Brand Differentiation | Green | Financial/disclosure register; Woodfine trademark block; institutional category taxonomy clearly differentiate this instance |

**Key issues (corporate):** The site is not publication-ready in its current form because two stub articles — `topic-perpetual-equity-model` and `topic-investment-units` — are directly linked from the home page lede text and are stub-depth (two to three sparse paragraphs, no section headings). An institutional reader following those links lands on incomplete content. The `reference-invariants.yaml` home-panel slugs omit the `topic-` prefix used in all article `slug:` fields — this is a silent mismatch that will produce broken home-panel links if the engine performs exact-match slug resolution.

---

### documentation.woodfinegroup.com (localhost:9090; external DNS not yet live)

| Dimension | Rating | Key Evidence |
|---|---|---|
| Navigation — tabs | Green | Article/Talk/Edit/History present; `aria-current="page"` correct; TOC sidebar active |
| Navigation — breadcrumbs | Green | Three-level `Home › Category › Title` rendered from `category:` frontmatter |
| Navigation — language switcher | Green | EN/ES toggle on every article; auto-detected from `.es.md` sibling; hreflang declared |
| Navigation — bottom bar (mobile) | Amber | Present in CSS (`display: none` on desktop); `padding-bottom` does not add `env(safe-area-inset-bottom)` — Home Indicator overlap on notched iPhones |
| Navigation — homepage topnav | Amber | `nav.left` (Disclaimer/Contact strip) absent from home chrome; present only on article pages |
| Typography — fonts | Green | Inter + Source Serif 4 self-hosted woff2; `font-display: swap`; 68ch measure; correct serif-for-body / sans-for-UI split |
| Typography — preload | Red | No `<link rel="preload" as="font">` in `<head>` for either typeface; fonts discovered only after CSS parse |
| Colour / Tokens | Green | DTCG oklch variables; auto dark mode via `prefers-color-scheme`; anti-FOUT script; manual override persisted to localStorage |
| Content Rendering — wikilinks | Amber | Resolved links correct; `/wiki/sys-adr-07` renders as `wikilink-missing` — content gap, not engine bug |
| Content Rendering — search | Green | 25 results for "substrate"; snippets present; BM25 via Tantivy; `/api/search` works |
| Content Rendering — Spanish | Green | `lang="es"` correct; full Spanish lede; article EN/ES toggle functional |
| Content Rendering — frontmatter errors | Amber | 2 YAML parse errors in 4h log window; engine degrades gracefully but those articles are unrenderable |
| Performance — font preload | Amber | No font preload tags; swap declared but fonts discovered late; CLS risk on first paint |
| Performance — serving | Green | Single binary; self-hosted fonts; no third-party calls; 23.7 MB RSS for 316 articles |
| Broken Elements — guide 404s | Red | 62 `page not found` errors in 4h; 51 unique guide slugs referenced in content but not yet committed to woodfine-fleet-deployment |
| Broken Elements — external DNS | Red | `documentation.woodfinegroup.com` is NXDOMAIN; site only reachable via localhost reverse proxy |
| Brand Differentiation | Green | Category taxonomy, PointSav wordmark, lede audience statement, guide/topic badge mix clearly read as technical documentation |

**Key issues (documentation):** The confirmed Red finding is the absence of `<link rel="preload" as="font">` for Inter and Source Serif 4 — both fonts are self-hosted and correctly declare `font-display: swap`, but without preload hints they are discovered only after CSS parse, creating a measurable first-paint CLS window. The 62 guide-slug 404s in four hours indicate that content articles reference operational guides that have not yet been committed to `woodfine-fleet-deployment` — this is content debt, not an engine bug, but it degrades the reader experience. The external DNS is NXDOMAIN, meaning the documentation instance has no public URL.

---

## 3. Content Repo Health

### media-knowledge-documentation (feeds documentation instance, ~514 files)

**Frontmatter completeness:** 7/8 sampled articles fully complete; the one failure (`reference/bim-aec-muscle-memory.md`) is missing `status:` and `last_edited:`. Extrapolating from the sample, approximately 8–9 articles across reference/ and architecture/ share the missing-status defect — roughly 98% completeness at corpus level.

**Bilingual gaps:** All 8 sampled articles have `.es.md` siblings. The archetypes category has 3 articles (2 archetypes committed, 1 archetype — `professional-centres` — does not exist in either EN or ES).

**Placeholder count:** 10 articles marked `quality: stub` across the corpus; 1 article (`architecture/3-layer-stack.md`) contains a committed `<!-- EXPAND: lead needs 200+ words -->` comment.

**Top 3 concerns:**
1. `professional-centres` article does not exist. It is wikilinked from `archetypes/vertical-warehouse.md` (a `quality: complete` article), producing a live red link on the public wiki. This is the missing PRO archetype article.
2. `featured-topic.yaml` candidate list contains a stale path `architecture/worm-ledger-design` — the file lives at `infrastructure/worm-ledger-design.md` after a 2026-05-09 category rebalance. If the engine cycles through candidates in order, this will produce a broken featured-article rotation.
3. 62 guide-slug 404s reference operational guides committed to content articles but not yet present in `woodfine-fleet-deployment`. These are read-path 404s that readers encounter.

**Publication-ready verdict: Conditionally yes** — the documentation corpus is substantively complete and BCSC-clean, but the three concerns above are visible defects on the public site. The missing `professional-centres` article is the blocking item; the others are maintenance items.

---

### media-knowledge-projects (feeds projects instance, ~102 files, 46 content articles audited)

**Frontmatter completeness:** 46/46 audited articles fully complete. Schema is uniformly `foundry-doc-v1`. No missing fields.

**Bilingual gaps:** None — every content article has an `.es.md` sibling. Infrastructure files (CLAUDE.md, NEXT.md, etc.) are correctly English-only.

**Placeholder count:** 11 BIM articles carry `status: pre-build`. Three contain explicit `TBD` markers for Professional Office Zone values pending V12 key plan completion — intentional in-progress stubs, not drift.

**Top 3 concerns:**
1. `reference-invariants.yaml` uses unprefixed slugs (`tier-index-north-america`) that do not match the `slug:` field values in articles (`topic-tier-index-north-america`). If the engine performs exact-match slug lookup for home-panel invariant links, these three panels silently 404.
2. `[[about]]` dead link in `contact.md`. The contact article directs readers to an `about` article that does not exist — a broken link on a high-traffic governance page.
3. The single `governance` category absorbs all 34 non-BIM articles: methodology, index articles, individual market profiles, and legal governance pages share one category label. The portal grid cannot distinguish content types at this scale.

**Publication-ready verdict: Conditionally yes** — frontmatter is clean and bilingual coverage is complete. The `reference-invariants.yaml` slug mismatch and `[[about]]` dead link are the blocking items; the category structure concern is medium-term.

---

### media-knowledge-corporate (feeds corporate instance, ~51 files, 38 article files audited)

**Frontmatter completeness:** 34/38 fully complete; 4 files (`topic-investment-units.md/.es.md`, `topic-perpetual-equity-model.md/.es.md`) missing `last_edited:`. Both are `status: stub`.

**Bilingual gaps:** None — all 17 EN topic articles have `.es.md` siblings; index, about, contact, and disclaimers pages are also fully paired.

**Placeholder count:** 4 stub-state files (2 EN + 2 ES for the two incomplete articles). Both articles have valid body content but are stub-length with no section headings, no See Also, and no copyright footer.

**Top 3 concerns:**
1. Two stub articles (`topic-perpetual-equity-model`, `topic-investment-units`) are linked directly from the `index.md` home page lede. An institutional reader following these links lands on thin, incomplete content. This is the blocking issue for publication.
2. `reference-invariants.yaml` slug prefix mismatch — same pattern as the projects repo. Three home-panel `link_slug` values omit the `topic-` prefix. Structural risk for the home page invariant panels.
3. Both stub articles are missing `last_edited:`, a required frontmatter field. This may cause engine rendering errors or sort anomalies on date-sorted pages.

**Publication-ready verdict: No** — the two stub articles are directly reachable from the home page lede and are not publication-depth. The site should not go live until those stubs are expanded to `status: active` or the home page wikilinks are replaced with plain text pending completion.

---

## 4. Borrow List: Web Benchmarking Findings

Benchmarked sites across the three proposals: Wikipedia (Vector skin), ArchWiki (MediaWiki Vector), rustdoc (Rust stdlib), Tailwind CSS Docs, Stripe Docs, Vercel Docs, Zola (Rust SSG), MkDocs Material, Hugo.

| Site Benchmarked | Pattern Observed | Which Proposal Adopts It | Feasible Within Sovereign / Single-Binary Constraint? |
|---|---|---|---|
| Wikipedia Vector | Sticky header with compact search re-entry on scroll | Architect A | Yes — `position: sticky` + IntersectionObserver on H1; vanilla JS; no dependency |
| Wikipedia Vector | Pinnable TOC via `data-feature-name` + `localStorage` | Architect A | Yes — `localStorage.setItem` + CSS class toggle; 40 lines JS |
| Wikipedia Vector / ArchWiki | Mobile dock-bottom portlet (`p-dock-bottom`) with native-feel tab bar | Architect A, B (both fix the mobile bar) | Yes — already partially implemented; safe-area-inset fix is CSS |
| ArchWiki | "Related articles" sidebar card driven by curated cross-links (not link-graph) | Architect A (via blueprints `relates_to` wiring) | Yes — blueprints.rs already has the field; rendering it is a chrome.rs addition |
| ArchWiki | Three-way night-mode toggle (light/dark/follow-system) | Architect A (partial; current impl has two-way) | Yes — extend the existing `html[data-theme]` + localStorage pattern |
| ArchWiki | Article status flags visible in chrome (`quality: stub` → visible notice) | Architect A | Yes — read frontmatter `quality:` field in chrome render; zero schema change |
| rustdoc | Resizable sidebar via CSS drag | Neither A nor B adopt | Technically feasible but adds JS complexity; low priority for this corpus |
| rustdoc | Font preload via inline script in `<head>` | Architect A (adds preload), B (adds preload), C (build-time) | Yes — all three proposals address this; easiest fix available |
| rustdoc | "Copy item path" single-button clipboard verb | Architect A | Yes — 6-line JS addition to wiki.js; uses existing Clipboard API |
| Tailwind CSS Docs | Persistent sticky left rail with accordion category groups | Architect B | Yes — server-side maud render; no external library |
| Tailwind CSS Docs | Cmd+K command palette over search-page navigation | Architect B (primary), Architect A (notes it as possible) | Yes — native `<dialog>` + fetch to existing `/api/search`; ~80 lines vanilla JS |
| Stripe Docs / Vercel Docs | Right-rail scroll-spy TOC with IntersectionObserver | Architect B | Yes — 30 lines vanilla JS against heading IDs already generated by comrak |
| Vercel Docs | Section switcher dropdown in top bar for cross-instance navigation | Architect B | Yes — server-side maud dropdown; replaces footer-only cross-links |
| Stripe Docs | Auth-gated edit affordance (pencil icon, not primary tab) | Architect B | Yes — existing auth system; no route changes |
| Zola | Filename-convention bilingual pairing already works | Architect C | Already implemented; not an addition |
| MkDocs Material | Client-side static search index (no server search endpoint) | Architect C (Pagefind) | Yes — but loses BM25 tunability and MCP agent search access |
| MkDocs Material | Build-time wikilink resolution with red-link class | Architect C | Yes — but loses real-time edit preview |
| Hugo + Pagefind | Multilingual search partition via `data-pagefind-lang` | Architect C | Yes — self-hosted, no CDN; payload ~80–150 KB for this corpus |
| Hugo + Pagefind | Build-time frontmatter validation with warnings | Architect C | Yes — catches the 7 missing `status:` fields automatically |

---

## 5. Architect Proposal Comparison

| Dimension | Architect A — Wikipedia-Faithful | Architect B — Stripe/Vercel Docs | Architect C — Minimal Sovereign |
|---|---|---|---|
| **UX model** | Wikipedia chrome preserved; gaps closed (sticky header, pinnable TOC, status badges, related-articles card) | Wikipedia tabs replaced; three-zone layout (persistent left rail + right-rail scroll-spy TOC + command palette) | Wikipedia chrome preserved in pre-built HTML; runtime is a file server only |
| **Navigation pattern** | Article/Talk/Edit/History tabs + sticky header + TOC pin/unpin | Left rail with accordion categories + right-rail scroll-spy + cross-instance section switcher | Article/Talk/Edit tabs (pre-built); no sticky header; no left rail |
| **Search UX** | Tantivy BM25 endpoint unchanged; Cmd+K mentioned as possible but not primary recommendation | Tantivy BM25 endpoint reused via Cmd+K command palette modal (`<dialog>` + `fetch`); removes search-results page as primary UX | Pagefind client-side BM25; static index shards; no server query path; modal UI via Pagefind Component UI |
| **CSS approach** | 3 files (tokens.css unchanged; style.css refactored into 5 labelled sections; theme-woodfine.css absorbed into tokens-woodfine.css) | 3 files per-instance (tokens.css + per-instance theme file replacing both woodfine files; style.css refactored, -30%) | 2 files per instance (tokens.css baked per-instance; style.css refactored; no runtime conditional load) |
| **Auth / edit workflow** | Unchanged — auth stays in same binary; CodeMirror moves to `/edit/` routes only | Auth-gated: Edit tab removed from anonymous view; pencil icon in top bar for authenticated users; otherwise unchanged | Edit path isolated to separate `mediakit-editor` binary (Option A) or removed entirely (Option B) |
| **Runtime model** | Single binary unchanged; monolith decomposed into 7 modules; mounts + blueprints wired | Single binary partially decomposed (chrome.rs split out); mounts wiring deferred to follow-on | Two or three binaries: `mediakit-static` (file server, ~150 lines), `mediakit-editor` (write path), `mediakit-mcp` (optional) |
| **Scope estimate** | ~8–9 developer-days; net −2,755 lines (primarily deletion of server.rs redundant code) | ~13 developer-days; net +250 lines across codebase; no markdown migration | ~14–19 developer-days; net −6,000–8,000 lines; no markdown migration |
| **Operator risk** | Low — no routes change, no content schema change, no deployment model change; risks are 1–2 day branch window and autocomplete correctness trade-off | Medium — anonymous readers see a different navigation model; left-rail accordion state must be persisted or navigation becomes disorienting across articles | Medium-High — real-time edit feedback is lost (2–5 second rebuild gap); BM25 tunability lost; server-side search API lost for MCP agents |

### Architect A — Core Argument

Architect A's position is that the Wikipedia UX model is correct for this content type and that the structural problem is organisational, not architectural. The 5,234-line `server.rs` monolith is the root cause of review drag; the incomplete wiring of `mounts.rs` and `blueprints.rs` (both fully tested) is the root cause of the Phase 0 federation gap. The refactor proposes a mechanical split of `server.rs` into seven files along boundaries that already exist in the code — routes, chrome render, article render, special pages, content walker, state — plus 60 lines of plumbing in `main.rs` to wire the two stranded modules. The net code change is a deletion. The best borrow from the reference web audit is the ArchWiki "Related articles" sidebar card driven by the `blueprints.rs` `relates_to` field that is already populated in YAML but never rendered — completing the half-built infrastructure is the highest-ROI single action this proposal identifies.

### Architect B — Core Argument

Architect B's position is that the Wikipedia model is structurally wrong for a 514-article practitioner corpus and that the live audit proves it: no ambient section orientation, no scroll position signal, no one-action search, no above-the-fold cross-instance navigation. The proposal replaces the Wikipedia tab set with a three-zone layout — persistent sticky left rail, right-rail scroll-spy TOC, Cmd+K command palette — while leaving the Rust/axum/maud/Tantivy stack and all 828 markdown files entirely unchanged. The best borrow from the reference web audit is the Cmd+K command palette via native `<dialog>` against the existing Tantivy endpoint: it reduces search-to-article interaction from three steps to one, requires no new dependencies, and uses the existing BM25 index that is already proven correct in production.

### Architect C — Core Argument

Architect C's position is that the production complexity of the current binary — SQLite, Tantivy, redb, git2, gix, argon2id in a single process that also serves public HTTP — is disproportionate to the content it serves. A Markdown content site's primary correctness property (HTML faithfully represents Markdown) is easier to verify in a build pipeline than in a live server. The proposal extracts the rendering logic (which already exists and is correct) into a build-time binary, reduces the public-facing server to a `tower-http::ServeDir` wrapper, and replaces server-side Tantivy with Pagefind (verified in production across Hugo and MkDocs ecosystems). The best borrow is build-time frontmatter validation: the seven articles missing `status:` and two missing `last_edited:` found in the content audit would have been caught automatically at build time rather than requiring a separate audit session. This represents a shift from reactive quality monitoring to proactive enforcement.

---

## 6. Recommendation

**Recommended path: Hybrid A+B, deferring C.**

The recommendation is to execute Architect A's server.rs decomposition and mounts/blueprints wiring as the structural foundation, then add Architect B's two highest-value UX features — scroll-spy right-rail TOC and Cmd+K command palette — on top of that foundation. Architect C's static pipeline is deferred unless the operator formally decides to retire in-browser editing.

**Three reasons:**

First, the server.rs decomposition is independently valuable regardless of which UX model is chosen. A 5,234-line monolith makes every subsequent change riskier — if Architect B's left-rail navigation is built into `server.rs`, it becomes part of the monolith. Doing the decomposition first creates reviewable modules that benefit any subsequent feature work. It is the lowest-risk, highest-leverage action available.

Second, Architect B's full navigation model (persistent left rail, cross-instance section switcher, removal of Wikipedia tabs) is a register change that carries real risk for the corporate instance. The corporate site's audience is institutional: investors, lawyers, procurement evaluators. These readers are more likely to find the encyclopaedic Wikipedia register (Article/Talk/History tabs, neutral chrome) appropriate and trustworthy than a developer-documentation pattern. The documentation instance, by contrast, is well-suited to the Stripe/Vercel pattern — its audience is technical practitioners. The hybrid resolves this: keep the Wikipedia tabs while adding the two UX features (scroll-spy TOC, command palette) that are valuable on all three instances without changing the register.

Third, the Cmd+K command palette and scroll-spy TOC are each self-contained additions. The command palette is 80 lines of vanilla JS against the existing `/api/search` endpoint; the scroll-spy TOC is 30 lines of JS against heading IDs already generated by comrak. Neither requires changing routes, schemas, or the deployment model. Adding them after the decomposition is a one-day task per feature.

**What to take from each proposal in the hybrid:**

From Architect A: the full seven-module server.rs decomposition; mounts wiring in AppState; blueprints wiring for the related-articles sidebar card; font preload inline script; article status badge (quality: stub → visible chrome notice); CodeMirror moved to `/edit/` routes only; collab.rs deleted; search_complete walk replaced by index query; TOC pin/unpin persistence.

From Architect B: scroll-spy right-rail TOC with IntersectionObserver; Cmd+K command palette via native `<dialog>` + existing Tantivy `/api/search` endpoint; mobile safe-area-inset-bottom fix (already confirmed as a defect in the documentation audit); `/es/` homepage chrome localisation fix (the mixed-language homepage is a Red finding on the projects instance — the template fix Architect B describes applies to all instances).

From Architect C: build-time frontmatter validation (implement as a Cargo test or a `cargo xtask check-frontmatter` that runs in CI against the content repos, rather than a full static build pipeline); the conceptual framing of `mediakit-editor` as a separate security boundary (noted for Phase 7 planning — not urgent but architecturally sound).

**Total scope of the hybrid:** approximately 11–12 developer-days. Larger than Architect A alone (8–9 days) but smaller than Architect B alone (13 days) or Architect C (14–19 days). No content files change. No routes change. No deployment model changes.

---

## 7. Minimum Viable Cleanup (Refactor Path)

If the operator decides not to pursue any of the three proposals in this sprint, the following ordered checklist restores UX coherence in a single sprint (estimated 3–4 developer-days total). Items are ordered by user-visible impact.

1. **Fix `/es/` homepage chrome language.** Navigation labels, section headings (Getting Started, Browse by Area), footer links, and featured article title on the `/es/` homepage must render from a localised strings map keyed by `lang`. This is the single highest-severity UX defect across all three instances (confirmed Red in the projects audit).

2. **Add font preload tags to `<head>`.** Add two `<link rel="preload" as="font" type="font/woff2" crossorigin>` tags in the base HTML template for `Inter-Regular-latin.woff2` and `Source-Serif-4-Regular-latin.woff2`. This closes the confirmed Red finding in the documentation audit and eliminates the CLS risk on cold first load.

3. **Fix mobile bottom bar `padding-bottom` safe-area gap.** Change `body { padding-bottom: 56px }` to `body { padding-bottom: calc(56px + env(safe-area-inset-bottom)) }` in `style.css`. Closes the Home Indicator overlap defect on notched iPhones confirmed in the documentation audit.

4. **Fix `featured-topic.yaml` stale candidate path in documentation repo.** Change `architecture/worm-ledger-design` to `infrastructure/worm-ledger-design` in `media-knowledge-documentation/featured-topic.yaml`. Prevents a broken featured-article rotation when the engine cycles through candidates.

5. **Fix `reference-invariants.yaml` slug prefix in projects and corporate repos.** Add `topic-` prefix to the three `link_slug:` values in each repo's `reference-invariants.yaml`. Prevents silent 404s on home-page invariant panels if the engine performs exact-match slug resolution.

6. **Fix recently-changed list formatting defect in projects instance.** The article title and date are concatenated without a separator (`"Contact Topic2026-06-03"`). Add the missing separator character or flex gap in the recent-changes list item template in `server.rs`.

7. **Add the missing `professional-centres` stub article in media-knowledge-documentation.** The PRO archetype article is wikilinked from `archetypes/vertical-warehouse.md` (a `quality: complete` article) but does not exist, producing a live red link. Create a minimal stub with valid frontmatter and a one-paragraph definition to resolve the red link.

8. **Expand the two corporate stub articles.** `topic-perpetual-equity-model` and `topic-investment-units` are linked from the corporate home page lede and are stub-depth. Add section headings, a See Also block, populate `last_edited:`, and promote to `status: active`. The corporate site is blocked on publication until these two articles are complete.

9. **Fix `[[about]]` dead link in `contact.md` in the projects repo.** Either create a minimal `about.md` article or replace the wikilink with plain text. This is a broken link on a governance page.

10. **Move CodeMirror bundle out of the main page payload.** CodeMirror is loaded on every article page but is only needed on `/edit/` routes. Move it to a separate `editor.js` bundle loaded only when the user navigates to `/edit/{slug}`. This is a page-weight improvement for all non-edit page views.

---

## 8. Open Questions (5 items)

| Question | Impact If Unresolved | Recommended Action |
|---|---|---|
| **Is in-browser editing (CodeMirror → git commit) a required feature for ongoing content production, or do all editors use direct git commits?** | Determines whether Architect C's static pipeline is viable and whether the edit server architecture (mediakit-editor isolation) is needed. If editing is git-only, the CodeMirror bundle, argon2id stack, and SQLite user store can be deleted entirely, substantially simplifying the runtime. | Survey active content editors (project-editorial, project-design, project-knowledge). Answer gates Architect C evaluation and the edit-server isolation decision. |
| **What is the intended audience for the corporate instance — institutional readers (investors, lawyers) or internal operators?** | Determines whether the Wikipedia encyclopaedic register (tabs, neutral chrome) or the Stripe/Vercel practitioner register (left rail, command palette) is the correct UX model for that instance. The two registers send different trust signals to different audiences. | Operator decision required. If the audience is institutional, keep Wikipedia tabs for corporate while adopting Stripe/Vercel patterns for documentation. If the audience is internal operators, the distinction does not matter. |
| **Does the engine perform exact-match or prefix-normalising slug resolution for `reference-invariants.yaml` `link_slug:` values?** | If exact-match, the three home-panel invariant links in both projects and corporate instances are silently broken today. If prefix-normalising (tries `topic-` prefix automatically), the defect does not exist in production. Determines urgency of the MVCC item #5. | Read the slug resolution code path in server.rs for the reference-invariants rendering function. One grep session in the monolith. |
| **Is `documentation.woodfinegroup.com` DNS planned for the current sprint or a later phase?** | The documentation instance is the only one that is not public-facing (NXDOMAIN confirmed in the audit). Until the DNS record is live, the documentation.wordcount.com audit findings are moot from a public-impact perspective — but the font preload, mobile safe-area, and guide-404 defects will be immediately visible at DNS cutover. | Command Session to confirm DNS target date. If cutover is within two weeks, the documentation UX fixes should be in the current sprint. |
| **What is the intended use of `mounts.rs` and `blueprints.rs` — are they Phase 0 federation infrastructure intended for activation in the next sprint, or are they deferred indefinitely?** | If intended for activation, wiring them is the highest-leverage engineering action available (Architect A's core recommendation) and should be the first development task after the MVCC sprint. If deferred indefinitely, they are dead code and should be documented as such in NEXT.md to avoid confusion. | Operator to confirm phase plan. The modules are fully tested and correct — they need a decision, not more engineering. |

---

## 9. Next Steps

Actions are assigned by session role and archive, ordered by dependency.

**Immediate (MVCC sprint — single sprint, ~3–4 days):**

- **project-knowledge Totebox:** Execute MVCC items 1–3 (ES homepage chrome fix, font preload tags, mobile safe-area fix) in `app-mediakit-knowledge` source. Commit via `commit-as-next.sh`. Stage 6 pending.
- **project-editorial Totebox:** Execute MVCC items 7–9 (professional-centres stub, corporate stubs expanded, `[[about]]` dead link) in the content repos. These are content commits, not engine commits.
- **project-knowledge Totebox:** Execute MVCC items 4–6 (featured-topic YAML fix, reference-invariants prefix fix, recent-changes separator fix). Items 4 and 5 are content repo commits; item 6 is an engine commit.
- **project-knowledge Totebox:** Execute MVCC item 10 (CodeMirror bundle split to editor-only load). Engine commit.

**Sprint +1 (structural foundation — ~8–9 days if hybrid A+B is approved):**

- **project-knowledge Totebox:** Execute Architect A's seven-module server.rs decomposition. Begin with `state.rs` and `walker.rs` (no HTML); then `chrome.rs`; then `pages/` modules. Run `cargo check` after each file boundary is introduced. The 1 failing test (`wiki_page_renders_navigation_portlet`) is pre-existing — do not regress additional tests.
- **project-knowledge Totebox:** Wire `mounts.rs` into `AppState` (replace three separate dir fields with `Vec<Mount>`); wire `blueprints.rs` into `AppState`. Operator confirmation required before this step (Open Question 5).
- **project-knowledge Totebox:** Add scroll-spy TOC (30 lines JS, CSS accent update). Add Cmd+K command palette (80 lines JS, `<dialog>` HTML in chrome.rs). Add article status badge for `quality: stub` / `quality: needs-update`. Add related-articles sidebar card driven by blueprints `relates_to` field.
- **project-knowledge Totebox:** Add blueprints-driven related-articles sidebar card. This depends on the blueprints wiring in the step above.
- **project-knowledge Totebox:** Delete `collab.rs` (138 lines dead code); replace `search_complete` filesystem walk with Tantivy index query; inline `toc-persistence.js` into `wiki.js`.

**Sprint +2 (design system export — once engine changes are committed):**

- **project-design Totebox:** Receive this DESIGN-RESEARCH artifact and the two pending DESIGN-* drafts (`DESIGN-doc-header-component.draft.md`, `DESIGN-docs-sidenav-component.draft.md`) from `drafts-outbound`. Review completeness against `token-intake-checklist.md`. Commit refined component/token specifications to `pointsav-design-system/dtcg-vault/research/`.
- **project-design Totebox:** If the hybrid A+B recommendation is approved, produce a DESIGN-TOKEN-CHANGE draft for the scroll-spy TOC accent color and the Cmd+K palette overlay token. Both require `master_cosign:` in frontmatter before commit.

**Ongoing (operator decisions required before action):**

- **Command Session:** Confirm DNS cutover date for `documentation.woodfinegroup.com`. If within two sprints, escalate documentation UX fixes to current sprint.
- **Command Session:** Confirm operator decision on in-browser editing (Open Question 1) before any Architect C evaluation proceeds.
- **Command Session:** Confirm operator decision on corporate instance audience (Open Question 2) before implementing Architect B's tab-removal pattern on the corporate instance.
- **Command Session:** Confirm `mounts.rs` / `blueprints.rs` phase plan (Open Question 5) before wiring step in Sprint +1.
- **project-knowledge Totebox:** Add `cargo xtask check-frontmatter` (or equivalent CI step) that validates required frontmatter fields across content repos on each build. This implements the Architect C frontmatter validation benefit without adopting the full static pipeline.

**Outbox routing for this artifact:**

This DESIGN-RESEARCH draft is staged at `clones/project-knowledge/.agent/drafts-outbound/knowledge-platform-rewrite-analysis.draft.md`. Route to `project-design` via outbox for intake per `token-intake-checklist.md`. Target: `pointsav-design-system/dtcg-vault/research/knowledge-platform-rewrite-analysis.md`.
