---
schema: foundry-draft-v1
artifact_type: DESIGN-RESEARCH
language_protocol: DESIGN-RESEARCH
state: staged
created: 2026-06-03
author: totebox@project-knowledge
destination: project-design
target_repo: pointsav/pointsav-design-system
master_cosign: ""
research_done_count: 8
research_suggested_count: 0
open_questions_count: 0
research_provenance: >
  Browser audit via WebFetch of 5 live sites (documentation.pointsav.com,
  projects.woodfinegroup.com, corporate.woodfinegroup.com, home.woodfinegroup.com,
  home.pointsav.com). Comparator research via WebFetch of bloomberg.com,
  blackrock.com, stripe.com/docs, developers.cloudflare.com, palantir.com,
  carbondesignsystem.com, atlassian.design. 9 Opus agents, 143 tool uses,
  502 seconds wall-clock.
research_inline: true
notes: >
  Token changes (color darkening, type scale adjustments) require master_cosign
  before project-design can commit. Template restructuring (category collapse,
  sidenav filtering, TOC gating) is a CODE artifact for project-knowledge to
  implement directly — not a design-system token change. Routing: §1-§5 + §7
  to project-design for design system guidance; §6 (marketing memo) routed
  separately to project-marketing via Command outbox.
---

# DESIGN-RESEARCH: Institutional Redesign Brief — Knowledge Wikis

## §1 Executive Summary

The three-wiki portfolio rates **C-minus / 3.6 out of 10** on an institutional credibility scale. The foundations are strong — self-hosted Source Serif 4 + Inter, navy #164679 + gold #C7A961, a DTCG token system, and WCAG 2.2 AAA structure — but they are consistently undercut by prototype-grade finish: dead links in global chrome, empty structural scaffolding rendered to customer-facing surfaces, vendor brand leaking onto customer sites, and layouts that leave 40% of a 27-inch monitor empty on every sparse page.

**The single biggest systemic problem** is that all three wiki properties were built for the dense-article case and never finished for the common sparse case — empty TOC rails, single-item categories adrift in 3-column shells, blank right columns, and exposed internal taxonomy. This is the textbook startup-vs-institution tell: institutions never expose raw empty scaffolding.

**The one decision with the largest positive impact** is adopting per-surface, content-aware layout governance: listing/category/home templates collapse to a content-filling grid with designed empty states, while only dense article pages retain the 3-column reading shell. Make that structural decision correctly and roughly two-thirds of the P0/P1 findings dissolve — they are all symptoms of a single layout system applied indiscriminately.

---

## §2 Ranked Audit Findings

### P0 — Must fix before any institutional demo

| Site | Location | Issue | Fix |
|---|---|---|---|
| documentation.pointsav.com | Category template TOC rail (category.html L24-35) | "Contents" header renders outside the `{% if toc %}` guard → permanent dead column showing only the word "Contents" on ~35 category pages | Remove the TOC rail from category templates entirely; drop the column from the grid; never render the static "Contents" header without list items |
| documentation.pointsav.com | Sparse categories in fixed 3-column shell (Research=1, Archetypes=2) | One link floats in a viewport-wide shell with two empty side columns; reads as prototype | Card-grid layout `repeat(auto-fill, minmax(258px,1fr))`; designed low-content state for <3 articles |
| documentation.pointsav.com | Templates vs CSS divergence (wiki-* vs shell/topnav/docs-sidenav) | Two design generations ship in one 4,200-line stylesheet; live HTML uses legacy wiki-* chrome while the institutional redesign sits unused in CSS | Migrate article/category/home/search templates to the modern shell markup; delete the ~800-line dead wiki-* block |
| projects.woodfinegroup.com | Global footer (every page) — /wiki/disclaimers, /wiki/contact | Both legal/contact footer links 404 on 44+ pages | Author real Disclaimer + Contact pages; CI link-checker fails the build on any chrome 404 |
| projects.woodfinegroup.com | Top nav + login + footer (inner pages) | Vendor pointsav.com domains + "PointSav Knowledge" login title leak onto a Woodfine customer site | Make tenant brand authoritative across all templates; nav resolves only to *.woodfinegroup.com; audit all hard-coded pointsav.com hrefs |
| projects.woodfinegroup.com | Footer "Media kit" → /wiki/pointsav-media-kit | 404 + vendor brand slug on a customer URL | Remove or author /wiki/media-kit under tenant slug; drop "pointsav-" prefix |
| projects.woodfinegroup.com | /special/all-pages taxonomy | 44 articles fragmented into 16 single-letter A–Z buckets (8 with one item) — reads as a database dump | Group all-pages by the three real categories / topic families; never render single-item alphabet headings |
| corporate.woodfinegroup.com | Home "Featured / Recently updated" block | Links to /wiki/topic-equity-transfer-**mechanics** and /wiki/topic-fiduciary-data-**obligations** both 404 (real slugs are -model, -mandate) | Fix slug references in home data source; build-time link-checker fails render on internal 404 |
| corporate.woodfinegroup.com | Category templates | Abandon the 3-column shell → single narrow centered column, no sidenav, no right rail; tiny content on wide monitors | Unify category templates with the article shell: left sidenav + dense card grid + right metadata rail |
| corporate.woodfinegroup.com | /category/reference (1 article) | Heading + "1 article" + one link + vast blank expanse; no empty-state design | Low-density pattern: category overview block, related-category cross-links, fallback cards; merge single-article categories into a parent |

### P1 — Must fix before public launch

| Site | Location | Issue | Fix |
|---|---|---|---|
| documentation.pointsav.com | category.html L48 / article.html L76 | Category links built without `/wiki/` prefix → duplicate non-canonical routes | Normalize all internal links to `/wiki/<category>/<slug>`; 301 prefix-less aliases |
| documentation.pointsav.com | Footer vs Browse-by-Area | Footer says "10 categories" above a list of ~35 | Derive footer count from the same source as Browse-by-Area |
| documentation.pointsav.com | /wiki/archetypes, /wiki/design-system | Intuitive "go up a level" 404s; no branded 404 page | Make /wiki/<category> serve the category index OR ship a branded navigable 404 with search + top categories |
| documentation.pointsav.com | Browse-by-Area | Internal ops taxonomy (cluster-totebox-*, fleet-infrastructure-*, foundry-workspace, vault-privategit-*, node-console-operator) published to a customer surface | Split public product docs from internal ops; publish only ~8–12 curated public areas; gate the rest |
| projects.woodfinegroup.com | /category/comms | Single-article category surfaced as peer of Governance (32) and BIM (11) in primary nav | Consolidate into "Updates"/"Coverage Notes" or demote until 3+ articles |
| projects.woodfinegroup.com | /special/login in top nav | Public editor login in the masthead of a public wiki | Remove "Log in" from public nav; move auth to unlinked/IP-restricted admin path |
| projects.woodfinegroup.com / corporate | 3-column grid on no-TOC templates | Right 248px rail renders dead-empty on category/search/all-pages/home; ~40% of wide screen blank | Collapse to 2 columns or fill the right rail with contextual modules when no TOC exists |
| projects.woodfinegroup.com | Homepage hero | "Woodfine Projects **Wiki**" — consumer-grade title burying strong proof (7,594 clusters / 229,054 locations) | Rename to "Woodfine Research" / "Location Intelligence Knowledge Base"; lead with quantified metrics as headline |
| corporate.woodfinegroup.com | Whole site | ~18 articles / 4 categories, several meta pages — reads as a side-project | Increase apparent density (key-metrics strip, disclosure timeline, glossary teaser, "See also" depth); content roadmap |
| corporate.woodfinegroup.com | Home / nav | Single-column home; sidenav present only on article pages — navigation disappears where new visitors need orientation | Persistent left sidenav on every template; compose home as a 2–3 column dashboard above the fold |

### P2 — Polish

| Site | Location | Issue | Fix |
|---|---|---|---|
| documentation.pointsav.com | TOC <1100px (style.css L1876-79) | TOC `display:none` below 1100px with no replacement; navigation vanishes on laptops | Use the existing `.toc-toggle-btn` (L1890) for a collapsible inline TOC disclosure on <1100px |
| documentation.pointsav.com | Body type step-up (L292,299) | 17px→19px at 640px skews "reading app" over institutional density | Hold body at 18px uniformly; tighten line-height to 1.6; remove the step-up |
| documentation.pointsav.com | Category list items | Inconsistent: Archetypes shows excerpts, Help shows bare titles+dates | Standardize a single card partial: title + excerpt + last-updated + status badge |
| projects.woodfinegroup.com | Article metadata | "Last edited:" is Wikipedia-chrome; no author/reviewer | Replace with "Methodology · Woodfine Management Corp. · Revised <date>" + named reviewer where appropriate |
| projects.woodfinegroup.com | Long em-dash titles in 256px sidenav | Titles wrap to 3–4 lines; sidenav becomes a wall of wrapped text | Add a `nav_title` frontmatter field or single-line truncation with `title` tooltip |
| projects.woodfinegroup.com / corporate | Footer city names | "Vancouver · New York · Berlin" with no addresses or entity detail; reads aspirational | Add registered-office/entity detail or remove geography; pair with real Contact/Disclaimer pages |
| projects.woodfinegroup.com | ES toggle | ES sibling links not verified; any 404 becomes a broken-link vector | Verify every EN article has a live ES sibling; suppress the toggle where no translation exists |
| corporate.woodfinegroup.com | Category meta pages | About/Contact/Disclaimers mixed into the same flat list as substantive topics | Demote utility pages to a separate "About this section" row |
| corporate.woodfinegroup.com | Recently-updated dates | All share 2026-05-25 → signals one-time bulk import | Surface genuine per-article timestamps; use "Published" framing until real edit cadence exists |

---

## §3 Institutional Comparator Lessons

Twelve concrete patterns from Bloomberg, BlackRock, Stripe, Cloudflare, Palantir, IBM Carbon, and Atlassian to adopt:

1. **[Bloomberg] Editorial weighting over uniform grids** — Importance is expressed through SIZE, not position. The lead module is 2–4× larger than secondaries. Apply to wiki homes: one editorially-weighted featured article, then smaller secondary cards — never a grid of 12 equal tiles.

2. **[Stripe/Cloudflare] Intent-first home, taxonomy demoted to "browse all"** — Lead with a hand-curated "Start here / Most popular" task list; surface only POPULATED categories as cards; relegate the full tree to a secondary view. Exactly how a sparse wiki hides its gaps.

3. **[Stripe/Cloudflare/Carbon] Hide empty categories from navigation entirely** — An empty nav node is the #1 hobby-wiki tell. Never auto-generate sidebar nodes for zero-article categories; if roadmap signalling is required, label "Coming soon" explicitly. Directly kills the documentation.pointsav.com empty-TOC and internal-taxonomy leaks.

4. **[Carbon/NN-g/Atlassian] Designed three-part empty states** — Every legitimately-empty section gets: short positive-action title + one explanatory line ("what will appear here") + at most one CTA. Text-only when many empties co-occur; the CTA stays in a STABLE location, never floating in the void.

5. **[BlackRock/Palantir] "Invisible UI" — restraint reads as confidence** — Quiet sidebar/nav chrome, neutral surfaces, content-forward. Strip decorative color from the reading layer so prose and tables dominate.

6. **[Carbon] Single interactive color discipline** — One neutral gray scale + ONE blue for all links, primary actions, and focus. Don't introduce a palette of accents. Maps directly to promoting navy #0E3A66 as the single canonical interactive token.

7. **[BlackRock/Carbon] Fluid clamp() type scale + 65–80ch measure** — Type scales with viewport via clamp(); reading measure stays constrained even on ≥1440px (full-bleed body text reads as amateur); constrained line length aids screen readers and institutional density perception.

8. **[Carbon] Hierarchy via weight AND size** — Small section labels Semi-Bold (600), body and most headings Regular (400), largest displays Light (300). Fixes the flat-hierarchy problem across all sites.

9. **[Stripe] 3-column only for dense reference** — Left collapsible nav (240–280px) + constrained prose column + right sticky TOC. The 3-column shell is correct ONLY for dense long-form reference; never for listings or sparse articles.

10. **[Cloudflare] Breadcrumbs + visible changelog/freshness** — Breadcrumbs above every article title (emit JSON-LD BreadcrumbList); a dated "recently updated" surface broadcasts maintenance. Non-negotiable institutional trust markers.

11. **[Carbon] Cap nesting at 2 levels; quiet item weight; left active-indicator** — 256px left panel, max 2 tiers, chevron-accordion, current section auto-expanded, ~32px compact rows, left selection indicator (not a loud color fill).

12. **[Bloomberg/BlackRock/Carbon] Accessibility + trust infrastructure as credibility signals** — WCAG 2.1 AA minimum, CVD-safe status colors, ~44px touch targets, visible timestamps/version/source attribution. Institutions are accountable to broad audiences; auditability reads as trust.

---

## §4 Redesign Decision: Wiki HOME PAGE

All decisions are concrete and final. No hedging.

**Decision 1 — Desktop Column Layout (≥1280px)**

2-column. Left nav rail **280px fixed**; content area fills the remainder up to a **1200px max content width**, the whole shell centered with neutral margins absorbing overflow on wider monitors. **No right column on the home page.** The content area uses a 12-column internal grid (32px gutters) so featured/secondary modules size by editorial weight, not equal tiles. The home page is a listing surface — it never carries a TOC rail.

**Decision 2 — Home Page Content Pattern**

Replace the auto-generated category list with three stacked zones:

- **Zone A — Featured (editorial weighting):** 1 large featured slot (2/3 width) + 2 secondary slots (1/3 width stacked). Featured card fields: title (H2-scale), excerpt (2 lines, ~160 chars), category badge, last-updated date. Secondary cards: title + category badge + date (no excerpt).
- **Zone B — "Start here / Most popular":** hand-curated intent list of 5–7 task-oriented links, editorially chosen, not auto-generated.
- **Zone C — "Browse by area":** card grid `repeat(auto-fill, minmax(258px,1fr))` of **only populated public categories**.

Minimum article count threshold to show a category: **3.** Categories with <3 articles are folded into a parent or omitted — never shown as a standalone card. Internal ops taxonomy (cluster-/fleet-/foundry-/vault-/node-) is excluded from the public home entirely. Featured slots: 3 total (1 primary + 2 secondary), editorially assigned via frontmatter `featured: true` + `featured_weight: 1|2|3`.

**Decision 3 — Empty State Design**

A category with zero articles is never linked from the home page (threshold = 3). If a visitor reaches an empty category by direct URL, they see a designed empty state — not a blank page, not raw "no articles" text:

- **Visual:** a single quiet line-art glyph (navy on neutral, `aria-hidden`, decorative only) — no large playful illustration.
- **Copy (positive-action title):** "This area is being built." Body: "Articles on [Category] will appear here. In the meantime, explore related areas below."
- **CTA + density fill:** 3–4 related-category cross-link cards + a "Most popular across the knowledge base" fallback row. The page is never less than a screenful of substantive, navigable content.

**Decision 4 — Typography**

Keep Source Serif 4 for body — it matches the comparator finding that serif signals editorial authority and long-form readability (Bloomberg Businessweek DNA, IBM Plex Serif for long-form Carbon contexts). Pair with Inter for all UI/nav/labels (precise, data-forward), consistent with org doctrine and the Stripe/Carbon neutral-grotesque consensus.

| Role | Family | Size | Weight | Line height |
|---|---|---|---|---|
| Body | Source Serif 4 | **18px** | 400 | **1.6** |
| H1 (page title) | Inter | `clamp(34px, 4vw, 44px)` | 600 | 1.15 |
| H2 (section/featured) | Inter | 28px | 600 | 1.2 |
| H3 (card title) | Inter | 20px | 600 | 1.25 |
| Nav / UI labels | Inter | **14px** | 500 | — |
| Microcopy / legal | Inter | 12px | 400 | 1.5 |

Body floor raised from 17px to 18px. The 17px→19px responsive step-up is removed. Reserve bold strictly for true emphasis; carry hierarchy through weight + size, not size alone.

**Decision 5 — Color**

Navy #164679 is institutionally appropriate but renders slightly mid-tone for headings and links at the institutional-austere bar set by Bloomberg and Carbon. Two tokens standardized:

- **`--color-interactive`** → **#0E3A66** (darkened navy; single link/focus/primary-action color, Carbon-style one-blue discipline)
- **`--color-brand-surface`** → **#164679** (retained for masthead fills and large brand moments)
- **`--color-accent`** → **#C7A961** gold (sparing only: rules, active indicators, key metrics — never a fill behind text)

The `master_cosign:` field in this draft's frontmatter must be populated by Master before project-design commits any token changes.

**Decision 6 — Density Target**

| Parameter | Value |
|---|---|
| Body line-height | 1.6 |
| Nav item height | 36px |
| Vertical section spacing (home) | 48px |
| Card grid gutter | 24px |
| Card internal padding | 24px |
| Max content width (home/listing) | 1200px |
| Reading measure (article) | see §5 Decision 2 |

---

## §5 Redesign Decision: Wiki ARTICLE PAGE

**Decision 1 — Column Layout**

3-column **only when the article warrants a TOC**: left sidenav **256px fixed** + center reading column (constrained, see Decision 2) + right TOC rail **248px**. **The right TOC column appears only when the article has ≥3 H2/H3 headings AND viewport ≥1100px.** Below those thresholds the layout is 2-column (sidenav + content), and the TOC becomes a collapsible inline disclosure at the top of the article using the existing `.toc-toggle-btn`. The TOC rail is **never rendered empty** — no headings, no rail; the column collapses and the reading measure widens within its cap.

**Decision 2 — Reading Column Width**

Change from **68ch to `clamp(62ch, 70vw, 80ch)`**. 68ch (~595px) left wide viewports half-empty; comparator consensus is ~65–80ch / 600–800px. Allowing growth toward 80ch on ≥1440px lets the center column carry the page while staying within the readability ceiling. Body 18px / 1.6 line-height.

**Decision 3 — Sidenav Behavior**

The sidenav renders only populated public categories (≥1 published article), driven from a single template partial that filters zero-article categories at render time — they are never emitted as dead nodes. No article-count badges (badges advertise sparseness in public docs; a clean tree outperforms). Max 2 nesting levels; current section auto-expanded; active item carries a left selection indicator (`3px solid #C7A961`), not a color fill. Internal ops taxonomy excluded from the public sidenav entirely. Zero-article categories handled at template level by a `{% if category.article_count > 0 %}` guard around each node.

**Decision 4 — Article Header Design**

Visual hierarchy, top to bottom:

1. **Breadcrumb** (Inter 13px, #0E3A66 links): `Home › [Category] › [Article]` — emit JSON-LD BreadcrumbList.
2. **Article title** (`<h1>`, Inter, `clamp(30px, 3.5vw, 40px)`, weight 600, #0E3A66).
3. **Metadata row** (single line, Inter 14px, `--color-fg-3` muted): **category badge** (gold-ruled pill) · **"Methodology · Woodfine Management Corp."** · **"Revised <date>"** — institutional citation block replacing "Last edited:" Wikipedia-chrome. Named reviewer appended where governance/regulated content warrants.
4. Horizontal rule (1px `--color-border`), then prose body.

**Decision 5 — Broken Links**

A **build-time internal link-checker runs against the rendered sitemap** before deploy. It catches:

- Any internal `href` that does not resolve to a published page in the known page set (the corporate.woodfinegroup.com -mechanics/-obligations class of error)
- Any 404 in global chrome (footer/nav — the projects.woodfinegroup.com Disclaimer/Contact class)
- Non-canonical alias links missing the `/wiki/` prefix
- ES toggle targets with no live ES sibling

**On any failure the Rust render binary refuses to emit the build and the deploy is blocked.** A 404 in global chrome or on a featured-article link is a hard build failure, not a warning. Cross-property sister links are included in the same check.

---

## §6 Implementation Priority Order

Ordered by impact. The first 5 are blocking for any institutional demo.

| Priority | Task | Scope | Owner |
|---|---|---|---|
| P0-1 | Build-time internal link-checker that blocks deploy on any chrome or featured-slot 404 | Medium | project-knowledge |
| P0-2 | Remove vendor brand leak from projects.woodfinegroup.com (nav/login/footer pointsav.com → woodfinegroup.com; "PointSav Knowledge" → Woodfine wordmark) | Medium | project-knowledge |
| P0-3 | Author real Disclaimer + Contact pages for projects.woodfinegroup.com; remove /wiki/pointsav-media-kit | Small | project-knowledge |
| P0-4 | Category-template redesign across all wikis: drop empty TOC rail, collapse to content-filling card grid, add designed empty states per §4 Decision 3 | Large | project-knowledge |
| P0-5 | Fix corporate.woodfinegroup.com featured-article slug references (-mechanics/-obligations → -model/-mandate) | Small | project-knowledge |
| P1-6 | Migrate documentation.pointsav.com templates to the modern shell/topnav/docs-sidenav markup; delete the ~800-line dead wiki-* CSS block | Large | project-knowledge |
| P1-7 | Split public docs from internal ops taxonomy; curate Browse-by-Area to ~8–12 public areas; zero-article categories hidden from sidenav and home per §5 Decision 3 | Medium | project-knowledge |
| P1-8 | Normalize all internal routing to canonical `/wiki/<category>/<slug>`; 301 prefix-less aliases; reconcile footer category count | Medium | project-knowledge |
| P1-9 | Token changes: `--color-interactive` → #0E3A66; `--color-brand-surface` retains #164679; body size 18px; nav minimum 14px (requires master_cosign) | Small | project-design |
| P1-10 | Home page zone restructure per §4 Decision 2: Zone A (featured) + Zone B (intent list) + Zone C (populated-category grid) | Large | project-knowledge |
| P1-11 | Article header redesign per §5 Decision 4: breadcrumb + clamp h1 + institutional metadata row | Medium | project-knowledge |
| P2-12 | TOC: collapsible inline fallback below 1100px using existing .toc-toggle-btn; suppress right rail when no headings | Small | project-knowledge |
