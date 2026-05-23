---
schema: foundry-draft-v1
state: draft-pending-design-pass
originating_cluster: project-knowledge
target_repo: pointsav-design-system
target_path: research/
target_filename: DESIGN-RESEARCH-service-design.md
audience: internal
bcsc_class: no-disclosure-implication
language_protocol: DESIGN-RESEARCH
authored: 2026-05-23
authored_by: totebox@project-knowledge
authored_with: claude-opus-4-7
---

# DESIGN-RESEARCH — Service design of the knowledge platform

A user-journey audit of `app-mediakit-knowledge` across its three deployment instances (`documentation.pointsav.com`, `projects.woodfinegroup.com`, `corporate.woodfinegroup.com`), grounded in `src/server.rs` lines 1031–1499 (`home_chrome`) and 2102–2674 (`wiki_chrome`). Three target audiences are auditors, engineers, and procurement evaluators. The brief: move the platform from a C-grade experience to A-minus by addressing information-architecture friction the chrome already accumulated during phases 1–5.

---

## 1. Executive summary

The home page presents a welcome banner and a four-box editorial grid (featured article, "Did you know", recently updated, reference invariants — `server.rs` 1216–1306) before the category grid (`server.rs` 1310–1341). The category grid itself was demoted to "Browse by area" and reduced to category titles + counts + a one-line description, with no preview of the articles beneath each category. The result is that an auditor or engineer arriving at the home page must scroll past four editorial panels and a wordmark band before reaching anything resembling structured access to the corpus, and once they reach it the category names are the deepest exposure the home page provides — they cannot see what is inside without a click.

The article page (`wiki_chrome`) shows a left-rail Navigation portlet (`server.rs` 2338–2352) duplicating site-wide links already present in the top nav, a left-rail TOC (2354–2381) which is correctly placed, and a right-rail Tools portlet (2604–2630) carrying four utility links of low reader value at the right margin where the eye expects either citation/research context or page progress. The three-column layout currently fails the reader by giving prime visual real-estate to redundant site navigation and underused page tools instead of the research provenance (`research_trail`) which is buried inside a closed `<details>` element at the article foot (2536–2563).

The five highest-leverage fixes, in priority order: (1) elevate cross-wiki navigation from the home-page foot to the persistent header alongside Home / All pages; (2) expand the category grid to show 3–5 article titles per category on the home page so the home page is a *map of the corpus*, not a list of *names of buckets*; (3) replace the left-rail Navigation portlet with category navigation on article pages; (4) hoist the research trail out of `<details>` and into the right rail as the reader's primary context; (5) remove or radically demote the DYK + Featured Article grid on the two Woodfine instances, where the audience is internal/auditorial and editorial rotation adds noise.

---

## 2. User journey map: Auditor

**Mission.** Verify a specific institutional claim, find its source, confirm provenance and freshness.

**Step 1.** Arrive at `documentation.pointsav.com` from an external link, an audit-trail footnote, or a typed URL.

**Step 2.** Read the welcome banner — "Welcome to PointSav Wiki, the corporate knowledge wiki for the PointSav Digital Systems platform" + article/category count (`server.rs` 1181–1202). *Useful — establishes scale and scope in two stats.*

**Step 3.** Scan the four-box editorial grid (`server.rs` 1217–1307): "From today's featured article", "Did you know…", "Recently updated", "Reference invariants". **[FRICTION-A1]** *None of these four panels answer "where is the claim I came to verify?" The auditor must skip them and scroll.*

**Step 4.** Reach "Browse by area" — the demoted category grid (`server.rs` 1310–1341). Sees 12 category names (after the 2026-05-09 split: architecture, substrate, patterns, systems, services, applications, governance, infrastructure, reference, design-system; `company/` and `help/` retired) with a single-line description and an article count. **[FRICTION-A2]** *The category card shows neither article titles nor a search affordance scoped to that category. The auditor must click `/category/<name>` to discover whether the right article exists, then back out and try another category if not.*

**Step 5.** Either (a) use the header search field — present at `server.rs` 1108–1114 — or (b) click into a category and visually scan. **[FRICTION-A3]** *Search-first is the right behaviour but the search box competes with a duplicated search input in the welcome banner (`server.rs` 1206–1213). Two search inputs above the fold dilutes the affordance.*

**Step 6.** Land on the target article. Read body. **[FRICTION-A4]** *The single most important auditor question — "what is the provenance of this claim and when was it last verified?" — is answered by the `research_trail` block, which is rendered as a collapsed `<details>` element at the article foot (`server.rs` 2536–2563). The auditor has to scroll past the entire body, find the closed disclosure widget, click it, and only then see provenance.* The `wiki-ivc-band` (`server.rs` 2470–2485) reads "Verification not yet available — Phase 7" — the masthead position that *should* answer the provenance question is currently occupied by a Phase-7 placeholder.

**Step 7.** Confirm freshness via `last_edited` in the article footer (`server.rs` 2592–2598). *Works — but the auditor reached it only after the full read.*

**Recommendations (Auditor).**

- **R-A1.** Move research-trail into the right rail as the persistent, open companion to the article body. Anchor at top-of-rail. This is the auditor's primary instrument; it should not be a foot-of-page disclosure.
- **R-A2.** Replace the Phase-7 placeholder `wiki-ivc-band` with a freshness + provenance summary band derived from `last_edited`, `editor`, and the count of unresolved citations. Even before Phase-7 verification ships, the band can carry a meaningful "Last reviewed YYYY-MM-DD · N citations resolved" line.
- **R-A3.** Remove the duplicated search input from the welcome banner (`server.rs` 1206–1213). One search affordance, in the header, where muscle-memory expects it.
- **R-A4.** Expose category-scoped search from each category card so an auditor scanning the home page can search inside `governance/` directly without first landing in the category.

---

## 3. User journey map: Engineer

**Mission.** Find the specific service / architecture / ADR doc, scan its scope, jump to the relevant section.

**Step 1.** Arrive at the home page (typed URL or external reference from code/PR).

**Step 2.** Skip past the editorial grid. **[FRICTION-E1]** *The editorial grid is high-pixel real estate the engineer will skip every time. On a 1080p viewport the four boxes plus welcome banner consume the first ~900 vertical pixels; the engineer scrolls past them on every visit.*

**Step 3.** Reach "Browse by area". Scan category names. **[FRICTION-E2]** *Engineers know what they want — usually a specific `service-*` or `sys-adr-*` slug. The category grid asks them to guess which category contains the article, when the cheapest action is search-by-slug or A–Z. The home page does not surface a clear "All services" or "All ADRs" entry-point at the level the engineer thinks at. (`/special/all-pages` exists in the nav at `server.rs` 1141, but it is one link in a horizontal nav row of five — easy to miss.)*

**Step 4.** Resort to header search (`server.rs` 1108–1114) and type a slug fragment. Land on the target article.

**Click count to a known target.** Optimistic path: header search → result → article = 1 click. Browse path: home → category → article = 2 clicks, contingent on category guess being correct. The browse path is unreliable because the home page does not surface article titles, only category names.

**Step 5.** On the article page: scan the TOC in the left rail (`server.rs` 2354–2381). *This works well — the TOC is hierarchically numbered (1, 1.1, 2, 2.1) and pinable. This is one of the article-shell's strongest features.*

**Step 6.** Within-article navigation. *Works.*

**Step 7.** Engineer needs the source: `View source` in the footer (`server.rs` 2646) and `Download as Markdown` (`server.rs` 2464). **[FRICTION-E3]** *The "View source" link is in the page footer, far from the article body. The engineer reading section 3.2 of a long article has to scroll to the bottom of the page to grab the raw markdown — and "View source" / "Download as Markdown" duplicate each other across the More-actions dropdown and the footer.*

**Step 8.** Engineer wants to know what links here (refactor question: "if I touch this article, what breaks?"). The "What links here" link is in the right-rail Tools portlet (`server.rs` 2608–2612). **[FRICTION-E4]** *The Tools portlet is buried at the right margin and contains only four links, two of which (`Permanent link`, `Cite this page`) the engineer does not need. The single useful tool is not visually separated from the noise.*

**Recommendations (Engineer).**

- **R-E1.** Replace the four-box editorial grid on `documentation.pointsav.com` with a compact `[Featured | Recently updated]` two-box band above the category grid, recovering ~400 vertical pixels for the corpus map. DYK and Reference Invariants do not earn their pixel cost for engineering readers — see §10.
- **R-E2.** Expand each category card in "Browse by area" to show the top 5 articles by inbound-link-count (the redb wikilink graph supports this), with an "All 47 →" footer link. The home page becomes a corpus *map*, not a corpus *legend*.
- **R-E3.** Promote `View source` and `What links here` from the footer/Tools-portlet into the article masthead next to `View history`. The engineering reader uses them often; the casual reader does not.
- **R-E4.** Add a "Browse by type" affordance to the home page: a short row of pill links — `Services (19)` · `ADRs (12)` · `Systems (10)` · `Patterns (10)` — based on the Nomenclature-Matrix entity-prefix taxonomy. Engineers think in slug prefixes, not categories.

---

## 4. User journey map: Procurement evaluator

**Mission.** Form a 60-second impression of the institutional quality of the platform. Decide whether the vendor is serious.

**Step 1.** Arrive at the home page — typically referred by sales material, an analyst note, or due-diligence research.

**Step 2.** Above-the-fold impression — first viewport (~900px at 1080p). Currently this shows: header + brand row + welcome banner ("Welcome to PointSav Wiki, the corporate knowledge wiki for the PointSav Digital Systems platform" + article count) + the *top* of the editorial grid. **[FRICTION-P1]** *The above-the-fold real estate is dominated by the word "wiki" and an editorial-magazine convention (featured article + DYK) which has been transplanted from Wikipedia. For an evaluator who has never heard the leapfrog framing, this reads as "open-source community wiki", not "institutional knowledge platform". The Bloomberg / financial register the operator wants is not signalled in the first 900 vertical pixels.*

**Step 3.** Read the lede (`server.rs` 1203–1205 renders `home_html` — the rendered `index.md` lede). The current `documentation.pointsav.com` lede is dense and on-register: *"PointSav builds operating systems and services for regulated businesses that need to own their data, their AI, and their record-keeping outright."* *This is the right copy in the wrong position — it sits inside the welcome-banner block, after the article-count statistic, where the magazine-convention chrome below it drowns it out.*

**Step 4.** Scroll. Encounter the four-box editorial grid. **[FRICTION-P2]** *For an evaluator, the DYK panel ("…that ToteboxOS is built on seL4…") and the Featured Article both *are* strong institutional signals — but they read as community-wiki conventions, not as a curated investor-grade prospectus. The reference-invariants panel is the closest the home page gets to a "platform claims register" but it competes for attention with DYK rather than anchoring above it.*

**Step 5.** Reach "Browse by area". Sees 10 categories. **[FRICTION-P3]** *The categories signal taxonomy depth, which is good — but with no article titles exposed, the evaluator has no way to form a quality impression of any individual category. "Governance — 20 articles" is a fact; the evaluator cannot tell whether those 20 articles are short notes or substantive ADRs.*

**Step 6.** Reach "Operational guides" (`server.rs` 1359–1383). *Useful — concrete and operational.*

**Step 7.** Reach "Sister surfaces" (`server.rs` 1386–1460) at the bottom of the page. **[FRICTION-P4]** *The cross-wiki story — that PointSav (vendor) ships an engine which Woodfine (customer) runs as two more instances — is the single strongest institutional signal on the page, and it is at the bottom where most evaluators will never see it. The "Sister surfaces" label itself is editorial-wiki vocabulary borrowed from Wikipedia; the institutional version is "Related platforms" or "Other deployments".*

**Recommendations (Procurement evaluator).**

- **R-P1.** Above-the-fold information hierarchy on the documentation instance should read, in vertical order: site title + cross-wiki bar (one row, persistent) → lede paragraph (the existing `index.md` body, promoted) → article-count stats line → header search → reference-invariants panel as the *single* anchoring editorial slot above the category grid. DYK and Featured Article are demoted below the category grid (they reward returning readers, not first-impression readers).
- **R-P2.** Add an explicit "About this wiki" link at the top of the welcome banner pointing to a one-paragraph plain-prose orientation page — the existing footer link to `/wiki/about` is not enough for an evaluator who will not scroll.
- **R-P3.** Rename "Sister surfaces" to "Other deployments" (vendor) / "Other platforms" (customer) and elevate as described in §8.
- **R-P4.** Surface a single statistical statement of institutional weight above the fold: "Editorial standard: Bloomberg register · All forward-looking statements per NI 51-102". Currently this discipline is documented inside articles but invisible at the home-page level.

---

## 5. Home page information hierarchy

### 5.1 What is currently above the fold

Assuming a 1080p viewport with ~80px header, the first 900–1000 vertical pixels of `/` on `documentation.pointsav.com` currently render approximately:

1. `header.mw-header` — site title, search input, appearance menu, site-nav (5 links), brand row with wordmark, nav-row with Disclaimer / Contact / pointsav.com / GitHub (`server.rs` 1106–1177). ~180px.
2. `div.wiki-home-welcome` — `<h1>` welcome, tagline, article-count stats line, lede paragraph (rendered from `index.md`), second search input (`server.rs` 1181–1214). ~360px.
3. Top of `div.wiki-home-top-panels` four-box editorial grid (`server.rs` 1217). The top two boxes of the 2×2 grid — Featured Article and DYK — begin in the next ~360px.

What is **not** above the fold: any category, any article title, any cross-wiki link, the reference-invariants panel.

### 5.2 What should be above the fold

For *all three audiences*, the home page's first 900px should answer: *what is this, what is in it, where else do I go?* Specifically:

1. **Persistent top bar** with site title, search, **and** cross-wiki tab strip (see §8). ~80px.
2. **Hero band**: `<h1>` (site title), one-line tagline, **lede paragraph as primary copy** (currently buried mid-welcome-banner), article-count stats line. No second search input. ~280px.
3. **Reference invariants panel** as the single anchor editorial slot — three structural claims, link-out per claim. ~200px.
4. **First row of "Browse by area"** — visible category cards with article-title previews. ~340px.

The four-box editorial grid (Featured / DYK / Recently updated / Reference invariants) is **decomposed**: Reference Invariants promoted above the fold; Featured + DYK + Recently updated demoted to a single horizontal strip *below* the category grid, where they reward returning readers without dominating the first impression.

### 5.3 Specific layout prescription

```
┌──────────────────────────────────────────────────────────────────────┐
│ HEADER ROW                                                           │
│ [PointSav Wiki]  [Search …]                       [Aa] [auth nav]    │
├──────────────────────────────────────────────────────────────────────┤
│ CROSS-WIKI TAB STRIP                                                 │
│ ● Documentation  ○ Projects  ○ Corporate Reference                   │
├──────────────────────────────────────────────────────────────────────┤
│ HERO                                                                 │
│ Welcome to PointSav Wiki                                             │
│                                                                      │
│ PointSav builds operating systems and services for regulated         │
│ businesses that need to own their data, their AI, and their          │
│ record-keeping outright. […single-paragraph lede…]                   │
│                                                                      │
│ 213 articles in 10 categories · last updated 2026-05-22              │
├──────────────────────────────────────────────────────────────────────┤
│ REFERENCE INVARIANTS (single anchor editorial slot, above fold)      │
│ Three structural claims · [more →] per claim                         │
├──────────────────────────────────────────────────────────────────────┤
│ BROWSE BY AREA — expanded category cards (3 columns × N rows)        │
│ ┌─ Architecture (35) ──┐ ┌─ Substrate (30) ──┐ ┌─ Services (19) ──┐  │
│ │ description          │ │ description       │ │ description      │  │
│ │ • three-ring-arch    │ │ • compounding-sub │ │ • service-email  │  │
│ │ • doorman-protocol   │ │ • disclosure-sub  │ │ • service-people │  │
│ │ • source-of-truth-…  │ │ • citation-sub    │ │ • service-slm    │  │
│ │ • foundry-doctrine-… │ │ • apprenticeship  │ │ • service-search │  │
│ │ • leapfrog-2030-arch │ │ • language-proto- │ │ • service-content│  │
│ │ All 35 →             │ │ All 30 →          │ │ All 19 →         │  │
│ └──────────────────────┘ └───────────────────┘ └──────────────────┘  │
│ […remaining categories…]                                             │
├──────────────────────────────────────────────────────────────────────┤
│ BROWSE BY TYPE (engineer affordance)                                 │
│ Services (19) · Systems (10) · ADRs (12) · Patterns (10) · …         │
├──────────────────────────────────────────────────────────────────────┤
│ OPERATIONAL GUIDES                                                   │
├──────────────────────────────────────────────────────────────────────┤
│ EDITORIAL STRIP (demoted)                                            │
│ Featured · Recently updated · (DYK only on vendor instance)          │
├──────────────────────────────────────────────────────────────────────┤
│ FOOTER                                                               │
└──────────────────────────────────────────────────────────────────────┘
```

The decisive structural change: the **category grid is the hero, not the chrome**. The home page's job is to be a map of the corpus. The editorial grid is a magazine-cover convention transplanted from Wikipedia and should be subordinate to the map, not above it.

---

## 6. Category structure audit

### 6.1 Are the 12 categories correct?

The post-2026-05-09 taxonomy has **10 active categories** (architecture, substrate, patterns, systems, services, applications, governance, infrastructure, reference, design-system) — `company/` and `help/` were retired. The task brief says "12 categories"; the engine iterates `RATIFIED_CATEGORIES` (`server.rs` 1312) and the live count is 10 on `documentation.pointsav.com`.

Strengths of the current taxonomy:

- **Material distinctiveness.** `architecture` (cross-cutting) / `substrate` (mechanism concepts) / `patterns` (named recurring shapes) is a genuine three-way distinction, not a synonym set.
- **Nomenclature-aligned entity buckets.** `systems` (`os-*`) / `services` (`service-*`) / `applications` (`app-*`) match the slug-prefix discipline.
- **Distinct platform-vs-deployment split.** `architecture` (abstract / logical) vs `infrastructure` (deployed / runtime) is a useful editorial line.

Weaknesses:

- **`reference` is a residual bucket.** It carries glossary, nomenclature, style guide, templates, *and* miscellaneous reference articles (e.g. `bim-market-context`). A reader scanning the home page does not know which subset to expect.
- **`design-system` is currently a four-pair category** (post-2026-05-16 split — design-philosophy, design-primitive-vocabulary, brand-family-swatch, brand-typography). It is undersized relative to the wiki mean and the four articles it contains are all "design system as a platform component" framings. **The category card on the home page will read "Design-system — 4 articles", which understates its conceptual weight.**
- **Discoverability of the entity-prefix taxonomy is implicit.** A reader looking for "an ADR" must know that ADRs live in `governance/` — there is no signpost.

### 6.2 Are they discoverable?

The category names are visible in the home-page grid (`server.rs` 1310–1341) and in the dedicated `/special/categories` page (link at `server.rs` 1142). However:

- The home-page category card shows only the title, count, and one-line description (`cat_descriptions` — `server.rs` 1335–1337) — no article previews. **A reader cannot tell from the home page whether a category is healthy or stub-heavy.**
- The two-search-input redundancy on the home page dilutes search-first discovery (`server.rs` 1108–1114 and 1206–1213).
- The "Operational guides" section (`server.rs` 1359–1383) groups by `domain` (returned from `bucket_guides_by_domain`), which is a *separate* taxonomy from category — useful, but the two taxonomies are not visually distinguished, which can confuse a first-time reader.

### 6.3 Recommendations

- **R-C1. Expand category cards.** Each card on "Browse by area" shows the category title, count, one-line description, and the top 3–5 article titles by inbound-link-count, with an "All N →" link. The redb wikilink graph already supports inbound-count retrieval; this is engine plumbing, not new data.
- **R-C2. Add a "Browse by type" strip** below the category grid: `Services · Systems · Apps · ADRs · Patterns · Substrates`. This is a slug-prefix-driven secondary navigation that surfaces the engineering taxonomy alongside the editorial one.
- **R-C3. Either merge `design-system/` into `reference/` as a `Design-system substrate` sub-grouping** — since the four surviving articles are all platform-framing pieces — **or rename it to `design/` and broaden its scope** to include design-research and design-system architecture without the implementation specifics (which now live in `pointsav-design-system/`).
- **R-C4. Re-examine `reference/`.** Split into `glossary/` (terminology, style guide, nomenclature, templates) and either rename or retire the residual `reference/` bucket. A "reference" category that is a grab-bag tells the reader nothing.

---

## 7. Article page IA prescription

The article page renders a three-column layout (`server.rs` 2335 `div.wiki-layout` → 2338 `#mw-panel` left rail / 2385 `main.mw-body` centre / 2604 `div.wiki-right-rail` right rail). Recommendations follow the three columns.

### 7.1 Left rail (`server.rs` 2338–2382) — currently competes with the content

The left rail today renders **Navigation portlet** (Main page, Random article, Wanted articles, All pages, Categories, Recent changes, Statistics, Search — 8 links; `server.rs` 2340–2352) plus **TOC portlet** (`server.rs` 2354–2381).

Problems:

- The Navigation portlet links are **duplicated** from the top-bar `nav.site-nav` (`server.rs` 2220–2226 — Home + lang toggle) and `nav.nav-row` (`server.rs` 2248–2266 — Disclaimer / Contact / etc.) and the mobile nav drawer (`server.rs` 2302–2310). The reader who has come to read an article does not need to be reminded of "Random article" and "Statistics" at the top of the left rail.
- The Navigation portlet pushes the TOC down. The TOC is the single most valuable left-rail element for an engineer scanning a long article.

Prescription:

- **R-L1. Remove the Navigation portlet from the article page left rail.** The links are already in the header and the mobile drawer.
- **R-L2. Replace the Navigation portlet with a Category navigation portlet** — show the current article's category name and the other articles in the same category, top 8 by inbound-link-count, marking the current article with a bold or marker. This converts dead chrome into a same-category sibling navigator, which is the reader's *actual* next-best-action when finishing an article.
- **R-L3. TOC stays where it is** (`server.rs` 2354–2381) — pinnable, hierarchically numbered, collapsible. It is one of the article shell's strongest components.

### 7.2 Centre column (`server.rs` 2385–2601) — promote provenance, demote the band placeholder

Current order: title row + tabs (2388–2468) → `wiki-ivc-band` placeholder (2470–2485) → density toggle (2477–2483) → redirected-from hatnote (2487–2493) → forward-looking-information notice (2496–2503) → stub notice (2505–2510) → hatnote (2512–2517) → disambiguation notice (2519–2527) → body (2530–2534) → research trail (2536–2563) → footer (2565–2600).

Problems:

- **The `wiki-ivc-band` reads "Verification not yet available — Phase 7"** (`server.rs` 2472–2474). This is the article masthead — the most-read pixels on the page. A placeholder there reads as broken.
- **The density toggle ("Citation marks: Off / Exceptions only / All")** at `server.rs` 2477–2483 is a power-user feature sitting next to a verification placeholder. The reader does not know what citation marks are; they see "Off" / "On" buttons and wonder what they will turn off.
- **The research trail at `server.rs` 2536–2563** is a collapsed `<details>` element below the article body. As noted in §2, this is the auditor's primary instrument — its current position is wrong.

Prescription:

- **R-M1. Replace the `wiki-ivc-band` placeholder with a useful provenance band immediately:** "Last reviewed `last_edited` by `editor`. N citations · M unresolved." This uses data the engine already has. The Phase-7 cryptographic verification message can return when Phase 7 ships.
- **R-M2. Move the density toggle into the Appearance menu** (`server.rs` 2196–2219) where it sits beside Color / Width — the other reader-density preferences.
- **R-M3. Hoist the research trail out of `<details>` and into the right rail (see §7.3).** Leave a "Research trail" anchor link in the article footer pointing at the right-rail position.

### 7.3 Right rail (`server.rs` 2604–2630) — currently underused; should carry context

Currently renders a "Tools" portlet with four links: What links here / Permanent link / Page information / Cite this page.

Problems:

- The right rail is **the prime context column** in a three-column reading layout. Wikipedia uses it for infobox, sister-project links, and (under Vector 2022) the article tools menu. The current PointSav engine uses it for four utility links, three of which the reader does not need on first read.
- The research trail — which *is* the article's context — sits in the foot of the centre column.

Prescription:

- **R-R1. The right rail becomes the Context rail.** Stack, top to bottom:
  1. **Research trail block** — fully expanded by default for `audit` audience, collapsible by reader preference. Each field of `research_trail` rendered as a labelled paragraph.
  2. **Citation summary** — "N citations resolved · M drift detected · K unresolved." Each row links to the in-body citation. (Engine plumbing: read `references:` from frontmatter, render counts.)
  3. **Page tools** — collapsed accordion of the current four-link Tools portlet. Surface "What links here" prominently; keep the other three under a `[more tools]` disclosure.
- **R-R2. Add a per-section freshness indicator** — small grey timestamp at the right margin of each `<h2>` indicating last-edited-date for that section (derivable from git blame when implemented). Until then, the per-article `last_edited` continues to anchor the article footer.

### 7.4 Tabs and More-actions duplication (`server.rs` 2456–2467 and 2643–2647)

Current state: `Article / Talk` tabs top-left, `Read / View history` tabs top-right, `▾` More-actions dropdown with `Print / Page information / Cite this page / Download as Markdown`, and the footer also carries `View source / Sitemap`.

- **R-T1. Single canonical location for `View source`.** Put it next to `View history` in the page-actions tab strip (`server.rs` 2444–2455). Remove from the footer.
- **R-T2. Remove `Cite this page` and `Page information` from the More-actions menu** when they are also in the right-rail Tools portlet. One affordance per link.

---

## 8. Cross-wiki navigation prescription

The platform is structurally three instances (per `KNOWLEDGE-PLATFORM-VISION.md` §3): documentation (vendor) / projects (customer) / corporate (customer). The cross-wiki story is the single strongest institutional signal on the home page — it tells the evaluator that this is *the same engine, multiple sovereign deployments*, which is the leapfrog claim made concrete.

### 8.1 Current state

Cross-wiki navigation lives in two places:

1. **`nav.nav-row` upper-right** (`server.rs` 1158–1176, also 2248–2266 for article page) — shows different links per theme:
   - vendor (PointSav theme): `pointsav.com` / `GitHub`
   - woodfine: `Projects` / `Newsroom`
   - woodfine-projects: `Corporate` / `Newsroom`
2. **"Sister surfaces" section** at the bottom of the home page (`server.rs` 1386–1460) — three or four cards listing the other deployment instances. **This is the only complete cross-wiki map on the site.**

Problems:

- The `nav.nav-row` upper-right link list is asymmetric and partial — the documentation instance does not link to either of the woodfine instances in the persistent header; it links to `pointsav.com` and `GitHub` instead. **The vendor wiki gives no header-level signal that customer instances exist.**
- "Sister surfaces" at the home-page foot will be seen by perhaps 15–25% of visitors (typical scroll-completion rates for a content-dense home page). The evaluator audience least likely to scroll to it is also the audience the cross-wiki map most rewards.
- The label "Sister surfaces" is community-wiki vocabulary borrowed from Wikipedia ("Sister projects"). For an institutional reader it reads as decorative; it does not name the architectural fact ("three sovereign deployments of one engine").

### 8.2 Prescription

- **R-X1. Elevate cross-wiki navigation to a persistent top-of-page strip** — between the header and the brand row on both home and article pages. A three-tab horizontal strip:

  ```
  ●Documentation     ○Projects     ○Corporate Reference
  ```

  The current tab is marked with a filled dot or active styling; the other two are plain links to the sister hostnames. On a customer instance the strip orders Projects / Corporate / Documentation or Corporate / Projects / Documentation per which is current.

- **R-X2. Rename "Sister surfaces" to "Other deployments"** (vendor instance) or **"Other platforms"** (customer instances). Keep the home-page bottom-of-page card grid as a *secondary, detailed* cross-wiki disclosure with one-line descriptions and external links (e.g. `gis.woodfinegroup.com` for the GIS app from the projects instance — `server.rs` 1397–1401).

- **R-X3. On article pages, the persistent cross-wiki strip carries one extra affordance**: a "Search across all three" link that submits the current query to a federated search endpoint (Phase 6 capability — for now, link to the search endpoint with a placeholder note).

- **R-X4. Drop the asymmetric `nav.nav-row` upper-right** links (`server.rs` 1164–1175). The cross-wiki strip replaces it. The remaining persistent header links collapse to: Disclaimer · Contact · Newsroom (customer instances only). External links to `pointsav.com` and `GitHub` move into the foot-of-page "Other deployments" grid.

---

## 9. Editorial grid recommendation

The four-box editorial grid (`server.rs` 1217–1306) renders, when populated: Featured Article, DYK, Recently Updated, Reference Invariants. Each box's appearance is gated by data availability (`@if let Some(ref featured) =`, `@if let Some(ref dyk) =`, etc.). All three instances run the same engine.

### 9.1 Per-instance disposition

**documentation.pointsav.com (vendor — engineers + procurement evaluators + auditors).**

- **Featured Article: KEEP, demote.** A rotating featured article is a useful institutional signal — it shows curation and editorial weight. Demote from above-the-fold to below the category grid.
- **Did You Know: KEEP, demote.** The current DYK content (`leapfrog-facts.yaml`) is on-register — claims about seL4, three-ring architecture, customer-hostability — and serves the procurement evaluator as a punchy claims register. Keep, demote, and rename to **"Platform claims"** or **"Highlights"** in the header. The label "Did you know" reads as community-wiki, not institutional.
- **Recently Updated: KEEP, demote.** Engineers and auditors both use it.
- **Reference Invariants: PROMOTE above the fold.** This panel — three plain-English structural claims — is the *single most institutional* editorial element on the page. It anchors the lede.

**projects.woodfinegroup.com (customer — internal teams + auditors).**

- **Featured Article: ADAPT or REMOVE.** The Woodfine projects instance is project-records, not editorial-rotation content. Featured Article should either become "Featured project" (one project record pinned per quarter) or be removed entirely.
- **Did You Know: REMOVE.** Projects content is not amenable to the punchy claims register format. DYK on a project-records wiki reads as noise. The data file (`leapfrog-facts.yaml`) is vendor-specific anyway — there is no Woodfine equivalent corpus.
- **Recently Updated: KEEP.** Project teams use it as a status indicator.
- **Reference Invariants: ADAPT or REMOVE.** If Woodfine has a small set of structural claims appropriate to the projects context (e.g. "All co-location projects carry continuous-disclosure-grade records"), populate it. Otherwise remove.

**corporate.woodfinegroup.com (customer — auditors + regulators + governance).**

- **Featured Article: REMOVE.** Corporate disclosure content does not rotate editorially.
- **Did You Know: REMOVE.** Inappropriate register for a continuous-disclosure surface.
- **Recently Updated: KEEP.** Critical — disclosure surfaces are read for what is *new*.
- **Reference Invariants: PROMOTE above the fold, and treat as the **disclosure-claims register**.** Each claim links to a published disclosure. This is the most-defensible institutional editorial slot on a corporate wiki.

### 9.2 Engine implementation note

The current engine renders the editorial grid as a single four-box block (`server.rs` 1217–1307) and gates each box by data presence. The per-instance recommendations above are achievable today by **populating or absenting the YAML data files** (`featured-topic.yaml`, `leapfrog-facts.yaml`, `reference-invariants.yaml`) per instance. No engine changes are required to remove DYK from the projects/corporate instances — simply do not ship `leapfrog-facts.yaml` in those content repos.

The only engine change needed is to allow per-instance **promotion of Reference Invariants out of the four-box grid into an above-the-fold slot** — i.e. render it in a hero band above the grid when a `home_layout_v2: true` flag is set in the instance config. This is a small `home_chrome` refactor.

---

## 10. Priority order — top 5 IA changes for A-minus

Ranked by reader-impact-per-engineering-hour.

**1. Expand the category grid (R-C1, R-E2).** Change "Browse by area" from category-titles-only to category-titles-plus-top-5-article-titles. Highest single-change ROI — converts the home page from a *legend* to a *map*. Engine work: `home_chrome` queries `buckets` for top-N by inbound-link-count (already in redb graph); render under each category card. **Reader benefit: all three audiences.**

**2. Elevate cross-wiki navigation to a persistent strip (R-X1, R-X4).** The cross-wiki story is the leapfrog claim made visible. Putting it in the persistent header — not the page foot — fixes the procurement-evaluator first-impression failure and unlocks an article-page affordance ("read the same topic on the projects wiki") that does not exist today. Engine work: new `cross_wiki_strip()` function called from both `home_chrome` and `wiki_chrome`. **Reader benefit: procurement evaluator (primary), all audiences (secondary).**

**3. Hoist the research trail into the right rail (R-R1, R-M3).** Auditor's primary instrument moves from a collapsed foot-of-page disclosure to a persistent right-rail column. The right rail today carries low-value page-tools; the research trail belongs there. Engine work: refactor `wiki_chrome` right-rail block (`server.rs` 2604–2630); move research-trail render block (2536–2563) into right rail; collapse the Tools portlet into a `[more tools]` accordion. **Reader benefit: auditor (primary), engineer (secondary — `What links here` becomes more prominent).**

**4. Replace the Phase-7 placeholder band with a useful provenance band (R-M1).** Article masthead currently reads "Verification not yet available — Phase 7" — a permanent broken-feeling slot. Replace immediately with `last_edited` + citation count + editor. Engine work: ~30 lines in `wiki_chrome` lines 2470–2485. **Reader benefit: all three audiences; auditor most.**

**5. Promote the lede + reference invariants above the fold; demote the editorial grid (R-P1, R-P2).** The first 900 vertical pixels of the home page should answer "what is this, what is in it, where else do I go?" — not show a community-wiki magazine cover. Engine work: refactor `home_chrome` 1181–1307 — promote `home_html` (lede) and `reference_invariants` panel above the four-box grid; demote Featured + DYK + Recently Updated below the category grid. Add `home_layout_v2: true` config flag to gate. **Reader benefit: procurement evaluator (primary), all audiences (secondary).**

A platform that ships these five changes moves from a competent Wikipedia-pattern rebuild (the current C-grade — chrome is correct, IA is conventional but unconsidered) to an institutional knowledge surface that visibly serves its three named audiences (the A-minus target). Phase 7 verification, the claim-native data model, and the contribution-model rebuild remain the leapfrog claims; the IA fixes above are the prerequisite to those landing on a surface the readers can actually use.

---

*Authored 2026-05-23 totebox@project-knowledge. Routes to project-design as DESIGN-RESEARCH; downstream of `wikipedia-leapfrog-2030.draft.md` (substrate research) and upstream of any DESIGN-COMPONENT recipes implementing the prescriptions in §5, §7, §8. No Master co-sign required for DESIGN-RESEARCH.*
