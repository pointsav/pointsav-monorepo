# Woodfine marketing site — congruence plan (Graphic Designer brief)

---

# 📋 End-of-session record — 2026-05-09 (pick up tomorrow from here)

> Operator wrapped this session 2026-05-09 with the v2 wireframes
> shipped and awaiting visual review. This record carries everything
> a fresh session needs to resume cold without re-reading the body
> of the plan below.

## Where we are

- **v1 wireframes** shipped + reviewed.
  - `~/sandbox/outputs/wireframe-home-header.html`
  - `~/sandbox/outputs/wireframe-corporate-header.html`
- **v2 wireframes** shipped (6 files) — three trademark-placement
  variants per property; awaiting operator visual review.
  - `~/sandbox/outputs/wireframe-home-header-v2a.html` — trademark inside paper-3 footer band
  - `~/sandbox/outputs/wireframe-home-header-v2b.html` — trademark on paper-2 between band and copyright
  - `~/sandbox/outputs/wireframe-home-header-v2c.html` — trademark below copyright as fine print (9px)
  - `~/sandbox/outputs/wireframe-corporate-header-v2a.html` — same trademark as home v2a + corporate extensions
  - `~/sandbox/outputs/wireframe-corporate-header-v2b.html` — same trademark as home v2b + corporate extensions
  - `~/sandbox/outputs/wireframe-corporate-header-v2c.html` — same trademark as home v2c + corporate extensions
- **Plan published to cluster** at
  `clones/project-marketing/docs/header-footer-decomposition.md`
  (last sync: after v2 wireframes; will sync once more on session
  exit so tomorrow's session opens to the canonical state).
- **Existing pointsav-design-system artefacts inventoried** —
  see "Research findings" section below.
- **Industry-pattern research done** — GOV.UK Frontend pattern is
  the highest-leverage precedent; Wikimedia multi-skin precedent
  validates the wiki integration approach (modify the wiki's skin
  code to consume tokens; don't wrap or iframe).

## What's gated on operator review (the next session's first action)

1. **Pick a trademark variant** — v2a (inside footer band), v2b
   (between band and copyright on paper-2), or v2c (below copyright
   as fine print)?
2. **Wiki toolbar fidelity** — currently option 1 (light sketch with
   dashed border, paper-2, italic ds-ink-3). Other options recorded
   below for future iteration:
   - Option 2: production-fidelity (solid hairlines, real-looking tabs)
   - Option 3: annotation-only (single labelled empty strip)
3. **Home utility row** — currently option 1 (keep with `EN ▾` only,
   right-aligned). Other options recorded below:
   - Option 2: remove the entire utility row from home (header → 2 rows)
   - Option 3: move `EN ▾` into the brand or nav row

## What's next (after operator picks a variant)

1. Prune the two non-chosen variants (or retain all 6 as
   historical reference — operator's call).
2. **Begin full catalog authoring on auto** — ~49 drafts total:
   - 11 `DESIGN-TOKEN-*` drafts (paper / ink / rule / brand / typography family / typography scale / typography weight / letterspacing / breakpoint / spacing / motion)
   - 3 `DESIGN-LAYOUT-*` drafts (page-container / topnav-grid / footer-grid)
   - 10 `DESIGN-COMPONENT-*` drafts (wordmark / nav-list / nav-link / active-page-chip / external-tab-link / skip-link / language-switcher / cities-list / footnav / copyright-line)
   - 2 `DESIGN-PATTERN-*` drafts (shell-header / shell-footer)
   - 5 `DESIGN-RESEARCH-*` drafts (responsive-strategy / i18n-strategy / accessibility-targets / neurodivergence-targets / multibrowser-fallbacks)
   - ~7 wiki-extension `DESIGN-*` drafts (breadcrumb / Article-Talk tab-pair / Read-Edit-View-History action-tabs / TOC collapsible rail / language-switcher button / density-toggle / hatnote)
   - 11 `ASSET-WOODFINE-*` drafts (wordmark light / wordmark dark / favicon set / nav-text EN / nav-text ES / cities-text / footnav-text EN / footnav-text ES / copyright-text / route-map / theme-instantiation)
3. Update `INDEX.md`, `ia-component-map.md`, `website-congruence-plan.md`
   to reflect the new catalog structure.
4. **Modify `app-mediakit-knowledge/src/server.rs`** chrome
   functions (around lines 766 home_chrome and 1281 wiki_chrome) to
   consume the catalog's tokens — this puts the marketing-shell
   topnav + footer on `corporate.woodfinegroup.com` and
   `projects.woodfinegroup.com` (same binary, both subdomains).
5. **Rebuild `home.woodfinegroup.com`** against the catalog so the
   landing page wears the same chrome.
6. Resume Phase 4-8 (Newsroom server, L.I./BIM tool shells,
   Spanish translations) — parked at the bottom of this plan.

## Critical files (paths a fresh session needs)

| Path | Role |
|---|---|
| `/home/jennifer/.claude/plans/tender-percolating-raccoon.md` | This plan file (ephemeral plan-mode) |
| `/srv/foundry/clones/project-marketing/docs/header-footer-decomposition.md` | Canonical cluster-side plan; sync target on session exit |
| `/srv/foundry/clones/project-marketing/docs/website-congruence-plan.md` | Older sprint plan with banner pointing at the new file |
| `/srv/foundry/clones/project-marketing/docs/ia-component-map.md` | IA component map (regenerates against catalog when it ships) |
| `/srv/foundry/clones/project-marketing/build/` | Style Dictionary token-emit pipeline (Phase 3 staging emit) |
| `/srv/foundry/clones/project-marketing/build/marketing-tokens.dtcg.json` | Staging DTCG source-of-truth (29 tokens) |
| `/srv/foundry/clones/project-marketing/templates/tokens.css` | Emitted CSS bundle |
| `/srv/foundry/clones/project-marketing/templates/_shell-header.html` | Phase 1a header template |
| `/srv/foundry/clones/project-marketing/templates/_shell-footer.html` | Phase 1a footer template |
| `/srv/foundry/clones/project-marketing/.claude/drafts-outbound/leapfrog-2030/` | 5 existing drafts from 2026-05-08 commit `22abc8c` plus 3 added 2026-05-09 |
| `/srv/foundry/clones/project-marketing/pointsav-monorepo/app-mediakit-marketing/src/server.rs` | Marketing-site Rust binary (`home.woodfinegroup.com`) |
| `/srv/foundry/clones/project-marketing/pointsav-monorepo/app-mediakit-knowledge/src/server.rs` | Wiki Rust binary (corporate + projects subdomains) — chrome at lines 729/766 home_chrome and 1250/1281 wiki_chrome |
| `/srv/foundry/clones/project-knowledge/content-wiki-corporate/TRADEMARK.md` | Canonical trademark text source |
| `/srv/foundry/clones/project-knowledge/pointsav-design-system/` | Existing design system — token bundle + theme + components + templates already present |
| `~/sandbox/outputs/wireframe-*.html` | All 8 wireframe files (2 v1 + 6 v2) |
| `~/sandbox/inputs/project-marketing/website/*.pdf` | Original wireframes from Ian Kiprono + hand sketches |

## Decisions made in this session (locked-in)

- **Logo-centre header on every property** (operator confirmed 2026-05-09).
- **New tab + visible ↗ glyph for Corporate/Projects/Newsroom** — operator confirmed, then reversed; ↗ glyph removed; `aria-label="… (opens in new tab)"` retained for screen readers.
- **No dark mode anywhere on Woodfine properties** (operator preference, recorded).
- **Catalog naming convention**: `--ds-*` for DESIGN-* (generic), `--wf-*` for ASSET-WOODFINE-*, `--ps-*` for ASSET-POINTSAV-*. Three-tier ladder: primitive → semantic → component → brand.
- **Distribution architecture**: monorepo with packages (ds-tokens, ds-components, asset-woodfine, asset-pointsav); Style Dictionary v4 emitter; build-time Tera/Nunjucks partials with Web Component fallback for runtime contexts.
- **Wiki integration: Option B (Replace + Reabsorb)** — modify the wiki binary's chrome function to consume the marketing tokens; don't overlay or iframe. Wikimedia multi-skin pattern is the precedent.
- **Wiki properties get extensions, not divergence**: corporate/projects share the marketing chrome verbatim and *add* utility-row Sign in + nav-row search slot + wiki-toolbar row. Home omits all three.
- **Trademark text** is the condensed line from `content-wiki-corporate/TRADEMARK.md`.
- **Home page does not have search or Sign in** — login + search are wiki-only conventions.

## Open decisions (parked for tomorrow)

- Trademark placement: v2a / v2b / v2c (the immediate next gate)
- Wiki toolbar fidelity (current: light sketch)
- Home utility row treatment (current: keep with EN ▾)
- Spanish translation timing
- Newsroom server (Phase 6) — unblock before or after corporate is themed?
- L.I. + BIM tool shells (Phase 7) — light utility shell, no dark mode, deferred

## Quick rsync command for tomorrow's review

```
rsync -avz <vm>:/home/jennifer/sandbox/outputs/wireframe-*-v2*.html ./
```

Open all 6 v2 files in browser tabs; compare trademark variants
side-by-side at desktop / 1024 / 768 / 480 px.

## Action queued for session exit (single command)

**Merge plan-mode file → cluster markdown** so we have **one
canonical markdown for this work**. Operator confirmed 2026-05-09
that the cluster file at
`/srv/foundry/clones/project-marketing/docs/header-footer-decomposition.md`
is the single source of truth; the plan-mode file is per-session
scratch.

```
cp /home/jennifer/.claude/plans/tender-percolating-raccoon.md \
   /srv/foundry/clones/project-marketing/docs/header-footer-decomposition.md
```

That's the only state change required to wrap.

## Convention going forward (record for tomorrow's session)

- **Canonical:** `/srv/foundry/clones/project-marketing/docs/header-footer-decomposition.md`
- **Per-session scratch:** `/home/jennifer/.claude/plans/tender-percolating-raccoon.md`
  (plan-mode only; recreated at each plan-mode entry; do not treat
  as durable storage)
- **At plan-mode entry:** read the cluster file first; either edit
  in place (after exit) or copy into the plan-mode file as the
  starting state.
- **At plan-mode exit:** overwrite the cluster file with the
  plan-mode file. One sync, one source of truth.
- **No drift between the two files** — if the cluster file is ever
  ahead of the plan-mode file, treat the cluster file as
  authoritative and pull it in.

This convention applies only to this single document
(`header-footer-decomposition.md`). Other ephemeral plan-mode files
in `/home/jennifer/.claude/plans/` follow the standard plan-mode
lifecycle.

---

> **2026-05-09 PIVOT (this session, late):** Operator re-scoped.
> Replace the page-by-page approach with a **top-down catalog**:
> decompose `home.woodfinegroup.com`'s header and footer into a
> granular set of `DESIGN-*` (global, generic) and `ASSET-*` (local,
> Woodfine-branded) drafts. Each draft must carry full responsive
> (mobile/tablet/multi-screen), multi-browser, multi-lingual, and
> accessibility-plus-neurodivergence specs so the same chrome can
> be rebuilt deterministically on `corporate.woodfinegroup.com`,
> `projects.woodfinegroup.com`, and even on `home.woodfinegroup.com`
> itself once the catalog is in place.
>
> **The header/footer decomposition catalog is now the active work.**
> The remainder of this plan (the original Phase 0–8 sprint shape) is
> parked in the "Background" section near the bottom; it resumes
> once the catalog ships and we have a deterministic rebuild substrate
> for chrome.
>
> **Visual fidelity is preserved** — the catalog captures what already
> ships on `home.woodfinegroup.com` (and on `disclaimer.html` /
> `contact.html`), encoded as a system. No graphic redesign in this
> sprint.

---

## Active focus — Header + Footer Decomposition Catalog

> **2026-05-09 (latest operator direction):** before authoring the
> full 39-draft catalog, **deliver a "very light" proof of concept**:
> a single standalone HTML file in `~/sandbox/outputs/` showing the
> Woodfine logo-centre topnav + cities footer wrapping a stub
> wiki body (sidebar + ToC + main + appearance panel). Operator
> reviews the sample first; the full catalog ships only after the
> visual approach is approved. This avoids "committing a large
> amount of tokens to this project" before the integration shape is
> validated.
>
> **Wiki-integration research (Explore agent, this session):**
> resolved the question of whether the wiki properties need their
> own `DESIGN-*` family.
>
> - `corporate.woodfinegroup.com` and `projects.woodfinegroup.com`
>   are served by the **same Rust binary** (`app-mediakit-knowledge`)
>   with different `WIKI_SITE_TITLE` + content-dir env vars. One
>   template change covers both subdomains.
> - The wiki chrome is structurally orthogonal to Woodfine's
>   marketing chrome. Wiki classes are `.wiki-*` BEM
>   (`.wiki-toc`, `.wiki-page-tabs`, `.wiki-breadcrumb`, etc.); the
>   Woodfine shell uses `--paper-2`, `.topnav`, `.footer`, etc.
>   They don't collide.
> - **Recommended composition: Option B (Replace + Reabsorb).** The
>   Woodfine topnav *replaces* the wiki's `<header class="site-header">`
>   block (server.rs lines 1281–1289 in `home_chrome()` and the
>   sibling chrome function). The wiki search form relocates *into*
>   the Woodfine topnav as a slot. Everything else — sidebar, ToC,
>   article tabs (Article/Talk/Read/Edit/View History), breadcrumb,
>   appearance panel, hatnote, backlinks, categories, license,
>   last-edited — stays in the body unchanged. Wikipedia muscle
>   memory survives in full; single nav bar (not stacked).
> - **Catalog impact:** **no new `DESIGN-*` families needed for the
>   basic integration.** The existing 28 marketing-shell `DESIGN-*`
>   drafts compose unchanged on the wiki properties. Two optional
>   future polish items if we want them later: `DESIGN-COMPONENT-wiki-integrated-search`
>   (search-input styled to fit the topnav grid) and
>   `DESIGN-PATTERN-wiki-toc-drawer-adapter` (mobile drawer for the
>   left ToC). Neither is required for the proof of concept.
> - **`ASSET-*` impact:** zero. Same wordmark, same nav strings,
>   same routes, same copyright. The Woodfine branding is identical
>   across all four properties.
>
> **Operator's hypothesis confirmed in part:** the operator
> guessed "maybe another set of DESIGN-* but not another set of
> ASSET-*". The research narrows that to: **no new families
> needed at all** for the base integration; two optional polish
> drafts can be deferred to a follow-up sprint.

### Web-research findings — multi-domain design-system architecture

> Full agent report at
> `/home/jennifer/.claude/plans/tender-percolating-raccoon-agent-aaeead2dae60d3017.md`.
> Lead recommendations summarised here so plan reviewers don't have
> to leave this file.

**Industry pattern (converged across IBM Carbon, Shopify Polaris,
Atlassian, Material Design, Salesforce SLDS, GitHub Primer, USWDS,
Microsoft Fluent, GOV.UK Design System, Wikimedia Codex):**

```
monorepo
├── packages/ds-tokens       ← DESIGN-TOKEN-* (DTCG JSON + emitted CSS/SCSS/JS)
├── packages/ds-components   ← DESIGN-COMPONENT-* (HTML partials + scoped CSS)
├── packages/asset-woodfine  ← ASSET-WOODFINE-* (SVGs, EN/ES strings, woodfine theme)
└── packages/asset-pointsav  ← ASSET-POINTSAV-* (parallel for the PointSav brand)
        │
        │ (each property's build pulls the packages it needs;
        │  Style Dictionary v4 emits CSS/SCSS/JSON/DTCG per package)
        ▼
  property build pipeline (Rust / npm / static-site)
        │
        ▼
  rendered HTML at runtime — chrome composed from package partials
```

**Concrete recommendations for our catalog:**

1. **Naming convention (three-tier ladder, Material-3 prefix discipline).**
   - `DESIGN-*` tokens emit as `--ds-*` (e.g. `--ds-paper-2`, `--ds-ink-3`, `--ds-brand-primary-ref`). These are abstract, brand-agnostic.
   - `ASSET-WOODFINE-*` tokens emit as `--wf-*` (e.g. `--wf-blue` resolves to `#164679`).
   - `ASSET-POINTSAV-*` tokens emit as `--ps-*` (e.g. `--ps-slate` resolves to `#869FB9`).
   - The brand layer aliases the abstract: `--wf-blue: var(--ds-brand-primary-ref);` with the woodfine theme then setting `--ds-brand-primary-ref: #164679`.
   - This is the universal three-tier ladder: **primitive → semantic → component → brand**.

2. **Distribution: build-time partials + npm packages, Web Component fallback for runtime cases.**
   - Each Rust binary (app-mediakit-marketing, app-mediakit-knowledge) imports the same Tera macros from `packages/ds-components` at build time.
   - For runtime inclusion (e.g. third-party embed, future RSS-reader injection), ship `<ds-header>` / `<ds-footer>` web components as a sibling bundle.
   - Avoid: Module Federation (wrong across separate subdomains), iframes (break muscle memory, deep linking, scroll, a11y tree).

3. **Wiki integration — Wikimedia multi-skin precedent confirmed.**
   - Wikimedia ships 45+ MediaWiki skins (Vector-2022, MonoBook, Minerva, Timeless, …) all consuming the same Codex tokens via `mediawiki.skin.variables.less`. The wiki engine renders the article body; the skin owns the chrome.
   - **Apply the same pattern here.** Modify `app-mediakit-knowledge/src/server.rs` chrome functions (the binary's "skin") to consume our `--ds-*` / `--wf-*` tokens. Don't overlay; don't wrap; don't iframe. Option B from the earlier Explore-agent research lines up perfectly with this — replace the wiki's `<header class="site-header">` block with the Woodfine topnav, search bar relocates into the topnav slot.
   - **Catalog impact = nothing new.** The 28 marketing-shell `DESIGN-*` drafts plus the 11 `ASSET-WOODFINE-*` drafts cover the wiki integration. Two optional polish items (`DESIGN-COMPONENT-wiki-integrated-search`, `DESIGN-PATTERN-wiki-toc-drawer-adapter`) are deferred to a follow-up sprint.

4. **Versioning: semver per package, per-property pinning, hash-named CDN paths. No atomic global rollout** — every mature design system treats it as an anti-pattern at scale. Renovate/Dependabot updates per property; changesets for breaking-change governance; stylelint-no-raw-values rule fails CI when a property regresses to inline hex.

5. **DTCG 1.0 + Style Dictionary v4 alignment.** DTCG 1.0 stable shipped 2025-10-28. Style Dictionary v4 has first-class DTCG support (the cluster's existing `build/style-dictionary.config.js` from Phase 3 is on the right track). Multi-brand pattern: one base-tokens directory + per-brand overrides → N output bundles.

6. **Highest-leverage pattern to copy: GOV.UK Frontend** (`alphagov/govuk-frontend`). One npm package; Nunjucks macros for chrome; precompiled CSS/JS as escape hatch; governance-gated header/footer. Their multi-agency, multi-stack, one-identity reality is structurally identical to ours (four properties, multiple backends, one Woodfine identity). The pattern has been in production since 2018; well-tested.

**Updates to the catalog architecture:**

- **No change to draft counts** (28 DESIGN + 11 ASSET).
- **Naming-prefix update:** when authoring drafts in the next sprint, emit CSS variable names with the `--ds-*` / `--wf-*` ladder. The current cluster build (Phase 3) emits `--paper-2`, `--wf-blue`, `--display`, etc.; rename in the next regen to `--ds-paper-2`, `--wf-blue`, `--ds-display` so we land on the industry-standard prefix discipline.
- **Distribution path:** the cluster's `build/` directory becomes the proto-monorepo. When the catalog ships, fork it into `packages/ds-tokens` + `packages/ds-components` + `packages/asset-woodfine` (each its own `package.json`). Style Dictionary v4 stays as the emitter.
- **Wiki integration:** server.rs chrome-function rewrite consumes the emitted CSS, no wrapping/embedding. Same approach as the marketing pages.
- **Web Component fallback:** ship `<ds-header>` + `<ds-footer>` as a sibling bundle from `ds-components` for any runtime-include context (Newsroom RSS reader, future third-party embeds, JS-only rendering surfaces). Optional — not in the light POC.

### Wireframe v2 — operator iteration 2026-05-09 (late)

> **Three-variant deliverable (operator direction).** Rather than
> picking one trademark-placement option, ship three versions of
> each wireframe so the operator can compare side-by-side. The
> other two AskUserQuestion answers (wiki-toolbar fidelity, home
> utility row) settle at option 1 (light sketch / keep `EN ▾`)
> across all three variants — alternates recorded below for
> potential future iteration.
>
> **Six files to write to `~/sandbox/outputs/`:**
>
> | File | Trademark placement |
> |---|---|
> | `wireframe-home-header-v2a.html` | inside paper-3 footer band, below cities + footnav |
> | `wireframe-home-header-v2b.html` | between footer band and copyright, on paper-2 canvas |
> | `wireframe-home-header-v2c.html` | below copyright, smallest text on canvas |
> | `wireframe-corporate-header-v2a.html` | (same as home v2a) |
> | `wireframe-corporate-header-v2b.html` | (same as home v2b) |
> | `wireframe-corporate-header-v2c.html` | (same as home v2c) |
>
> v1 files (`wireframe-{home,corporate}-header.html`) stay in place
> as historical reference per operator memory ("HTML tool versioning
> — create new file, never overwrite").
>
> **Alternates parked for future iteration (not in v2):**
>
> - Wiki-toolbar fidelity options not chosen for v2:
>   - Option 2 — production-fidelity sketch (solid hairlines, real-looking tabs).
>   - Option 3 — annotation-only (single labelled empty strip, no sketch content).
>   v2 ships option 1 (light sketch with dashed border, paper-2,
>   ds-ink-3 text).
> - Home utility-row options not chosen for v2:
>   - Option 2 — remove the entire utility row from home (header becomes two rows).
>   - Option 3 — move `EN ▾` into the brand or nav row (keep two rows).
>   v2 ships option 1 (keep the row with `EN ▾` only, right-aligned).
>
> If a future review wants either of those alternates, they're
> single-row CSS edits in the existing wireframe — no architectural
> rework.

> Operator review of v1 wireframes
> (`~/sandbox/outputs/wireframe-{home,corporate}-header.html`)
> surfaced four corrections:
>
> 1. **Drop search bar from the home page.** Home is a marketing
>    landing — not a wiki — so the search slot belongs only on
>    corporate (and projects, also a wiki). Search returns to where
>    Wikipedia muscle memory expects it: only on the wiki properties.
> 2. **Drop "Sign in" from the home page.** Same reasoning — login
>    is a wiki convention. Home page has nothing to sign into.
>    Utility row on home keeps only the language toggle (`EN ▾`).
> 3. **Add a trademark disclaimer to both.** Single condensed line
>    in the footer, under the cities band but above the copyright,
>    listing the registered marks.
> 4. **Sketch the wiki toolbar slot UNDER the Woodfine three-row
>    header on the corporate wireframe.** A faint placeholder strip
>    showing breadcrumb + Article/Talk tabs + Read/Edit/View History
>    so the operator can see where Wikipedia's article chrome lands
>    relative to the new shared header. This is sketch-fidelity, not
>    production fidelity — just enough to show the transition.
>
> **Architectural shift recorded:** the v1 plan assumed home would
> *adapt to* wiki muscle memory by carrying search + sign-in. v2
> reverses that — home stays a clean marketing landing; corporate
> *extends* the shared base header with sign-in (utility row) +
> search (nav row right) + wiki toolbar (new row 4). The shared
> base is now: brand row + nav row's 5 marketing links + footer +
> trademark line + copyright. Corporate adds three extensions on
> top of that base. Home does not.

#### Research findings (Explore agent, 2026-05-09)

**Trademark disclaimer text — canonical source.**

`/srv/foundry/clones/project-knowledge/content-wiki-corporate/TRADEMARK.md`
lines 1–11 carry the full notice. Condensed footer line for the
wireframes:

> Woodfine Capital Projects™, Woodfine Management Corp™,
> PointSav Digital Systems™, Totebox Orchestration™, and
> Totebox Archive™ are trademarks of Woodfine Capital Projects
> Inc. used in Canada, the United States, Latin America, and
> Europe. All other trademarks are the property of their
> respective owners.

This appears as a small `.trademark` row in the footer between the
cities/footnav band and the copyright line.

**Wiki toolbar elements (DOM order, per `wiki_chrome()` in
`pointsav-monorepo/app-mediakit-knowledge/src/server.rs`):**

| Element | Class | Line | Wikipedia muscle memory |
|---|---|---|---|
| Breadcrumb nav | `.wiki-breadcrumb` | 1321 | **Yes** — "Documentation › Category › Title" |
| Article/Talk tabs | `.wiki-page-tabs` | 1333 | **Yes** — Wikipedia primary tab pair |
| Page title H1 | `.page-title` | 1348 | (article level, below toolbar) |
| Language switcher | `.wiki-lang-switcher` | 1351 | **Yes** — globalisation signal |
| Read/Edit/View History | `.wiki-action-tabs` | 1382 | **Yes** — top-right action bar |
| IVC band + density toggle | `.wiki-ivc-band` / `.wiki-density-toggle` | 1397/1404 | Phase 7 placeholder (sketch-only) |
| Hatnote | `.wiki-hatnote` | 1424 | (article level, below toolbar) |

For the v2 wireframe, the **wiki-toolbar row** combines breadcrumb
+ Article/Talk + Read/Edit/View History into a single sketch strip.
That's the visual transition between Woodfine chrome and Wikipedia
content.

**Existing `pointsav-design-system` artefacts (adapt vs author).**

| Already present (adapt) | Missing (must author) |
|---|---|
| `tokens/dtcg-bundle.json` (three-tier DTCG) | breadcrumb component |
| `theme-woodfine.css`, `theme-generic.css` | Article/Talk tab-pair component |
| Components: `home-grid`, `citation-authority-ribbon`, `research-trail-footer`, `freshness-ribbon` | Read/Edit/View-History action-tabs component |
| `components/typography.css`, `layout.css`, `interactive.css`, `controls.css` | TOC collapsible rail component |
| Templates: `pointsav-index-scaffold.html`, `woodfine-index-scaffold.html` | language-switcher button component |
| | density toggle component |
| | hatnote styling |

The catalog can **adapt** the existing tokens + theme + typography
+ home-grid scaffold; it must **author** breadcrumb / tab-pair /
action-tabs / TOC / language-switcher / density-toggle / hatnote
components from scratch. These additions slot into the wiki
properties' DESIGN-* family without affecting the marketing-only
properties.

**corporate.woodfinegroup.com — current rendered shape (from
`home_chrome()` server.rs:729–950 + `content-wiki-corporate/index.md`):**

- **Top:** site header (logo + search form + Home link) — replaced
  by the Woodfine three-row header in v2.
- **Welcome panel:** corporate lede ("Woodfine Management Corp.
  structures Direct-Hold Solutions…").
- **Stats banner:** N articles across N categories.
- **Middle:** featured + did-you-know panels (2-col), 9-category
  grid (Architecture / Services / Systems / Applications /
  Governance / Infrastructure / Company / Reference / Help) with
  3–5 articles preview per category, "Operational Guides" section.
- **Lower:** platform telemetry placeholder (Phase 10), all-articles
  catch-all, Recent additions (top 5 by last_edited).
- **Footer:** site footer with engine attribution; **no copyright
  or trademark line** (those are the marketing site's responsibility).

**Article shell:** left sticky TOC rail (with [hide] toggle), then
the wiki toolbar (breadcrumb / tabs / action tabs / language
switcher / IVC band / density toggle), then page title H1, then
hatnote, then article body, then backlinks, then categories +
last-edited + license + footer links.

#### v2 wireframe deltas — what changes vs v1

**`wireframe-home-header.html`:**

- Utility row: drop **"Sign in"**, drop the `·` separator. Row now
  holds only `EN ▾` right-aligned.
- Nav row: drop the **search slot** (the entire `.search-slot`
  form). Row right-side ends at the Newsroom link.
- Footer: add **`.trademark`** row between `.shell-footer` and
  `.copyright` with the condensed trademark line.
- Body slot annotation stays: "[ page content ]".

**`wireframe-corporate-header.html`:**

- Utility row: keep `EN ▾ · Sign in` (Sign in is wiki muscle
  memory).
- Nav row: keep search slot (`Search corporate wiki`).
- Active-page chip on Corporate: keep.
- **NEW: wiki-toolbar row** sketched between the three-row header
  and the body slot — single horizontal strip with three sketch
  zones:
  - left: `Documentation › Direct-Hold Framework` (breadcrumb)
  - centre: `Article · Talk` (tabs)
  - right: `Read · Edit · View History` (action tabs)
  - styled lightly (paper-2 background, ds-rule-hairline above and
    below, ds-fs-utility text, ds-ink-3) so it reads as a
    placeholder strip rather than production-final UI.
- Footer: add `.trademark` row (same as home).
- Body slot annotation updates to: "[ wiki article body — sidebar /
  ToC / page title / appearance panel render here ]".

#### Token additions for v2

No new `--ds-*` or `--wf-*` tokens beyond v1. The wiki-toolbar uses
existing `--ds-paper-2`, `--ds-rule-hairline`, `--ds-ink-3`,
`--ds-display`, `--ds-fs-utility`, `--ds-ls-medium`, `--ds-sp-*`.
The trademark row uses `--ds-fs-copyright`, `--ds-ink-4`,
`--ds-paper-3`, `--ds-sans`.

#### Catalog impact (forward-looking)

When the full catalog authors after the v2 wireframes are approved:

- The 28 base `DESIGN-*` drafts cover home + corporate's shared
  chrome (header + footer + trademark).
- A **wiki-extension family** of `DESIGN-*` drafts (~7 items) covers
  the corporate-only additions: breadcrumb, tab-pair, action-tabs,
  TOC, language-switcher, density-toggle, hatnote. These compose
  on top of the base; they don't replace anything.
- The 11 `ASSET-WOODFINE-*` drafts stay the same — the wiki
  extensions are generic (DESIGN-*), not brand-specific.
- Existing pointsav-design-system components (`home-grid`,
  `citation-authority-ribbon`, `research-trail-footer`,
  `freshness-ribbon`) are reused as-is; they don't conflict with
  the new chrome.

Total catalog count: 28 base DESIGN + ~7 wiki-extension DESIGN +
11 ASSET-WOODFINE ≈ 46 drafts. Up from 39, but the wiki extensions
are smaller drafts (just component recipes, no token primitives).

---

### Light proof-of-concept (revised 2026-05-09 latest direction)

> **Constraint inversion (operator's design move):** instead of
> trying to cram wiki muscle-memory features (search, login,
> language toggle) into a marketing-only header on
> corporate.woodfinegroup.com, **adapt `home.woodfinegroup.com` to
> already carry those features at a header level** — so the same
> header travels unchanged across all four properties (home,
> corporate, projects, newsroom).
>
> Wikipedia's Vector-2022 skin established the canonical pattern: a
> three-row header (utility / brand / nav) with search inline in
> the nav row. Adopting that pattern at home means the four-property
> environment functions holistically — same header, same buttons,
> same muscle memory — without losing graphic identity. Visual
> design stays Woodfine (logo-centre, paper-2 canvas, `--wf-blue`
> accent); the structural shape adapts to the wiki convention.

**Deliverable: two standalone HTML wireframe samples** in the
sandbox so the operator can review side-by-side before any catalog
drafts are authored.

| File | Renders | Active-page state |
|---|---|---|
| `~/sandbox/outputs/wireframe-home-header.html` | `home.woodfinegroup.com` header + footer | none (root URL, no link is active) |
| `~/sandbox/outputs/wireframe-corporate-header.html` | `corporate.woodfinegroup.com` header + footer | active-page chip on **Corporate** |

Both files: **header AND footer**, no body content (no sidebar, no
article body, no appearance panel). Pure chrome wireframe to
validate the three-row header + cities-footer layout + the
`--ds-*` / `--wf-*` naming ladder + the responsive collapse
behaviour before any drafts go on auto.

Footer included per operator confirmation 2026-05-09 (latest):
*Vancouver \| New York* cities band + Contact us / Disclaimer
footnav + copyright line. Same `<footer>` on both samples — only
the header's active-page chip differs.

**Publication step alongside the wireframes** — save the plan
content to a cluster markdown so it survives session boundaries:

| File | Role |
|---|---|
| `clones/project-marketing/docs/header-footer-decomposition.md` | new — carries the full catalog architecture + research findings + constraint-inversion rationale + two-sample wireframe spec |
| `clones/project-marketing/docs/website-congruence-plan.md` | existing — gets a banner at the top pointing at the new file ("PIVOTED — see header-footer-decomposition.md") |

#### Three-row header pattern (shared by both samples)

```
+============================================================+
| [utility row: ds-paper-3 background, ds-fs-utility text]   |
|                          EN ▾  ·  Sign in                  |
+============================================================+
| [brand row: ds-paper-2 background]                         |
|                                                            |
|                  [WOODFINE WORDMARK SVG]                   |
|                                                            |
+============================================================+
| [nav row: ds-paper-2 background, ds-rule-hairline above]   |
|  DISCLAIMER  CONTACT US      |      CORPORATE  PROJECTS  NEWSROOM   🔍 [search]  |
+============================================================+
```

- **Utility row** (paper-3, ds-fs-utility 10px wide-tracked uppercase): language toggle + sign-in slot. Right-aligned at desktop; centred at mobile. This is the wiki muscle-memory "utility bar".
- **Brand row** (paper-2): Woodfine wordmark only, centred. ds-display font, 320×80 px at desktop, scales 240/200/160 down through breakpoints.
- **Nav row** (paper-2 with ds-rule-hairline above): the six-link marketing nav (logo is implicit via the brand row, so the count is 5 visible links: Disclaimer + Contact us on the left; Corporate + Projects + Newsroom on the right) plus an inline search icon → expanding search input on the far right. ds-display font, 11px wide-tracked uppercase. Active-page chip on the link matching the current property (none on home; "Corporate" filled `--wf-blue` chip on corporate).

#### Why three rows (not the current two-with-flanking-links)

The current `disclaimer.html` topnav is a single grid row: `1fr / auto / 1fr` with left links / wordmark / right links. There's no slot for search, login, or language toggle. To preserve the wikipedia muscle memory at corporate without a second header bar, we need a row for utility (search etc.) ABOVE or BELOW the brand row.

Three-row puts utility above brand (Wikipedia Vector-2022 convention) and nav below brand. This:
- Reads as a single composite header (one cohesive `<header>` element)
- Scales gracefully — at tablet, utility row condenses to icons; at mobile, nav row goes hamburger
- Preserves logo-centre as the visual anchor (operator's earlier confirmed preference)
- Adopts the highest-leverage Wikipedia pattern verbatim

The marketing pages (Disclaimer, Contact, Newsroom) inherit the same header. Disclaimer and Contact get an active-page chip on their corresponding link in the nav row; Newsroom gets a chip on Newsroom.

#### Token list consumed by both samples (preview)

`DESIGN-*` tokens (`--ds-*`):

| Token | Value | Usage |
|---|---|---|
| `--ds-paper-1` | #FFFFFF | (not used in header — body card) |
| `--ds-paper-2` | #F7F9FA | Brand row + nav row background |
| `--ds-paper-3` | #E6E7E8 | Utility row background |
| `--ds-ink-1` | #111827 | Wordmark institutional fill, active-link text shadow |
| `--ds-ink-2` | #374151 | (reserved for body content) |
| `--ds-ink-3` | #6B7280 | Default nav-link text colour |
| `--ds-ink-4` | #9CA3AF | Utility row text |
| `--ds-rule-hairline` | #E6E7E8 | Hairline above and below nav row |
| `--ds-display` | "Oswald", … | Nav-link + utility-row text |
| `--ds-fs-nav` | 11px | Nav-link font size |
| `--ds-fs-utility` | 10px | Utility-row font size |
| `--ds-fw-medium` | 500 | Nav-link weight |
| `--ds-ls-wide` | 0.16em | Nav-link letter-spacing |
| `--ds-ls-extra-wide` | 0.18em | Utility-row letter-spacing |
| `--ds-bp-tablet` | 768px | Tablet collapse breakpoint |
| `--ds-bp-mobile` | 480px | Mobile collapse breakpoint |
| `--ds-sp-1` … `--ds-sp-12` | 4 / 8 / … / 140 px | Padding + gap scale |
| `--ds-motion-fast` | 160ms | Hover transition |

`ASSET-WOODFINE-*` tokens (`--wf-*`):

| Token | Value | Usage |
|---|---|---|
| `--wf-blue` | #164679 | Active-page chip background, right-side nav text colour |
| `--wf-blue-on` | #FFFFFF | Active-page chip text colour |
| `--wf-blue-tint` | #E8EFF7 | (reserved for hover surface) |

ASSET-WOODFINE strings (EN, inline):
- "Disclaimer", "Contact us", "Corporate", "Projects", "Newsroom"
- "Sign in" (utility slot)
- "EN" (language toggle current state)

#### Responsive behaviour (both samples)

| Breakpoint | Layout |
|---|---|
| ≥ 1200px (desktop wide) | Three rows; nav row left-aligned + right-aligned with central spacer |
| 1024–1199px | Same; tighter padding |
| 768–1023px (tablet) | Utility row condenses to icons (🌐 + 👤); nav row keeps full text |
| 480–767px | Nav row collapses to a hamburger; nav links go vertical in a flyout |
| < 480px | Same; tighter padding; wordmark scales to 160×40 |

#### What stays in the operator review

After the operator inspects both wireframes via rsync from
`~/sandbox/outputs/`, two questions to answer before the catalog
ships:

1. Does the three-row pattern render right at every breakpoint?
2. Is the visual identity preserved (Woodfine logo-centre, paper-2 canvas, `--wf-blue` accent)? Or does anything need to shift before the catalog locks the design in?

Once approved, the full 28-DESIGN + 11-ASSET catalog authors against
the validated wireframe. If the wireframe needs iteration, only the
wireframe HTML changes — no draft files committed yet.

---

### (Older "single-file POC" spec — superseded 2026-05-09 by the
### two-sample / header-only / constraint-inversion design above.)

Originally proposed: a single standalone HTML file at
`~/sandbox/outputs/woodfine-chrome-on-wiki.html` showing:

1. The Woodfine logo-centre topnav at the top — Disclaimer + Contact us \| [Woodfine wordmark] \| Corporate (active-chip) + Projects + Newsroom — using the existing `disclaimer.html` visual style verbatim.
2. Below the topnav, a **stub wiki body** that mimics the
   `app-mediakit-knowledge` chrome — left sidebar with collapsible
   ToC, main article body (one short article with H1 + 2 paragraphs +
   a section break + a [edit] pencil), right appearance panel
   (Text/Width/Color), Article/Talk tabs, breadcrumb, density
   toggle. Visually faithful to the wiki style summarised by the
   Explore agent (Georgia serif body, system-stack chrome,
   Wikipedia-blue links, beige aside band, 76em max-width).
3. The Woodfine cities footer at the bottom — *Vancouver \| New York*
   + Contact us / Disclaimer footnav + copyright line.
4. Embedded `<style>` block with the minimal token set inline (no
   external `/tokens.css` dependency — file must be standalone).
5. Active-page chip on "Corporate" in the topnav (since the sample
   is previewing what `corporate.woodfinegroup.com` will look like
   once Option B is implemented).
6. Self-contained: opens correctly via `file://` or any static
   server; no runtime dependencies; no external image/font hosts
   beyond the existing Google Fonts CDN that the marketing pages
   already use.

**Scope guard:** this POC does NOT author any `DESIGN-*` or
`ASSET-*` draft files in the cluster. It only produces a single
standalone HTML for visual review. The full catalog is gated on
operator approval of the POC.

**File placement:** `~/sandbox/outputs/woodfine-chrome-on-wiki.html`
per the established rsync access path (operator's memory
"Sandbox outputs — rsync access path"). Operator pulls via rsync
to a local machine for visual inspection.

### Context

`home.woodfinegroup.com` ships a logo-centre header (Disclaimer + Contact us \| [wordmark] \| Corporate + Projects + Newsroom) and a *Vancouver \| New York* footer with footnav + copyright. This chrome lives on the marketing site but does **not** live on the sister properties (`corporate.woodfinegroup.com` served by `app-mediakit-knowledge`; `projects.woodfinegroup.com` served by another Rust binary). To rebuild the same chrome on those properties without copy-pasting drift-prone CSS, the operator wants a **catalog** of atomic design artefacts.

Two artefact families:

- **`DESIGN-*`** — global, generic, design-system-level abstractions. Token families, layout primitives, component recipes, patterns, cross-cutting research notes. These compose; a single `DESIGN-COMPONENT-nav-link` recipe produces nav links anywhere.
- **`ASSET-*`** — local, Woodfine-branded instantiations of the abstract `DESIGN-*` slots. Concrete wordmark SVG, concrete EN/ES strings, concrete city names, concrete copyright line, the theme CSS that maps abstract tokens to `#164679` blue and the rest.

Every `DESIGN-*` draft must encode (in its frontmatter and recipe body):

1. **Responsive** — desktop / wide / tablet-landscape / tablet-portrait / mobile behaviour at 1440 / 1200 / 1024 / 768 / 480 px.
2. **Multi-browser** — Chrome / Safari / Firefox / Edge support; vendor prefixes where required; `@supports` feature-detection for fallbacks; legacy graceful-degradation notes.
3. **Multi-lingual** — text direction (LTR base; RTL impact noted), max-line-length headroom for long-word languages (German, Spanish), language-attribute markup, character-set safety, language-switcher placement.
4. **Accessibility + neurodivergence** — WCAG 2.2 AA contrast targets, `:focus-visible` styling, `prefers-reduced-motion` respect, keyboard navigation, heading hierarchy, dyslexia-friendly typography alternates, low-distraction mode option, generous click-target sizes, unambiguous labels.

### Catalog — `DESIGN-*` (global, generic) — 28 drafts

#### Tokens — Tier 1 primitives + Tier 2 semantic (10 drafts)

| # | Draft | Captures |
|---|---|---|
| D1 | `DESIGN-TOKEN-color-paper-scale` | `paper` (#FFFFFF), `paper-2` (#F7F9FA), `paper-3` (#E6E7E8) — three-step paper scale used for cards, canvas, and footer/page-hero band |
| D2 | `DESIGN-TOKEN-color-ink-scale` | `ink` (#111827), `ink-2` (#374151), `ink-3` (#6B7280), `ink-4` (#9CA3AF) — body / mid / small / pale ink |
| D3 | `DESIGN-TOKEN-color-rule-scale` | `rule` (#E6E7E8), `rule-strong` (#9CA3AF) — hairline + emphasised divider |
| D4 | `DESIGN-TOKEN-color-brand` | `brand-primary` (#164679), `brand-on-primary` (#FFFFFF), `brand-tint` (#E8EFF7) — institutional blue + on-brand text + tint |
| D5 | `DESIGN-TOKEN-typography-family` | Display (Oswald), serif (Roboto Slab), sans (Nunito Sans) — three Google-Fonts-distributable substitutes for the licensed Trade Gothic / Caecilia / Avenir brand stack, plus four documented alternate pairings |
| D6 | `DESIGN-TOKEN-typography-scale` | Font sizes (`fs-h1` 22px, `fs-hero-kicker` 13px, `fs-body` 14px, `fs-nav` 11px, `fs-footnav` 10px) with `clamp()` responsive shrink rules |
| D7 | `DESIGN-TOKEN-typography-weight` | `regular` 400, `medium` 500, `semibold` 600, `bold` 700 |
| D8 | `DESIGN-TOKEN-typography-letterspacing` | `tight` 0.04em, `medium` 0.10em–0.12em, `wide` 0.16em, `extra-wide` 0.18em–0.22em |
| D9 | `DESIGN-TOKEN-breakpoint` | `wide` 1200, `desktop` 1024, `tablet` 768, `mobile` 480 + `container-max` 1440 — desktop-first scale |
| D10 | `DESIGN-TOKEN-spacing` | 4 / 8 / 12 / 16 / 20 / 24 / 36 / 48 / 56 / 80 / 120 / 140 px scale used across paddings, gaps, and margins |
| D11 | `DESIGN-TOKEN-motion` | `duration-fast` 160ms, easing-standard `ease`, with `prefers-reduced-motion` short-circuit pattern |

#### Layouts — Tier 3 composition primitives (3 drafts)

| # | Draft | Captures |
|---|---|---|
| D12 | `DESIGN-LAYOUT-page-container` | `.page` wrapper — `max-width: 1440px; width: 100%; margin: 0 auto; background: paper-2; padding: 32/24/16/12 px responsive` |
| D13 | `DESIGN-LAYOUT-topnav-grid` | `1fr / auto / 1fr` three-column at desktop/wide; collapses to single-column-stack at tablet (768px) with wordmark on `order: -1` |
| D14 | `DESIGN-LAYOUT-footer-grid` | `1fr / auto` two-column at desktop/wide; single-column-stack at tablet with footnav switching to horizontal-wrap |

#### Components — Tier 3 UI atoms (10 drafts)

| # | Draft | Captures |
|---|---|---|
| D15 | `DESIGN-COMPONENT-wordmark` | Logo container; consumes an `ASSET-*` SVG; size variants 320×80 / 240×60 / 200×50 / 160×40 keyed to breakpoint |
| D16 | `DESIGN-COMPONENT-nav-list` | Horizontal flex container for nav-link children; gap-collapse + alignment shifts at tablet |
| D17 | `DESIGN-COMPONENT-nav-link` | Single link — base / hover / focus-visible / active states; display-font 11px wide-tracked uppercase |
| D18 | `DESIGN-COMPONENT-active-page-chip` | Active-state variant — `brand-primary` fill, `brand-on-primary` text, padded chip |
| D19 | `DESIGN-COMPONENT-external-tab-link` | Off-property variant — `target="_blank"` + `rel="noopener"` + `aria-label="… (opens in new tab)"` (no visual ↗ glyph per operator) |
| D20 | `DESIGN-COMPONENT-skip-link` | a11y skip-to-main-content link; visually-hidden until `:focus` |
| D21 | `DESIGN-COMPONENT-language-switcher` | EN/ES picker; minimal-footprint inline header element; persists choice via `lang` cookie or session |
| D22 | `DESIGN-COMPONENT-cities-list` | Horizontal separator-delimited list of city names (`Vancouver | New York`); serif body font |
| D23 | `DESIGN-COMPONENT-footnav` | Vertical-on-desktop, horizontal-wrap-on-mobile list of footer links; display-font small-caps |
| D24 | `DESIGN-COMPONENT-copyright-line` | Single small-caps copyright + entity name + year template-variable |

#### Patterns — Tier 4 assembled chrome (2 drafts)

| # | Draft | Captures |
|---|---|---|
| D25 | `DESIGN-PATTERN-shell-header` | The full topnav: `topnav-grid` assembling left `nav-list` (Disclaimer + Contact us as `nav-link` + optional `active-page-chip`) + `wordmark` + right `nav-list` (Corporate + Projects + Newsroom as `external-tab-link`). Carries the active-page slot rule. |
| D26 | `DESIGN-PATTERN-shell-footer` | The full footer: `footer-grid` assembling `cities-list` + `footnav`, plus `copyright-line` row beneath |

#### Cross-cutting research (5 drafts)

| # | Draft | Captures |
|---|---|---|
| D27 | `DESIGN-RESEARCH-responsive-strategy` | Desktop-first scale rationale, breakpoint progression, container-max behaviour, fluid-type via `clamp()`, grid-collapse patterns |
| D28 | `DESIGN-RESEARCH-i18n-strategy` | LTR base + RTL future support; max-line-length headroom for German/Spanish; language-attribute markup; character-set safety; locale-aware date/number formatting (footer copyright year) |
| D29 | `DESIGN-RESEARCH-accessibility-targets` | WCAG 2.2 AA contrast ratios per atom; `:focus-visible` style; `prefers-reduced-motion` respect; keyboard nav; heading hierarchy; ARIA-label conventions |
| D30 | `DESIGN-RESEARCH-neurodivergence-targets` | Dyslexia-friendly typography alternates; low-distraction mode toggle; tightened line-spacing options; generous click-target sizes; unambiguous labels; cognitive-load minimisation |
| D31 | `DESIGN-RESEARCH-multibrowser-fallbacks` | Chrome / Safari / Firefox / Edge support matrix; vendor prefixes where required (`-webkit-backdrop-filter`, etc.); `@supports` feature-detection; IE11 / legacy Edge graceful degradation; CSS-grid fallback to flexbox |

### Catalog — `ASSET-*` (local, Woodfine-branded) — 11 drafts

| # | Draft | Captures |
|---|---|---|
| A1 | `ASSET-WOODFINE-wordmark-svg-light` | Institutional-fill (#111827) wordmark for light-paper backgrounds; canonical 144×36 viewBox with the existing 4-path glyph data |
| A2 | `ASSET-WOODFINE-wordmark-svg-dark` | White-fill (#FFFFFF) wordmark for dark-blue (`brand-primary`) backgrounds; same paths, fill swap (or `currentColor` once the parent-style pattern is adopted) |
| A3 | `ASSET-WOODFINE-favicon-set` | Browser-tab favicons derived from the wordmark — 16×16, 32×32, 180×180 (Apple touch), 512×512 (PWA manifest); cropped to a representative glyph or monogram |
| A4 | `ASSET-WOODFINE-nav-text-en` | English nav strings — Disclaimer, Contact us, Corporate, Projects, Newsroom |
| A5 | `ASSET-WOODFINE-nav-text-es` | Spanish nav strings — Aviso legal, Contáctenos, Corporativo, Proyectos, Sala de prensa (final wording subject to project-language Task validation) |
| A6 | `ASSET-WOODFINE-cities-text` | EN: "Vancouver \| New York"; ES: "Vancouver \| Nueva York" |
| A7 | `ASSET-WOODFINE-footnav-text-en` | EN: Contact us / Disclaimer |
| A8 | `ASSET-WOODFINE-footnav-text-es` | ES: Contáctenos / Aviso legal |
| A9 | `ASSET-WOODFINE-copyright-text` | EN/ES variants with year template-variable: "© {{year}} Woodfine Capital Projects Inc. All rights reserved." / "© {{year}} Woodfine Capital Projects Inc. Todos los derechos reservados." |
| A10 | `ASSET-WOODFINE-route-map` | URL map: Disclaimer → `/page/disclaimer`, Contact us → `/page/contact`, Corporate → `https://corporate.woodfinegroup.com/`, Projects → `https://projects.woodfinegroup.com/`, Newsroom → `https://newsroom.woodfinegroup.com/` (Newsroom subdomain pending Phase 6) |
| A11 | `ASSET-WOODFINE-theme-instantiation` | `woodfine-tokens.css` — the file consumed by every Woodfine page that maps each `DESIGN-TOKEN-*` abstract value to its Woodfine concrete value. Replaces the inline `:root { … }` block currently in `disclaimer.html` and `contact.html` |

### TODO list (auto-executable after approval)

Each row is one auto task. Order: tokens first (D1–D11), layouts next (D12–D14), components (D15–D24), patterns (D25–D26), research (D27–D31), then assets (A1–A11), then assembly + verification.

| # | Task | Output | Effort |
|---|---|---|---|
| T0 | Publish this catalog to `clones/project-marketing/docs/header-footer-decomposition.md` so future sessions can resume | new markdown file | 2 min |
| T1–T11 | Author 11 `DESIGN-TOKEN-*` drafts (D1–D11) under `.claude/drafts-outbound/leapfrog-2030/tokens/` | 11 new draft files; each carries the full frontmatter + responsive / i18n / a11y notes | ~15 min each |
| T12–T14 | Author 3 `DESIGN-LAYOUT-*` drafts (D12–D14) under `.../layouts/` | 3 new draft files | ~20 min each |
| T15–T24 | Author 10 `DESIGN-COMPONENT-*` drafts (D15–D24) under `.../components/` | 10 new draft files | ~20 min each |
| T25–T26 | Author 2 `DESIGN-PATTERN-*` drafts (D25–D26) under `.../patterns/` | 2 new draft files | ~25 min each |
| T27–T31 | Author 5 `DESIGN-RESEARCH-*` drafts (D27–D31) under `.../research/` | 5 new draft files | ~25 min each |
| T32–T42 | Author 11 `ASSET-WOODFINE-*` drafts (A1–A11) under `.../assets/` | 11 new draft files | ~15 min each |
| T43 | Update `.../leapfrog-2030/INDEX.md` to enumerate all 39 new drafts (and reconcile with the 8 already there) | 1 file edit | 10 min |
| T44 | Update `clones/project-marketing/docs/ia-component-map.md` to reference the catalog atoms | 1 file edit | 5 min |
| T45 | Reconcile existing drafts (token-woodfine-brand-color / breakpoints / typography from yesterday) with the new catalog — either fold into the new structure or supersede | up to 3 file edits | 20 min |
| T46 | Update `clones/project-marketing/docs/website-congruence-plan.md` with a "PIVOTED — see header-footer-decomposition.md" status banner | 1 file edit | 5 min |

**Total catalog effort:** roughly 12–14 h to author all 39 drafts at the proposed quality level (each draft is a complete recipe with responsive / multi-browser / i18n / a11y / neurodivergence specs). Auto mode runs unattended.

### Summary (what you're approving)

You are approving:

1. **Pivot direction** — replace page-by-page chrome work with a top-down catalog of 28 `DESIGN-*` + 11 `ASSET-*` drafts that decompose the home.woodfinegroup.com header and footer into a deterministic rebuild substrate.
2. **Visual fidelity** — preserve the existing graphic design (logo-centre topnav, paper-2 canvas, paper-3 footer band, Vancouver \| New York cities, etc.). No redesign.
3. **Cross-cutting depth** — every `DESIGN-*` draft carries responsive, multi-browser, multi-lingual, and accessibility-plus-neurodivergence specs.
4. **Publication** — the catalog publishes to `clones/project-marketing/docs/header-footer-decomposition.md` immediately after approval so it survives session boundaries.
5. **Auto execution** — once approved I run on auto, authoring all 39 drafts in catalog order (tokens → layouts → components → patterns → research → assets), then update the INDEX + IA map + website-congruence-plan. I'll surface the result for your review when complete.
6. **Background-parking** — the existing Phase 4–8 sprint shape (Newsroom server, L.I./BIM tool shells, Spanish translations, the Phase 2.5 / 3.5 / 5 corrective sub-phases) remains in this plan as background context but is **not active** until the catalog ships.

### What this unlocks (post-catalog)

Once the catalog ships, the rebuild work on `corporate.woodfinegroup.com` and `projects.woodfinegroup.com` becomes mechanical: each Rust binary's chrome template just composes from the catalog's recipes (or imports the emitted CSS once a `tokens.css` route is wired). Same chrome, three properties, zero drift.

The page-level work (disclaimer + contact regression fix, ↗ glyph removal) collapses into a one-shot "regenerate from catalog" step rather than spot-edits — and the inline-tokens revert (Phase 3.5) becomes unnecessary because the chrome rebuild reads directly from the catalog.

---

# Background — parked sprint (resumes after catalog)

> Everything below this line was the active plan up to 2026-05-09 mid-session. It is preserved for context but **not active** until the header/footer decomposition catalog ships.

---

## Original status note (now superseded)
> **Status — 2026-05-09 (this session):**
> Phases 0–3 shipped. Operator review surfaced three regressions /
> gaps requiring corrective sub-phases (2.5 — drop ↗ glyph, 3.5 —
> revert Phase 3 deployment changes, plus a brand-new Phase 5
> Corporate before the previously-planned Newsroom Phase 4).
> Phase 4 (Newsroom) and what was deferred Phase 5 (L.I./BIM)
> renumber to 6 and 7 respectively.

> **Operator corrections recorded 2026-05-09 (this re-entry into plan mode):**
>
> 1. **Remove the ↗ glyph everywhere.** Operator initially confirmed
>    it then changed direction. Drop the `::after` rule from
>    `templates/shell.css` and from the inline CSS in
>    `disclaimer.html` and `contact.html`; remove from the
>    `component-marketing-topnav` recipe; remove from the
>    `ia-component-map.md` doc and the cluster `website-congruence-plan.md`.
>
> 2. **Disclaimer + Contact pages lost their formatting.** Phase 3
>    introduced `<link rel="stylesheet" href="/tokens.css">` but the
>    Rust binary serving the site (`app-mediakit-marketing/src/server.rs`)
>    routes only `/`, `/page/{slug}`, `/wp-admin/*`, `/healthz` — no
>    `/tokens.css` route. The `<link>` 404s in production, every CSS
>    variable becomes undefined, and both pages render unstyled. Fix
>    by **reverting the Phase 3 deployment changes** — restore the
>    inline `:root { … }` token block in both pages and drop the
>    `<link>` line. The build/ pipeline (DTCG source +
>    Style Dictionary emitter) stays in the cluster as a forward-
>    looking artefact; it activates once the Rust binary gains a
>    `/tokens.css` route (Phase 6 work). The deployed disclaimer
>    text is the compliance-vetted version with "direct-hold
>    solutions" terminology + jurisdiction table — **must remain
>    verbatim** (operator emphasised this).
>
> 3. **Corporate page has no marketing-shell chrome.** Investigation
>    (this session) found `corporate.woodfinegroup.com` is served by
>    `app-mediakit-knowledge` (a different Rust binary at
>    `pointsav-monorepo/app-mediakit-knowledge/`); it renders the
>    Markdown content at `customer/content-wiki-corporate/*.md`
>    (index + 5 topics). Its `home_chrome()` and topic-page chrome
>    emit `<header class="site-header">` + `form.header-search` —
>    not the marketing-site logo-centre topnav. Operator wants the
>    same six-link header (1=logo, 2=Disclaimer, 3=Contact us,
>    4=Corporate, 5=Projects, 6=Newsroom) at the top of every
>    Corporate page. New **Phase 5** modifies `app-mediakit-knowledge`
>    to wear the marketing-shell topnav.

## Context

The user (Jennifer) opened TASK Claude on `clones/project-marketing/`,
asked for a **graphic-designer-led plan** to make the Woodfine
marketing site congruent across pages. The site currently has three
fully-styled pages (Landing, Disclaimer, Contact us) but lacks the
three top-level destination pages the wireframes call for —
Corporate, Projects, Newsroom. The plan must reconcile:

1. Header + footer **shared shell** across Corporate / Projects /
   Newsroom (same chrome on all three).
2. **New-tab vs same-tab** decision for the Corporate / Projects /
   Newsroom links.
3. Header + footer **parity** between the inner pages
   (Disclaimer, Contact us) and the new top-level pages.
4. **Carve-out** for Location Intelligence and BIM Tokens — the user
   suspects those sub-products need a different shell, not the
   marketing-site chrome.
5. **Woodfine first** — PointSav is parked.

This is a design + IA plan, not a construction order. No file edits
are made until the user approves.

---

## Source-of-truth wireframes (parsed)

**Wireframe sketches V2 — Ian Kiprono.pdf (4 pages, digital):**

| Page | URL pattern | Header order | Body shell |
|---|---|---|---|
| Landing | `www.woodfinegroup.com` | Disclaimer · Contact us · **[Logo centre]** · Corporate · Projects · News | Hero space → Graphics (SVG) → Key highlights |
| Corporate | `www.woodfinegroup.corporate.com` (intent: `corporate.woodfinegroup.com`) | Disclaimer · Contact us · Corporate · Projects/Documentation Wiki · News + **Logo / Menu / Search** strip below | MediaWiki-style: side ToC + Article/Discussion/Read/View History tabs + Languages/Appearance panel |
| Newsroom | `www.woodfinegroup.newsroom.com` (intent: `newsroom.woodfinegroup.com`) | **[Logo left]** · Disclaimer · Contact us · Corporate · Projects/Documentation Wiki · News | Sidebar ToC (Current/Media Release/Regulatory/Archives/Future Designs) + Date/Month filter + News list |
| PointSav Documentation | `www.pointsav.documentation.com` | Same as Corporate, plus **Download** button | Same as Corporate |

**www.woodfinegroup.com hand sketches.pdf (4 pages, paper):**

Same four pages, hand-drawn, with explicit design notes:

- **Theme:** Blue `#0FA17B` (likely transcription error — Woodfine canonical is `#164679`), white Roboto, other-section `#FFFFFF`.
- **Common CSS / defined structure** — the operator already wants a shared shell.
- **Footer:** "Included in other deployments?" / "Footer from homepage?" — operator's instinct is one footer everywhere.
- **Numbering question raised in the sketch itself:** "Maybe we use common order of numbering ie Disclaimer being 7 instead of 16. Move logo to point a instead of point b. Case in-point → Arch Linux homepage." → The operator wants ONE component-numbering scheme reused across all three pages, with the **logo as point #1** (Arch Linux / Wikipedia pattern).
- **Same-domain question:** "Hosted separately?" / "together due to fee?" / **"Same domain"** (crossed out) — operator concluded subdomains, separate hosts.
- **CMS choices vary by page:** WordPress (landing) / MediaWiki (Corporate, Documentation) / FreshRSS → WordPress later (Newsroom).

---

## Current site audit

**Files actually deployed at `/srv/foundry/deployments/media-marketing-landing-1/content/`:**

| File | Role | Header | Footer | Issues |
|---|---|---|---|---|
| `index.html` | Landing (bundled — 180-line bundler shell wrapping a JSON manifest of the rendered DOM) | Cannot inspect without unbundling | — | Bundled artefact — source-of-truth is the manifest inside, not the HTML itself |
| `disclaimer.html` | Inner page — fully responsive, 4 breakpoints | `.topnav` grid: **Disclaimer · Contact us** \| **[wordmark centre]** \| **Corporate · Projects · Newsroom** | `.footer`: *Vancouver \| New York* + Contact us / Disclaimer | Newsroom link points to `https://woodfinegroup.com/` (root) — should be `newsroom.woodfinegroup.com`; Corporate/Projects/Newsroom open with `target="_blank"` (new tab) |
| `contact.html` | Inner page — same shell as disclaimer | Same | Same | Same Newsroom-link bug |

**Existing chrome (from `disclaimer.html` 691–778):**

```
.page (max-width 1440 + paper-2 #F7F9FA)
├── header.topnav  (grid 1fr-auto-1fr)
│   ├── nav.left   → Disclaimer · Contact us  (small caps, ink-3 grey)
│   ├── a.wordmark → SVG logo (institutional-fill #111827)
│   └── nav.right  → Corporate · Projects · Newsroom (wf-blue, target=_blank)
├── div.page-hero  → centred H1 in display font, all-caps
├── main.subpage-main → page content
└── footer.footer  → cities (left) + footnav (right)
```

This shell is solid and matches the wireframes' intent — it just
needs to be applied to **three more pages** (Corporate, Projects,
Newsroom) and the same-tab/new-tab + Newsroom-link decisions need
to land.

---

## Where Location Intelligence + BIM Tokens live

- **Location Intelligence** — referenced in `pointsav-monorepo/USER_GUIDE_2026-03-30_V2.md`; this is the GIS app surface, not a static marketing page.
- **BIM Tokens** — `customer/woodfine-design-bim/tokens/bim/*.dtcg.json` (DTCG design tokens for BIM concepts: assemblies, relationships, systems, performance, identity-codes). This is a design-system data product, not a marketing page.

Both are **operational sub-products** with their own technology
substrate. They should not share marketing-site chrome. (Decision
captured in the Recommendation section.)

---

## Three design decisions confirmed by operator (2026-05-09)

1. **Header layout — logo-centre everywhere.** ✓ Confirmed.
2. **New tab + visible ↗ glyph for Corporate/Projects/Newsroom.** ✓ Confirmed.
3. **Separate shell for L.I. + BIM Tokens, surfaced as product cards under Projects.** ✓ Confirmed in principle, but **deferred** — see "Phase 4 deferred" below. **No dark mode.**

The detailed reasoning behind each is preserved below for the
record. Two further decisions (footer scope, IA numbering map) carry
my recommendation as the working assumption — they are operational
defaults, not open questions, but operator can override at any time.

### Production reality check (2026-05-09)

- `https://projects.woodfinegroup.com/` — **live and functioning**.
- `https://corporate.woodfinegroup.com/` — **live and functioning**.
- `https://newsroom.woodfinegroup.com/` — status unconfirmed; assume **not yet provisioned** until verified.

This collapses Phase 2 substantially: Corporate and Projects do not
need placeholder pages on the landing host. The header links point
straight at the live subdomains. Only **Newsroom** needs a treatment
in this sprint, and only if its subdomain is not yet live.

## Five design decisions needed (and my recommendation)

### Decision 1 — Header pattern: one or two layouts?

The wireframes show two visually distinct top-bars:

- **Landing:** Disclaimer · Contact us · **[Logo centre]** · Corporate · Projects · News  (logo as centre anchor, links flank it)
- **Inner pages (Corporate / Newsroom / Documentation):** **[Logo left]** · Disclaimer · Contact us · Corporate · Projects · News  (logo demoted to left, full menu inline)

Operator's hand-sketch question explicitly asks: **"Move logo to point
a instead of point b. Case in-point → Arch Linux homepage."** This
asks whether the inner-page pattern (logo-left) should be used
everywhere — including the landing page.

**Recommendation:** Use **one header on all five pages** —
**logo-centre, links-flanking**, the pattern already implemented in
`disclaimer.html` / `contact.html`. Reasons:

1. The current built pages already use logo-centre and they look
   institutional and editorial — fits the Bloomberg-article tone
   `CLAUDE.md` mandates.
2. Reusing one shell means one CSS bundle, one breakpoint sheet, one
   place to fix bugs.
3. Logo-centre is a stronger brand statement on a corporate-LP site
   than a left-anchored utility wordmark; the Arch Linux precedent
   the operator cited works for distros, not for capital-projects
   firms.
4. "Common order of numbering" works fine with logo-centre — we
   simply assign Logo = 1 in the unified IA map.

If the operator strongly prefers logo-left on inner pages (the
Wikipedia/Arch convention), a one-line CSS swap can enforce that —
but I do not recommend it.

### Decision 2 — New tab vs same tab for Corporate / Projects / Newsroom?

Currently: `target="_blank" rel="noopener"` (new tab). This makes
sense **only if** those three live on different subdomains and
function as logically separate properties (different CMS, different
auth, different content cadence). Per the wireframes they do exactly
that: Corporate = MediaWiki, Newsroom = FreshRSS, Projects = MediaWiki
documentation. Different backends, different update cadences,
different mental contexts.

**Recommendation:** **Keep `target="_blank"` for Corporate, Projects,
Newsroom.** Reasons:

1. They are subdomain-isolated CMSes, not landing-page sections.
   Tab-spawning preserves the visitor's place on `woodfinegroup.com`.
2. Investor visitors typically want to scan disclosure and
   regulatory content while keeping the marketing context open.
3. New-tab is the standard pattern for "external-but-related"
   properties (Bloomberg Terminal → Bloomberg.com pages do this).
4. **However:** Disclaimer ↔ Contact us links should be **same-tab**
   (they currently are) — those are inner pages of the same property.

Add `aria-label="opens in new tab"` and a small visual indicator
(↗) on the right-side links to set the right expectation.

### Decision 3 — Footer scope: one or two?

The hand sketch asks "Footer from homepage?" (operator's instinct is
yes). The current `disclaimer.html`/`contact.html` footer is *Vancouver
| New York* + nav + copyright — appropriate everywhere.

**Recommendation:** **Same footer on Landing + Disclaimer + Contact +
Corporate + Projects + Newsroom**. The footer is the disclosure
anchor (cities, contact, disclaimer link, copyright, BCSC posture
language) — it should never disappear on an investor-facing page.

### Decision 4 — Carve-out for Location Intelligence + BIM Tokens?

Operator's instinct (correct in my view): **these are sub-products,
not marketing pages.** They have:

- Their own technology substrate (GIS app for L.I.; DTCG token JSON for BIM).
- Their own audiences (technical/design users, not LPs).
- Their own update cadence (engineering velocity, not quarterly disclosure).

**Recommendation:** **Do not put them in the marketing shell at all.**
Instead:

- Surface them under **"Projects"** (subdomain `projects.woodfinegroup.com`) as **product cards** with a **"Launch tool ↗"** button that opens the standalone app/site in a new tab.
- The L.I. and BIM tools then live on their own subdomains
  (e.g. `gis.woodfinegroup.com`, `bim.woodfinegroup.com`) with
  **product-app chrome** — minimal header (back-arrow to
  woodfinegroup.com + tool name + auth controls), no marketing
  footer. They are tools, not pages.
- This matches the wireframes' "Hosted separately?" question.

### Decision 5 — Numbering / IA map (graphic-designer deliverable)

Resolve the wireframe's numbering inconsistency (Landing 1–6,
Corporate 7–19, Newsroom 20–27, with Disclaimer = 1 on Landing but
16 on Corporate). The operator already flagged this in the hand
sketch. Propose a single IA component map:

| # | Component | Lives on |
|---|---|---|
| 1 | Logo / wordmark | All pages (header centre) |
| 2 | Disclaimer link | All pages (header left) |
| 3 | Contact us link | All pages (header left) |
| 4 | Corporate link | All pages (header right, target=_blank) |
| 5 | Projects link | All pages (header right, target=_blank) |
| 6 | Newsroom link | All pages (header right, target=_blank) |
| 7 | Page hero (H1 band) | Inner pages only (Disclaimer, Contact, Corporate, Projects, Newsroom) |
| 8 | Side ToC | Corporate + Newsroom + Projects |
| 9 | Topic / Article / Discussion / Read / View History tabs | Corporate + Projects |
| 10 | Date/Month filter | Newsroom |
| 11 | Languages / Appearance panel | Corporate + Projects |
| 12 | Search | Corporate + Projects |
| 13 | Log in (admin only) | Corporate + Projects |
| 14 | Cities + footnav + copyright (footer) | All pages |

This map is the artefact the next person (or session) builds from.

---

## Recommended approach (the design plan)

### Phase 1 — Shell extraction + design-draft reconciliation (do first)

Two parallel tracks:

**1a. Extract the shell.**
- Pull header / footer / responsive CSS from `disclaimer.html` into
  `clones/project-marketing/templates/_shell-header.html` +
  `_shell-footer.html` + `shell.css`. Source-of-truth lives in the
  cluster, not the deployment.
- Codify the IA component map (table earlier in this plan) at
  `clones/project-marketing/docs/ia-component-map.md`.

**1b. Reconcile yesterday's design drafts (commit `22abc8c`) with
the as-built site.** Audit found four mismatches:

| Draft | Issue | Resolution |
|---|---|---|
| `token-woodfine-brand-color` | none | leave as-is ✓ |
| `token-woodfine-breakpoints` | Drafted Carbon scale 320/672/1056/1312/1584; built uses 1200/1024/768/480 | **Iterate the draft in place** — replace with the built breakpoints; document the divergence-from-Carbon rationale in the `notes_for_designer:` field |
| `token-woodfine-typography` | Drafted system-font stack; built uses Oswald + Roboto Slab + Nunito Sans + Barlow Condensed (Google Fonts brand stack) | **Iterate the draft in place** — replace the font tokens with the actual Google Fonts brand stack and the 3-font system (display / body-slab / UI-sans) |
| `component-marketing-topnav` | Drafted logo-left + hamburger + 3 placeholder links; built is logo-centre + 6 specific links, no hamburger | **Iterate the draft in place** — rewrite the recipe to match the built `.topnav` (logo-centre, 6 links, target=_blank on right-side, ↗ glyph) |

`master_cosign: required` stays on the token drafts. The drafts can
be edited in this cluster freely; promotion-to-canonical at
`pointsav-design-system` still requires Master co-sign — that
happens later via project-design.

**1c. Author the missing drafts** (newly identified in this audit):

| New draft | Type | Captures |
|---|---|---|
| `component-marketing-footer.draft.md` | DESIGN-COMPONENT | The *Vancouver \| New York* + footnav + copyright band currently in `disclaimer.html` lines 773–778 |
| `component-marketing-page-hero.draft.md` | DESIGN-COMPONENT | The centred H1 band on inner pages (`disclaimer.html` lines 717–719) |
| `asset-woodfine-wordmark-svg.draft.md` | ASSET | Extract the Woodfine wordmark SVG (`disclaimer.html` lines 698–708) and propose writing it to `woodfine-media-assets/icons/woodfine-wordmark.svg`. This is the Woodfine-side companion to yesterday's PointSav `asset-favicon-ps-badge-svg-2026-05-08` |

These four reconciliation actions plus three new drafts replace the
old "Phase 1 = pure shell extraction" with a richer Phase 1 that
keeps the design-system substrate honest about what's actually
deployed.

**Out of scope for Phase 1:** building the token→CSS pipeline.
The marketing pages will continue to use inline `:root` CSS custom
properties; the tokens-to-CSS emit is a separate engineering track.
Drafts capture *intent* — the build still uses inline values until
the pipeline is constructed.

### Phase 2 — Disclaimer + Contact link audit

Two small fixes (lowest risk, highest signal):

1. **Newsroom link bug** — change `href="https://woodfinegroup.com/"`
   to `href="https://newsroom.woodfinegroup.com/"` in both
   `disclaimer.html` and `contact.html`.
2. Add a small `↗` glyph after each right-nav link via CSS
   `::after` to signal "opens new tab" without bloating the markup.

### Phase 3 — Token emission pipeline (operator addition 2026-05-09)

**Goal:** stop consuming raw hex/px values inline; have marketing
pages consume tokens from a single emitted CSS file.

**Why now:** Phase 4 (Newsroom server) and any future page added to
the Woodfine surface should consume tokens out of the box. Building
the pipeline before Phase 4 means the new Newsroom HTML is
token-driven from day one, not retrofitted.

**Pipeline shape:**

```
clones/project-marketing/.claude/drafts-outbound/leapfrog-2030/tokens/*.draft.md
    │ (refined by project-design via cluster-design-draft-pipeline.md)
    ▼
pointsav-design-system/tokens/dtcg-bundle.json   ← canonical source
    │ (style-dictionary or equivalent emitter)
    ▼
pointsav-design-system/exports/
    ├── tokens.css           ← :root { --wf-blue-primary: #164679; ... }
    ├── tokens.scss
    ├── tokens.tailwind.js
    └── tokens.figma.json
    │
    ▼ (deployed to landing host alongside content)
deployments/media-marketing-landing-1/content/tokens.css
    │ (consumed by all .html via <link rel="stylesheet" href="/tokens.css">)
```

**Tasks in this phase:**

1. Pick the emitter: recommend **Style Dictionary** (Amazon, the de
   facto standard for DTCG JSON → CSS). Open source, MIT licensed,
   no service dependency.
2. Author a `pointsav-design-system/build/style-dictionary.config.js`
   that consumes the DTCG bundle and emits the four export files.
3. Wire the emit step into the design-system build (an `npm run
   build` or equivalent invocation) so token changes regenerate
   exports atomically.
4. Update the existing marketing HTML (`disclaimer.html`,
   `contact.html`, plus any new Phase-4 pages) to:
   - Drop inline `:root { --wf-blue: #164679; ... }` blocks.
   - Add `<link rel="stylesheet" href="/tokens.css">` in `<head>`.
   - Reference token names (`var(--wf-color-brand-primary)`) instead
     of raw hex.
5. Stage a deployment-side copy step so `tokens.css` lands in
   `media-marketing-landing-1/content/` whenever the design-system
   exports refresh.

**Gating:** the token-source-of-truth lives in
pointsav-design-system. Promotion of the token drafts (commit
`22abc8c` plus any iterations from Phase 1b) into canonical
`dtcg-bundle.json` requires **Master co-sign** per
`master_cosign: required` in the draft frontmatter. Until that
co-sign happens, this phase produces a *staging* `tokens.css`
emitted from the cluster's draft files (not the canonical bundle)
so the marketing pages can move to token-driven without waiting
on co-sign. Swap the source once tokens ratify.

**Effort:** ~4–6 hours for an experienced design-systems
engineer; depends mostly on Master co-sign turnaround.

**Risks:**
- Co-sign turnaround unknown — keep the staging-emit fallback
  ready so this phase doesn't block on it.
- Style Dictionary's default DTCG transform may need a custom
  format for the Carbon-divergence rationale fields (purely
  cosmetic — descriptive text, no impact on emitted CSS).
- Existing inline CSS in `disclaimer.html` / `contact.html` is
  ~500 lines per file; mechanical replacement only.

### Phase 4 — Newsroom server (operator direction 2026-05-09)

**Reference template:**
`/srv/foundry/vendor/pointsav.github.io/index.html` (529 lines).

**Why this template fits:**

The pointsav.github.io page is already a single-document editorial
shell with everything a Newsroom needs:

| Feature | Where in the template |
|---|---|
| `NewsArticle` Schema.org JSON-LD | Lines 18–38 |
| Sticky header bar with backdrop blur | `.header-bar` |
| Wordmark + desktop-actions slot | `.nav-container` |
| Content + sidebar grid (`1fr / 300px`) | `.main-grid` |
| Document canvas with editorial typography | `.document-canvas` |
| Sticky right-rail "ledger" sidebar (mono font) | `.sidebar-ledger` |
| Hook title / executive abstract / takeaways box | dedicated linguistic-token blocks |
| Ledger divider + manuscript title | for press-release sections |
| Legal-ledger footer block | for disclosure lines |
| Light-only colour scheme | `meta name="color-scheme" content="light only"` (matches operator's "no dark mode") |
| Mobile-first responsive | `viewport-fit=cover`, viewport-safe-area envs |
| Self-contained (no external CSS/JS) | one file, drop-in |

The only re-skin work needed is **token swap**:

| PointSav token (template) | Woodfine target |
|---|---|
| `--ps-canvas: #F9FAFB` | `--wf-canvas: #F7F9FA` |
| `--ps-card: #FFFFFF` | unchanged (`--wf-paper: #FFFFFF`) |
| `--ps-text: #09090B` | unchanged or `--wf-ink: #111827` to match marketing pages |
| `--ps-accent: #869FB9` (PointSav slate) | `--wf-blue-primary: #164679` (Woodfine institutional blue) |
| `--ps-muted: #6B7280` | unchanged (`--wf-ink-3: #6B7280`) |
| `--ps-border: #E5E7EB` | unchanged |
| Wordmark text "POINTSAV" | replaced with the Woodfine wordmark SVG (Phase 1c asset) |
| `theme-color: #F9FAFB` | `theme-color: #F7F9FA` |

**Server architecture (operator direction 2026-05-09):**

Two-binary split, both Rust, both sovereign — a leapfrog-2030
RUST equivalent of the FreshRSS + WordPress combo from the wireframe:

```
                    ┌─────────────────────────────────────┐
                    │  External press-release sources     │
                    │  (.md files, partner RSS feeds, …)  │
                    └─────────────┬───────────────────────┘
                                  │
                                  ▼
              ┌──────────────────────────────────────┐
              │  service-rss  (NEW Rust binary)      │
              │  — RUST equivalent of FreshRSS       │
              │  — aggregates inbound feeds          │
              │  — produces canonical RSS/Atom       │
              │  — serves /feed.xml                  │
              └─────────────┬────────────────────────┘
                            │ canonical RSS/Atom
                            ▼
       ┌────────────────────────────────────────────────┐
       │  app-mediakit-marketing  (existing scaffold)   │
       │  — reader: consumes service-rss feed           │
       │  — renders via reskinned pointsav.github.io    │
       │    editorial template + shared topnav/footer   │
       │  — exposes /newsroom and /newsroom/<slug>      │
       └─────────────┬──────────────────────────────────┘
                     │
       ┌─────────────┴──────────────┐
       ▼                            ▼
deployments/                  deployments/
media-marketing-landing-1     media-marketing-landing-2
(Woodfine — newsroom.         (PointSav — newsroom.
 woodfinegroup.com)            pointsav.com or equivalent)
```

**Naming, per operator:**

- **`service-rss`** = the SERVER (RSS aggregator/producer; new
  folder under `pointsav-monorepo/`).
- **`app-mediakit-marketing`** = the READER (existing folder; the
  Newsroom website-as-RSS-reader).

**Folder-creation permissions:**

`pointsav-monorepo/service-rss/` does not yet exist. Per
`CLAUDE.md` §11, creating new top-level folders inside a vendor
canonical repo requires **Master permission**. Phase 4 begins with
an outbox message to Master requesting:

1. Permission to scaffold `pointsav-monorepo/service-rss/` as a new
   Rust crate (Cargo workspace member).
2. Confirmation of the RUST RSS aggregator approach (so Master can
   surface any conflicts with the broader stack).
3. DNS + Doorman provisioning for `newsroom.woodfinegroup.com` and
   the equivalent PointSav newsroom subdomain.

Until Master approves, Phase 4 holds at the template-reskin step
(no new folders created, no commits to pointsav-monorepo).

**Why this split is the right call:**

1. **Source/reader separation.** Aggregator and presenter have
   different release cadences, different operational risks
   (aggregator pulls untrusted RSS; reader doesn't), and benefit
   from different test surfaces.
2. **Multi-tenancy clean.** Two deployments
   (`media-marketing-landing-1` Woodfine + `media-marketing-landing-2`
   PointSav) consume the same `service-rss` feed structure with
   tenant-specific theming via `SERVICE_MARKETING_MODULE_ID`.
3. **Sovereign per leapfrog-2030.** No PHP, no MariaDB, no
   FreshRSS-PHP install — all-Rust, Tier-0 compatible.
4. **Operator-directed.** This is the architecture chosen
   2026-05-09; not my recommendation, the operator's call.

**Tasks:**

**Phase 4a — Outbox to Master (do first, blocks everything else in Phase 4):**

1. Compose an outbox message from `task@project-marketing` to
   `master@claude-code` requesting:
   - Permission to scaffold `pointsav-monorepo/service-rss/` as a
     new Cargo workspace member.
   - Confirmation of the RUST-aggregator + reader split for the
     Newsroom architecture.
   - DNS + Doorman provisioning for `newsroom.woodfinegroup.com`
     (Woodfine deployment 1) and the PointSav newsroom subdomain
     (deployment 2).
2. Hold all subsequent Phase 4 work until Master responds.

**Phase 4b — Template reskin (does not need Master; can run in parallel with 4a):**

3. Reskin `vendor/pointsav.github.io/index.html`:
   - Save as `clones/project-marketing/templates/newsroom-page.html`.
   - Token-swap (PointSav → Woodfine via Phase 3 emitted tokens.css).
   - Replace the wordmark text with the Woodfine wordmark SVG
     (Phase 1c asset).
   - **Replace `.header-bar` with the shared `topnav`** from Phase 1
     (operator confirmed 2026-05-09). Newsroom wears the same
     logo-centre chrome as Disclaimer / Contact / Corporate / Projects.
   - Replace the bottom of `.document-footer` with the shared
     `footer` from Phase 1.
4. Author a tenant-variant template stub for PointSav at
   `clones/project-marketing/templates/newsroom-page-pointsav.html`
   (token-swap only — this is the deployment-2 variant).

**Phase 4c — Server scaffold (gated on Master approval from 4a):**

5. Scaffold `pointsav-monorepo/service-rss/`:
   - Cargo workspace member, axum-based.
   - Inbound: ingests press-release Markdown files from a
     configured directory (per-tenant via env var) and external
     partner RSS feeds (later — not in v0.0.1).
   - Outbound: serves `/feed.xml` (Atom 1.0 / RSS 2.0).
   - Storage: flat-file (no DB in v0.0.1) — Tier-0 compatible.
6. Add Newsroom routes to `pointsav-monorepo/app-mediakit-marketing/`:
   - `/newsroom` — index listing recent press releases (consumed
     from `service-rss /feed.xml`).
   - `/newsroom/<slug>` — single-release page rendered using the
     reskinned template from Phase 4b.
   - Sidebar-ledger surfaces date / source / version metadata.

**Phase 4d — Deployment + content:**

7. Create `deployments/media-marketing-landing-1/content/newsroom/`
   with two seed Markdown press releases (one operational
   "Newsroom launches" release; one disclosure placeholder for
   shape-checking the template).
8. Same for `deployments/media-marketing-landing-2/content/newsroom/`
   (PointSav tenant).
9. Coordinate with Master on subdomain DNS + Doorman routing once
   `service-rss` and the new `app-mediakit-marketing` routes pass
   `cargo check` and basic local smoke tests.

**Effort:** ~12–18 h total — re-skin is ~2 h; service-rss scaffold
is ~6–8 h; app-mediakit-marketing route work is ~4–6 h; deployment
plumbing ~2 h. Master coordination is the critical-path bottleneck.

**Verification:**

- Local smoke: `cargo run -p service-rss` serves `/feed.xml` with
  the seed Markdown; `cargo run -p app-mediakit-marketing` consumes
  it and serves `/newsroom` + `/newsroom/<slug>`.
- Browser smoke: open `http://localhost:<port>/newsroom` in browser,
  confirm shared topnav (logo-centre, 6 links) at top, editorial
  layout below, shared footer at bottom; click a release headline,
  confirm `/newsroom/<slug>` renders the full release with token-
  driven Woodfine colours.
- Production smoke (post-DNS): `https://newsroom.woodfinegroup.com`
  resolves; `target="_blank"` from `disclaimer.html` → Newsroom
  opens cleanly; light-mode only (no dark-mode toggle anywhere).

### Phase 5 — DEFERRED (Location Intelligence + BIM Tokens shells)

**Parked as a future TODO.** Out of scope for this sprint, queued
*after* Phase 4 lands.

When the operator picks this up later:

- The carve-out treatment will be a **light** utility shell, not
  dark mode. Operator preference recorded 2026-05-09: **no dark
  mode anywhere on Woodfine properties.**
- Likely shape: minimal top bar (`← woodfinegroup.com / [Tool name]`),
  same paper background as the marketing site, monospace UI font
  to signal "tool" without changing colour mode, shared
  `--wf-blue-primary` accent.
- Surfacing under `projects.woodfinegroup.com` as product cards
  with "Launch tool ↗" buttons remains the recommended pattern.

### Phase 6 — DEFERRED (Spanish translation)

Per `CLAUDE.md` §6 the investor-facing pages need Spanish
counterparts. Track as a separate task; not in this sprint.

---

## Plan-as-deliverable

This plan file currently lives at
`/home/jennifer/.claude/plans/tender-percolating-raccoon.md` (an
ephemeral location used only while plan mode is active). The
operator has asked that **the plan itself become a markdown file
checked into the cluster**.

**Proposed destination (when plan exits plan-mode and execution begins):**

`clones/project-marketing/docs/website-congruence-plan.md`

That path:

- Sits inside the Totebox archive that owns this work, so a future
  Task session opens it on session-start.
- Lives under `docs/` (the right home for a working-document plan,
  per `CLAUDE.md` §13 root-files-discipline).
- Persists across sessions and survives plan-mode exit.

A single workspace commit (staging-tier via `bin/commit-as-next.sh`)
adds the file alongside any Phase 1 work.

## Critical files to read or modify

- **Read:** `/srv/foundry/deployments/media-marketing-landing-1/content/disclaimer.html` — current shell reference
- **Read:** `/srv/foundry/deployments/media-marketing-landing-1/content/contact.html` — same shell, contact variant
- **Read:** `/home/jennifer/sandbox/inputs/project-marketing/website/Wireframe sketches V2- Ian Kiprono.pdf` — digital wireframes
- **Read:** `/home/jennifer/sandbox/inputs/project-marketing/website/www.woodfinegroup.com hand sketches.pdf` — operator's hand sketches with design questions
- **Modify (Phase 1a):** create `clones/project-marketing/docs/ia-component-map.md`, `clones/project-marketing/templates/_shell-header.html`, `_shell-footer.html`, `shell.css` (new files)
- **Modify (Phase 1b):** edit existing drafts in `clones/project-marketing/.claude/drafts-outbound/leapfrog-2030/`:
  - `tokens/token-woodfine-breakpoints.draft.md` (replace Carbon scale → built scale)
  - `tokens/token-woodfine-typography.draft.md` (replace system stack → Google Fonts brand stack)
  - `components/component-marketing-topnav.draft.md` (rewrite recipe to match built logo-centre header)
- **Modify (Phase 1c):** author new drafts:
  - `clones/project-marketing/.claude/drafts-outbound/leapfrog-2030/components/component-marketing-footer.draft.md`
  - `clones/project-marketing/.claude/drafts-outbound/leapfrog-2030/components/component-marketing-page-hero.draft.md`
  - `clones/project-marketing/.claude/drafts-outbound/leapfrog-2030/assets/asset-woodfine-wordmark-svg.draft.md`
  - Update `clones/project-marketing/.claude/drafts-outbound/leapfrog-2030/INDEX.md` to add the new entries
- **Modify (Phase 2):** two single-line edits to `disclaimer.html` + `contact.html` (Newsroom href fix + `↗` glyph CSS).
- **Modify (Phase 3 — token emission):**
  - New: `pointsav-design-system/build/style-dictionary.config.js`
  - New: `pointsav-design-system/exports/tokens.css` (emitted)
  - New: `deployments/media-marketing-landing-1/content/tokens.css` (deployed copy)
  - Edit: `deployments/media-marketing-landing-1/content/disclaimer.html` and `contact.html` — drop inline `:root` block, add `<link rel="stylesheet" href="/tokens.css">`, replace hex with `var(--…)` references.
- **Modify (Phase 4 — Newsroom server):**
  - Read template: `/srv/foundry/vendor/pointsav.github.io/index.html`
  - New: `clones/project-marketing/templates/newsroom-page.html` (re-skinned)
  - Edit (in `pointsav-monorepo/app-mediakit-marketing/`): add `/newsroom` and `/newsroom/<slug>` routes, Markdown rendering pipeline.
  - New: `deployments/media-marketing-landing-1/content/newsroom/` directory with seed `*.md` files.
  - Coordination: outbox to Master/ops to provision `newsroom.woodfinegroup.com` DNS + Doorman routing.
- **Phase 5:** deferred — no files to modify this sprint.
- **Phase 6:** deferred — no files to modify this sprint.

---

## Verification

For each phase:

- **Phase 1:** open `disclaimer.html` and the new template fragment side-by-side; the rendered header + footer must be byte-identical.
- **Phase 2:** open all five inner pages at viewport widths 1440 / 1200 / 1024 / 768 / 480 px; header position, link order, wordmark size, and footer layout must be visually identical at every breakpoint. Verify all six top-nav links resolve (200 OK on placeholder phase, then to subdomains in production).
- **Phase 3:** click every Newsroom link from disclaimer.html and contact.html; it must open `newsroom.woodfinegroup.com` in a new tab (confirmed by the new ↗ glyph being visible).
- **Phase 4:** a product card on `projects.html` clicked launches a new tab with the tool-shell chrome (no marketing header/footer); the back-arrow returns to `woodfinegroup.com`.
- **Phase 5:** every English page has a Spanish sibling at `/es/<slug>` with the same shell.

End-to-end: visitor lands on `woodfinegroup.com`, clicks Corporate
in header → new tab opens with same logo-centre header on the
Corporate destination, same footer, same disclaimer behaviour. They
return to the original tab and click Projects → see L.I. and BIM
product cards → click "Launch tool ↗" → arrives in tool-shell with
back-arrow to woodfinegroup.com.

---

## Out of scope (explicit)

- PointSav site (parked per user direction).
- Building MediaWiki / FreshRSS instances (subdomain provisioning is
  Master + ops scope; this plan unblocks the marketing-site front
  end so those backends can be plumbed in later).
- Token-driven CSS pipeline (the existing inline `:root` CSS
  variables are fine for now; design-system tokenisation is a
  parallel track captured at commit `22abc8c`).
- Spanish translation (Phase 5 — surfaced but explicitly deferred).

---

## Sprint shape (revised 2026-05-09 — corrective sub-phases added after operator review)

| # | Phase | Status | Effort | Blockers |
|---|---|---|---|---|
| 0 | Plan committed to cluster docs/ | ✅ done | — | — |
| 1a | Shell extraction → cluster templates + IA map | ✅ done | ~2 h | none |
| 1b | Reconcile yesterday's drafts (breakpoints, typography, topnav) with as-built | ✅ done | ~3 h | none (drafts can be edited freely) |
| 1c | Author missing drafts (footer component, page-hero component, Woodfine wordmark ASSET) | ✅ done | ~2 h | none |
| 2 | Newsroom-link fix + `↗` glyph in Disclaimer/Contact | ✅ done | ~30 min | none |
| **2.5** | **Drop the ↗ glyph everywhere** — operator changed direction. Strip the `::after` rule from `templates/shell.css`, `disclaimer.html` inline CSS, `contact.html` inline CSS; remove glyph notes from `component-marketing-topnav.draft.md`, `ia-component-map.md`, `website-congruence-plan.md`. Keep the `aria-label="… (opens in new tab)"` on the right-side links — that's purely additive and useful for screen readers. | **this sprint (corrective)** | ~20 min | none |
| 3 | Token emission pipeline (Style Dictionary → tokens.css) | ✅ done (cluster artefacts); **deployment consumption reverted in 3.5** | ~4 h | none |
| **3.5** | **Revert Phase 3 deployment changes** — restore inline `:root { … }` token block in `disclaimer.html` and `contact.html`; drop the `<link rel="stylesheet" href="/tokens.css">` line. The `app-mediakit-marketing/src/server.rs` Rust binary has no `/tokens.css` route, so the stylesheet 404s and the pages render unstyled. Build/ pipeline (DTCG source + emitter) stays in the cluster as-is — it's a forward-looking artefact that activates once Phase 6 adds a `/tokens.css` route to the Rust binary. **Disclaimer text remains verbatim** — the deployed compliance-vetted version (with "direct-hold solutions" terminology + jurisdiction table) is operationally correct; do not touch. | **this sprint (corrective)** | ~15 min | none |
| **5** | **NEW — Corporate page wears the marketing topnav.** `corporate.woodfinegroup.com` is served by `app-mediakit-knowledge` (a different Rust binary at `pointsav-monorepo/app-mediakit-knowledge/`; renders `customer/content-wiki-corporate/*.md`). Modify the `home_chrome()` function and the topic-page chrome (around `server.rs` lines 766 + 1281) to emit the marketing-shell logo-centre topnav (1=logo, 2=Disclaimer, 3=Contact us, 4=Corporate, 5=Projects, 6=Newsroom) instead of the existing `<header class="site-header">` + `form.header-search`. Also wear the shared footer at the bottom. Search bar + side ToC + appearance panel can stay; they're knowledge-wiki conveniences that don't conflict with the marketing topnav. | **this sprint** | ~6–8 h | none directly; coordinate with Master before pushing the Rust change to vendor canonical |
| 6 | Newsroom: **service-rss** (NEW Rust aggregator) + **app-mediakit-marketing** (reader; reskinned pointsav.github.io editorial template + shared topnav). Two-tenant deployment (Woodfine + PointSav). When this lands, also add a `/tokens.css` static route to the Rust binary so Phase 3.5's revert can be undone. | next sprint | ~12–18 h | Master permission to scaffold `service-rss/`; DNS + Doorman provisioning for two newsroom subdomains |
| 7 | Location Intelligence + BIM tool shells (light, no dark mode) | DEFERRED | — | parked as TODO |
| 8 | Spanish translations | DEFERRED | — | parked as TODO |

(Old "Phase 3 / 4 / 5 / 6" rows below this corrected table are kept
in the body of the plan for historical context. The numbering in
this table supersedes them — execute against this table.)

**Plan-as-deliverable:** the plan was committed to
`clones/project-marketing/docs/website-congruence-plan.md` as part
of Phase 0 so it persists past plan mode and acts as the
implementation reference for subsequent sessions. Phase 2.5, 3.5
and the new Phase 5 changes will be folded back into that committed
copy.

## Verification (this sprint's corrective work)

- **Phase 2.5:** open `disclaimer.html` and `contact.html` in a
  browser at any breakpoint; the right-side nav links (Corporate /
  Projects / Newsroom) must show **no trailing arrow glyph**. The
  `aria-label="… (opens in new tab)"` attribute remains for screen-
  reader users.
- **Phase 3.5:** open `disclaimer.html` and `contact.html` in a
  browser; full styling restored — `#164679` blue accent on the
  right-side nav, paper-2 canvas, paper-3 page-hero band, full
  responsive behaviour at 1200/1024/768/480 px. Disclaimer text
  remains the compliance-vetted version with "direct-hold
  solutions" terminology, the "Woodfine" definition in parentheses,
  and the four-row jurisdiction table — every word verbatim
  against the deployed copy as of pre-Phase 3 state.
- **Phase 5:** load `corporate.woodfinegroup.com` (or run
  `app-mediakit-knowledge` locally and load
  `http://localhost:<port>/`); the topnav must render as
  Disclaimer + Contact us | [Woodfine wordmark centre] | Corporate
  + Projects + Newsroom — identical pixel-positions to
  `disclaimer.html` at every breakpoint. The shared footer
  (Vancouver | New York + Contact us / Disclaimer + copyright)
  must render at the bottom of every Corporate page (home + each
  of the five topic pages: direct-hold-framework,
  equity-transfer-model, fiduciary-data-mandate,
  interest-coverage-ratio, redemption-elimination).
